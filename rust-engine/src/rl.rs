//! Fast card-play and observation API for the compiled native session.

use crate::session::Session;
use serde_json::{json, Value};
use std::path::Path;

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

    pub fn world_step(&mut self) -> Result<(), String> {
        self.engine.world_step()
    }

    pub fn is_terminal(&self) -> bool {
        self.engine.is_terminal()
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
