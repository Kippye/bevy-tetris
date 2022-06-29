use std::time::Duration;

use crate::audio::PlaySoundEvent;
use crate::consts::*;
use crate::board::*;
use crate::helper::*;
use crate::audio::*;
use crate::score::*;
use bevy::ecs::event::Events;
use bevy::prelude::*;
use rand::prelude::*;

// RESOURCES
struct SegmentMaterialResource {
    piece_texture: Handle<Image>
}
impl FromWorld for SegmentMaterialResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let piece_handle = asset_server.load("textures/segment.png");

        SegmentMaterialResource { piece_texture: piece_handle }
    }
}

pub struct GameState {
    pub started: bool,
    pub starting_level: usize,
    pub level: usize, // TODO: would be cool if i added negative levels lmao
    pub has_piece: bool
}

/// Contains the next piece to spawn and the current 7-bag state, if enabled
pub struct PieceSelectionResource {
    pub next_piece: Option<Piece>
}

#[derive(Default)]
pub struct InputState {
    /// Is the up key being held down?
    pub up: bool,
    /// Is the down key being held down?
    pub down: bool,
    /// Is the left key being held down?
    pub left: bool,
    /// Is the right key being held down?
    pub right: bool,
    /// Was the up key just pressed?
    pub up_jp: bool,
    /// Was the down key just pressed?
    pub down_jp: bool,
    /// Was the left key just pressed?
    pub left_jp: bool,
    /// Was the right key just pressed?
    pub right_jp: bool,
    pub rotate_clockwise_jp: bool,
    pub rotate_anticlockwise_jp: bool
}

pub struct FallTimer(Timer);
pub struct InputTimer(Timer);
pub struct DropInputTimer(Timer);

// COMPONENTS

// EVENTS
pub struct PlacePieceEvent;
pub struct CheckLinesEvent;
pub struct LevelUpEvent;
pub struct LossEvent;

