use clash_rust::*;

fn with_first(id: &str) -> Battle {
    let mut deck = default_deck();
    let index = deck.iter().position(|c| c == id).unwrap();
    deck.swap(0, index);
    Battle::with_decks(7, [deck.clone(), deck], BattleConfig::default()).unwrap()
}

#[test]
fn validated_eight_card_deck_and_four_card_hand_cycle() {
    let mut b = Battle::new(1);
    assert_eq!(b.players[0].hand.len(), 4);
    assert_eq!(b.players[0].queue.len(), 4);
    let played = b.players[0].hand[0].clone();
    let next = b.players[0].next_card().unwrap().to_owned();
    b.play(Team::Blue, 0, Vec2::new(4500, 20000)).unwrap();
    assert_eq!(b.players[0].hand[0], next);
    assert_eq!(b.players[0].queue.back(), Some(&played));
    let mut bad = default_deck();
    bad[1] = bad[0].clone();
    assert!(matches!(
        Battle::with_decks(1, [bad, default_deck()], BattleConfig::default()),
        Err(PlayError::InvalidDeck(_))
    ));
}

#[test]
fn invalid_actions_never_spend_elixir_or_cycle_hand() {
    let mut b = Battle::new(1);
    let before = serde_json::to_string(&b.players).unwrap();
    assert_eq!(
        b.play(Team::Blue, 4, Vec2::new(4500, 20000)),
        Err(PlayError::InvalidHandSlot)
    );
    assert_eq!(
        b.play(Team::Blue, 0, Vec2::new(-1, 20000)),
        Err(PlayError::OutsideArena)
    );
    assert_eq!(
        b.play(Team::Blue, 0, Vec2::new(4500, 5000)),
        Err(PlayError::EnemyTerritory)
    );
    assert_eq!(serde_json::to_string(&b.players).unwrap(), before);
    b.players[0].elixir_milli = 0;
    let before = serde_json::to_string(&b.players).unwrap();
    assert_eq!(
        b.play(Team::Blue, 0, Vec2::new(4500, 20000)),
        Err(PlayError::InsufficientElixir)
    );
    assert_eq!(serde_json::to_string(&b.players).unwrap(), before);
}

#[test]
fn deployment_waits_for_recovered_delay() {
    let mut b = Battle::new(1);
    b.catalog
        .iter_mut()
        .find(|c| c.id == "knight")
        .unwrap()
        .deploy_ms = 1000;
    b.play(Team::Blue, 0, Vec2::new(4500, 20000)).unwrap();
    let id = b.entities.last().unwrap().id;
    let start = b.entities.last().unwrap().pos;
    for _ in 0..19 {
        b.step();
    }
    assert_eq!(b.entities.iter().find(|e| e.id == id).unwrap().pos, start);
    b.step();
    assert_ne!(b.entities.iter().find(|e| e.id == id).unwrap().pos, start);
}

#[test]
fn ground_troops_cross_only_at_bridges() {
    let mut b = Battle::new(2);
    for e in &mut b.entities {
        e.damage = 0;
    }
    b.play(Team::Blue, 0, Vec2::new(9000, 18500)).unwrap();
    let id = b.entities.last().unwrap().id;
    let mut crossed = false;
    for _ in 0..400 {
        b.step();
        let e = b.entities.iter().find(|e| e.id == id).unwrap();
        if (15000..=17000).contains(&e.pos.y) {
            assert!(BRIDGES.contains(&e.pos.x), "crossed river at {:?}", e.pos);
        }
        if e.pos.y < 15000 {
            crossed = true;
            break;
        }
    }
    assert!(crossed);
}

#[test]
fn air_troops_cross_directly_without_bridge_waypoint() {
    let mut b = with_first("minions");
    b.play(Team::Blue, 0, Vec2::new(9000, 18500)).unwrap();
    let e = b.entities.last().unwrap();
    let destination = Vec2::new(9000, 5000);
    assert!(e.flying);
    assert_eq!(Battle::movement_waypoint(e, destination), destination);
}

