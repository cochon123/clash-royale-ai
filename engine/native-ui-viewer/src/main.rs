use clash_native_ui_viewer::{Playback, Rendered, Result, Scene};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use serde_json::json;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

struct Options {
    root: PathBuf,
    document: String,
    export: Option<String>,
    object_id: Option<u32>,
    frame: Option<usize>,
    label: Option<String>,
    object_frames: BTreeMap<u32, usize>,
    list: bool,
    inspect: bool,
    play: bool,
    render: Option<PathBuf>,
    metadata: Option<PathBuf>,
    window_frames: Option<usize>,
}

fn arguments() -> Result<Options> {
    let mut options = Options {
        root: Path::new(env!("CARGO_MANIFEST_DIR")).join("../analysis/native-ui"),
        document: "ui".into(),
        export: None,
        object_id: None,
        frame: None,
        label: None,
        object_frames: BTreeMap::new(),
        list: false,
        inspect: false,
        play: false,
        render: None,
        metadata: None,
        window_frames: None,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{arg} requires a value"));
        match arg.as_str() {
            "--assets" => options.root = value()?.into(),
            "--document" => options.document = value()?,
            "--export" => options.export = Some(value()?),
            "--object" => options.object_id = Some(value()?.parse()?),
            "--frame" => options.frame = Some(value()?.parse()?),
            "--label" => options.label = Some(value()?),
            "--object-frame" => {
                let setting = value()?;
                let (id, frame) = setting
                    .split_once(':')
                    .ok_or("--object-frame syntax is OBJECT_ID:FRAME_INDEX")?;
                options.object_frames.insert(id.parse()?, frame.parse()?);
            }
            "--render" => options.render = Some(value()?.into()),
            "--metadata" => options.metadata = Some(value()?.into()),
            "--window-frames" => options.window_frames = Some(value()?.parse()?),
            "--list" => options.list = true,
            "--inspect" => options.inspect = true,
            "--play" => options.play = true,
            "-h" | "--help" => {
                println!(
                    "Original SC5 display-object viewer (asset playback, not a battle client)\n\
                    --assets DIR           Recovered analysis/native-ui directory\n\
                    --document NAME        ui, ui_ability_button, ui_spectate_hud, etc.\n\
                    --list                 List original exports and frame counts\n\
                    --export NAME          Choose an original named export\n\
                    --object ID            Render one original object id instead of an export\n\
                    --inspect              Print original root instances and frame labels\n\
                    --frame N              Explicit root timeline frame\n\
                    --label NAME           Select an original state label recursively\n\
                    --object-frame ID:N    Set a descendant's authored state frame\n\
                    --play                 Play root timeline at its original frame rate\n\
                    --render FILE.png      Render entirely in Rust without a display\n\
                    --metadata FILE.json   Record source identity, origin and render gaps\n\
                    --window-frames N      Close the viewer after N displayed frames\n\n\
                    Window keys: Space play/pause; Left/Right step; Tab next export;\n\
                    Shift+Tab previous export; Escape close. Native text fields are\n\
                    explicitly skipped; original font and text data remain recovered."
                );
                std::process::exit(0);
            }
            _ => return Err(format!("Unknown option {arg}; use --help").into()),
        }
    }
    Ok(options)
}

fn metadata(
    scene: &Scene,
    export: &str,
    playback: &Playback,
    rendered: Option<&Rendered>,
) -> Result<serde_json::Value> {
    Ok(json!({
        "kind": "original SC5 asset playback; not a native battle client",
        "document": scene.document_path,
        "original_sc_source": scene.document.source,
        "original_sc_sha256": scene.document.sha256,
        "export": export,
        "object_id": scene.export_id(export)?,
        "frame": playback.frame,
        "label": playback.label,
        "object_frames": playback.object_frames,
        "original_frame_count": scene.frame_count(export)?,
        "original_framerate": scene.framerate(export)?,
        "render": rendered.map(|r| json!({"width":r.image.width(),"height":r.image.height(),"origin_x":r.x,"origin_y":r.y,"stats":r.stats})),
        "limitations": [
            "Native text fields and text layout are skipped; original fonts and metadata are preserved.",
            "Native code must bind game state, select child states and position/resize controls.",
            "Descendants remain at default or explicitly selected frames; only the root auto-advances.",
            "Color-transform selection and raster sampling follow the independent Python asset reference; original native parity is not established.",
            "Scaling grids are retained in source documents but resizing/9-slice layout is not implemented."
        ]
    }))
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(value)? + "\n")?;
    Ok(())
}

