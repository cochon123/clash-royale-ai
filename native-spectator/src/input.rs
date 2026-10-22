//! Player selection and projection, independent of window and native execution.
use crate::frame::{BattleFrame, CardSelection};
use serde_json::{json, Value};

pub struct ControlLayout {
    pub field: [f64; 4],
    pub world: [f64; 2],
    pub cards: [[f64; 4]; 4],
}

impl ControlLayout {
    pub fn card_at(&self, x: f64, y: f64) -> Option<u32> {
        self.cards
            .iter()
            .position(|&[cx, cy, w, h]| x >= cx && x < cx + w && y >= cy && y < cy + h)
            .map(|s| s as u32)
    }

    pub fn to_world(&self, x: f64, y: f64) -> Option<(i32, i32)> {
        let [left, top, width, height] = self.field;
        if x < left || x > left + width || y < top || y > top + height {
            return None;
        }
        let wx = (x - left) / width * self.world[0];
        let wy = (1. - ((y - top) / height * 828. - 60.) / 768.) * self.world[1];
        if !(0. ..=self.world[0]).contains(&wx) || !(0. ..=self.world[1]).contains(&wy) {
            return None;
        }
        Some((wx.round() as i32, wy.round() as i32))
    }

    pub fn to_screen(&self, x: i32, y: i32) -> (f64, f64) {
        let [left, top, width, height] = self.field;
        (
            left + x as f64 / self.world[0] * width,
            top + ((1. - y as f64 / self.world[1]) * 768. + 60.) / 828. * height,
        )
    }
}

#[derive(Default)]
pub struct PlayerInput {
    pub selection: Option<CardSelection>,
    pub cursor: Option<(i32, i32)>,
    match_id: u64,
    serial: u64,
    selection_serial: u64,
}

impl PlayerInput {
    pub fn synchronize(&mut self, frame: &BattleFrame) {
        if frame.match_id != self.match_id {
            self.match_id = frame.match_id;
            self.selection = None;
            self.cursor = None;
            self.selection_serial = 0;
        }
        let Some(human) = &frame.human else {
            self.selection = None;
            return;
        };
        if human.last_serial >= self.selection_serial {
            self.selection = human.selection;
        }
        if frame.terminated
            || self.selection.is_some_and(|s| {
                !frame.hands[human.side]
                    .iter()
                    .any(|c| c.available && c.slot == s.slot && c.card_id == s.card_id)
            })
        {
            self.selection = None;
        }
    }

    pub fn command(&mut self, op: &str) -> Value {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos() as u64);
        self.serial = stamp.max(self.serial + 1);
        json!({"op":op,"serial":self.serial,"match_id":self.match_id})
    }

    pub fn select(&mut self, frame: &BattleFrame, slot: u32) -> Option<Value> {
        let human = frame.human.as_ref()?;
        let card = frame.hands[human.side]
            .iter()
            .find(|c| c.slot == slot && c.available)?;
        self.selection = Some(CardSelection {
            slot,
            card_id: card.card_id,
        });
        let mut command = self.command("select");
        self.selection_serial = self.serial;
        command["slot"] = json!(slot);
        command["card_id"] = json!(card.card_id);
        Some(command)
    }

    pub fn cancel(&mut self) -> Value {
        self.selection = None;
        let command = self.command("cancel");
        self.selection_serial = self.serial;
        command
    }

    pub fn target_command(&mut self, op: &str) -> Option<Value> {
        let selection = self.selection?;
        let (x, y) = self.cursor?;
        let mut command = self.command(op);
        if op == "play" {
            self.selection_serial = self.serial;
        }
        command["slot"] = json!(selection.slot);
        command["card_id"] = json!(selection.card_id);
        command["x"] = json!(x);
        command["y"] = json!(y);
        Some(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_projection_inverts_the_arena_camera_without_clicking_hud_or_headroom() {
        let layout = ControlLayout {
            field: [43., 127., 454., 646.],
            world: [18000., 32000.],
            cards: [
                [140., 784., 55., 69.],
                [201., 784., 55., 69.],
                [262., 784., 55., 69.],
                [323., 784., 55., 69.],
            ],
        };
        for (x, y) in [(9000, 16000), (3500, 6500), (14500, 25500), (4000, 12345)] {
            let p = layout.to_screen(x, y);
            assert_eq!(layout.to_world(p.0, p.1), Some((x, y)));
        }
        assert_eq!(layout.card_at(203., 800.), Some(1));
        assert!(layout.to_world(203., 800.).is_none());
        assert!(layout.to_world(260., 128.).is_none());
        assert!(layout.to_world(20., 500.).is_none());
        assert!(layout.card_at(197., 800.).is_none());
    }

    #[test]
    fn hand_and_match_changes_clear_selection_and_commands_pin_card_identity() {
        let mut frame: BattleFrame = serde_json::from_value(json!({"tick":100,"elapsed_ms":0,
            "terminated":false,"winner_side":null,"arena_size":[18000,32000],"elixir":[5.,5.],
            "entities":[],"match_id":1,"human":{"side":0},
            "hands":[[{"slot":0,"card_id":26,"name":"HogRider","cost":4,"available":true}],[]]}))
        .unwrap();
        let mut input = PlayerInput::default();
        input.synchronize(&frame);
        input.select(&frame, 0).unwrap();
        input.cursor = Some((4500, 13000));
        let play = input.target_command("play").unwrap();
        assert_eq!(play["card_id"], 26);
        assert_eq!(play["match_id"], 1);
        input.synchronize(&frame); // unacknowledged local selection survives
        assert!(input.selection.is_some());
        frame.hands[0][0].card_id = 27;
        input.synchronize(&frame);
        assert!(input.selection.is_none());
        input.select(&frame, 0).unwrap();
        input.cancel();
        assert!(input.target_command("play").is_none());
        input.select(&frame, 0).unwrap();
        frame.terminated = true;
        input.synchronize(&frame);
        assert!(input.selection.is_none());
        frame.terminated = false;
        input.select(&frame, 0).unwrap();
        frame.match_id = 2;
        input.synchronize(&frame);
        assert!(input.selection.is_none());
        assert!(input.cursor.is_none());
    }
}