#[test]
fn damage_and_spells_never_hit_friendly_units() {
    let mut b = with_first("fireball");
    let blue_princess = b
        .entities
        .iter()
        .position(|e| e.team == Team::Blue && e.kind == EntityKind::PrincessTower)
        .unwrap();
    let red_princess = b
        .entities
        .iter()
        .position(|e| e.team == Team::Red && e.kind == EntityKind::PrincessTower)
        .unwrap();
    let pos = b.entities[red_princess].pos;
    b.entities[blue_princess].pos = pos;
    for e in &mut b.entities {
        e.damage = 0;
    }
    let blue_hp = b.entities[blue_princess].hp;
    let red_hp = b.entities[red_princess].hp;
    b.play(Team::Blue, 0, pos).unwrap();
    for _ in 0..100 {
        b.step();
    }
    assert_eq!(b.entities[blue_princess].hp, blue_hp);
    assert!(b.entities[red_princess].hp < red_hp);
}

#[test]
fn building_targeting_units_ignore_enemy_troops() {
    let mut b = with_first("giant");
    b.play(Team::Blue, 0, Vec2::new(4500, 17500)).unwrap();
    b.players[1].hand[0] = "knight".into();
    b.play(Team::Red, 0, Vec2::new(4500, 14500)).unwrap();
    let giant = b
        .entities
        .iter()
        .position(|e| e.card_id.as_deref() == Some("giant"))
        .unwrap();
    let knight = b
        .entities
        .iter()
        .position(|e| e.kind == EntityKind::Troop && e.team == Team::Red)
        .unwrap();
    b.entities[giant].pos = Vec2::new(4500, 14500);
    b.entities[giant].ready_tick = 0;
    b.entities[knight].ready_tick = u64::MAX;
    let knight_hp = b.entities[knight].hp;
    for _ in 0..40 {
        b.step();
    }
    assert_eq!(
        b.entities
            .iter()
            .find(|e| e.id == b.entities[knight].id)
            .unwrap()
            .hp,
        knight_hp
    );
    let e = b
        .entities
        .iter()
        .find(|e| e.card_id.as_deref() == Some("giant"))
        .unwrap();
    if let Some(id) = e.target_id {
        assert!(b
            .entities
            .iter()
            .find(|e| e.id == id)
            .unwrap()
            .kind
            .is_building());
    }
}

#[test]
fn king_activates_after_friendly_princess_destruction_and_match_freezes() {
    let mut b = Battle::new(3);
    assert!(
        !b.entities
            .iter()
            .find(|e| e.team == Team::Red && e.kind == EntityKind::KingTower)
            .unwrap()
            .active
    );
    b.entities
        .iter_mut()
        .find(|e| e.team == Team::Red && e.kind == EntityKind::PrincessTower)
        .unwrap()
        .hp = 0;
    b.step();
    assert!(
        b.entities
            .iter()
            .find(|e| e.team == Team::Red && e.kind == EntityKind::KingTower)
            .unwrap()
            .active
    );
    b.entities
        .iter_mut()
        .find(|e| e.team == Team::Red && e.kind == EntityKind::KingTower)
        .unwrap()
        .hp = 0;
    b.step();
    assert_eq!(b.outcome.unwrap().winner, Some(Team::Blue));
    let before = serde_json::to_string(&b).unwrap();
    b.step();
    b.bot_step(Team::Red);
    assert_eq!(serde_json::to_string(&b).unwrap(), before);
    assert_eq!(
        b.play(Team::Blue, 0, Vec2::new(4500, 20000)),
        Err(PlayError::MatchFinished)
    );
}

#[test]
fn overtime_draw_and_tiebreaker_are_bounded() {
    let config = BattleConfig {
        regulation_ms: 100,
        overtime_ms: 100,
        ..BattleConfig::default()
    };
    let mut draw = Battle::with_decks(1, [default_deck(), default_deck()], config.clone()).unwrap();
    for _ in 0..4 {
        draw.step();
    }
    assert_eq!(draw.outcome.unwrap().reason, FinishReason::Draw);
    let mut tie = Battle::with_decks(1, [default_deck(), default_deck()], config).unwrap();
    tie.entities
        .iter_mut()
        .find(|e| e.team == Team::Red && e.kind == EntityKind::PrincessTower)
        .unwrap()
        .hp -= 1;
    for _ in 0..4 {
        tie.step();
    }
    assert_eq!(tie.outcome.unwrap().winner, Some(Team::Blue));
    assert_eq!(tie.outcome.unwrap().reason, FinishReason::Tiebreaker);
}

