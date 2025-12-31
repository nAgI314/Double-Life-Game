use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        resource::Resource,
        schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
        system::{Commands, ParamSet, Query, Res, ResMut},
    },
    input::{
        ButtonInput,
        keyboard::KeyCode,
        mouse::{MouseButton, MouseWheel},
    },
    log::info,
    time::{Time, Timer, TimerMode},
    ui::{
        BackgroundColor, BorderColor, Interaction, Node, Val, ZIndex,
        widget::{Button, Text},
    },
    utils::default,
};
use bevy_state::{
    app::AppExtStates,
    condition::in_state,
    state::{NextState, OnEnter, OnExit, State, States},
};
use rand::Rng;

use crate::in_game::ui::IngameUiPlugin;

pub mod ui;

#[derive(Component)]
pub struct InGameScene;

#[derive(Component)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
    pub life: usize,
}

#[derive(Resource)]
pub struct GameConfig {
    pub grid_num: usize,
    pub max_life: usize,
    pub damage_amount: usize,
    pub heal_amount: usize,

    // 生存条件
    pub survive_neighbors_min: usize,
    pub survive_neighbors_max: usize,

    // 誕生条件
    pub birth_neighbors_min: usize,
    pub birth_neighbors_max: usize,

    pub update_interval: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_num: 100,
            max_life: 2,
            damage_amount: 1,
            heal_amount: 1,

            // Conway's Life (B3/S23)
            survive_neighbors_min: 2,
            survive_neighbors_max: 3,
            birth_neighbors_min: 3,
            birth_neighbors_max: 3,

            update_interval: 0.5,
        }
    }
}

#[derive(Resource)]
pub struct GameUpdateTimer {
    pub timer: Timer,
}

impl Default for GameUpdateTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum InGameState {
    #[default]
    Stop,
    Processing,
}

#[derive(Event, Default)]
pub(crate) struct NextTextEvent;

pub(crate) struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InGameState>()
            .init_resource::<GameConfig>()
            .init_resource::<GameUpdateTimer>()
            .add_plugins(IngameUiPlugin)
            .add_systems(OnEnter(crate::AppState::InGame), setup_in_game_scene)
            .add_systems(OnExit(crate::AppState::InGame), close_in_game)
            .add_systems(
                Update,
                update_game_timer.run_if(in_state(InGameState::Processing)),
            )
            .add_systems(
                Update,
                update_grid.run_if(in_state(InGameState::Processing)),
            );
    }
}

pub(crate) fn setup_in_game_scene(mut commands: Commands, config: Res<GameConfig>) {
    let mut rng = rand::thread_rng();
    let mut alive_count = 0;
    let initial_alive_count = 10000; // 初期に生きているマスの個数

    for x in 0..config.grid_num {
        for y in 0..config.grid_num {
            let life = if alive_count < initial_alive_count && rng.gen_bool(0.1) {
                alive_count += 1;
                config.max_life
            } else {
                0
            };

            commands.spawn((
                InGameScene,
                GridCell { x, y, life },
                Node {
                    width: Val::Px(10.),
                    height: Val::Px(10.),
                    position_type: bevy::ui::PositionType::Absolute,
                    left: Val::Px(x as f32 * 10.),
                    bottom: Val::Px(y as f32 * 10.),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                BorderColor::all(Color::BLACK),
            ));
        }
    }

    info!("Game started with {} alive cells", alive_count);
}

pub fn update_game_timer(
    mut timer: ResMut<GameUpdateTimer>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    timer.timer.tick(time.delta());

    // タイマーの期間を動的に変更したい場合に対応
    if timer.timer.duration().as_secs_f32() != config.update_interval {
        timer.timer = Timer::from_seconds(config.update_interval, TimerMode::Repeating);
    }
}

pub fn update_grid(
    mut param_set: ParamSet<(
        Query<&GridCell>,
        Query<&mut GridCell>,
        Query<(&GridCell, &mut BackgroundColor)>,
    )>,
    config: Res<GameConfig>,
    timer: Res<GameUpdateTimer>,
) {
    if !timer.timer.finished() {
        return;
    }

    // 隣接セル数カウント
    let mut neighbor_counts = std::collections::HashMap::<(usize, usize), usize>::new();

    {
        let query = param_set.p0();
        for cell in query.iter() {
            if cell.life > 0 {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = cell.x as i32 + dx;
                        let ny = cell.y as i32 + dy;

                        if nx >= 0
                            && ny >= 0
                            && (nx as usize) < config.grid_num
                            && (ny as usize) < config.grid_num
                        {
                            *neighbor_counts
                                .entry((nx as usize, ny as usize))
                                .or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    // 次世代 life を計算
    let mut next_life = Vec::new();

    {
        let query = param_set.p0();
        for cell in query.iter() {
            let neighbors = *neighbor_counts.get(&(cell.x, cell.y)).unwrap_or(&0);

            let mut life = cell.life;

            if life > 0 {
                // 生存セルの更新
                if neighbors < config.survive_neighbors_min
                    || neighbors > config.survive_neighbors_max
                {
                    life = life.saturating_sub(config.damage_amount);
                } else if neighbors >= config.birth_neighbors_min
                    && neighbors <= config.birth_neighbors_max
                {
                    life = life.saturating_add(config.heal_amount);
                }
            } else {
                // 死亡セルの更新
                if neighbors >= config.birth_neighbors_min
                    && neighbors <= config.birth_neighbors_max
                {
                    life = config.max_life;
                }
            }

            life = life.clamp(0, config.max_life);
            next_life.push(life);
        }
    }

    // 反映
    {
        let mut query = param_set.p1();
        for (mut cell, life) in query.iter_mut().zip(next_life.into_iter()) {
            cell.life = life;
        }
    }

    // 色更新（life に応じてグラデーション）
    {
        let mut query = param_set.p2();
        for (cell, mut bg_color) in query.iter_mut() {
            if cell.life == 0 {
                bg_color.0 = Color::srgba(0.1, 0.1, 0.1, 1.0);
            } else {
                let ratio = cell.life as f32 / config.max_life as f32;
                bg_color.0 = Color::srgba(0.2 + 0.6 * ratio, 0.8 * ratio, 0.2 + 0.4 * ratio, 1.0);
            }
        }
    }
}

fn close_in_game(in_game_query: Query<Entity, With<InGameScene>>, mut commands: Commands) {
    for entity in in_game_query.iter() {
        info!("InGame closed (queued despawn for {:?})", entity);
        commands.entity(entity).despawn();
    }
}
