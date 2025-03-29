use bevy::prelude::*;

mod map;
use map::{
    MAP_WIDTH,
    MAP_HEIGHT,
    TILE_SIZE,
    generate_map
};

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct MapTile;

#[derive(Component)]
struct SelectedTile;

#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;

const RESOLUTION_X: f32 = 1024.0;
const RESOLUTION_Y: f32 = 1024.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, (setup, spawn_tiles))
        .add_systems(
            Update,
            (
                move_camera,
                position_tiles,
                position_markers,
                mouse_coordinates,
            ),
        )
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .run();
}

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(RESOLUTION_X / 2.0, RESOLUTION_Y / 2.0, 0.0),
    ));
    window.resolution.set(RESOLUTION_X, RESOLUTION_Y);
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("tiles.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 10, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let map = generate_map();

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            commands
                .spawn(Sprite::from_atlas_image(
                    texture.clone(), // TODO find a way to not use clone
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: map[x][y],
                    },
                ))
                .insert(Position {
                    x: x as i32,
                    y: y as i32,
                })
                .insert(MapTile);
        }
    }
}

fn position_tiles(mut q: Query<(&Position, &mut Transform), With<MapTile>>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            0.0,
        );
    }
}

fn position_markers(mut q: Query<(&Position, &mut Transform), With<SelectedTile>>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            pos.y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            1.0,
        );
    }
}

fn mouse_coordinates(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut tile_sprites: Query<(&mut Sprite, &Position), With<MapTile>>,
) {
    let (camera, camera_transform) = *camera_query;

    if !buttons.just_pressed(MouseButton::Left) && !buttons.pressed(MouseButton::Left) {
        return;
    }

    if let Some(world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        tile_sprites
            .iter_mut()
            .filter(|(_sprite, pos)| {
                pos.x as f32 * TILE_SIZE <= world_position.x
                    && world_position.x < (pos.x as f32 + 1.0) * TILE_SIZE
                    && pos.y as f32 * TILE_SIZE <= world_position.y
                    && world_position.y < (pos.y as f32 + 1.0) * TILE_SIZE
            })
            .for_each(|(mut sprite, pos)| {
                sprite.color = Color::BLACK;
                println!("Clicked on tile at {:?}", pos);
            });
    }
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    for mut transform in &mut query {
        // TODO Add time
        transform.translation += direction;
        // clamp translation to map bounds
        transform.translation.x = transform.translation.x.clamp(
            RESOLUTION_X / 2.0,
            MAP_WIDTH as f32 * TILE_SIZE - RESOLUTION_X / 2.0,
        );
        transform.translation.y = transform.translation.y.clamp(
            RESOLUTION_Y / 2.0,
            MAP_HEIGHT as f32 * TILE_SIZE - RESOLUTION_Y / 2.0,
        );
    }
}