fn draw_window(
    rendered: Option<&Rendered>,
    buffer: &mut [u32],
    width: usize,
    height: usize,
    bounds: &mut Option<[f64; 4]>,
) {
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = if (x / 16 + y / 16) % 2 == 0 {
                0x192333
            } else {
                0x222e40
            };
        }
    }
    let Some(rendered) = rendered else {
        return;
    };
    let current = [
        rendered.x,
        rendered.y,
        rendered.x + rendered.image.width() as f64,
        rendered.y + rendered.image.height() as f64,
    ];
    let bound = match bounds {
        Some(b) => {
            b[0] = b[0].min(current[0]);
            b[1] = b[1].min(current[1]);
            b[2] = b[2].max(current[2]);
            b[3] = b[3].max(current[3]);
            *b
        }
        None => {
            *bounds = Some(current);
            current
        }
    };
    let scale = ((width as f64 - 40.) / (bound[2] - bound[0]).max(1.))
        .min((height as f64 - 40.) / (bound[3] - bound[1]).max(1.))
        .min(4.);
    let left =
        (width as f64 - (bound[2] - bound[0]) * scale) / 2. + (rendered.x - bound[0]) * scale;
    let top =
        (height as f64 - (bound[3] - bound[1]) * scale) / 2. + (rendered.y - bound[1]) * scale;
    let out_width = (rendered.image.width() as f64 * scale).ceil() as usize;
    let out_height = (rendered.image.height() as f64 * scale).ceil() as usize;
    for y in 0..out_height {
        for x in 0..out_width {
            let (dx, dy) = (left as isize + x as isize, top as isize + y as isize);
            if dx < 0 || dy < 0 || dx >= width as isize || dy >= height as isize {
                continue;
            }
            let sx = ((x as f64 / scale) as u32).min(rendered.image.width() - 1);
            let sy = ((y as f64 / scale) as u32).min(rendered.image.height() - 1);
            let p = rendered.image.get_pixel(sx, sy);
            let offset = dy as usize * width + dx as usize;
            let background = buffer[offset];
            let alpha = u32::from(p[3]);
            let mut color = 0;
            for (component, shift) in [(0, 16), (1, 8), (2, 0)] {
                let b = (background >> shift) & 255;
                color |= ((u32::from(p[component]) * alpha + b * (255 - alpha)) / 255) << shift;
            }
            buffer[offset] = color;
        }
    }
}

