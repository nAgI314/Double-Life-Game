use bevy::{
    app::{App, Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
        system::{Commands, Query, Res, ResMut},
    },
    input::{
        ButtonInput,
        keyboard::KeyCode,
        mouse::{MouseButton, MouseWheel},
    },
    log::info,
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

use crate::in_game::{InGameScene, InGameState};

#[derive(Component)]
enum BottomButtons {
    Start,
    Stop,
    Exit,
}

pub(crate) struct IngameUiPlugin;

impl Plugin for IngameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::AppState::InGame), setup_in_game_scene)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(crate::AppState::InGame)),
            );
    }
}

pub(crate) fn setup_in_game_scene(mut commands: Commands) {
    // Footer buttons
    commands
        .spawn((
            InGameScene,
            Node {
                width: Val::Percent(100.),
                height: Val::Px(50.),
                bottom: Val::Px(0.),
                right: Val::Px(0.),
                position_type: bevy::ui::PositionType::Absolute,
                justify_content: bevy::ui::JustifyContent::FlexEnd,
                align_items: bevy::ui::AlignItems::Center,
                ..default()
            },
            ZIndex(100),
        ))
        // Footer buttons
        .with_children(|footer_parent| {
            for button in [
                ("START", BottomButtons::Start),
                ("STOP", BottomButtons::Stop),
                ("EXIT", BottomButtons::Exit),
            ] {
                footer_parent.spawn((
                    button.1,
                    Button,
                    Node {
                        width: Val::Px(128.),
                        height: Val::Px(30.),
                        margin: bevy::ui::UiRect {
                            left: Val::Px(10.),
                            right: Val::Px(10.),
                            top: Val::Px(0.),
                            bottom: Val::Px(0.),
                        },
                        justify_content: bevy::ui::JustifyContent::Center,
                        align_items: bevy::ui::AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(Color::srgba(0.13, 0.49, 0.23, 0.5)),
                    Text::new(button.0.to_string()),
                ));
            }
        });
}

pub(crate) fn handle_button_interaction(
    mut button_query: Query<
        (&Interaction, &BottomButtons, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<InGameState>>,
    mut next_app_state: ResMut<NextState<crate::AppState>>,
) {
    for (interaction, button, mut bg_color) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                match button {
                    BottomButtons::Start => {
                        info!("Game started");
                        next_game_state.set(InGameState::Processing);
                        *bg_color = BackgroundColor(Color::srgba(0.13, 0.49, 0.23, 0.8));
                    }
                    BottomButtons::Stop => {
                        info!("Game stopped");
                        next_game_state.set(InGameState::Stop);
                        *bg_color = BackgroundColor(Color::srgba(0.13, 0.49, 0.23, 0.8));
                    }
                    BottomButtons::Exit => {
                        info!("Exiting game");
                        next_app_state.set(crate::AppState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgba(0.13, 0.49, 0.23, 0.9));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgba(0.13, 0.49, 0.23, 0.5));
            }
        }
    }
}