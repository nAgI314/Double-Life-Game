use bevy::app::{App, AppExit, Plugin, Update};
use bevy::ecs::event::EventWriter;
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::ResMut;
use bevy::ui::{BorderColor, UiRect};
use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    ui::{
        BackgroundColor, Interaction, Node, Val,
        widget::{Button, Text},
    },
    utils::default,
};
use bevy_state::prelude::*;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::AppState::MainMenu), setup_main_menu_scene)
            .add_systems(
                Update,
                handle_main_menu_actions.run_if(in_state(crate::AppState::MainMenu)),
            )
            .add_systems(OnExit(crate::AppState::MainMenu), cleanup_main_menu_scene);
    }
}

#[derive(Component)]
pub struct MainMenuScene;
#[derive(Component)]
pub struct MainMenu;
#[derive(Component, Clone, Copy, Debug)]
pub enum MainMenuAction {
    NewGame,
    Exit,
}

impl MainMenuAction {
    /// メニューのラベル文字列
    pub fn label(&self) -> &'static str {
        match self {
            MainMenuAction::NewGame => "Start New Game",
            MainMenuAction::Exit => "Exit Game",
        }
    }
}

pub(crate) fn setup_main_menu_scene(
    mut commands: Commands,
) {
    let menu_items = vec![
        MainMenuAction::NewGame,
        MainMenuAction::Exit,
    ];

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                position_type: bevy::ui::PositionType::Relative,
                justify_content: bevy::ui::JustifyContent::Center,
                align_items: bevy::ui::AlignItems::Center,
                ..default()
            },
            MainMenuScene,
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5)),
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn((Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(40.),
                    top: Val::Percent(0.),
                    position_type: bevy::ui::PositionType::Absolute,
                    justify_content: bevy::ui::JustifyContent::Center,
                    align_items: bevy::ui::AlignItems::Center,
                    ..default()
                },))
                .with_children(|child_parent| {
                    child_parent.spawn((
                        Node {
                            width: Val::Percent(80.),
                            justify_content: bevy::ui::JustifyContent::Center,
                            align_items: bevy::ui::AlignItems::Center,
                            ..default()
                        },
                        Text::new("Double-Life-Game"),
                    ));
                });
            // menus
            parent
                .spawn((
                    MainMenu,
                    Node {
                        width: Val::Percent(60.),
                        height: Val::Percent(40.),
                        bottom: Val::Px(0.),
                        padding: bevy::ui::UiRect {
                            bottom: Val::Percent(5.),
                            ..default()
                        },
                        position_type: bevy::ui::PositionType::Absolute,
                        justify_content: bevy::ui::JustifyContent::Center,
                        align_items: bevy::ui::AlignItems::Center,
                        flex_wrap: bevy::ui::FlexWrap::Wrap,
                        ..default()
                    },
                ))
                // menu buttons
                .with_children(|footer_parent| {
                    for item in menu_items {
                        footer_parent.spawn((
                            item,
                            Button,
                            Node {
                                width: Val::Percent(40.),
                                height: Val::Px(30.),
                                justify_content: bevy::ui::JustifyContent::Center,
                                align_items: bevy::ui::AlignItems::Center,
                                border: UiRect {
                                    bottom: Val::Px(1.0),
                                    ..default()
                                },
                                ..default()
                            },
                            BorderColor::all(Color::WHITE),
                            Text::new(item.label().to_string()),

                        ));
                    }
                });
        });
}

/// delete entity in main menu
pub(crate) fn cleanup_main_menu_scene(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuScene>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_main_menu_actions(
    menu_button_query: Query<(&Interaction, &MainMenuAction), With<Button>>,
    mut next_game_state: ResMut<NextState<crate::AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in menu_button_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MainMenuAction::NewGame => {
                    println!("New Game");
                    next_game_state.set(crate::AppState::InGame);
                }
                MainMenuAction::Exit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}
