//! Read-only presentation bindings to verified original object layouts.
//! See docs/native-spectator.md for the source accessors and scope.
use crate::frame::{BattleFrame, Entity};
use clash_recompiled::{memory::Memory, session::Session};
use std::{collections::BTreeMap, path::Path};

type Result<T> = std::result::Result<T, String>;
fn bytes<const N: usize>(m: &Memory, p: u64) -> Result<[u8; N]> {
    m.read_bytes(p, N)?
        .try_into()
        .map_err(|_| "short native read".into())
}
fn q(m: &Memory, p: u64) -> Result<u64> {
    Ok(u64::from_le_bytes(bytes(m, p)?))
}
fn u(m: &Memory, p: u64) -> Result<u32> {
    Ok(u32::from_le_bytes(bytes(m, p)?))
}
fn i(m: &Memory, p: u64) -> Result<i32> {
    Ok(i32::from_le_bytes(bytes(m, p)?))
}
fn native_string(m: &Memory, p: u64) -> Result<String> {
    let n = u(m, p + 4)? as usize;
    if n > 256 {
        return Err(format!("native row string too long at {p:#x}: {n}"));
    }
    if n == 0 {
        return Ok(String::new());
    }
    let text = m.read_bytes(if n < 8 { p + 8 } else { q(m, p + 8)? }, n)?;
    String::from_utf8(text).map_err(|e| format!("native row string: {e}"))
}

pub struct Battle {
    pub session: Session,
    result_slot: u64,
    initial_tick: u32,
    names: BTreeMap<u64, String>,
    previous_positions: BTreeMap<u64, (i32, i32)>,
}
impl Battle {
    pub fn load(checkpoint: &Path) -> Result<Self> {
        let mut session = Session::load(checkpoint)?;
        let result_slot = session
            .runtime
            .heap
            .alloc(8, 16, &mut session.runtime.memory)?;
        session.runtime.memory.write_bytes(result_slot, &[0; 8])?;
        let initial_tick = session.ticks()[0];
        Ok(Self {
            session,
            result_slot,
            initial_tick,
            names: BTreeMap::new(),
            previous_positions: BTreeMap::new(),
        })
    }
    pub fn reset(&mut self) -> Result<()> {
        self.session.reset()?;
        self.result_slot =
            self.session
                .runtime
                .heap
                .alloc(8, 16, &mut self.session.runtime.memory)?;
        self.session
            .runtime
            .memory
            .write_bytes(self.result_slot, &[0; 8])?;
        self.initial_tick = self.session.ticks()[0];
        self.previous_positions.clear();
        Ok(())
    }
    pub fn step(&mut self) -> Result<()> {
        if self.session.is_terminal() {
            return Ok(());
        }
        let before = self.session.ticks()[0];
        self.session
            .runtime
            .memory
            .write_bytes(self.result_slot, &[0; 8])?;
        self.session
            .call(0x11d6398, &[self.session.harness], self.result_slot)?;
        let after = self.session.ticks()[0];
        if after != before + 1 && !self.session.is_terminal() {
            return Err(format!(
                "native harness did not advance one tick: {before}->{after}"
            ));
        }
        Ok(())
    }
    pub fn snapshot(&mut self) -> Result<BattleFrame> {
        let m = &self.session.runtime.memory;
        let world = self.session.world;
        let battle = self.session.battle;
        let manager = q(m, world + 0x40)?;
        let count = i(m, manager + 0x14)?;
        if !(0..=8192).contains(&count) {
            return Err(format!("invalid native entity count {count}"));
        }
        let array = q(m, manager + 8)?;
        let mut entities = Vec::with_capacity(count as usize);
        let mut positions = BTreeMap::new();
        for index in 0..count as u64 {
            let address = q(m, array + 8 * index)?;
            let row = q(m, address + 0x48)?;
            if !self.names.contains_key(&row) {
                self.names.insert(row, native_string(m, row + 0x28)?);
            }
            let side = i(m, address + 0x78)?;
            let x = i(m, address + 0x7c)?;
            let y = i(m, address + 0x80)?;
            let z = i(m, address + 0x74)?.saturating_add(i(m, address + 0x8c)?);
            let component_count = i(m, address + 0x24)?;
            if !(0..=32).contains(&component_count) {
                return Err("unrecognized native component layout".into());
            }
            let mut hp = -1;
            let mut max_hp = 0;
            if component_count > 2 {
                let table = q(m, address + 0x18)?;
                let health = q(m, table + 16)?;
                // Original health component class; unknown slot-2 classes are
                // not interpreted as HP merely because the slot matches.
                if health != 0 && q(m, health)? == 0x1193b660 {
                    hp = i(m, health + 0x10)?;
                    max_hp = i(m, health + 0x14)?;
                }
            }
            let moving = self
                .previous_positions
                .get(&address)
                .is_some_and(|p| *p != (x, y));
            positions.insert(address, (x, y));
            entities.push(Entity {
                address,
                row: self.names[&row].clone(),
                side,
                x,
                y,
                z,
                hp,
                max_hp,
                moving,
                attacking: false,
                ..Entity::default()
            });
        }
        self.previous_positions = positions;
        let mut elixir = [0.; 2];
        for side in 0..2u64 {
            // d35d5c -> battle+e0+side*8; fa1c84/f99808 use fixed-point
            // current mana and the sum of queued reservation costs.
            let s = q(m, battle + 0xe0 + side * 8)?;
            let raw = i(m, s + 0x2f8)?;
            let reservations = q(m, s + 0x248)?;
            let mut reserved = 0i32;
            if reservations != 0 {
                let n = i(m, reservations + 0x1c)?;
                if !(0..=4096).contains(&n) {
                    return Err("native mana reservations out of range".into());
                }
                let data = q(m, reservations + 0x10)?;
                for k in 0..n as u64 {
                    reserved = reserved.wrapping_add(i(m, q(m, data + k * 8)? + 0x10)?);
                }
            }
            let context = q(m, q(m, s + 0x48)? + 0x10)?;
            let game_globals = q(m, q(m, context + 0x78)? + 0x18)?;
            let cap = i(m, game_globals + 0x1a8)?;
            elixir[side as usize] =
                (raw as f64 / 10000. - reserved as f64).clamp(0., cap.max(0) as f64);
        }
        let tick = self.session.ticks()[0];
        let terminated = self.session.is_terminal();
        let winner_side = if terminated {
            Some(i(m, battle + 0x1b8)?)
        } else {
            None
        };
        Ok(BattleFrame {
            human: None,
            tick,
            elapsed_ms: (tick - self.initial_tick) as u64 * 50,
            terminated,
            winner_side,
            arena_size: [i(m, battle + 0x188)? * 500, i(m, battle + 0x18c)? * 500],
            elixir,
            entities,
            hands: Default::default(),
            controller_names: Default::default(),
            plays: Vec::new(),
            match_id: 0,
        })
    }
}