fn spawn_pieces(mut commands: Commands, mut game_state: ResMut<GameState>, asset_server: Res<AssetServer>, mut piece_selection: ResMut<PieceSelectionResource>) {
    let mut rng = rand::thread_rng();

    if game_state.has_piece == false {
        let mut transform = Transform::from_scale(Vec3::new(SEGMENT_SIZE, SEGMENT_SIZE, 1.0));

        let (pxw, pyw) = coords_to_world(PIECE_SPAWN_XP as f32, PIECE_SPAWN_YP as f32);
        let piece_pos = Vec3::new(pxw, pyw, 0.0);

        let shape: Shape;
        let rotation: usize;

        // spawn the piece that was selected to be next
        match &piece_selection.next_piece {
            // there already is a next piece selected, use it.
            Some(next_piece) => {
                shape = next_piece.shape;
                rotation = next_piece.rotation;
            },
            // choose a new piece to spawn next
            None => {
                shape = Shape::get_random();
                rotation = rng.gen_range(0..=3);
            }
        }

        let mut segments: Vec<Entity> = vec![];

        let mut index = 0;

        // spawn a sprite for each segment in the shape
        for _ in 1..=4 {
            //(transform.translation.x, transform.translation.y) = coords_to_world(BOARD_WIDTH as f32 / 2.0 + pos.x, BOARD_HEIGHT as f32 - 2.0 + pos.y);
            let pos = shape.get_rotation_offset(index, rotation);
            transform.translation = piece_pos + pos.extend(0.0) * SEGMENT_SIZE;

            let segment = commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load("textures/segment.png"),
                    sprite: Sprite {
                        color: shape.get_color(),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(Segment { segment_index: index })
                .id();

            segments.push(segment);
            index += 1;
        }

        // TEMP
        let next_piece_shape = Shape::get_random();
        let next_piece_rot = rng.gen_range(0..=3);
        // let next_piece_shape = Shape::get_random();
        // let next_piece_rot = rng.gen_range(0..=3);

        // select the next piece to spawn
        piece_selection.next_piece = Some(Piece { 
            shape: next_piece_shape,
            position: Vec3::new(pxw, pyw, 0.0),
            rotation: next_piece_rot,
            segment_entities: vec![],
            dropped_pixels: 0
        });
        
        // create the piece entity containing the shape, a random rotation and all the segment entities
        commands
            .spawn()
            .insert(Piece { shape, position: piece_pos, rotation, segment_entities: segments, dropped_pixels: 0 });

        // TEMP
        game_state.has_piece = true;
    }
}

pub fn piece_movement_input(input: Res<Input<KeyCode>>, mut input_state: ResMut<InputState>, mut input_timer: ResMut<InputTimer>, mut drop_input_timer: ResMut<DropInputTimer>) {
    input_state.up = input.pressed(KeyCode::Up);
    input_state.down = input.pressed(KeyCode::Down);
    input_state.left = input.pressed(KeyCode::Left);
    input_state.right = input.pressed(KeyCode::Right);
    input_state.up_jp = input.just_pressed(KeyCode::Up);
    input_state.down_jp = input.just_pressed(KeyCode::Down);
    input_state.left_jp = input.just_pressed(KeyCode::Left);
    input_state.right_jp = input.just_pressed(KeyCode::Right);
    input_state.rotate_clockwise_jp = input.just_pressed(KeyCode::X);
    input_state.rotate_anticlockwise_jp = input.just_pressed(KeyCode::Z);

    // reset the side movement input timer so the player can hold the button to move
    if input_state.left_jp || input_state.right_jp {
        input_timer.0.reset();
    }
    // reset the drop input timer (faster than side to side movement)
    if input_state.down_jp {
        drop_input_timer.0.reset();
    }
}

/// Handles pieces being moved by the player and gravity, collision checking
pub fn move_pieces(mut pieces: Query<&mut Piece>, mut segments: Query<(&mut Transform, &mut Segment)>, time: Res<Time>, tower_segments: Query<(&Transform, &TowerSegment), Without<Segment>>, mut timer: ResMut<FallTimer>, input_state: Res<InputState>, mut input_timer: ResMut<InputTimer>, mut drop_input_timer: ResMut<DropInputTimer>, mut place_events: ResMut<Events<PlacePieceEvent>>) {
    timer.0.tick(time.delta());
    input_timer.0.tick(time.delta());
    drop_input_timer.0.tick(time.delta());

    for mut piece in pieces.iter_mut() {
        // whether or not the piece is being dropped manually this frame
        let mut dropping = false;

        // convert the movement input into movement
        let mut movement = Vec3::new(
            if input_state.left_jp || (input_state.left && input_timer.0.just_finished()) { -1.0 } else if input_state.right_jp || (input_state.right && input_timer.0.just_finished()) { 1.0 } else { 0.0 },
            if input_state.down_jp || (input_state.down && drop_input_timer.0.just_finished()) { dropping = true; -1.0 } else { 0.0 },
            0.0
        );

        // // TEMP: in the future, perhaps just make the newest move override the older one
        // if (movement.x > 0.0 || movement.x < 0.0) && movement.y < 0.0 {
        //     movement.x = 0.0;
        //     movement.y = 0.0;
        // }

        // apply "gravity"
        if timer.0.just_finished() {
            movement.y = -1.0;
        }

        // rotation and rotation collision / edge checking
        if input_state.rotate_clockwise_jp || input_state.rotate_anticlockwise_jp {
            let mut can_rotate = true;
            let mut desired_rotation = piece.rotation;
            // change rotation
            desired_rotation = if input_state.rotate_clockwise_jp { piece.rotation + 1 } else { if piece.rotation > 0 { piece.rotation - 1 } else { 3 } };
            // wrap around
            if desired_rotation > 3 { desired_rotation = 0; }

            // loop through all the segments of the piece to check if their desired rotation pos is valid
            for segment_entity in piece.segment_entities.iter().cloned() {
                // get the transform and segment of the current segment
                let (_, segment) = segments.get_mut(segment_entity).unwrap();
                // get the target position after rotation in normal and pixel form
                let rotated_pos = piece.position + (movement * SEGMENT_SIZE) + Vec2::extend(piece.shape.get_rotation_offset(segment.segment_index, desired_rotation) * SEGMENT_SIZE, 0.0);
                let (rxp, ryp) = coords_to_pixel(rotated_pos.x, rotated_pos.y);

                // is the rotated pos even on the board?
                if Board::is_on_board(rxp, ryp, true) == false {
                    can_rotate = false;
                    break;
                }

                // check if the rotated position overlaps any tower segments
                for (tower_transform, _) in tower_segments.iter() {
                    let (txp, typ) = coords_to_pixel(tower_transform.translation.x, tower_transform.translation.y);
    
                    // same X as the tower segment and about to hit it - place the piece
                    if rxp + movement.x as i32 == txp && ryp + movement.y as i32 == typ {
                        can_rotate = false;
                        break;
                    }
                }

                // if we have already discovered that the piece can't be rotated, quit the loop
                if can_rotate == false { break };
            }

            // the piece can be rotated without issue, do it.
            if can_rotate == true {
                piece.rotation = desired_rotation;

                for segment_entity in piece.segment_entities.iter().cloned() {
                    let (mut transform, segment) = segments.get_mut(segment_entity).unwrap();
                    transform.translation = piece.position + Vec2::extend(piece.shape.get_rotation_offset(segment.segment_index, piece.rotation) * SEGMENT_SIZE, 0.0);
                }
            }
        }

        let mut place_piece = false;
        let mut place_diagonal = false;
        let mut was_blocked = false;

        // check for collisions with edges and tower segments
        for segment_entity in piece.segment_entities.iter().cloned() {
            let (transform, _) = segments.get(segment_entity).unwrap();

            // check if hitting the side edge
            let (xp, yp) = coords_to_pixel(transform.translation.x, transform.translation.y);

            // piece is trying to move out of the board to the left or right, set horizontal movement to 0
            if (xp + movement.x as i32) < 0 {
                movement.x = 0.0;
            }
            else if (xp + movement.x as i32) > BOARD_WIDTH as i32 - 1 {
                movement.x = 0.0;
            }

            // piece is at the bottom of the board and trying to fall through it, place it
            if (yp + movement.y as i32) < 0 {
                place_piece = true;
                movement.y = 0.0;
            }

            // OPTIMIZABLE: i can break the entire loop if one of these fires, no need to check other segments
            // iterate through all the current tower segments to check if side movement is going to collide with any
            for (tower_transform, _) in tower_segments.iter() {
                let (txp, typ) = coords_to_pixel(tower_transform.translation.x, tower_transform.translation.y);

                // check if dropping a piece directly on top of a tower segment - always placement
                if movement.x == 0.0 {
                    if xp == txp && yp + movement.y as i32 == typ {
                        movement.y = 0.0;
                        place_piece = true;
                    }
                }

                // check if only trying to move a piece into the side of a tower segment - always blocked
                if movement.y == 0.0 {
                    if yp == typ && xp + movement.x as i32 == txp {
                        movement.x = 0.0;
                        was_blocked = true;
                    }
                }

                // check if trying to move a piece diagonally into a tower segment - i am going to force these to also be placements, because the tile would end up on top of it and going down, if X movement was applied first
                if (movement.y > 0.0 || movement.y < 0.0) && (movement.x > 0.0 || movement.x < 0.0) {
                    if xp + movement.x as i32 == txp && yp + movement.y as i32 == typ {
                        movement.y = 0.0;
                        place_diagonal = true;
                    }
                }
            }

            if was_blocked == true { place_diagonal = false; }
        }

        // move the piece's position along
        piece.position += movement * SEGMENT_SIZE;

        let mut piece_has_been_placed = false;

        // move segments contained in this piece
        for segment_entity in piece.segment_entities.iter().cloned() {
            let (mut transform, _) = segments.get_mut(segment_entity).unwrap();

            transform.translation += movement * SEGMENT_SIZE;
            Board::clamp_to_board(&mut transform.translation);
            
            if (place_piece || place_diagonal) && piece_has_been_placed == false {
                place_events.send(PlacePieceEvent {});
                timer.0.reset();
                input_timer.0.reset();
                drop_input_timer.0.reset();
                // we still want to move the other pieces as well, but not spawn a new piece for every one of them
                piece_has_been_placed = true;
            }
        }

        if dropping == true && place_piece == false && place_diagonal == false && piece_has_been_placed == false {
            piece.dropped_pixels += 1;
        }
    }
}

pub fn place_piece(mut commands: Commands, mut pieces: Query<(Entity, &mut Piece)>, mut game_state: ResMut<GameState>, mut place_event_reader: EventReader<PlacePieceEvent>, mut loss_events: ResMut<Events<LossEvent>>, mut check_lines_events: ResMut<Events<CheckLinesEvent>>, mut play_sound_events: ResMut<Events<PlaySoundEvent>>, mut score_resource: ResMut<ScoreResource>) {
    let mut loss_event_sent = false;
    
    for _ in place_event_reader.iter() {
        for (piece_entity, mut piece) in pieces.iter_mut() {
            for segment_entity in piece.segment_entities.iter() {
                commands
                    .entity(*segment_entity)
                    .remove::<Segment>()
                    .insert(TowerSegment { });
            }

            // award score for dropped pixels
            score_resource.reward_drop_score(piece.dropped_pixels, 1);
            
            // TEMP: this would not work well for all pieces
            let (_, pyp) = coords_to_pixel(0.0, piece.position.y);
            if pyp >= PIECE_SPAWN_YP - 2 && loss_event_sent == false {
                loss_events.send(LossEvent {});
                loss_event_sent = true;
            }

            piece.segment_entities.clear();
            // despawn the piece (because we'll be spawning a new one)
            commands
                .entity(piece_entity)
                .remove::<Piece>()
                .despawn();

            play_sound_events.send(PlaySoundEvent { sound: Sound::PiecePlace });
            check_lines_events.send(CheckLinesEvent {});
        }
        if loss_event_sent == false {
            game_state.has_piece = false;
        }
    }
}

pub fn check_lines(mut commands: Commands, mut tower_segments: Query<(&mut Transform, &mut Sprite, Entity), (Without<Segment>, With<TowerSegment>)>, mut check_lines_reader: EventReader<CheckLinesEvent>, game_state: Res<GameState>, mut play_sound_events: ResMut<Events<PlaySoundEvent>>, mut score_resource: ResMut<ScoreResource>, mut level_up_events: ResMut<Events<LevelUpEvent>>) {
    for _ in check_lines_reader.iter() {
        let mut line_segment_counts = [0; BOARD_HEIGHT];

        // get the count of tower segments in each layer of the tower
        for (tower_transform,_, _) in tower_segments.iter() {
            let (_, layer) = coords_to_pixel(0.0, tower_transform.translation.y);

            line_segment_counts[layer as usize] += 1; 
        }

        let mut full_layers: Vec<i32> = vec![];
        // check if any lines are full (also keep count of that, for singles, doubles, triples, tetrises)
        for i in 0..line_segment_counts.len() {
            if line_segment_counts[i] == 10 {
                full_layers.push(i as i32);
            }
            else if line_segment_counts[i] > 10 {
                println!("WARNING: A line was cleared with MORE than 10 segments in it");
                full_layers.push(i as i32);
            }
        }

        // clear the lines and make lines above them fall
        for (mut tower_transform, mut sprite, tower_segment_entity) in tower_segments.iter_mut() {
            let (_, typ) = coords_to_pixel(0.0, tower_transform.translation.y);

            // the tower segment is within one of complete lines
            if full_layers.contains(&typ) {
                commands
                    .entity(tower_segment_entity)
                    .remove::<TowerSegment>()
                    .despawn();
            } else {
                for i in 0..full_layers.len() {
                    if typ > full_layers[i] {
                        //sprite.color = Color::CRIMSON;
                        tower_transform.translation.y -= SEGMENT_SIZE;
                    }
                }
            }
        }

        // reward score for lines (also increases the line counter on the score)
        score_resource.reward_line_score(full_layers.len(), game_state.level);
        
        // check if enough lines have been cleared to progress to the next level (depending on whether or not this is the starting level)
        // on the starting level, where more lines need to be cleared
        if game_state.level == game_state.starting_level {
            if score_resource.lines() >= (game_state.starting_level + 1) * LINES_PER_LEVEL {
                level_up_events.send(LevelUpEvent {});
            }
        }
        // on levels past the starting level
        else if game_state.level > game_state.starting_level {
            if score_resource.lines() >= (game_state.starting_level + 1) * LINES_PER_LEVEL + (game_state.level - game_state.starting_level) * LINES_PER_LEVEL {
                level_up_events.send(LevelUpEvent {});
            }
        }

        // play line clear sounds
        if full_layers.len() == 4 {
            play_sound_events.send(PlaySoundEvent { sound: Sound::TetrisClear });
        }
        else if full_layers.len() > 0 {
            play_sound_events.send(PlaySoundEvent { sound: Sound::LineClear });
        }
    }
}

pub fn level_up(mut level_up_events: EventReader<LevelUpEvent>, mut game_state: ResMut<GameState>, mut fall_timer: ResMut<FallTimer>) {
    for _ in level_up_events.iter() {
        game_state.level += 1;
        println!("Leveled up to level {}", game_state.level);

        // make pieces fall faster now
        let new_duration = Duration::from_millis((fall_timer.0.duration().as_millis() as f32 / LEVEL_SPEED_INCREASE_PERCENTAGE) as u64);
        //fall_timer.0.reset();
        fall_timer.0.set_duration(new_duration);
    }
}

pub fn restart(mut commands: Commands, mut pieces: Query<(&mut Piece, Entity)>, tower_segments: Query<Entity, (Without<Segment>, With<TowerSegment>)>, mut loss_reader: EventReader<LossEvent>, mut game_state: ResMut<GameState>, mut score_resource: ResMut<ScoreResource>, mut play_sound_events: ResMut<Events<PlaySoundEvent>>) {
    for _ in loss_reader.iter() {
        // erase all tower segments
        for tower_entity in tower_segments.iter() {
            commands
                .entity(tower_entity)
                .remove::<TowerSegment>()
                .despawn();
        }

        // erase the current piece
        for (piece, piece_entity) in pieces.iter_mut() {
            for segment_entity in piece.segment_entities.iter() {
                commands
                    .entity(*segment_entity)
                    .remove::<Segment>()
                    .despawn();
            }
            commands
                .entity(piece_entity)
                .remove::<Piece>()
                .despawn();
        }

        play_sound_events.send(PlaySoundEvent { sound: Sound::GameOver });

        // reset score
        score_resource.reset();

        // reset level to starting level for quick restart ig
        game_state.level = game_state.starting_level;

        // spawn new piece
        game_state.has_piece = false;
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SegmentMaterialResource>()
            .init_resource::<Events<PlacePieceEvent>>()
            .init_resource::<Events<CheckLinesEvent>>()
            .init_resource::<Events<LevelUpEvent>>()
            .init_resource::<Events<LossEvent>>()
            .insert_resource(PieceSelectionResource { next_piece: None })
            .insert_resource(FallTimer(Timer::from_seconds(FALL_DELAY, true)))
            .insert_resource(InputTimer(Timer::from_seconds(MOVE_DELAY, true)))
            .insert_resource(DropInputTimer(Timer::from_seconds(DROP_DELAY, true)))
            .insert_resource(InputState::default())
            .add_system(piece_movement_input.before(move_pieces))
            .add_system(move_pieces)
            .add_system(restart.before(place_piece))
            .add_system(spawn_pieces.after(restart))
            .add_system(check_lines.before(place_piece))
            .add_system(level_up)
            .add_system(place_piece.after(move_pieces));
    }
}