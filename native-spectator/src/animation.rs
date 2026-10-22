//! Source-bound animation selection helpers for recovered SC5 documents.
//!
//! Entity motion/target state is still supplied by the native observer. These
//! helpers select only authored exports and frames; they do not infer attacks,
//! facing, deployment, or projectile motion from proximity.
use clash_native_ui_viewer::{MovieClip, Playback, Scene};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AnimationState {
    Idle,
    Moving,
    Attacking,
    Deploying,
}

pub struct PlaybackSelection {
    pub export: String,
    pub playback: Playback,
    pub frame_index: usize,
    /// Authored `_1`..`_9` export suffix when available.
    pub direction_index: Option<u8>,
    /// True only when the caller supplies a heading outside the authored
    /// half-turn and the source convention has been explicitly identified.
    pub flip_horizontal: bool,
    pub matched_state: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeadingSource {
    /// Delta between observed native positions, exposed by the session adapter.
    MotionDeltaPresentation,
    /// Last nonzero presentation heading while the entity is stationary.
    RetainedPresentationHeading,
    /// Initial portrait-facing default: blue/north, red/south.
    SidePresentationDefault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentationHeading {
    pub direction_index: u8,
    pub flip_horizontal: bool,
    pub source: HeadingSource,
}

/// Draw instructions for the original King tower's cross-document child.
/// The tower's `king_dummy` child is an identity matrix in the source. Its
/// placeholder is hidden and replaced with chr_king's authored export.
pub struct KingTowerLayers {
    pub root_export: &'static str,
    pub root_playback: Playback,
    pub king_export: &'static str,
    pub king_playback: Playback,
    /// True when an observed presentation activation age was supplied; the
    /// animation remains on the authored `stop` frame once it completes.
    pub activation_active: bool,
    pub turret_id: u32,
    pub turret_playback: Playback,
    pub turret_flip_horizontal: bool,
    pub turret_offset: [f64; 2],
    /// The source child order differs by team and controls overlap.
    pub order: KingLayerOrder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KingLayerOrder {
    TurretThenKing,
    KingThenTurret,
}

/// Split King tower rendering into the original tower root, original King
/// character export, and original turret child. The tower's dummy character
/// placeholder is empty; its transform is identity. The turret remains at its
/// source child position (blue y=-57, red y=-33) and uses its authored
/// half-turn azimuth bank. `heading` is presentation motion, not engine
/// target/facing.
pub fn king_tower_layers(
    tower_scene: &Scene,
    king_scene: &Scene,
    red_team: bool,
    heading: PresentationHeading,
    elapsed_ms: u64,
) -> Result<KingTowerLayers, String> {
    king_tower_layers_with_activation(tower_scene, king_scene, red_team, heading, elapsed_ms, None)
}

/// Variant that accepts age since the renderer's observed activation cue.
/// This cue is a presentation inference from real tower damage/destruction,
/// not a recovered native activation flag.
pub fn king_tower_layers_with_activation(
    tower_scene: &Scene,
    king_scene: &Scene,
    red_team: bool,
    heading: PresentationHeading,
    elapsed_ms: u64,
    activation_age_ms: Option<u64>,
) -> Result<KingTowerLayers, String> {
    let (root_export, king_export, turret_offset, order) = if red_team {
        (
            "KingTower_red",
            "King_red",
            [0.0, -33.0],
            KingLayerOrder::KingThenTurret,
        )
    } else {
        (
            "KingTower_blue",
            "King_blue",
            [0.0, -57.0],
            KingLayerOrder::TurretThenKing,
        )
    };
    if !tower_scene.document.exports.contains_key(root_export) {
        return Err(format!("missing original King tower export {root_export}"));
    }
    if !king_scene.document.exports.contains_key(king_export) {
        return Err(format!(
            "missing original King character export {king_export}"
        ));
    }
    let root_movie_id = tower_scene
        .export_id(root_export)
        .map_err(|e| e.to_string())?;
    let root_idle = tower_scene
        .document
        .movies
        .get(&root_movie_id)
        .and_then(|movie| movie.frame_labels.iter().position(|label| label == "idle"));
    let root_frame = root_idle.unwrap_or(0);
    let king_movie_id = king_scene
        .export_id(king_export)
        .map_err(|e| e.to_string())?;
    let Some(king_movie) = king_scene.document.movies.get(&king_movie_id) else {
        return Err(format!("original King export {king_export} has no movie"));
    };
    if !king_movie.frame_labels.iter().any(|label| label == "idle") {
        return Err(format!(
            "original King export {king_export} has no idle label"
        ));
    }
    let (root_frame, king_playback, activation_active, timeline_seconds) =
        if let Some(age_ms) = activation_age_ms {
            let root_movie = tower_scene.document.movies.get(&root_movie_id).unwrap();
            let root_frame = authored_start_stop_frame(root_movie, age_ms)?;
            let king_frame = authored_start_stop_frame(king_movie, age_ms)?;
            (
                root_frame,
                Playback {
                    frame: Some(king_frame),
                    time_seconds: Some(age_ms as f64 / 1000.0),
                    ..Playback::default()
                },
                true,
                age_ms,
            )
        } else {
            (
                root_frame,
                Playback {
                    label: Some("idle".into()),
                    time_seconds: Some(elapsed_ms as f64 / 1000.0),
                    ..Playback::default()
                },
                false,
                elapsed_ms,
            )
        };
    let turret_frame = king_turret_frame(heading);
    Ok(KingTowerLayers {
        root_export,
        root_playback: Playback {
            frame: Some(root_frame),
            time_seconds: Some(timeline_seconds as f64 / 1000.0),
            hidden_objects: [238u32].into_iter().collect(),
            ..Playback::default()
        },
        king_export,
        king_playback,
        activation_active,
        turret_id: 238,
        turret_playback: Playback {
            frame: Some(turret_frame),
            ..Playback::default()
        },
        turret_flip_horizontal: king_turret_flip(heading),
        turret_offset,
        order,
    })
}

/// Map a nine-way world heading into the original turret's 18-view half-turn
/// bank. The timeline stores each view twice (frames 0/1, 2/3, ..., 34/35).
/// The audited bank runs from south at frame 0 through west to north at frame
/// 34; eastward headings use a horizontal mirror.
pub fn king_turret_frame(heading: PresentationHeading) -> usize {
    let frame = (9u8.saturating_sub(heading.direction_index) as f64 * 4.25).round();
    ((frame / 2.0).round() as usize * 2).min(34)
}

/// The audited turret bank contains the west-side half-turn. Mirror the
/// corresponding source pose only for east-side headings; north/south end
/// views need no mirror.
pub fn king_turret_flip(heading: PresentationHeading) -> bool {
    !heading.flip_horizontal && (2..=8).contains(&heading.direction_index)
}

fn authored_start_stop_frame(movie: &MovieClip, age_ms: u64) -> Result<usize, String> {
    let start = movie
        .frame_labels
        .iter()
        .position(|label| label == "start")
        .ok_or_else(|| "original King activation clip has no start label".to_owned())?;
    let stop = movie
        .frame_labels
        .iter()
        .position(|label| label == "stop")
        .ok_or_else(|| "original King activation clip has no stop label".to_owned())?;
    if stop < start {
        return Err("original King activation stop precedes start".into());
    }
    let fps = movie.framerate.max(1) as u64;
    Ok((start as u64 + age_ms.saturating_mul(fps) / 1000).min(stop as u64) as usize)
}

/// Convert an observed world-coordinate translation into the nearest one of
/// the original nine authored 180-degree poses. The nine-view mapping is
/// visually audited from the original Knight and Mini P.E.K.K.A SC5 exports:
/// index 1 faces north/away, 5 faces east, and 9 faces south/toward. The other
/// half-turn uses an explicit horizontal mirror of the corresponding pose.
/// This is presentation heading derived from motion, not native logical facing.
pub fn direction_for_motion(
    motion_dx: i64,
    motion_dy: i64,
    side: i32,
    last: Option<PresentationHeading>,
) -> PresentationHeading {
    if motion_dx == 0 && motion_dy == 0 {
        return last
            .map(|heading| PresentationHeading {
                source: HeadingSource::RetainedPresentationHeading,
                ..heading
            })
            .unwrap_or(PresentationHeading {
                direction_index: if side == 0 { 1 } else { 9 },
                flip_horizontal: false,
                source: HeadingSource::SidePresentationDefault,
            });
    }
    // Native board y increases northward (blue is at low y, red at high y).
    let angle = (motion_dx as f64).atan2(motion_dy as f64).to_degrees();
    let flip_horizontal = angle < 0.0;
    let quantized = (angle.abs().min(180.0) / 22.5).round() as u8;
    PresentationHeading {
        direction_index: (quantized + 1).clamp(1, 9),
        flip_horizontal,
        source: HeadingSource::MotionDeltaPresentation,
    }
}

/// Select an exact original export by row prefix, team prefix, action token,
/// and optional authored directional suffix. `red_prefix` comes from the
/// original nested logic row; it is not synthesized from a color assumption.
pub fn select_export(
    scene: &Scene,
    blue_prefix: &str,
    red_prefix: Option<&str>,
    red_team: bool,
    state: AnimationState,
    direction_index: Option<u8>,
    elapsed_ms: u64,
) -> Result<Option<PlaybackSelection>, String> {
    if direction_index.is_some_and(|d| !(1..=9).contains(&d)) {
        return Err("authored SC direction index must be in 1..=9".into());
    }
    let prefix = if red_team {
        red_prefix.unwrap_or(blue_prefix)
    } else {
        blue_prefix
    };
    // Exclude the other team's family only when its stem is more specific
    // than ours: Musketeer's red stem `chr_musketeer` prefixes the blue stem
    // `chr_musketeer_blue`, while Knight's blue stem prefixes `Knight_enemy`.
    let other = if red_team {
        Some(blue_prefix)
    } else {
        red_prefix
    };
    let is_allowed_export = |name: &str| {
        name.starts_with(prefix)
            && other.is_none_or(|other| {
                other == prefix || other.len() <= prefix.len() || !name.starts_with(other)
            })
    };
    let token = match state {
        AnimationState::Idle => "idle",
        AnimationState::Moving => "run",
        AnimationState::Attacking => "attack",
        AnimationState::Deploying => "deploy",
    };
    let exports = scene.document.exports.keys();
    let mut candidates: Vec<(String, Option<u8>)> = exports
        .filter(|name| is_allowed_export(name))
        .filter_map(|name| {
            let lower = name.to_ascii_lowercase();
            if !lower.contains(token) {
                return None;
            }
            let suffix = name.rsplit('_').next().and_then(|s| s.parse::<u8>().ok());
            Some((name.clone(), suffix.filter(|d| (1..=9).contains(d))))
        })
        .collect();
    if candidates.is_empty() {
        // A source may define only a static form for a row. Keep the state
        // mismatch visible instead of relabelling its idle artwork as attack.
        candidates = scene
            .document
            .exports
            .keys()
            .filter(|name| is_allowed_export(name))
            .map(|name| {
                let suffix = name.rsplit('_').next().and_then(|s| s.parse::<u8>().ok());
                (name.clone(), suffix.filter(|d| (1..=9).contains(d)))
            })
            .collect();
        if candidates.is_empty() {
            return Ok(None);
        }
    }
    if candidates.is_empty() {
        return Ok(None);
    }

    // Direction banks vary by row: Knight has nine poses, while Mortar has
    // five at 45-degree intervals. Remap the normalized half-turn to each
    // source bank instead of requiring a suffix that the bank does not have.
    let authored_directions: Vec<u8> = candidates
        .iter()
        .filter_map(|(_, direction)| *direction)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let desired = direction_index.unwrap_or(5);
    let mapped_direction = if let (Some(first), Some(last)) =
        (authored_directions.first(), authored_directions.last())
    {
        let fraction = (desired.saturating_sub(1) as f64) / 8.0;
        let raw = *first as f64 + fraction * (*last - *first) as f64;
        authored_directions
            .iter()
            .copied()
            .min_by_key(|candidate| ((*candidate as f64 - raw).abs() * 1000.) as u32)
    } else {
        None
    };
    let selected = mapped_direction
        .and_then(|direction| candidates.iter().find(|(_, d)| *d == Some(direction)))
        .or_else(|| candidates.iter().find(|(_, d)| d.is_none()));
    let Some((export, authored_direction)) = selected else {
        return Ok(None);
    };
    let matched_state = export.to_ascii_lowercase().contains(token);
    let fps = scene.framerate(export).map_err(|e| e.to_string())? as u64;
    let count = scene.frame_count(export).map_err(|e| e.to_string())?.max(1) as u64;
    let idle_label_frame = scene
        .document
        .movies
        .get(&scene.export_id(export).map_err(|e| e.to_string())?)
        .and_then(|movie| {
            if state == AnimationState::Idle {
                movie.frame_labels.iter().position(|label| label == "idle")
            } else {
                None
            }
        });
    let elapsed_frames = elapsed_ms * fps / 1000;
    // Deployment effects are authored one-shots. Returning no selection after
    // the source clip ends lets the caller remove the transient overlay.
    if state == AnimationState::Deploying && elapsed_frames >= count {
        return Ok(None);
    }
    let frame_index = idle_label_frame.unwrap_or_else(|| {
        if state == AnimationState::Deploying {
            elapsed_frames as usize
        } else {
            (elapsed_frames % count) as usize
        }
    });
    let has_idle_label = idle_label_frame.is_some();
    let playback = if has_idle_label {
        Playback {
            label: Some("idle".into()),
            time_seconds: Some(elapsed_ms as f64 / 1000.),
            ..Playback::default()
        }
    } else {
        Playback {
            frame: Some(frame_index),
            time_seconds: Some(elapsed_ms as f64 / 1000.),
            ..Playback::default()
        }
    };
    Ok(Some(PlaybackSelection {
        export: export.clone(),
        playback,
        frame_index,
        direction_index: *authored_direction,
        // The nine-way source convention was visually audited; mirroring is
        // supplied separately by select_for_heading from presentation motion.
        flip_horizontal: false,
        matched_state,
    }))
}

/// Direction-aware entry point for a rendered entity. The returned mirror
/// flag is based on the source pose contact-sheet convention documented above.
pub fn select_for_heading(
    scene: &Scene,
    blue_prefix: &str,
    red_prefix: Option<&str>,
    red_team: bool,
    state: AnimationState,
    heading: PresentationHeading,
    elapsed_ms: u64,
) -> Result<Option<PlaybackSelection>, String> {
    let mut selection = select_export(
        scene,
        blue_prefix,
        red_prefix,
        red_team,
        state,
        Some(heading.direction_index),
        elapsed_ms,
    )?;
    if let Some(selection) = selection.as_mut() {
        selection.flip_horizontal = heading.flip_horizontal;
    }
    Ok(selection)
}

/// Source-configured base/top exports for an original tower. Princess towers
/// use separate base and top exports; the King tower is an animated composite
/// export and therefore passes `top_export=None`.
pub fn tower_layers(
    scene: &Scene,
    base_export: &str,
    top_export: Option<&str>,
    elapsed_ms: u64,
) -> Result<Vec<PlaybackSelection>, String> {
    let mut layers = Vec::with_capacity(if top_export.is_some() { 2 } else { 1 });
    for export in std::iter::once(base_export).chain(top_export) {
        if !scene.document.exports.contains_key(export) {
            return Err(format!(
                "tower export is absent from original scene: {export}"
            ));
        }
        let fps = scene.framerate(export).map_err(|e| e.to_string())? as u64;
        let count = scene.frame_count(export).map_err(|e| e.to_string())?.max(1) as u64;
        let frame_index = ((elapsed_ms * fps / 1000) % count) as usize;
        let playback = Playback {
            frame: Some(frame_index),
            time_seconds: Some(elapsed_ms as f64 / 1000.),
            ..Playback::default()
        };
        layers.push(PlaybackSelection {
            export: export.to_owned(),
            playback,
            frame_index,
            direction_index: None,
            flip_horizontal: false,
            matched_state: true,
        });
    }
    Ok(layers)
}

/// A placement helper for callers that want a clean original idle pose. The
/// labels and timeline remain fully source-authored.
pub fn idle_playback() -> Playback {
    Playback {
        label: Some("idle".into()),
        ..Playback::default()
    }
}

/// Known state animation tokens present in the original SC document. Kept
/// public so the renderer can explain which native state drove the art choice.
pub fn available_states(scene: &Scene, prefix: &str) -> BTreeSet<AnimationState> {
    let mut states = BTreeSet::new();
    for name in scene
        .document
        .exports
        .keys()
        .filter(|name| name.starts_with(prefix))
    {
        let lower = name.to_ascii_lowercase();
        if lower.contains("idle") {
            states.insert(AnimationState::Idle);
        }
        if lower.contains("run") || lower.contains("walk") {
            states.insert(AnimationState::Moving);
        }
        if lower.contains("attack") {
            states.insert(AnimationState::Attacking);
        }
        if lower.contains("deploy") {
            states.insert(AnimationState::Deploying);
        }
    }
    states
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{imageops, Rgba, RgbaImage};
    use std::path::Path;

    #[test]
    #[ignore = "writes original-art direction audit to /tmp"]
    fn write_direction_contact_sheet() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let knight = Scene::load(&root, "chr_knight").unwrap();
        let mini = Scene::load(&root, "chr_mini_pekka").unwrap();
        let cell = 144u32;
        let rows = [
            (&knight, "Knight_idle1", "Knight idle"),
            (&knight, "Knight_run1", "Knight run"),
            (&knight, "Knight_enemy_idle1", "Knight enemy idle"),
            (&knight, "Knight_enemy_run1", "Knight enemy run"),
            (&mini, "minipekka_idle1", "MiniPekka idle blue"),
            (&mini, "minipekka_run1", "MiniPekka run blue"),
            (&mini, "minipekka_red_idle1", "MiniPekka idle red"),
            (&mini, "minipekka_red_run1", "MiniPekka run red"),
        ];
        let mut sheet =
            RgbaImage::from_pixel(cell * 9, cell * rows.len() as u32, Rgba([40, 44, 48, 255]));
        for (row_index, (scene, stem, label)) in rows.iter().enumerate() {
            for direction in 1..=9 {
                let export = format!("{stem}_{direction}");
                let rendered = scene
                    .render(&export, &Playback::default())
                    .unwrap_or_else(|e| panic!("{label} {export}: {e}"))
                    .unwrap_or_else(|| panic!("{label} {export}: no image"));
                let max_w = cell - 8;
                let max_h = cell - 8;
                let scale = (max_w as f64 / rendered.image.width() as f64)
                    .min(max_h as f64 / rendered.image.height() as f64)
                    .min(1.0);
                let w = (rendered.image.width() as f64 * scale).round().max(1.0) as u32;
                let h = (rendered.image.height() as f64 * scale).round().max(1.0) as u32;
                let sprite =
                    imageops::resize(&rendered.image, w, h, imageops::FilterType::Triangle);
                let x = direction as u32 * cell - cell + (cell - w) / 2;
                let y = row_index as u32 * cell + (cell - h) / 2;
                imageops::overlay(&mut sheet, &sprite, x as i64, y as i64);
            }
            eprintln!(
                "contact sheet row {row_index}: {label}; columns are authored suffixes 1 through 9"
            );
        }
        sheet
            .save("/tmp/native-spectator-direction-contact.png")
            .unwrap();
    }

    #[test]
    #[ignore = "writes original-art King turret frame audit to /tmp"]
    fn write_king_turret_contact_sheet() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let towers = Scene::load(&root, "building_tower").unwrap();
        let cell = 120u32;
        let mut sheet = RgbaImage::from_pixel(9 * cell, 2 * cell, Rgba([40, 44, 48, 255]));
        for (index, frame) in (0usize..36).step_by(2).enumerate() {
            let rendered = towers
                .render_id(
                    238,
                    &Playback {
                        frame: Some(frame),
                        ..Playback::default()
                    },
                )
                .unwrap()
                .unwrap();
            let scale = ((cell - 18) as f64 / rendered.image.width() as f64)
                .min((cell - 18) as f64 / rendered.image.height() as f64)
                .min(1.0);
            let width = (rendered.image.width() as f64 * scale).round().max(1.0) as u32;
            let height = (rendered.image.height() as f64 * scale).round().max(1.0) as u32;
            let sprite = imageops::resize(
                &rendered.image,
                width,
                height,
                imageops::FilterType::Triangle,
            );
            let column = index as u32 % 9;
            let row = index as u32 / 9;
            imageops::overlay(
                &mut sheet,
                &sprite,
                (column * cell + (cell - width) / 2) as i64,
                (row * cell + (cell - height) / 2) as i64,
            );
            eprintln!(
                "turret bank index {index}, child frame {frame}, origin=({:.1},{:.1}), size={}x{}",
                rendered.x,
                rendered.y,
                rendered.image.width(),
                rendered.image.height()
            );
        }
        sheet
            .save("/tmp/native-spectator-king-turret-contact.png")
            .unwrap();
    }

    #[test]
    fn selects_exact_direction_and_authored_king_idle_label() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let knight = Scene::load(&root, "chr_knight").unwrap();
        let selection = select_export(
            &knight,
            "Knight",
            Some("Knight_enemy"),
            true,
            AnimationState::Moving,
            Some(7),
            200,
        )
        .unwrap()
        .unwrap();
        assert_eq!(selection.export, "Knight_enemy_run1_7");
        assert_eq!(selection.direction_index, Some(7));
        assert!(selection.matched_state);

        let towers = Scene::load(&root, "building_tower").unwrap();
        let king = select_export(
            &towers,
            "KingTower_blue",
            Some("KingTower_red"),
            false,
            AnimationState::Idle,
            None,
            10_000,
        )
        .unwrap()
        .unwrap();
        assert_eq!(king.export, "KingTower_blue");
        assert_eq!(king.playback.label.as_deref(), Some("idle"));
        assert_eq!(king.frame_index, 0);

        let princess =
            tower_layers(&towers, "StarTower_base_red", Some("StarTower_top_red"), 50).unwrap();
        assert_eq!(
            princess
                .iter()
                .map(|layer| layer.export.as_str())
                .collect::<Vec<_>>(),
            vec!["StarTower_base_red", "StarTower_top_red"]
        );
        for layer in &princess {
            let image = towers
                .render(&layer.export, &layer.playback)
                .unwrap()
                .unwrap();
            eprintln!(
                "{} original render origin=({:.2},{:.2}) size={}x{}",
                layer.export,
                image.x,
                image.y,
                image.image.width(),
                image.image.height()
            );
        }
    }

