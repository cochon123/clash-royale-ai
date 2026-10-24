//! Offline original-art gallery fixtures.
//!
//! A case frame's scene JSON and CPU reference are produced from the same
//! display tick. Replay captures are the source. Synthetic marches are only
//! isolated presentation fixtures, not an official animation comparison.
use crate::draw::ShownSubject;
use crate::frame::{BattleFrame, Entity};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryFrame {
    pub time_ms: u64,
    pub scene: String,
    pub reference: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryCase {
    pub id: String,
    pub card: String,
    pub label: String,
    pub team: i32,
    pub state: String,
    pub heading: u8,
    pub frames: Vec<GalleryFrame>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubjectKey {
    pub card: String,
    pub team: i32,
    pub state: String,
    pub heading: u8,
}

/// Display ticks from `from_tick` through `to_tick` at `fps` wall frames per
/// second. The step is `20/fps` native ticks, so 30 fps is not the 20 Hz grid.
pub fn display_samples(from_tick: f64, to_tick: f64, fps: f64) -> Vec<f64> {
    if !from_tick.is_finite() || !to_tick.is_finite() || from_tick > to_tick || !(1. ..=240.).contains(&fps)
    {
        return Vec::new();
    }
    let step = 20. / fps;
    let mut t = from_tick;
    let mut out = Vec::new();
    while t <= to_tick + 1e-9 && out.len() < 20_000 {
        out.push(t);
        t += step;
    }
    out
}

/// One engine tick of lookahead. Combat past this horizon is not observed.
pub fn observe_horizon(display_tick: f64) -> f64 {
    display_tick + 1.
}

pub fn sanitize_id(raw: &str) -> String {
    let mut id: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    while id.contains("--") {
        id = id.replace("--", "-");
    }
    let id = id.trim_matches('-').to_owned();
    if id.is_empty() {
        "case".into()
    } else {
        id
    }
}

pub fn primary_subject<'a>(
    subjects: &'a [ShownSubject],
    preferred: &[String],
) -> Option<&'a ShownSubject> {
    if let Some(subject) = subjects
        .iter()
        .find(|subject| !subject.projectile && preferred.iter().any(|card| card == &subject.row))
    {
        return Some(subject);
    }
    subjects
        .iter()
        .find(|subject| !subject.projectile && !subject.tower)
        .or_else(|| subjects.iter().find(|subject| !subject.projectile))
}

/// Append a frame, starting a new case when the subject changes or the case
/// already holds 24 frames. Paths are relative to the gallery root.
pub fn push_frame(cases: &mut Vec<GalleryCase>, key: &SubjectKey, time_ms: u64) -> (String, String) {
    let fits = cases.last().is_some_and(|case| {
        case.card == key.card
            && case.team == key.team
            && case.state == key.state
            && case.heading == key.heading
            && case.frames.len() < 24
    });
    if !fits {
        let ordinal = cases.len();
        let id = sanitize_id(&format!(
            "{}-t{}-{}-h{}-{}",
            key.card, key.team, key.state, key.heading, ordinal
        ));
        cases.push(GalleryCase {
            id,
            card: key.card.clone(),
            label: format!(
                "{} team {} {} heading {}",
                key.card, key.team, key.state, key.heading
            ),
            team: key.team,
            state: key.state.clone(),
            heading: key.heading,
            frames: Vec::new(),
        });
    }
    let case = cases.last_mut().expect("case");
    let index = case.frames.len();
    let scene = format!("cases/{}/{index:04}.json", case.id);
    let reference = format!("cases/{}/{index:04}.png", case.id);
    case.frames.push(GalleryFrame {
        time_ms,
        scene: scene.clone(),
        reference: reference.clone(),
    });
    (scene, reference)
}

pub fn merge_cases(existing: &mut Vec<GalleryCase>, incoming: Vec<GalleryCase>) {
    for case in incoming {
        if let Some(slot) = existing.iter_mut().find(|old| old.id == case.id) {
            *slot = case;
        } else {
            existing.push(case);
        }
    }
}

pub fn read_manifest(path: &Path) -> Result<Vec<GalleryCase>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| format!("visual gallery manifest: {e}"))?;
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return Err("visual gallery manifest version must be 1".into());
    }
    serde_json::from_value(value.get("cases").cloned().unwrap_or(Value::Array(Vec::new())))
        .map_err(|e| format!("visual gallery cases: {e}"))
}

pub fn write_manifest(path: &Path, cases: &[GalleryCase]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let body = serde_json::to_string_pretty(&serde_json::json!({"version": 1, "cases": cases}))
        .map_err(|e| e.to_string())?;
    fs::write(path, body + "\n").map_err(|e| e.to_string())
}

