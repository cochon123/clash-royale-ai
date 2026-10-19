use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use clash_rust::{Battle, BattleConfig, CardKind, Event, Team, Vec2};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    process::Stdio,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::{Mutex, RwLock},
};
use tower_http::services::ServeDir;

type Shared = Arc<App>;
type ApiError = (StatusCode, Json<Value>);
fn bad(message: impl ToString) -> ApiError {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": message.to_string()})),
    )
}
fn not_found() -> ApiError {
    (
        StatusCode::NOT_FOUND,
        Json(json!({"error":"Match not found"})),
    )
}

struct App {
    matches: RwLock<HashMap<u64, Arc<Mutex<Session>>>>,
    next: AtomicU64,
    models: Value,
}

#[derive(Clone, Deserialize, Serialize)]
struct Side {
    controller: String,
    deck: Vec<String>,
}
#[derive(Clone, Deserialize, Serialize)]
struct MatchRequest {
    blue: Side,
    red: Side,
    #[serde(default = "seed_default")]
    seed: u64,
    #[serde(default = "speed_default")]
    speed: f64,
}
fn seed_default() -> u64 {
    42
}
fn speed_default() -> f64 {
    1.0
}

struct Session {
    id: u64,
    request: MatchRequest,
    battle: Battle,
    paused: bool,
    stopped: bool,
    error: Option<String>,
    history: Vec<Value>,
    recent_events: Vec<Value>,
    decisions: [u64; 2],
    policy_actions: Vec<Value>,
}
impl Session {
    fn view(&self) -> Value {
        json!({"id":self.id,"config":self.request,"paused":self.paused,"stopped":self.stopped,"error":self.error,
            "tick":self.battle.tick,"elapsed_ms":self.battle.elapsed_ms(),"remaining_ms":self.battle.remaining_ms(),
            "overtime":self.battle.overtime,"outcome":self.battle.outcome,"crowns":self.battle.crowns(),
            "players":self.battle.players,"entities":self.battle.entities,"events":self.recent_events,
            "decisions":self.decisions,"plays":self.history.len(),
            "policy_adjustments":self.policy_actions.iter().filter(|a|a["adjusted"]==true).count(),
            "policy_rejections":self.policy_actions.iter().filter(|a|a.get("error").is_some()).count()})
    }
    fn collect_events(&mut self) {
        for event in &self.battle.events {
            if let Event::Played { team, card_id, pos } = event {
                self.history.push(json!({"team":team_name(*team),"elapsed_ms":self.battle.elapsed_ms(),"card":card_id,"x":pos.x,"y":pos.y}));
            }
            self.recent_events
                .push(json!({"tick":self.battle.tick,"event":event}));
        }
        if self.recent_events.len() > 100 {
            self.recent_events.drain(..self.recent_events.len() - 100);
        }
        self.battle.events.clear();
    }
    fn replay(&self) -> Value {
        json!({"format":"clash-rust-actions-v1","engine_version":env!("CARGO_PKG_VERSION"),"config":self.request,"history":self.history,"policy_actions":self.policy_actions,"outcome":self.battle.outcome,"tick":self.battle.tick,"stopped":self.stopped,"error":self.error})
    }
}
fn team_name(team: Team) -> &'static str {
    if team == Team::Blue {
        "blue"
    } else {
        "red"
    }
}
fn parse_team(name: &str) -> Result<Team, ApiError> {
    match name.to_lowercase().as_str() {
        "blue" => Ok(Team::Blue),
        "red" => Ok(Team::Red),
        _ => Err(bad("Team must be blue or red")),
    }
}

pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut host = "127.0.0.1".to_string();
    let mut port = 8788u16;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--host" => host = args.next().ok_or("--host requires a value")?,
            "--port" => port = args.next().ok_or("--port requires a value")?.parse()?,
            "--help" => {
                println!("clash-rust [--host 127.0.0.1] [--port 8788]\nOpen the printed URL to configure and watch a match.");
                return Ok(());
            }
            _ => return Err(format!("Unknown argument: {arg}").into()),
        }
    }
    let models = match Command::new("python3")
        .arg(root().join("tools/policy_bridge.py"))
        .arg("--list")
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            serde_json::from_slice(&output.stdout).unwrap_or(json!({"models":[]}))
        }
        _ => json!({"models":[]}),
    };
    let app = Arc::new(App {
        matches: RwLock::new(HashMap::new()),
        next: AtomicU64::new(1),
        models,
    });
    let router = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../web/index.html")) }),
        )
        .route(
            "/api/health",
            get(|| async {
                Json(json!({"ok":true,"engine":"rust","version":env!("CARGO_PKG_VERSION")}))
            }),
        )
        .route("/api/catalog", get(catalog))
        .route("/api/matches", post(create_match))
        .route("/api/matches/{id}", get(match_state))
        .route("/api/matches/{id}/action", post(action))
        .route("/api/matches/{id}/control", post(control))
        .route("/api/matches/{id}/replay", get(replay))
        .nest_service("/assets", ServeDir::new(root().join("web")))
        .with_state(app);
    let listener = tokio::net::TcpListener::bind((host.as_str(), port)).await?;
    println!("Arena Lab is running at http://{host}:{port}\nRust simulation · local models · Ctrl+C to stop");
    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