fn run() -> Result<()> {
    let options = arguments()?;
    let scene = Scene::load(&options.root, &options.document)?;
    if options.list {
        for (name, id) in &scene.document.exports {
            println!(
                "{name}\tobject={id}\tframes={}\tfps={}",
                scene.frame_count(name)?,
                scene.framerate(name)?
            );
        }
        return Ok(());
    }
    let mut export = options.export.unwrap_or_else(|| {
        if scene.document.exports.contains_key("HUD_player") {
            "HUD_player".into()
        } else {
            scene
                .document
                .exports
                .keys()
                .next()
                .cloned()
                .unwrap_or_default()
        }
    });
    let id = scene.export_id(&export)?;
    if options.inspect {
        let movie = scene.document.movies.get(&id);
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "export": export, "object_id": id,
                "frames": scene.frame_count(&export)?, "framerate": scene.framerate(&export)?,
                "frame_labels": movie.map(|m| m.frame_labels.iter().enumerate().filter(|(_,s)|!s.is_empty()).collect::<Vec<_>>()),
                "children": movie.map(|m| m.children_ids.iter().enumerate().map(|(i,id)|json!({"index":i,"object":id,"name":m.children_names.get(i)})).collect::<Vec<_>>())
            }))?
        );
        return Ok(());
    }
    let mut playback = Playback {
        frame: options.frame,
        label: options.label,
        object_frames: options.object_frames,
        ..Playback::default()
    };
    let mut rendered = if let Some(object_id) = options.object_id {
        scene.render_id(object_id, &playback)?
    } else {
        scene.render(&export, &playback)?
    };
    if let Some(path) = options.render {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        if let Some(rendered) = &rendered {
            rendered.image.save(&path)?;
        } else {
            image::RgbaImage::new(1, 1).save(&path)?;
        }
        let report = if let Some(object_id) = options.object_id {
            json!({
                "kind": "original SC5 object playback; not a native battle client",
                "object_id": object_id,
                "render": rendered.as_ref().map(|r| json!({"width":r.image.width(),"height":r.image.height(),"origin_x":r.x,"origin_y":r.y})),
            })
        } else {
            metadata(&scene, &export, &playback, rendered.as_ref())?
        };
        if let Some(path) = options.metadata {
            write_json(&path, &report)?;
        }
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }
    eprintln!("Original asset viewer. Native text/layout/game-state bindings are absent. Space: play/pause, arrows: step, Tab: next export, Escape: close.");
    let (width, height) = (1100, 800);
    let mut window = Window::new(
        "Original Clash SC5 asset viewer",
        width,
        height,
        WindowOptions::default(),
    )?;
    window.set_target_fps(60);
    let mut buffer = vec![0u32; width * height];
    let exports: Vec<String> = scene.document.exports.keys().cloned().collect();
    let mut export_index = exports.iter().position(|e| *e == export).unwrap_or(0);
    let mut playing = options.play;
    let mut start = Instant::now();
    let mut start_frame = playback
        .frame
        .or_else(|| {
            playback.label.as_ref().and_then(|label| {
                scene
                    .document
                    .movies
                    .get(&id)?
                    .frame_labels
                    .iter()
                    .position(|candidate| candidate == label)
            })
        })
        .unwrap_or(0);
    let mut frame = start_frame;
    let mut bounds = None;
    let mut displayed_frames = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut changed = false;
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            playing = !playing;
            start = Instant::now();
            start_frame = frame;
        }
        if window.is_key_pressed(Key::Tab, KeyRepeat::No) {
            let previous =
                window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift);
            export_index = if previous {
                (export_index + exports.len() - 1) % exports.len()
            } else {
                (export_index + 1) % exports.len()
            };
            export = exports[export_index].clone();
            frame = 0;
            start_frame = 0;
            start = Instant::now();
            bounds = None;
            changed = true;
        }
        let count = scene.frame_count(&export)?;
        if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            frame = (frame + 1) % count;
            playing = false;
            changed = true;
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            frame = (frame + count - 1) % count;
            playing = false;
            changed = true;
        }
        if playing {
            let next = (start_frame
                + (start.elapsed().as_secs_f64() * scene.framerate(&export)? as f64) as usize)
                % count;
            if next != frame {
                frame = next;
                changed = true;
            }
        }
        if changed {
            playback.frame = Some(frame);
            rendered = scene.render(&export, &playback)?;
        }
        draw_window(rendered.as_ref(), &mut buffer, width, height, &mut bounds);
        let skipped = rendered
            .as_ref()
            .map_or(0, |r| r.stats.skipped_text_fields.len());
        window.set_title(&format!(
            "Clash asset viewer (no battle client): {} / {} | frame {}/{} | text skipped: {} | {}",
            options.document,
            export,
            frame,
            count,
            skipped,
            if playing { "playing" } else { "paused" }
        ));
        window.update_with_buffer(&buffer, width, height)?;
        displayed_frames += 1;
        if options
            .window_frames
            .is_some_and(|limit| displayed_frames >= limit)
        {
            break;
        }
    }
    if let Some(path) = options.metadata {
        write_json(
            &path,
            &metadata(&scene, &export, &playback, rendered.as_ref())?,
        )?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("SC5 viewer: {error}");
        std::process::exit(1);
    }
}
