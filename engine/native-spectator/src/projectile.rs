//! Original projectile movie selection, using the source logic's animation mode.
//!
//! This is presentation only: position, motion and age come from the native
//! observer. In particular `use360Frames` movies are view banks, not clocks.
use crate::animation::PlaybackSelection;
use clash_native_ui_viewer::{Playback, Scene};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectileMovieMode {
    /// Ordinary authored timeline. Advance by native-observed age at the movie fps.
    Timeline,
    /// Authored half-turn view bank; select its neutral broadside frame and
    /// rotate that plane pose to the native XY motion direction.
    View180,
    /// Original logic row declares `use360Frames=true`; hold the audited
    /// broadside neutral frame until the original client's pitch mapping is known.
    View360,
    /// The movie's frames are repeated identical source content; use its first.
    Static,
    /// An authored rolling/flight movie whose in-plane spin follows native motion.
    RollingTimeline,
}

pub struct ProjectilePresentation {
    pub selection: PlaybackSelection,
    /// Radians from the export's authored pose in the renderer's x-right/y-down
    /// plane. View-bank depth poses stay neutral while XY motion turns the art.
    pub rotation_radians: f64,
    /// Native vertical delta is carried through for the caller to apply to its
    /// world-to-screen projection; the source sprites do not encode it here.
    pub native_z_delta: Option<i64>,
}

/// Pick the original clip frame from explicit logic semantics and native motion.
/// `motion_dx/dy` are the native x/y deltas over the observer's sample interval.
/// `native_z_delta` is returned unchanged so the scene adapter can project height.
pub fn select_projectile(
    scene: &Scene,
    export: &str,
    mode: ProjectileMovieMode,
    motion_dx: i64,
    motion_dy: i64,
    age_ms: u64,
    native_z_delta: Option<i64>,
) -> Result<ProjectilePresentation, String> {
    let n = scene.frame_count(export).map_err(|e| e.to_string())?;
    if n == 0 {
        return Err(format!("original projectile export {export} has no frames"));
    }
    let (frame_index, mirror) = match mode {
        ProjectileMovieMode::Static => (0, false),
        ProjectileMovieMode::View180 => {
            // Broadside neutral source pose; the movie bank's exact client
            // pitch/view lookup is not recovered. Rotate the authored right-
            // facing sprite to observed screen-plane motion instead.
            (7.min(n - 1), false)
        }
        ProjectileMovieMode::View360 => (15.min(n - 1), false),
        ProjectileMovieMode::Timeline | ProjectileMovieMode::RollingTimeline => {
            let fps = scene.framerate(export).map_err(|e| e.to_string())?;
            let frame = ((age_ms as f64 * fps as f64 / 1000.0).floor() as usize).min(n - 1);
            (frame, false)
        }
    };
    let movie_id = scene.export_id(export).map_err(|e| e.to_string())?;
    let _movie = scene
        .document
        .movies
        .get(&movie_id)
        .ok_or_else(|| format!("missing timeline for original projectile export {export}"))?;
    let playback = Playback {
        frame: Some(frame_index),
        // Explicit root frame takes precedence. Keep authored descendant
        // timelines moving for true time clips, but freeze source pose banks.
        time_seconds: match mode {
            ProjectileMovieMode::Timeline | ProjectileMovieMode::RollingTimeline => {
                Some(age_ms as f64 / 1000.0)
            }
            ProjectileMovieMode::View360
            | ProjectileMovieMode::View180
            | ProjectileMovieMode::Static => None,
        },
        ..Playback::default()
    };
    let rotation_radians = match mode {
        ProjectileMovieMode::View180 if motion_dx != 0 || motion_dy != 0 => {
            (-(motion_dy as f64)).atan2(motion_dx as f64)
        }
        ProjectileMovieMode::View360 if motion_dx != 0 || motion_dy != 0 => {
            (-(motion_dy as f64)).atan2(motion_dx as f64) - std::f64::consts::FRAC_PI_2
        }
        ProjectileMovieMode::RollingTimeline if motion_dx != 0 || motion_dy != 0 => {
            // The original barrel and log roll art lies along the screen's
            // vertical axis, i.e. it is authored rolling east. Turn it with
            // the motion so the roller stays perpendicular to its path.
            (-(motion_dy as f64)).atan2(motion_dx as f64)
        }
        _ => 0.0,
    };
    Ok(ProjectilePresentation {
        selection: PlaybackSelection {
            export: export.to_owned(),
            playback,
            frame_index,
            direction_index: None,
            flip_horizontal: mirror,
            matched_state: true,
        },
        rotation_radians,
        native_z_delta,
    })
}