fn root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
async fn catalog(State(app): State<Shared>) -> Json<Value> {
    let mut controllers = vec![
        json!({"id":"human","name":"Human","description":"Select a card, then click the arena."}),
        json!({"id":"balanced","name":"Balanced bot","description":"Seeded baseline with mixed pressure."}),
        json!({"id":"cycle","name":"Cycle bot","description":"Deploys the cheapest available card."}),
        json!({"id":"pressure","name":"Pressure bot","description":"Builds pushes with expensive troops."}),
    ];
    for model in app.models["models"].as_array().into_iter().flatten() {
        if model["available"].as_bool() == Some(true) {
            controllers.push(json!({"id":format!("policy:{}",model["id"].as_str().unwrap_or("")),"name":model["name"],"description":"Your trained policy, running locally on CPU."}));
        }
    }
    let presets: Value = std::fs::read_to_string(root().join("data/presets.json"))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(json!([]));
    Json(
        json!({"cards":clash_rust::card_catalog(),"presets":presets,"controllers":controllers,"models":app.models,
        "engine":"Offline reconstruction: extracted baseline stats; battle mechanics are approximate. Not validated for training transfer to the original game."}),
    )
}
async fn create_match(
    State(app): State<Shared>,
    Json(request): Json<MatchRequest>,
) -> Result<Json<Value>, ApiError> {
    if !request.speed.is_finite() || !(0.25..=32.).contains(&request.speed) {
        return Err(bad("Speed must be between 0.25 and 32"));
    }
    for side in [&request.blue, &request.red] {
        if !["human", "balanced", "cycle", "pressure"].contains(&side.controller.as_str())
            && !side.controller.starts_with("policy:")
        {
            return Err(bad("Unknown controller"));
        }
        if let Some(model) = side.controller.strip_prefix("policy:") {
            if !app.models["models"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|v| v["id"] == model && v["available"] == true)
            {
                return Err(bad(format!("Model {model} is unavailable")));
            }
        }
    }
    let battle = Battle::with_decks(
        request.seed,
        [request.blue.deck.clone(), request.red.deck.clone()],
        BattleConfig::default(),
    )
    .map_err(bad)?;
    // Load both requested models before publishing the match. A model failure never becomes a different bot.
    let mut bridges = [None, None];
    for (i, side) in [&request.blue, &request.red].iter().enumerate() {
        if let Some(model) = side.controller.strip_prefix("policy:") {
            bridges[i] = Some(Bridge::start(model).await.map_err(bad)?);
        }
    }
    let mut matches = app.matches.write().await;
    if matches.len() >= 64 {
        return Err(bad(
            "Session limit reached; restart the local server to clear old matches",
        ));
    }
    let id = app.next.fetch_add(1, Ordering::Relaxed);
    let session = Arc::new(Mutex::new(Session {
        id,
        request,
        battle,
        paused: false,
        stopped: false,
        error: None,
        history: vec![],
        recent_events: vec![],
        decisions: [0, 0],
        policy_actions: vec![],
    }));
    matches.insert(id, session.clone());
    drop(matches);
    tokio::spawn(run_match(session.clone(), bridges));
    let view = session.lock().await.view();
    Ok(Json(view))
}
async fn lookup(app: &App, id: u64) -> Result<Arc<Mutex<Session>>, ApiError> {
    app.matches
        .read()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(not_found)
}
async fn match_state(
    State(app): State<Shared>,
    Path(id): Path<u64>,
) -> Result<Json<Value>, ApiError> {
    let s = lookup(&app, id).await?;
    let view = s.lock().await.view();
    Ok(Json(view))
}
async fn replay(State(app): State<Shared>, Path(id): Path<u64>) -> Result<Json<Value>, ApiError> {
    let s = lookup(&app, id).await?;
    let view = s.lock().await.replay();
    Ok(Json(view))
}
#[derive(Deserialize)]
struct Action {
    team: String,
    hand_index: usize,
    x: i32,
    y: i32,
}
async fn action(
    State(app): State<Shared>,
    Path(id): Path<u64>,
    Json(action): Json<Action>,
) -> Result<Json<Value>, ApiError> {
    let session = lookup(&app, id).await?;
    let mut s = session.lock().await;
    let team = parse_team(&action.team)?;
    let controller = if team == Team::Blue {
        &s.request.blue.controller
    } else {
        &s.request.red.controller
    };
    if controller != "human" {
        return Err(bad("This side is AI controlled"));
    }
    if s.paused || s.stopped || s.error.is_some() {
        return Err(bad("Resume the match before playing"));
    }
    s.battle
        .play(team, action.hand_index, Vec2::new(action.x, action.y))
        .map_err(bad)?;
    s.collect_events();
    Ok(Json(s.view()))
}
#[derive(Deserialize)]
struct Control {
    paused: Option<bool>,
    speed: Option<f64>,
    stop: Option<bool>,
}
async fn control(
    State(app): State<Shared>,
    Path(id): Path<u64>,
    Json(control): Json<Control>,
) -> Result<Json<Value>, ApiError> {
    if let Some(speed) = control.speed {
        if !speed.is_finite() || !(0.25..=32.).contains(&speed) {
            return Err(bad("Invalid speed"));
        }
    }
    let session = lookup(&app, id).await?;
    let mut s = session.lock().await;
    if let Some(paused) = control.paused {
        s.paused = paused;
    }
    if let Some(speed) = control.speed {
        s.request.speed = speed;
    }
    if control.stop == Some(true) {
        s.stopped = true;
    }
    Ok(Json(s.view()))
}

