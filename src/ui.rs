use crate::consts::*;
use crate::score::ScoreResource;
use crate::game::{PieceSelectionResource, GameState};
use crate::ProgramData;
use crate::board::{ Piece, Shape };
use bevy::prelude::*;

// COMPONENTS
#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct LinesText;
#[derive(Component)]
pub struct LevelText;
#[derive(Component)]
pub struct NextPieceImage {
    x: i32,
    y: i32
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(update_score_text)
            .add_system(update_level_text)
            .add_system(update_next_piece_display)
            .add_system(update_lines_text);
    }
}

fn setup_ui(mut commands: Commands, asset_server: ResMut<AssetServer>, pd: Res<ProgramData>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0),
                    height: Val::Px(pd.window_height)
                },
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.1, 0.1, 0.2)),
            ..Default::default()
        })
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0),
                    height: Val::Px(pd.window_height)
                },
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.1, 0.1, 0.2)),
            ..Default::default()
        })
        .commands()
        // SCORE AND LEVEL DISPLAY NODE
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0 + (BOARD_WIDTH as f32 * SEGMENT_SIZE) + 100.0),
                    bottom: Val::Px(pd.window_height / 2.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Score: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    ..Default::default()
                })
                .insert(ScoreText);
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(100.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Level: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    ..Default::default()
                })
                .insert(LevelText);
        })
        .commands()
        // BOTTOM BAR NODE
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(BOARD_WIDTH as f32 * SEGMENT_SIZE),
                    height: Val::Px((pd.window_height - (BOARD_HEIGHT as f32 * SEGMENT_SIZE)) / 2.0)
                },
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.1, 0.1, 0.2)),
            ..Default::default()
        })
        .commands()
        // LINES DISPLAY TOP BAR NODE
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0),
                    bottom: Val::Px(BOARD_HEIGHT as f32 * SEGMENT_SIZE + ((pd.window_height - BOARD_HEIGHT as f32 * SEGMENT_SIZE) / 2.0)),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(BOARD_WIDTH as f32 * SEGMENT_SIZE),
                    height: Val::Px((pd.window_height - (BOARD_HEIGHT as f32 * SEGMENT_SIZE)) / 2.0)
                },
                ..Default::default()
            },
            color: UiColor(Color::rgb(0.1, 0.1, 0.2)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px((BOARD_WIDTH / 2 - 1) as f32 * SEGMENT_SIZE),
                            bottom: Val::Px(30.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Lines: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    ..Default::default()
                })
                .insert(LinesText);
        })
        .commands()
        // NEXT PIECE TEXT & DISPLAY NODE
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px((pd.window_width - (BOARD_WIDTH as f32 * SEGMENT_SIZE)) / 2.0 + (BOARD_WIDTH as f32 * SEGMENT_SIZE) + 100.0),
                    bottom: Val::Px(pd.window_height / 2.0 + SEGMENT_SIZE * 5.0 + 20.0),
                    ..Default::default()
                },
                flex_wrap: FlexWrap::Wrap,
                flex_basis: Val::Px(SEGMENT_SIZE),
                size: Size {
                    width: Val::Px(SEGMENT_SIZE * 4.0),
                    height: Val::Px(SEGMENT_SIZE * 4.0)
                },
                ..Default::default()
            },
            color: UiColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Next piece",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9)
                        },
                        Default::default()
                    ),
                    ..Default::default()
                });

            for y in 0..4 {
                for x in 0..4 {
                    parent
                        .spawn_bundle(ImageBundle {
                            style: Style {
                                position: Rect {
                                    bottom: Val::Px(-SEGMENT_SIZE * 5.0 - 20.0),
                                    ..Default::default()
                                },
                                size: Size {
                                    width: Val::Px(SEGMENT_SIZE),
                                    height: Val::Px(SEGMENT_SIZE)
                                },
                                ..Default::default()
                            },
                            image: UiImage(asset_server.load("textures/segment.png")),
                            color: UiColor(Color::rgba(1.0, 1.0, 1.0, 1.0)),
                            ..Default::default()
                        })
                        .insert(NextPieceImage { x, y });
                }
            }
        });
}

fn update_score_text(score: Res<ScoreResource>, mut score_text_query: Query<(&mut Text, &ScoreText)>) {
    if !score.is_changed() {
        return;
    }

    for (mut text, _) in score_text_query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}",
            score.score()
        );
    }
}

fn update_level_text(game_state: Res<GameState>, mut level_text_query: Query<(&mut Text, &LevelText)>) {
    if !game_state.is_changed() {
        return;
    }

    for (mut text, _) in level_text_query.iter_mut() {
        text.sections[0].value = format!(
            "Level: {}",
            game_state.level
        );
    }
}

fn update_lines_text(score: Res<ScoreResource>, mut lines_text_query: Query<(&mut Text, &LinesText)>) {
    if !score.is_changed() {
        return;
    }

    for (mut text, _) in lines_text_query.iter_mut() {
        text.sections[0].value = format!(
            "Lines: {}",
            score.lines()
        );
    }
}

fn update_next_piece_display(piece_selection: Res<PieceSelectionResource>, mut images: Query<(&mut UiColor, &NextPieceImage)>) {
    if !piece_selection.is_changed() {
        return;
    }

    let next_piece: &Piece;

    match &piece_selection.next_piece {
        Some(piece) => { next_piece = piece; },
        // no piece selected, hide all the tiles
        None => {
            for (mut image_color, _) in images.iter_mut() {
                image_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.0);
            }
            return;
        }
    }

    // get all the image tiles that should be shown for the piece
    let mut filled_positions: Vec<Vec2> = vec![];

    for i in 0..=3 {
        // get the rotated position of each segment of the next piece
        // NOTE: the positions will be given an offset of (1; 1) due to negative offsets
        // NOTE: the I piece has special offsets
        filled_positions.push(next_piece.shape.get_rotation_offset(i, next_piece.rotation) 
        + match next_piece.shape { Shape::I => { if next_piece.rotation % 2 == 0 { Vec2::new(1.0, 0.0) } else { Vec2::new(2.0, 1.0) }}, _ => Vec2::ONE });
    }

    // hide or show tiles depending on if they contain the next piece's segments or not
    for (mut image_color, piece_image) in images.iter_mut() {
        if filled_positions.contains(&Vec2::new(piece_image.x as f32, piece_image.y as f32)) {
            image_color.0 = next_piece.shape.get_color();
        }
        else {
            image_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.0);
        }
    }
}