/// A short walk used only when `--synthetic` is requested. Positions change
/// and the source attack flag stays false, with raw timers at 0.
pub fn synthetic_march(card: &str, team: i32, heading: u8, ticks: u32) -> Vec<BattleFrame> {
    let heading = heading.clamp(1, 9);
    let angle = f64::from(heading - 1) * 22.5_f64.to_radians();
    let (dx, dy) = (angle.sin(), angle.cos());
    let (x0, y0) = if team == 0 { (9000, 8000) } else { (9000, 24000) };
    (0..ticks)
        .map(|tick| {
            let x = x0 + (dx * 180. * f64::from(tick)).round() as i32;
            let y = y0 + (dy * 180. * f64::from(tick)).round() as i32;
            BattleFrame {
                tick,
                elapsed_ms: u64::from(tick) * 50,
                terminated: false,
                winner_side: None,
                arena_size: [18000, 32000],
                elixir: [5., 5.],
                entities: vec![Entity {
                    address: 1,
                    row: card.to_owned(),
                    side: team,
                    x,
                    y,
                    z: 0,
                    hp: 1000,
                    max_hp: 1000,
                    moving: false,
                    attacking: false,
                    native_action_timer_raw: Some(0),
                    native_secondary_timer_raw: Some(0),
                    ..Entity::default()
                }],
                hands: Default::default(),
                controller_names: Default::default(),
                plays: Vec::new(),
                match_id: 1,
                human: None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_at_30hz_keep_one_tick_of_lookahead() {
        let samples = display_samples(100., 102., 30.);
        assert!((samples[0] - 100.).abs() < 1e-9);
        assert!(samples.iter().any(|t| (*t - 100.).abs() > 0.2 && *t < 101.));
        assert!(samples.iter().all(|t| *t <= 102. + 1e-9));
        assert!(samples.windows(2).all(|w| (w[1] - w[0] - 20. / 30.).abs() < 1e-9));
        for t in &samples {
            assert!((observe_horizon(*t) - (t + 1.)).abs() < 1e-9);
        }
        assert!(display_samples(3., 1., 30.).is_empty());
    }

    #[test]
    fn cases_keep_card_names_and_split_when_the_state_changes() {
        let mut cases = Vec::new();
        let walk = SubjectKey {
            card: "P.E.K.K.A".into(),
            team: 0,
            state: "walk".into(),
            heading: 5,
        };
        let (scene, reference) = push_frame(&mut cases, &walk, 72_550);
        assert_eq!(cases[0].card, "P.E.K.K.A");
        assert!(!cases[0].id.contains('.'));
        assert!(scene.ends_with(".json") && reference.ends_with(".png"));
        assert_eq!(scene.trim_end_matches(".json"), reference.trim_end_matches(".png"));
        push_frame(&mut cases, &walk, 72_583);
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].frames.len(), 2);
        let idle = SubjectKey {
            state: "idle".into(),
            ..walk
        };
        push_frame(&mut cases, &idle, 72_600);
        assert_eq!(cases.len(), 2);
        let mut again = Vec::new();
        merge_cases(&mut again, cases.clone());
        merge_cases(&mut again, cases);
        assert_eq!(again.len(), 2);
    }

    #[test]
    fn preferred_card_wins_over_the_first_troop() {
        let subjects = vec![
            ShownSubject {
                row: "Knight".into(),
                team: 0,
                state: "walk",
                heading: 1,
                projectile: false,
                tower: false,
            },
            ShownSubject {
                row: "P.E.K.K.A".into(),
                team: 1,
                state: "attack",
                heading: 9,
                projectile: false,
                tower: false,
            },
            ShownSubject {
                row: "KingTower".into(),
                team: 0,
                state: "idle",
                heading: 1,
                projectile: false,
                tower: true,
            },
        ];
        assert_eq!(
            primary_subject(&subjects, &["P.E.K.K.A".into()]).unwrap().row,
            "P.E.K.K.A"
        );
        assert_eq!(primary_subject(&subjects, &[]).unwrap().row, "Knight");
    }

    #[test]
    fn synthetic_march_moves_without_trusting_the_attack_flag() {
        let frames = synthetic_march("P.E.K.K.A", 1, 5, 4);
        assert_eq!(frames.len(), 4);
        assert!(frames[3].entities[0].x > frames[0].entities[0].x);
        assert!(!frames[0].entities[0].attacking);
        assert_eq!(frames[0].entities[0].native_action_timer_raw, Some(0));
        assert_eq!(frames[0].entities[0].side, 1);
        let north = synthetic_march("Knight", 0, 1, 3);
        assert!(north[2].entities[0].y > north[0].entities[0].y);
    }
}