struct Bridge {
    child: Child,
    input: ChildStdin,
    output: tokio::io::Lines<BufReader<ChildStdout>>,
}
impl Bridge {
    async fn start(model: &str) -> Result<Self, String> {
        let mut child = Command::new("python3")
            .arg(root().join("tools/policy_bridge.py"))
            .arg("--model")
            .arg(model)
            .env("OMP_NUM_THREADS", "1")
            .env("MKL_NUM_THREADS", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| e.to_string())?;
        let input = child.stdin.take().ok_or("Missing bridge stdin")?;
        let output = BufReader::new(child.stdout.take().ok_or("Missing bridge stdout")?).lines();
        let mut bridge = Self {
            child,
            input,
            output,
        };
        let ready = bridge.read().await?;
        if ready["ready"] != true {
            return Err(format!("Model failed to load: {ready}"));
        }
        Ok(bridge)
    }
    async fn read(&mut self) -> Result<Value, String> {
        let line = tokio::time::timeout(Duration::from_secs(60), self.output.next_line())
            .await
            .map_err(|_| "Model response timed out")?
            .map_err(|e| e.to_string())?
            .ok_or("Model process stopped")?;
        serde_json::from_str(&line).map_err(|e| format!("Invalid model response: {e}"))
    }
    async fn decide(&mut self, request: &Value) -> Result<Value, String> {
        let mut line = serde_json::to_vec(request).map_err(|e| e.to_string())?;
        line.push(b'\n');
        self.input
            .write_all(&line)
            .await
            .map_err(|e| e.to_string())?;
        self.input.flush().await.map_err(|e| e.to_string())?;
        let result = self.read().await?;
        if let Some(error) = result.get("error") {
            return Err(error.to_string());
        }
        Ok(result)
    }
}
impl Drop for Bridge {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

async fn run_match(session: Arc<Mutex<Session>>, mut bridges: [Option<Bridge>; 2]) {
    let mut next_query = [0u64; 2];
    let mut pending: [Option<(u64, usize, Value)>; 2] = [None, None];
    let mut next_tick = tokio::time::Instant::now();
    let mut last_speed = 0.;
    loop {
        let (paused, speed, finished) = {
            let s = session.lock().await;
            (
                s.paused,
                s.request.speed,
                s.battle.outcome.is_some() || s.error.is_some() || s.stopped,
            )
        };
        if finished {
            let s = session.lock().await;
            let path = root().join("runs");
            let _ = tokio::fs::create_dir_all(&path).await;
            let _ = tokio::fs::write(
                path.join(format!("match-{}.json", s.id)),
                s.replay().to_string(),
            )
            .await;
            break;
        }
        if paused {
            tokio::time::sleep(Duration::from_millis(50)).await;
            next_tick = tokio::time::Instant::now();
            continue;
        }
        if speed != last_speed {
            next_tick = tokio::time::Instant::now();
            last_speed = speed;
        }
        for team in [Team::Blue, Team::Red] {
            let i = team.index();
            let mut s = session.lock().await;
            if s.paused || s.stopped || s.error.is_some() || s.battle.outcome.is_some() {
                break;
            }
            let now = s.battle.elapsed_ms();
            if pending[i]
                .as_ref()
                .is_some_and(|(_, history_len, _)| *history_len != s.history.len())
            {
                pending[i] = None;
                next_query[i] = now;
            }
            if let Some((due, _, decision)) = &pending[i] {
                if now >= *due {
                    let slot = decision["hand_index"].as_u64().unwrap_or(99) as usize;
                    let x = decision["x"].as_i64().unwrap_or(-1) as i32;
                    let y = decision["y"].as_i64().unwrap_or(-1) as i32;
                    // Project a model's continuous prediction to this engine's legal rectangle.
                    // Both the prediction and actual execution are retained in the replay.
                    let bounds = s
                        .battle
                        .legal_hand(team)
                        .get(slot)
                        .map(|c| (c.x_min, c.x_max, c.y_min, c.y_max));
                    let (px, py) = bounds
                        .map(|(x0, x1, y0, y1)| (x.clamp(x0, x1), y.clamp(y0, y1)))
                        .unwrap_or((x, y));
                    let result = s.battle.play(team, slot, Vec2::new(px, py));
                    let mut record = json!({"team":team_name(team),"elapsed_ms":now,"proposal":decision,"x":px,"y":py,"adjusted":x!=px||y!=py});
                    if let Err(error) = result {
                        record["error"] = json!(error.to_string());
                    }
                    s.policy_actions.push(record);
                    s.collect_events();
                    pending[i] = None;
                    next_query[i] = now + 500;
                }
            }
            if now < next_query[i] || pending[i].is_some() {
                continue;
            }
            let side = if i == 0 {
                s.request.blue.clone()
            } else {
                s.request.red.clone()
            };
            if side.controller == "human" {
                continue;
            }
            s.decisions[i] += 1;
            if let Some(bridge) = bridges[i].as_mut() {
                let other = if i == 0 {
                    &s.request.red
                } else {
                    &s.request.blue
                };
                let request = json!({"model":side.controller.strip_prefix("policy:").unwrap_or(""),"team":team_name(team),"match_id":s.id,"seed":s.request.seed,"elapsed_ms":now,
                    "deck":side.deck,"opponent_deck":other.deck,"hand":s.battle.players[i].hand,"elixir":s.battle.players[i].elixir(),"history":s.history});
                let expected_history = s.history.len();
                drop(s);
                match bridge.decide(&request).await {
                    Ok(decision) => {
                        let current = session.lock().await;
                        if current.history.len() != expected_history
                            || current.paused
                            || current.stopped
                            || current.error.is_some()
                            || current.battle.outcome.is_some()
                        {
                            next_query[i] = now;
                            continue;
                        }
                        drop(current);
                        if decision["wait"].as_bool().unwrap_or(false) {
                            next_query[i] = now
                                + decision["requery_after_ms"]
                                    .as_u64()
                                    .unwrap_or(500)
                                    .clamp(50, 10000);
                        } else {
                            let delay = decision["delay_ms"].as_u64().unwrap_or(0).min(12000);
                            pending[i] = Some((now + delay, expected_history, decision));
                        }
                    }
                    Err(error) => {
                        let mut s = session.lock().await;
                        s.error = Some(format!("{} policy failed: {error}", team_name(team)));
                    }
                }
            } else {
                if side.controller == "balanced" {
                    s.battle.bot_step(team);
                } else {
                    simple_bot(&mut s.battle, team, &side.controller);
                }
                s.collect_events();
                next_query[i] = now + 500;
            }
        }
        {
            let mut s = session.lock().await;
            if !s.paused && !s.stopped && s.error.is_none() {
                s.battle.step();
                s.collect_events();
            }
        }
        // Anchor deadlines to simulation time rather than adding timer rounding
        // and computation cost to every tick at high playback speeds.
        next_tick += Duration::from_secs_f64(0.05 / speed);
        let now = tokio::time::Instant::now();
        if now.saturating_duration_since(next_tick) > Duration::from_millis(250) {
            next_tick = now;
        }
        tokio::time::sleep_until(next_tick).await;
    }
}
fn simple_bot(battle: &mut Battle, team: Team, strategy: &str) {
    let p = &battle.players[team.index()];
    let mut choices: Vec<_> = p
        .hand
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            battle
                .card(id)
                .filter(|c| i32::from(c.elixir) * 1000 <= p.elixir_milli)
                .map(|c| (i, c.clone()))
        })
        .collect();
    if strategy == "pressure" {
        choices.sort_by_key(|(_, c)| std::cmp::Reverse(c.elixir));
    } else {
        choices.sort_by_key(|(_, c)| c.elixir);
    }
    if let Some((slot, card)) = choices.first() {
        let lane = if (battle.tick / 100 + team.index() as u64).is_multiple_of(2) {
            4500
        } else {
            13500
        };
        let target = if card.kind == CardKind::Spell || card.can_deploy_enemy_side {
            battle
                .entities
                .iter()
                .find(|e| e.team != team && e.hp > 0 && e.kind == clash_rust::EntityKind::Troop)
                .map(|e| e.pos)
                .unwrap_or(Vec2::new(
                    lane,
                    if team == Team::Blue { 6500 } else { 25500 },
                ))
        } else {
            Vec2::new(lane, if team == Team::Blue { 19500 } else { 12500 })
        };
        let _ = battle.play(team, *slot, target);
    }
}
