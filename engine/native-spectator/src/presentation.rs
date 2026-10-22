//! Original logic rows plus legacy effect/emitter definitions written by
//! `tools/spectator_presentation.py`. Everything here is presentation input:
//! it selects artwork, timing hints and particle parameters, never game state.
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Default, Deserialize)]
pub struct RowLogic {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub fields: Map<String, Value>,
}

impl RowLogic {
    pub fn str(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(Value::as_str)
    }
    pub fn num(&self, key: &str) -> Option<f64> {
        self.fields.get(key).and_then(Value::as_f64)
    }
    pub fn flag(&self, key: &str) -> bool {
        self.fields
            .get(key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }
    pub fn is_projectile(&self) -> bool {
        self.r#type.as_deref() == Some("PROJECTILE")
    }
    pub fn is_area_effect(&self) -> bool {
        self.r#type.as_deref() == Some("AEO")
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectComponent {
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub particle_emitter_name: Option<String>,
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub export_name: Option<String>,
    /// Delay in milliseconds; negative values start immediately.
    #[serde(default)]
    pub time: Option<f64>,
    #[serde(default)]
    pub renderable_scale: Option<f64>,
    #[serde(default)]
    pub flash_width: Option<f64>,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectDef {
    #[serde(default)]
    pub renderable_components: Vec<EffectComponent>,
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default, rename = "Loop")]
    pub looping: bool,
    #[serde(default)]
    pub enemy_version: Option<String>,
    #[serde(default)]
    pub follow_parent: bool,
}

/// Fields of an original legacy particle emitter. Units follow the source
/// data: times in milliseconds, distances in SC pixels, speeds per second.
#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct EmitterDef {
    pub particle_count: Option<f64>,
    pub particle_export_name: Vec<String>,
    pub particle_resource: Vec<String>,
    pub min_life: Option<f64>,
    pub max_life: Option<f64>,
    pub particle_min_life: Option<f64>,
    pub particle_max_life: Option<f64>,
    pub particle_min_interval: Option<f64>,
    pub particle_max_interval: Option<f64>,
    pub particle_min_speed: Option<f64>,
    pub particle_max_speed: Option<f64>,
    pub particle_min_velocity_z: Option<f64>,
    pub particle_max_velocity_z: Option<f64>,
    pub particle_min_angle: Option<f64>,
    pub particle_max_angle: Option<f64>,
    pub particle_min_radius: Option<f64>,
    pub particle_max_radius: Option<f64>,
    pub particle_start_z: Option<f64>,
    pub particle_start_x: Option<f64>,
    #[serde(rename = "ParticleStartXYAreaRadius")]
    pub particle_start_xy_area_radius: Option<f64>,
    pub particle_gravity: Option<f64>,
    pub particle_random_angle: bool,
    pub particle_radius_aspect: Option<f64>,
    pub inertia: Option<f64>,
    pub start_scale: Option<f64>,
    pub end_scale: Option<f64>,
    pub random_scale: Option<f64>,
    pub fade_in_duration: Option<f64>,
    pub fade_out_duration: Option<f64>,
    pub rotate_to_direction: bool,
    pub rotate_min_speed: Option<f64>,
    pub rotate_max_speed: Option<f64>,
    pub loop_particle_clip: bool,
    pub play_particle_clip_once: bool,
    pub no_bounce: bool,
    pub stop_on_bounce: bool,
}

#[derive(Default, Deserialize)]
pub struct Presentation {
    #[serde(default)]
    pub rows: BTreeMap<String, RowLogic>,
    #[serde(default)]
    pub effects: BTreeMap<String, EffectDef>,
    #[serde(default)]
    pub emitters: BTreeMap<String, EmitterDef>,
}

impl Presentation {
    /// Missing file means older bundles: the viewer still renders, without
    /// logic-timed attacks or original particle effects.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("presentation.json");
        if !path.is_file() {
            eprintln!(
                "native-spectator: {} missing; run tools/spectator_presentation.py for original effects",
                path.display()
            );
            return Ok(Self::default());
        }
        let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn row(&self, name: &str) -> Option<&RowLogic> {
        self.rows.get(name)
    }

    /// Rows that fire `projectile` (source `Projectile` field).
    pub fn shooters_of<'a>(&'a self, projectile: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.rows
            .iter()
            .filter(move |(_, row)| row.str("Projectile") == Some(projectile))
            .map(|(name, _)| name.as_str())
    }
}