    #[test]
    fn blue_prefix_excludes_enemy_exports_and_short_direction_banks_remap() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let knight = Scene::load(&root, "chr_knight").unwrap();
        let blue = select_export(
            &knight,
            "Knight",
            Some("Knight_enemy"),
            false,
            AnimationState::Moving,
            Some(7),
            0,
        )
        .unwrap()
        .unwrap();
        assert_eq!(blue.export, "Knight_run1_7");
        assert!(!blue.export.starts_with("Knight_enemy"));

        let musketeer = Scene::load(&root, "chr_musketeer").unwrap();
        for (red, expected) in [
            (false, "chr_musketeer_blue_idle1_5"),
            (true, "chr_musketeer_idle1_5"),
        ] {
            let selected = select_export(
                &musketeer,
                "chr_musketeer_blue",
                Some("chr_musketeer"),
                red,
                AnimationState::Idle,
                Some(5),
                0,
            )
            .unwrap()
            .unwrap();
            assert_eq!(selected.export, expected);
        }

        let mortar = Scene::load(&root, "building_mortar").unwrap();
        let blue_east = select_export(
            &mortar,
            "building_mortar1",
            Some("building_mortar1_enemy"),
            false,
            AnimationState::Idle,
            Some(5),
            0,
        )
        .unwrap()
        .unwrap();
        assert_eq!(blue_east.export, "building_mortar1_idle1_3");
        let red_north = select_export(
            &mortar,
            "building_mortar1",
            Some("building_mortar1_enemy"),
            true,
            AnimationState::Idle,
            Some(1),
            0,
        )
        .unwrap()
        .unwrap();
        assert_eq!(red_north.export, "building_mortar1_enemy_idle1_1");
        let red_south = select_export(
            &mortar,
            "building_mortar1",
            Some("building_mortar1_enemy"),
            true,
            AnimationState::Idle,
            Some(9),
            0,
        )
        .unwrap()
        .unwrap();
        assert_eq!(red_south.export, "building_mortar1_enemy_idle1_5");
        assert_eq!(red_south.direction_index, Some(5));
    }

    #[test]
    fn motion_maps_to_original_half_turn_and_retains_stationary_heading() {
        let n = direction_for_motion(0, 20, 0, None);
        let ne = direction_for_motion(20, 20, 0, None);
        let e = direction_for_motion(20, 0, 0, None);
        let se = direction_for_motion(20, -20, 0, None);
        let s = direction_for_motion(0, -20, 1, None);
        let w = direction_for_motion(-20, 0, 1, None);
        assert_eq!(n.direction_index, 1);
        assert_eq!(ne.direction_index, 3);
        assert_eq!(e.direction_index, 5);
        assert_eq!(se.direction_index, 7);
        assert_eq!(s.direction_index, 9);
        assert_eq!(w.direction_index, 5);
        assert!(w.flip_horizontal);
        assert_eq!(direction_for_motion(0, 0, 0, None).direction_index, 1);
        assert_eq!(direction_for_motion(0, 0, 1, None).direction_index, 9);
        assert_eq!(
            direction_for_motion(0, 0, 1, Some(w)),
            PresentationHeading {
                source: HeadingSource::RetainedPresentationHeading,
                ..w
            }
        );
    }

    #[test]
    fn king_tower_split_uses_original_exports_offsets_order_and_azimuth() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-spectator");
        let tower_scene = Scene::load(&root, "building_tower").unwrap();
        let king_scene = Scene::load(&root, "chr_king").unwrap();
        let north = PresentationHeading {
            direction_index: 1,
            flip_horizontal: false,
            source: HeadingSource::SidePresentationDefault,
        };
        let blue = king_tower_layers(&tower_scene, &king_scene, false, north, 0).unwrap();
        assert_eq!(blue.root_export, "KingTower_blue");
        assert_eq!(blue.king_export, "King_blue");
        assert_eq!(blue.turret_offset, [0.0, -57.0]);
        assert_eq!(blue.order, KingLayerOrder::TurretThenKing);
        assert!(blue.root_playback.hidden_objects.contains(&238));
        assert_eq!(blue.turret_playback.frame, Some(34));
        assert!(!blue.turret_flip_horizontal);
        assert!(tower_scene
            .render(blue.root_export, &blue.root_playback)
            .unwrap()
            .is_some());
        assert!(king_scene
            .render(blue.king_export, &blue.king_playback)
            .unwrap()
            .is_some());
        assert!(tower_scene
            .render_id(blue.turret_id, &blue.turret_playback)
            .unwrap()
            .is_some());

        let south = PresentationHeading {
            direction_index: 9,
            ..north
        };
        let red = king_tower_layers(&tower_scene, &king_scene, true, south, 0).unwrap();
        assert_eq!(red.root_export, "KingTower_red");
        assert_eq!(red.king_export, "King_red");
        assert_eq!(red.turret_offset, [0.0, -33.0]);
        assert_eq!(red.order, KingLayerOrder::KingThenTurret);
        assert_eq!(red.turret_playback.frame, Some(0));
        assert!(!red.turret_flip_horizontal);

        let activating = king_tower_layers_with_activation(
            &tower_scene,
            &king_scene,
            false,
            north,
            5000,
            Some(1000),
        )
        .unwrap();
        assert!(activating.activation_active);
        assert_eq!(activating.root_playback.frame, Some(31));
        assert_eq!(activating.king_playback.frame, Some(31));
        let held_stop = king_tower_layers_with_activation(
            &tower_scene,
            &king_scene,
            false,
            north,
            5000,
            Some(90_000),
        )
        .unwrap();
        assert_eq!(held_stop.root_playback.frame, Some(97));
        assert_eq!(held_stop.king_playback.frame, Some(97));
        let east = king_turret_frame(PresentationHeading {
            direction_index: 5,
            flip_horizontal: false,
            source: HeadingSource::MotionDeltaPresentation,
        });
        let west = king_turret_frame(PresentationHeading {
            direction_index: 5,
            flip_horizontal: true,
            source: HeadingSource::MotionDeltaPresentation,
        });
        assert_eq!(east, 18);
        assert_eq!(west, 18);
        assert!(king_turret_flip(PresentationHeading {
            direction_index: 5,
            flip_horizontal: false,
            source: HeadingSource::MotionDeltaPresentation,
        }));
        assert!(!king_turret_flip(PresentationHeading {
            direction_index: 5,
            flip_horizontal: true,
            source: HeadingSource::MotionDeltaPresentation,
        }));
    }
}
