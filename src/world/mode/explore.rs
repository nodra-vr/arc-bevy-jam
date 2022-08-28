use sha2::digest::typenum::Len;

use super::*;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct EnergyText;

pub struct ExploreModePlugin;

impl Plugin for ExploreModePlugin {
    fn build(&self, app: &mut App) {
        let explore_grid = AppState::GamePlay(GameMode::ExploreGrid);

        // app.add_system_set(SystemSet::on_exit(explore_grid).with_system(exit_explore_gameplay));
        // app.add_system_set(SystemSet::on_enter(explore_grid).with_system(enter_explore_gameplay));

        app.add_system_set(SystemSet::on_update(explore_grid).with_system(update_health_text));
        app.add_system_set(SystemSet::on_update(explore_grid).with_system(update_energy_text));
    }
}

// fn exit_explore_gameplay(mut commands: Commands) {
//     log::info!("exit_explore_gameplay");
// }

// fn enter_explore_gameplay(mut commands: Commands) {
//     log::info!("enter_explore_gameplay");
// }

fn update_health_text(
    stats_query: Query<&HealthRecource, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let stat = stats_query.single();
    let mut text = text_query.single_mut();

    if text.sections.len() == 0 {
        return;
    }
    let max = stat.max.to_string();
    let value = stat.value.to_string();
    text.sections[0].value = format!("{value}/{max}");
}
fn update_energy_text(
    stats_query: Query<&EnergyRecource, With<Player>>,
    mut text_query: Query<&mut Text, With<EnergyText>>,
) {
    let stat = stats_query.single();
    let mut text = text_query.single_mut();

    if text.sections.len() == 0 {
        return;
    }
    let max = stat.max.to_string();
    let value = stat.value.to_string();
    text.sections[0].value = format!("{value}/{max}");
}

////////////////////////
/// Gamehud functions
////////////////////////

pub(crate) fn explore_mode_stats(commands: &mut Commands, font: &Handle<Font>) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(TILE_SIZE * 5.), Val::Percent(100.)),
                margin: UiRect::new(Val::Px(0.), Val::Px(24.), Val::Px(0.), Val::Px(0.)),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("explore-stats"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.), Val::Px(65.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "hp:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent
                        .spawn_bundle(TextBundle::from_section(
                            format!("80/80"),
                            TextStyle {
                                font: font.clone(),
                                font_size: 24.0,
                                color: gui::TEXT_BUTTON,
                            },
                        ))
                        .insert(HealthText);
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Px(TILE_SIZE * 2.5), Val::Px(65.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0., 0., 0., 0.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "energy:",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: gui::TEXT_BUTTON,
                        },
                    ));
                    parent
                        .spawn_bundle(TextBundle::from_section(
                            //format!("{0}/{0}"),
                            format!("120/120"),
                            TextStyle {
                                font: font.clone(),
                                font_size: 24.0,
                                color: gui::TEXT_BUTTON,
                            },
                        ))
                        .insert(EnergyText);
                });
        })
        .id();

    root
}

pub(crate) fn explore_mode_dialog(
    commands: &mut Commands,
    app_assets: &Res<AppAssets>,
    world_assets: &Res<WorldAssets>,
) -> Entity {
    let root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::FlexStart,

                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Name::new("explore-dialog"))
        .id();

    root
}