#[test]
fn complete_seeded_bot_matches_are_bitwise_reproducible() {
    let mut a = Battle::new(8675309);
    let mut b = Battle::new(8675309);
    for _ in 0..6000 {
        for battle in [&mut a, &mut b] {
            battle.bot_step(Team::Blue);
            battle.bot_step(Team::Red);
            battle.step();
        }
        if a.outcome.is_some() {
            break;
        }
    }
    assert!(
        a.outcome.is_some(),
        "match must terminate by regulation plus overtime"
    );
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    );
    assert!(a
        .players
        .iter()
        .all(|p| (0..=10000).contains(&p.elixir_milli)));
}

#[test]
fn serialized_checkpoint_preserves_pending_spells_and_rng() {
    let mut original = with_first("fireball");
    original.play(Team::Blue, 0, Vec2::new(4500, 6500)).unwrap();
    let snapshot = serde_json::to_string(&original).unwrap();
    let mut resumed: Battle = serde_json::from_str(&snapshot).unwrap();
    for _ in 0..100 {
        for battle in [&mut original, &mut resumed] {
            battle.bot_step(Team::Blue);
            battle.bot_step(Team::Red);
            battle.step();
        }
    }
    assert_eq!(
        serde_json::to_string(&original).unwrap(),
        serde_json::to_string(&resumed).unwrap()
    );
}

#[test]
fn legal_hand_masks_follow_elixir_and_spell_territory() {
    let mut battle = with_first("fireball");
    assert_eq!(battle.legal_hand(Team::Blue)[0].y_min, 500);
    assert_eq!(battle.legal_hand(Team::Red)[0].y_max, 31500);
    battle.players[0].elixir_milli = 0;
    assert!(battle
        .legal_hand(Team::Blue)
        .iter()
        .all(|card| !card.playable));
    assert_eq!(battle.legal_hand(Team::Blue)[1].y_min, 17500);
}

#[test]
fn bridge_choice_cannot_switch_mid_river() {
    let mut battle = Battle::new(1);
    battle.play(Team::Blue, 0, Vec2::new(13500, 20000)).unwrap();
    let mut troop = battle.entities.last().unwrap().clone();
    troop.pos = Vec2::new(13500, 16500);
    assert_eq!(
        Battle::movement_waypoint(&troop, Vec2::new(4500, 6500)),
        Vec2::new(13500, 14900)
    );
}

#[test]
fn king_activation_uses_recovered_delay() {
    let mut battle = Battle::new(1);
    battle
        .entities
        .iter_mut()
        .find(|e| e.team == Team::Red && e.kind == EntityKind::PrincessTower)
        .unwrap()
        .hp = 0;
    battle.step();
    let king = battle
        .entities
        .iter()
        .find(|e| e.team == Team::Red && e.kind == EntityKind::KingTower)
        .unwrap();
    assert!(king.active);
    assert_eq!(king.ready_tick - battle.tick, 3300 / u64::from(TICK_MS));
}

#[test]
fn ground_only_spells_do_not_damage_air_units() {
    let mut battle = with_first("fireball");
    battle
        .catalog
        .iter_mut()
        .find(|c| c.id == "fireball")
        .unwrap()
        .target = Target::Ground;
    battle.play(Team::Red, 3, Vec2::new(4500, 9000)).unwrap();
    for e in &mut battle.entities {
        e.damage = 0;
        if e.kind == EntityKind::Troop {
            e.ready_tick = u64::MAX;
        }
    }
    let air: Vec<_> = battle
        .entities
        .iter()
        .filter(|e| e.flying)
        .map(|e| (e.id, e.hp))
        .collect();
    assert!(!air.is_empty());
    battle.play(Team::Blue, 0, Vec2::new(4500, 9000)).unwrap();
    for _ in 0..100 {
        battle.step();
    }
    for (id, hp) in air {
        assert_eq!(battle.entities.iter().find(|e| e.id == id).unwrap().hp, hp);
    }
}
