//! Fast card-play and observation API for the compiled native session.

use crate::session::{num, Session};
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path, time::Instant};

#[derive(Clone, Copy, Debug)]
pub struct PlayCardRequest {
    pub side: u32,
    pub slot: u32,
    pub x: i32,
    pub y: i32,
}

pub struct RlSession {
    pub engine: Session,
    descriptor_scratch: u64,
}

impl RlSession {
    pub fn load(checkpoint: &Path) -> Result<Self, String> {
        let engine = Session::load(checkpoint)?;
        Self::with_engine(engine)
    }

    fn with_engine(mut engine: Session) -> Result<Self, String> {
        let descriptor_scratch = engine
            .runtime
            .heap
            .alloc(24, 16, &mut engine.runtime.memory)?;
        engine
            .runtime
            .memory
            .write_bytes(descriptor_scratch, &[0; 24])?;
        Ok(Self {
            engine,
            descriptor_scratch,
        })
    }

    /// Restore the prepared checkpoint in place and recreate the descriptor
    /// allocation with the same first post-checkpoint heap identity.
    pub fn reset(&mut self) -> Result<(), String> {
        self.engine.reset()?;
        self.descriptor_scratch =
            self.engine
                .runtime
                .heap
                .alloc(24, 16, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(self.descriptor_scratch, &[0; 24])
    }

    /// Rebuild a fresh native harness after replacing the loaded Normal
    /// scenario's AlwaysSpells vector with exact source-row pointers.
    ///
    /// This follows the original configure_normal_deck_and_rebuild sequence:
    /// enumerate table 7 from the retained context, change only the scenario
    /// row's vector, then call the original battle-input and harness
    /// constructors. Both sides consume the same deck, as in the source
    /// harness. No world/game fields are patched.
    pub fn configure_normal_deck_and_rebuild(
        &mut self,
        cards: &[&str],
        seed: u32,
    ) -> Result<Value, String> {
        self.configure_normal_decks_and_rebuild(cards, cards, seed)
    }

    /// Reproduce the reference's per-side source-vector hooks, preserving all
    /// original constructor instructions, allocation order and native seed.
    pub fn configure_normal_decks_and_rebuild(
        &mut self,
        cards: &[&str],
        cards_side1: &[&str],
        seed: u32,
    ) -> Result<Value, String> {
        if cards.len() != 8 || cards_side1.len() != 8 {
            return Err(format!(
                "configured decks must each contain exactly 8 cards, got {} and {}",
                cards.len(),
                cards_side1.len()
            ));
        }
        for deck in [cards, cards_side1] {
            let mut unique = std::collections::BTreeSet::new();
            if deck
                .iter()
                .any(|card| card.is_empty() || !unique.insert(*card))
            {
                return Err("deck names must be nonempty and unique".into());
            }
        }

        let values = &self.engine.checkpoint["call_plan"]["values"];
        let context = num(&values["context"])?;
        let normal_test_row = num(&values["confirm_original_Normal"])?;
        let row_count = self.engine.call(0xe1ac8c, &[context], 0)?;
        if row_count == 0 || row_count > 100_000 {
            return Err(format!("unexpected native card-row count {row_count}"));
        }
        let mut source_rows = BTreeMap::<String, u64>::new();
        for index in 0..row_count {
            let row = self.engine.call(0xe1ad4c, &[context, index], 0)?;
            if row == 0 {
                return Err(format!("native card row {index} is null"));
            }
            let name = self.native_string(row + 0x28)?;
            if (cards.contains(&name.as_str()) || cards_side1.contains(&name.as_str()))
                && source_rows.insert(name.clone(), row).is_some()
            {
                return Err(format!("duplicate native source row name {name}"));
            }
        }
        let missing: Vec<_> = cards
            .iter()
            .chain(cards_side1)
            .filter(|name| !source_rows.contains_key(**name))
            .copied()
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "native source rows not found by exact name: {}",
                missing.join(", ")
            ));
        }
        let row_ptrs: Vec<u64> = cards.iter().map(|name| source_rows[*name]).collect();
        let vector_data = self.engine.runtime.heap.alloc(
            8 * row_ptrs.len() as u64,
            8,
            &mut self.engine.runtime.memory,
        )?;
        let mut vector_bytes = Vec::with_capacity(row_ptrs.len() * 8);
        for row in &row_ptrs {
            vector_bytes.extend_from_slice(&row.to_le_bytes());
        }
        self.engine
            .runtime
            .memory
            .write_bytes(vector_data, &vector_bytes)?;
        self.engine
            .runtime
            .memory
            .write_bytes(normal_test_row + 0x70, &vector_data.to_le_bytes())?;
        self.engine.runtime.memory.write_bytes(
            normal_test_row + 0x78,
            &(row_ptrs.len() as u32).to_le_bytes(),
        )?;

        let second_vector = if cards_side1 != cards {
            let pointers: Vec<u64> = cards_side1.iter().map(|name| source_rows[*name]).collect();
            let data = self.engine.runtime.heap.alloc(
                8 * pointers.len() as u64,
                8,
                &mut self.engine.runtime.memory,
            )?;
            self.engine.runtime.memory.write_bytes(
                data,
                &pointers
                    .iter()
                    .flat_map(|p| p.to_le_bytes())
                    .collect::<Vec<_>>(),
            )?;
            Some((data, pointers.len() as u32))
        } else {
            None
        };
        self.engine.runtime.memory.write_bytes(
            normal_test_row + 0x7c,
            &(row_ptrs.len() as u32).to_le_bytes(),
        )?;

        let name_bytes = self
            .engine
            .runtime
            .heap
            .alloc(7, 8, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(name_bytes, b"Normal\0")?;
        let name_string =
            self.engine
                .runtime
                .heap
                .alloc(0x10, 8, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(name_string, &[0; 0x10])?;
        self.engine.call(0x142fde4, &[name_string, name_bytes], 0)?;
        let resolved = self.engine.call(0xe1b7e8, &[context, name_string, 0], 0)?;
        if resolved != normal_test_row {
            return Err(format!(
                "Normal lookup changed unexpectedly: {resolved:#x} != {normal_test_row:#x}"
            ));
        }
        let input_sret = self
            .engine
            .runtime
            .heap
            .alloc(8, 8, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(input_sret, &[0; 8])?;
        self.engine.call(
            0x11d7968,
            &[context, name_string, seed as u64, 0],
            input_sret,
        )?;
        let battle_input = read_u64(&self.engine, input_sret)?;
        if battle_input == 0 {
            return Err("original battle-input constructor returned null".into());
        }
        let new_harness =
            self.engine
                .runtime
                .heap
                .alloc(0x78, 16, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(new_harness, &[0; 0x78])?;
        self.engine.runtime.deck_swap =
            second_vector.map(|(data, count)| crate::runtime::DeckSwap {
                slot: normal_test_row + 0x70,
                second_ptr: data,
                second_count: count,
                original_ptr: vector_data,
                original_count: row_ptrs.len() as u32,
                calls: 0,
            });
        let constructed = self
            .engine
            .call(0x11d58e0, &[new_harness, context, input_sret], 0);
        let swap = self.engine.runtime.deck_swap.take();
        constructed?;
        if let Some(swap) = swap {
            if swap.calls != 2 {
                return Err(format!(
                    "expected two native 11d6f18 deck builds, observed {}",
                    swap.calls
                ));
            }
        }
        let new_world = read_u64(&self.engine, new_harness + 0x20)?;
        if new_world == 0 {
            return Err("original harness constructor returned a null world".into());
        }
        let new_battle = read_u64(&self.engine, new_world + 0xa8)?;
        let new_entities = read_u64(&self.engine, new_world + 0x40)?;
        let new_commands = read_u64(&self.engine, new_world + 0x38)?;
        if new_battle == 0 || new_entities == 0 || new_commands == 0 {
            return Err(
                "fresh native harness has a null battle, entity manager, or command queue".into(),
            );
        }
        self.engine.harness = new_harness;
        self.engine.world = new_world;
        self.engine.battle = new_battle;

        let mut actual_decks = Vec::new();
        for side in 0..2u64 {
            let deck = read_u64(&self.engine, new_battle + 0x88 + side * 8)?;
            if deck == 0 {
                return Err(format!("fresh native side {side} has null deck"));
            }
            let count = read_u32(&self.engine, deck + 0x2c)?;
            if count != 8 {
                return Err(format!(
                    "fresh native side {side} deck has {count} rows, expected 8"
                ));
            }
            let entries = read_u64(&self.engine, deck + 0x20)?;
            let side_obj = self.side_object(side as u32)?;
            let mut names = Vec::new();
            for index in 0..count as u64 {
                let wrapper = read_u64(&self.engine, entries + index * 8)?;
                if wrapper == 0 {
                    return Err(format!("native side {side} deck entry {index} is null"));
                }
                self.engine
                    .runtime
                    .memory
                    .write_bytes(self.descriptor_scratch, &[0; 24])?;
                self.engine
                    .call(0xe80bbc, &[wrapper, side_obj, 1], self.descriptor_scratch)?;
                let row = self.engine.call(0xe80680, &[self.descriptor_scratch], 0)?;
                if row == 0 {
                    return Err(format!(
                        "native side {side} entry {index} has null source row"
                    ));
                }
                names.push(self.native_string(row + 0x28)?);
            }
            let mut sorted_actual = names.clone();
            let expected = if side == 0 { cards } else { cards_side1 };
            let mut sorted_expected = expected
                .iter()
                .map(|name| name.to_string())
                .collect::<Vec<_>>();
            sorted_actual.sort();
            sorted_expected.sort();
            if sorted_actual != sorted_expected {
                return Err(format!("native side {side} deck rows mismatch: {names:?}"));
            }
            actual_decks.push(json!({"side":side,"cards":names}));
        }
        let observation = self.observe()?;
        let deck_source_rows = if cards == cards_side1 {
            json!(cards)
        } else {
            json!([cards, cards_side1])
        };
        Ok(
            json!({"scenario":"Normal","seed":seed,"deck_source_rows":deck_source_rows,"actual_native_decks":actual_decks,"observation":observation,"construction":"Original 11d7968 input and 11d58e0 fresh-harness constructors with source-bound per-side deck vector hooks; no engine-code patches."}),
        )
    }

    pub fn native_match_until_terminal(&mut self) -> Result<Value, String> {
        let start = Instant::now();
        let before = self.engine.ticks();
        let sret = self
            .engine
            .runtime
            .heap
            .alloc(8, 8, &mut self.engine.runtime.memory)?;
        self.engine.runtime.memory.write_bytes(sret, &[0; 8])?;
        let harness = self.engine.harness;
        self.engine.call(0x11d6364, &[harness], sret)?;
        let event = read_u64(&self.engine, sret)?;
        self.engine.runtime.heap.free_ptr(sret)?;
        if event == 0 {
            return Err("original until-terminal harness returned a null event".into());
        }
        let event_code = read_u32(&self.engine, event)?;
        let after = self.engine.ticks();
        if !self.engine.is_terminal() {
            return Err("original until-terminal harness returned before native completion".into());
        }
        let observation = self.observe()?;
        Ok(
            json!({"ticks_before":before,"ticks_after":after,"world_steps":after[0].wrapping_sub(before[0]),"event":format!("{event:#x}"),"event_code":event_code,"terminated":true,"winner_side":observation["winner_side"],"elapsed_seconds":start.elapsed().as_secs_f64()}),
        )
    }

    pub fn world_step(&mut self) -> Result<(), String> {
        self.engine.world_step()
    }

    pub fn is_terminal(&self) -> bool {
        self.engine.is_terminal()
    }

    /// Read the same source-proven entity identity and health component fields
    /// as NativeSession::presentation_snapshot, without presentation work for
    /// unrelated troops/projectiles. Removed towers are filled as zero by the
    /// protocol server using their verified initial identities.
    pub fn tower_snapshot(&self) -> Result<Value, String> {
        let manager = read_u64(&self.engine, self.engine.world + 0x40)?;
        let count = read_i32(&self.engine, manager + 0x14)?;
        if !(0..=8192).contains(&count) {
            return Err(format!("invalid native entity count {count}"));
        }
        let array = read_u64(&self.engine, manager + 8)?;
        let mut towers = Vec::new();
        for index in 0..count as u64 {
            let address = read_u64(&self.engine, array + index * 8)?;
            let row = read_u64(&self.engine, address + 0x48)?;
            let name = self.native_string(row + 0x28)?;
            if name != "KingTower" && name != "PrincessTower" {
                continue;
            }
            let component_count = read_i32(&self.engine, address + 0x24)?;
            if !(3..=32).contains(&component_count) {
                return Err("unrecognized native tower component layout".into());
            }
            let table = read_u64(&self.engine, address + 0x18)?;
            let health = read_u64(&self.engine, table + 16)?;
            if health == 0 || read_u64(&self.engine, health)? != 0x1193b660 {
                return Err("unrecognized native tower health component".into());
            }
            towers.push(json!({
                "native_entity_id":read_i32(&self.engine, address + 8)?,
                "row":name,
                "side":read_i32(&self.engine, address + 0x78)?,
                "x":read_i32(&self.engine, address + 0x7c)?,
                "y":read_i32(&self.engine, address + 0x80)?,
                "hp":read_i32(&self.engine, health + 0x10)?,
                "max_hp":read_i32(&self.engine, health + 0x14)?,
            }));
        }
        Ok(json!({"tick":self.engine.ticks()[0],"entities":towers}))
    }

    /// Query the original target validator at every policy heatmap center.
    /// The same descriptor/validator calls as the reference preserve legality.
    pub fn placement_mask(&mut self, side: u32, slot: u32) -> Result<Value, String> {
        if side > 1 || slot > 3 {
            return Err("placement_mask needs side 0..1 and slot 0..3".into());
        }
        let object = self.side_object(side)?;
        if object == 0 {
            return Err("null native battle side".into());
        }
        let ticks = self.engine.ticks();
        let commands = read_u64(&self.engine, self.engine.world + 0x38)?;
        let queue = read_u32(&self.engine, commands + 0x14)?;
        let wrapper = self.engine.call(0xf9dd98, &[object, slot as u64], 0)?;
        if wrapper == 0 {
            return Err("placement_mask requested an empty hand slot".into());
        }
        self.engine
            .runtime
            .memory
            .write_bytes(self.descriptor_scratch, &[0; 24])?;
        self.engine
            .call(0xe80bbc, &[wrapper, object, 1], self.descriptor_scratch)?;
        let proxy = self.engine.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 {
            return Err("null native card selection proxy".into());
        }
        let start = Instant::now();
        let mut mask = Vec::with_capacity(576);
        for tile in 0..576 {
            let x = (((tile % 32) as f64 + 0.5) * 18000.0 / 32.0).round() as u64;
            let y = ((tile / 32) as f64 + 0.5) / 18.0;
            let y = ((if side == 1 { 1.0 - y } else { y }) * 32000.0).round() as u64;
            let code = self
                .engine
                .call(0xd42214, &[self.engine.battle, x, y, proxy, 0], 0)?
                as u32;
            mask.push(code == 0);
        }
        if ticks != self.engine.ticks() || queue != read_u32(&self.engine, commands + 0x14)? {
            return Err("placement_mask changed native tick or queue".into());
        }
        Ok(
            json!({"side":side,"slot":slot,"mask":mask,"tick":ticks[0],"query_seconds":start.elapsed().as_secs_f64()}),
        )
    }

    fn side_object(&mut self, side: u32) -> Result<u64, String> {
        let battle = self.engine.battle;
        self.engine.call(0xd35d5c, &[battle, side as u64], 0)
    }

    fn native_string(&self, address: u64) -> Result<String, String> {
        let header = self.engine.runtime.memory.read_bytes(address, 16)?;
        let len = u32::from_le_bytes(header[4..8].try_into().unwrap()) as usize;
        if len > 256 {
            return Err(format!("native row string too long: {len}"));
        }
        if len == 0 {
            return Ok(String::new());
        }
        let bytes = if len < 8 {
            header[8..8 + len].to_vec()
        } else {
            let pointer = u64::from_le_bytes(header[8..16].try_into().unwrap());
            self.engine.runtime.memory.read_bytes(pointer, len)?
        };
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    }

    /// Submit a card through the recovered native descriptor, validator, and
    /// command queue path.
    pub fn play_card(&mut self, request: PlayCardRequest) -> Result<Value, String> {
        if request.side > 1 {
            return Err("side must be 0 or 1".into());
        }
        if request.slot > 3 {
            return Err("hand slot must be in 0..=3".into());
        }
        let battle = self.engine.battle;
        let side_object = self.side_object(request.side)?;
        if side_object == 0 {
            return Err(format!("native battle side {} is null", request.side));
        }
        let actor = self
            .engine
            .call(0xd3c980, &[battle, request.side as u64], 0)?;
        if actor == 0 {
            return Err(format!("native actor for side {} is null", request.side));
        }
        let wrapper = self
            .engine
            .call(0xf9dd98, &[side_object, request.slot as u64], 0)?;
        if wrapper == 0 {
            return Ok(
                json!({"accepted":false,"rejection":"empty_hand_slot","side":request.side,"hand_slot":request.slot}),
            );
        }

        self.engine
            .runtime
            .memory
            .write_bytes(self.descriptor_scratch, &[0; 24])?;
        self.engine.call(
            0xe80bbc,
            &[wrapper, side_object, 1],
            self.descriptor_scratch,
        )?;
        let proxy = self.engine.call(0xe80680, &[self.descriptor_scratch], 0)?;
        if proxy == 0 {
            return Err("native card descriptor produced a null selection proxy".into());
        }
        let validity = self.engine.call(
            0xd42214,
            &[
                battle,
                i64::from(request.x) as u64,
                i64::from(request.y) as u64,
                proxy,
                0,
            ],
            0,
        )? as u32;
        let mana = self.engine.call(0xfa1c84, &[side_object, 1], 0)? as u32;
        let cost = self.engine.call(0xe80638, &[self.descriptor_scratch], 0)? as u32;
        let commands = read_u64(&self.engine, self.engine.world + 0x38)?;
        let queue_before = read_u32(&self.engine, commands + 0x14)?;
        let rejection = if validity != 0 {
            Some(format!("invalid_target_code_{validity}"))
        } else if mana < cost {
            Some("insufficient_mana".to_string())
        } else {
            None
        };
        if let Some(reason) = rejection {
            return Ok(
                json!({"accepted":false,"rejection":reason,"target_validity_code":validity,"mana":mana,"cost":cost,"queue_count_before":queue_before,"queue_count_after":queue_before}),
            );
        }

        let command = self
            .engine
            .runtime
            .heap
            .alloc(0x58, 16, &mut self.engine.runtime.memory)?;
        let context = read_u64(&self.engine, self.engine.harness)?;
        self.engine.call(0xd774ac, &[command, context], 0)?;
        self.engine
            .call(0xd77c8c, &[command, self.descriptor_scratch], 0)?;
        self.engine.call(
            0xd77a14,
            &[
                command,
                i64::from(request.x) as u64,
                i64::from(request.y) as u64,
            ],
            0,
        )?;
        let slot = self
            .engine
            .runtime
            .heap
            .alloc(8, 8, &mut self.engine.runtime.memory)?;
        self.engine
            .runtime
            .memory
            .write_bytes(slot, &command.to_le_bytes())?;
        let harness = self.engine.harness;
        self.engine.call(0x11d680c, &[harness, slot, actor], 0)?;
        let slot_after = read_u64(&self.engine, slot)?;
        let queue_after = read_u32(&self.engine, commands + 0x14)?;
        self.engine.runtime.heap.free_ptr(slot)?;
        let command_live = self.engine.runtime.heap.is_live(command);
        let accepted =
            queue_after == queue_before.wrapping_add(1) && slot_after == command && command_live;
        Ok(
            json!({"accepted":accepted,"rejection":if accepted {Value::Null} else {json!("original_submission_rejected")},"target_validity_code":validity,"mana":mana,"cost":cost,"queue_count_before":queue_before,"queue_count_after":queue_after,"command_allocation_live":command_live}),
        )
    }

    /// Read authoritative state and resolve each current native hand row.
    pub fn observe(&mut self) -> Result<Value, String> {
        let ticks = self.engine.ticks();
        let battle = self.engine.battle;
        let terminated = self.engine.is_terminal();
        let winner_side = if terminated {
            let winner = read_i32(&self.engine, battle + 0x1b8)?;
            if !matches!(winner, -1 | 0 | 1) {
                return Err(format!(
                    "native terminal winner field has unsupported value {winner}"
                ));
            }
            json!(winner)
        } else {
            Value::Null
        };
        let entities = read_u32(
            &self.engine,
            read_u64(&self.engine, self.engine.world + 0x40)? + 0x14,
        )?;
        let width = self.engine.call(0xd358a0, &[battle], 0)? as u32;
        let height = self.engine.call(0xd358a8, &[battle], 0)? as u32;
        let mut sides = Vec::new();
        let mut elixir = Vec::new();
        let mut hands = Vec::new();
        for side in 0..2u32 {
            let side_object = self.side_object(side)?;
            if side_object == 0 {
                return Err(format!(
                    "native battle side {side} is null during observation"
                ));
            }
            let mana = self.engine.call(0xfa1c84, &[side_object, 1], 0)? as u32;
            elixir.push(mana);
            let mut hand = Vec::with_capacity(4);
            for slot in 0..4u32 {
                let wrapper = self.engine.call(0xf9dd98, &[side_object, slot as u64], 0)?;
                if wrapper == 0 {
                    hand.push(json!({"slot":slot,"card_id":Value::Null,"name":Value::Null,"cost":Value::Null,"available":false}));
                    continue;
                }
                self.engine
                    .runtime
                    .memory
                    .write_bytes(self.descriptor_scratch, &[0; 24])?;
                self.engine.call(
                    0xe80bbc,
                    &[wrapper, side_object, 1],
                    self.descriptor_scratch,
                )?;
                let row = self.engine.call(0xe80680, &[self.descriptor_scratch], 0)?;
                if row == 0 {
                    hand.push(json!({"slot":slot,"card_id":Value::Null,"name":Value::Null,"cost":Value::Null,"available":false}));
                    continue;
                }
                let global_id = read_u32(&self.engine, row + 0x40)?;
                let name = self.native_string(row + 0x28)?;
                let cost = self.engine.call(0xe80638, &[self.descriptor_scratch], 0)? as u32;
                hand.push(json!({"slot":slot,"card_id":global_id,"name":name,"cost":cost,"available":true}));
            }
            hands.push(hand);
            sides.push(json!({"side":side,"elixir":mana}));
        }
        Ok(
            json!({"tick":ticks[0],"ticks":ticks,"terminated":terminated,"winner_side":winner_side,"entity_count":entities,"elixir":elixir,"hands":hands,"arena_cells":[width,height],"sides":sides}),
        )
    }
}

fn read_u32(engine: &Session, address: u64) -> Result<u32, String> {
    let bytes = engine.runtime.memory.read_bytes(address, 4)?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_i32(engine: &Session, address: u64) -> Result<i32, String> {
    let bytes = engine.runtime.memory.read_bytes(address, 4)?;
    Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_u64(engine: &Session, address: u64) -> Result<u64, String> {
    let bytes = engine.runtime.memory.read_bytes(address, 8)?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}