/// Source-audited presets for the original effects and Barbarian Barrel movies.
/// These are descriptive convenience constants; call sites still supply the
/// exact team-specific export name from the recovered logic row.
pub mod source_modes {
    use super::ProjectileMovieMode;
    pub const ARROW: ProjectileMovieMode = ProjectileMovieMode::View180;
    pub const MORTAR_CATAPULT: ProjectileMovieMode = ProjectileMovieMode::Static;
    pub const CANNONBALL: ProjectileMovieMode = ProjectileMovieMode::Static;
    pub const ROCKET: ProjectileMovieMode = ProjectileMovieMode::View360;
    pub const BARBARIAN_BARREL: ProjectileMovieMode = ProjectileMovieMode::RollingTimeline;
}

/// Resolve rendering mode from the original PROJECTILE row and exact export.
/// `use360Frames` is an explicit source flag; arrow and barrel modes use their
/// exact original clip names. Unrecognized clips remain ordinary timelines.
pub fn mode_for_source(
    fields: &serde_json::Map<String, serde_json::Value>,
    export: &str,
) -> ProjectileMovieMode {
    if fields
        .get("use360Frames")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        return ProjectileMovieMode::View360;
    }
    if export == "projectile_rocket" || export.starts_with("projectile_rocket_") {
        return ProjectileMovieMode::View360;
    }
    if matches!(
        export,
        "projectile_arrow_basic" | "projectile_arrow_basic_enemy" | "projectile_princess"
    ) {
        return ProjectileMovieMode::View180;
    }
    if matches!(
        export,
        "catapult_projectile1" | "projectile_cannonball_small"
    ) {
        return ProjectileMovieMode::Static;
    }
    if export.starts_with("barb_barrel_roll_projectile_")
        || export.starts_with("log_roll_projectile")
        || export == "log_projectile"
    {
        return ProjectileMovieMode::RollingTimeline;
    }
    ProjectileMovieMode::Timeline
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn effects() -> Scene {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("analysis/native-spectator");
        Scene::load(&root, "effects").expect("recovered effects scene")
    }

    #[test]
    fn source_modes_keep_banks_out_of_time_and_rotate_neutral_poses() {
        let scene = effects();
        let north = select_projectile(
            &scene,
            "projectile_rocket",
            source_modes::ROCKET,
            0,
            1,
            0,
            None,
        )
        .unwrap();
        let south = select_projectile(
            &scene,
            "projectile_rocket",
            source_modes::ROCKET,
            0,
            -1,
            5000,
            None,
        )
        .unwrap();
        assert_eq!(north.selection.frame_index, 15);
        assert_eq!(south.selection.frame_index, 15);
        let east = select_projectile(
            &scene,
            "projectile_rocket",
            source_modes::ROCKET,
            1,
            0,
            0,
            None,
        )
        .unwrap();
        let west = select_projectile(
            &scene,
            "projectile_rocket",
            source_modes::ROCKET,
            -1,
            0,
            0,
            None,
        )
        .unwrap();
        assert_eq!(east.selection.frame_index, 15);
        assert_eq!(west.selection.frame_index, 15);
        assert!((north.rotation_radians + std::f64::consts::PI).abs() < 1e-9);
        assert!((east.rotation_radians + std::f64::consts::FRAC_PI_2).abs() < 1e-9);
        assert_eq!(north.selection.playback.time_seconds, None);
        let arrow0 = select_projectile(
            &scene,
            "projectile_arrow_basic_enemy",
            source_modes::ARROW,
            1,
            0,
            0,
            None,
        )
        .unwrap();
        let arrow200 = select_projectile(
            &scene,
            "projectile_arrow_basic_enemy",
            source_modes::ARROW,
            1,
            0,
            200,
            None,
        )
        .unwrap();
        assert_eq!(arrow0.selection.frame_index, 7);
        assert_eq!(arrow200.selection.frame_index, 7);
        assert_eq!(arrow0.selection.playback.time_seconds, None);
        assert!(!arrow0.selection.flip_horizontal);
        assert_eq!(arrow0.rotation_radians, 0.0);
        let arrow_north = select_projectile(
            &scene,
            "projectile_arrow_basic",
            source_modes::ARROW,
            0,
            1,
            0,
            None,
        )
        .unwrap();
        assert!((arrow_north.rotation_radians + std::f64::consts::FRAC_PI_2).abs() < 1e-9);
        let west_arrow = select_projectile(
            &scene,
            "projectile_arrow_basic_enemy",
            source_modes::ARROW,
            -1,
            0,
            0,
            None,
        )
        .unwrap();
        assert!(!west_arrow.selection.flip_horizontal);
    }

    #[test]
    fn repeated_mortar_movie_is_static_and_barrel_keeps_native_age() {
        let scene = effects();
        let mortar = select_projectile(
            &scene,
            "catapult_projectile1",
            source_modes::MORTAR_CATAPULT,
            1,
            1,
            4000,
            Some(12),
        )
        .unwrap();
        assert_eq!(mortar.selection.frame_index, 0);
        assert_eq!(mortar.native_z_delta, Some(12));
        let barrel_scene = Scene::load(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("analysis/native-spectator"),
            "chr_barbarian_barrel_dl",
        )
        .unwrap();
        let barrel = select_projectile(
            &barrel_scene,
            "barb_barrel_roll_projectile_blue",
            source_modes::BARBARIAN_BARREL,
            1,
            1,
            100,
            None,
        )
        .unwrap();
        assert_eq!(barrel.selection.frame_index, 6);
        // North-east travel turns the east-authored roller by -45 degrees.
        assert!((barrel.rotation_radians + std::f64::consts::FRAC_PI_4).abs() < 1e-9);
        let empty = serde_json::Map::new();
        assert_eq!(
            mode_for_source(&empty, "log_roll_projectile_red"),
            ProjectileMovieMode::RollingTimeline
        );
    }

    #[test]
    fn source_field_resolver_and_arrow_neutral_pose_phase_are_explicit() {
        let mut fields = serde_json::Map::new();
        fields.insert("use360Frames".into(), serde_json::Value::Bool(true));
        assert_eq!(
            mode_for_source(&fields, "projectile_rocket"),
            ProjectileMovieMode::View360
        );
        fields.insert("use360Frames".into(), serde_json::Value::Bool(false));
        assert_eq!(
            mode_for_source(&fields, "projectile_arrow_basic"),
            ProjectileMovieMode::View180
        );
        assert_eq!(
            mode_for_source(&fields, "projectile_princess"),
            ProjectileMovieMode::View180
        );
        assert_eq!(
            mode_for_source(&fields, "catapult_projectile1"),
            ProjectileMovieMode::Static
        );

        let scene = effects();
        for (export, dx, dy) in [
            ("projectile_arrow_basic", 0, 1),
            ("projectile_arrow_basic", 1, 0),
            ("projectile_arrow_basic", 0, -1),
            ("projectile_arrow_basic", -1, 0),
            ("projectile_arrow_basic_enemy", 0, 1),
        ] {
            let selected = select_projectile(
                &scene,
                export,
                ProjectileMovieMode::View180,
                dx,
                dy,
                0,
                None,
            )
            .unwrap();
            assert_eq!(
                selected.selection.frame_index, 7,
                "{export} dx={dx} dy={dy}"
            );
            assert!(!selected.selection.flip_horizontal);
        }
    }
}
