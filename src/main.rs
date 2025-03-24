use bevy::prelude::*;
use rand::Rng; // 0.8.5
use rand::prelude::IndexedRandom;

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

const MAP_HEIGHT: usize = 100;
const MAP_WIDTH: usize = 100;
const TILE_SIZE: f32 = 32.0;

const RESOLUTION_X: f32 = 1024.0;
const RESOLUTION_Y: f32 = 1024.0;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum TileKind {
    Water,
    Grass,
    Forest,
    Road,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Tile {
    kind: TileKind,
    top: (TileKind, TileKind),
    bottom: (TileKind, TileKind),
    left: (TileKind, TileKind),
    right: (TileKind, TileKind),
}

impl Tile {
    fn tile_indexes() -> std::collections::HashMap<usize, Tile> {
        use TileKind::*;

        std::collections::HashMap::from([
            (
                0,
                Tile {
                    kind: Forest,
                    top: (Grass, Grass),
                    bottom: (Grass, Forest),
                    left: (Grass, Grass),
                    right: (Grass, Forest),
                },
            ),
            (
                1,
                Tile {
                    kind: Forest,
                    top: (Grass, Grass),
                    bottom: (Forest, Forest),
                    left: (Grass, Forest),
                    right: (Grass, Forest),
                },
            ),
            (
                2,
                Tile {
                    kind: Forest,
                    top: (Grass, Grass),
                    bottom: (Forest, Grass),
                    left: (Grass, Forest),
                    right: (Grass, Grass),
                },
            ),
            (
                10,
                Tile {
                    kind: Forest,
                    top: (Grass, Forest),
                    bottom: (Grass, Forest),
                    left: (Grass, Grass),
                    right: (Forest, Forest),
                },
            ),
            (
                11,
                Tile {
                    kind: Forest,
                    top: (Forest, Forest),
                    bottom: (Forest, Forest),
                    left: (Forest, Forest),
                    right: (Forest, Forest),
                },
            ),
            (
                12,
                Tile {
                    kind: Forest,
                    top: (Forest, Grass),
                    bottom: (Forest, Grass),
                    left: (Forest, Forest),
                    right: (Grass, Grass),
                },
            ),
            (
                20,
                Tile {
                    kind: Forest,
                    top: (Grass, Forest),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Forest, Grass),
                },
            ),
            (
                21,
                Tile {
                    kind: Forest,
                    top: (Forest, Forest),
                    bottom: (Grass, Grass),
                    left: (Grass, Forest),
                    right: (Grass, Forest),
                },
            ),
            (
                22,
                Tile {
                    kind: Forest,
                    top: (Forest, Grass),
                    bottom: (Grass, Grass),
                    left: (Forest, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                3,
                Tile {
                    kind: Water,
                    top: (Grass, Grass),
                    bottom: (Grass, Water),
                    left: (Grass, Grass),
                    right: (Grass, Water),
                },
            ),
            (
                4,
                Tile {
                    kind: Water,
                    top: (Grass, Grass),
                    bottom: (Water, Water),
                    left: (Grass, Water),
                    right: (Grass, Water),
                },
            ),
            (
                5,
                Tile {
                    kind: Water,
                    top: (Grass, Grass),
                    bottom: (Water, Grass),
                    left: (Grass, Water),
                    right: (Grass, Grass),
                },
            ),
            (
                13,
                Tile {
                    kind: Water,
                    top: (Grass, Water),
                    bottom: (Grass, Water),
                    left: (Grass, Grass),
                    right: (Water, Water),
                },
            ),
            (
                14,
                Tile {
                    kind: Water,
                    top: (Water, Water),
                    bottom: (Water, Water),
                    left: (Water, Water),
                    right: (Water, Water),
                },
            ),
            (
                15,
                Tile {
                    kind: Water,
                    top: (Water, Grass),
                    bottom: (Water, Grass),
                    left: (Water, Water),
                    right: (Grass, Grass),
                },
            ),
            (
                23,
                Tile {
                    kind: Water,
                    top: (Grass, Water),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Water, Grass),
                },
            ),
            (
                24,
                Tile {
                    kind: Water,
                    top: (Water, Water),
                    bottom: (Grass, Grass),
                    left: (Grass, Water),
                    right: (Grass, Water),
                },
            ),
            (
                25,
                Tile {
                    kind: Water,
                    top: (Water, Grass),
                    bottom: (Grass, Grass),
                    left: (Water, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                30,
                Tile {
                    kind: Forest,
                    top: (Forest, Forest),
                    bottom: (Forest, Grass),
                    left: (Forest, Forest),
                    right: (Forest, Grass),
                },
            ),
            (
                31,
                Tile {
                    kind: Forest,
                    top: (Forest, Forest),
                    bottom: (Grass, Forest),
                    left: (Forest, Grass),
                    right: (Forest, Forest),
                },
            ),
            (
                40,
                Tile {
                    kind: Forest,
                    top: (Forest, Grass),
                    bottom: (Forest, Forest),
                    left: (Forest, Forest),
                    right: (Grass, Forest),
                },
            ),
            (
                41,
                Tile {
                    kind: Forest,
                    top: (Grass, Forest),
                    bottom: (Forest, Forest),
                    left: (Grass, Forest),
                    right: (Forest, Forest),
                },
            ),
            (
                70,
                Tile {
                    kind: Grass,
                    top: (Grass, Grass),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                32,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Road, Road),
                    left: (Grass, Grass),
                    right: (Road, Road),
                },
            ),
            (
                33,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Road, Road),
                    left: (Road, Road),
                    right: (Road, Road),
                },
            ),
            (
                34,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Road, Road),
                    left: (Road, Road),
                    right: (Grass, Grass),
                },
            ),
            (
                42,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Road, Road),
                    left: (Grass, Grass),
                    right: (Road, Road),
                },
            ),
            //(
            //    43,
            //    Tile {
            //        kind: Road,
            //        top: (Road, Road),
            //        bottom: (Road, Road),
            //        left: (Road, Road),
            //        right: (Road, Road),
            //    },
            //),
            (
                44,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Road, Road),
                    left: (Road, Road),
                    right: (Grass, Grass),
                },
            ),
            (
                52,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Road, Road),
                },
            ),
            (
                53,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Grass, Grass),
                    left: (Road, Road),
                    right: (Road, Road),
                },
            ),
            (
                54,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Grass, Grass),
                    left: (Road, Road),
                    right: (Grass, Grass),
                },
            ),
            (
                35,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Grass, Grass),
                    left: (Road, Road),
                    right: (Road, Road),
                },
            ),
            (
                36,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Road, Road),
                    left: (Grass, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                45,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Grass, Grass),
                    left: (Road, Road),
                    right: (Grass, Grass),
                },
            ),
            (
                46,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Road, Road),
                    left: (Grass, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                55,
                Tile {
                    kind: Road,
                    top: (Grass, Grass),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Road, Road),
                },
            ),
            (
                56,
                Tile {
                    kind: Road,
                    top: (Road, Road),
                    bottom: (Grass, Grass),
                    left: (Grass, Grass),
                    right: (Grass, Grass),
                },
            ),
            (
                50,
                Tile {
                    kind: Water,
                    top: (Water, Water),
                    bottom: (Water, Grass),
                    left: (Water, Water),
                    right: (Water, Grass),
                },
            ),
            (
                51,
                Tile {
                    kind: Water,
                    top: (Water, Water),
                    bottom: (Grass, Water),
                    left: (Water, Grass),
                    right: (Water, Water),
                },
            ),
            (
                60,
                Tile {
                    kind: Water,
                    top: (Water, Grass),
                    bottom: (Water, Water),
                    left: (Water, Water),
                    right: (Grass, Water),
                },
            ),
            (
                61,
                Tile {
                    kind: Water,
                    top: (Grass, Water),
                    bottom: (Water, Water),
                    left: (Grass, Water),
                    right: (Water, Water),
                },
            ),
        ])
    }
}

fn find_possible_tiles_given_neighbours(
    possible_tiles: &[usize],
    neighbours: (&[usize], &[usize], &[usize], &[usize]),
) -> Vec<usize> {
    let top_neighbour = neighbours.0;
    let bottom_neighbour = neighbours.1;
    let left_neighbour = neighbours.2;
    let right_neighbour = neighbours.3;

    let tile_indexes = Tile::tile_indexes();

    let top_neighbours_edges = top_neighbour
        .iter()
        .map(|tile_index| tile_indexes.get(tile_index).unwrap().bottom)
        .collect::<Vec<_>>();

    let bottom_neighbours_edges = bottom_neighbour
        .iter()
        .map(|tile_index| tile_indexes.get(tile_index).unwrap().top)
        .collect::<Vec<_>>();

    let left_neighbours_edges = left_neighbour
        .iter()
        .map(|tile_index| tile_indexes.get(tile_index).unwrap().right)
        .collect::<Vec<_>>();

    let right_neighbours_edges = right_neighbour
        .iter()
        .map(|tile_index| tile_indexes.get(tile_index).unwrap().left)
        .collect::<Vec<_>>();

    let new_possible: Vec<usize> = possible_tiles
        .iter()
        .filter(|tile_index| {
            let tile = tile_indexes.get(tile_index).unwrap();
            let top = &tile.top;
            let bottom = &tile.bottom;
            let left = &tile.left;
            let right = &tile.right;

            if (top_neighbours_edges.len() == 0 && bottom_neighbours_edges.len() == 0)
                || (left_neighbours_edges.len() == 0 && right_neighbours_edges.len() == 0)
            {
                return false;
            }
            // Use Option instead of len 0
            (top_neighbours_edges.len() == 0 || top_neighbours_edges.contains(&top))
                && (bottom_neighbours_edges.len() == 0 || bottom_neighbours_edges.contains(&bottom))
                && (left_neighbours_edges.len() == 0 || left_neighbours_edges.contains(&left))
                && (right_neighbours_edges.len() == 0 || right_neighbours_edges.contains(&right))
        })
        .copied()
        .collect();

    if new_possible.len() == 0 {
        vec![50]
    } else {
        new_possible
    }
}

fn generate_map() -> Vec<Vec<usize>> {
    let tile_indexes = Tile::tile_indexes();
    let all_indexes = tile_indexes.keys().copied().collect::<Vec<usize>>();

    let mut map = vec![vec![0; MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    let mut possible_tiles =
        vec![vec![all_indexes.clone(); MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    fn get_lowest_entropy_tile(possible_tiles: &Vec<Vec<Vec<usize>>>) -> Option<(usize, usize)> {
        let mut lowest_entropy = std::usize::MAX;
        let mut lowest_entropy_tile = None;

        for (x, row) in possible_tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let entropy = tile.len();
                if entropy < lowest_entropy && entropy > 1 {
                    lowest_entropy = entropy;
                    lowest_entropy_tile = Some((x, y));
                }
            }
        }

        lowest_entropy_tile
    }

    fn all_collapsed(possible_tiles: &Vec<Vec<Vec<usize>>>) -> bool {
        possible_tiles
            .iter()
            .all(|row| row.iter().all(|tile| tile.len() == 1))
    }

    fn find_neighbours((x, y): (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut neighbours = [None; 4];

        if y < MAP_HEIGHT - 1 {
            neighbours[0] = Some((x, y + 1));
        }

        if y > 0 {
            neighbours[1] = Some((x, y - 1));
        }

        if x > 0 {
            neighbours[2] = Some((x - 1, y));
        }

        if x < MAP_WIDTH - 1 {
            neighbours[3] = Some((x + 1, y));
        }

        neighbours
    }

    while !all_collapsed(&possible_tiles) {
        let Some((x, y)) = get_lowest_entropy_tile(&possible_tiles) else {
            break;
        };

        // collapse one
        let tile = possible_tiles[x][y].choose(&mut rand::rng());
        if let Some(tile) = tile {
            possible_tiles[x][y] = vec![*tile];
        }

        let mut to_rearrange: Vec<(usize, usize)> = find_neighbours((x, y))
            .iter()
            .filter_map(|neighbour| neighbour.map(|(x, y)| (x, y)))
            .collect();

        while let Some((x, y)) = to_rearrange.pop() {
            let neighbours = find_neighbours((x, y));
            let new_possible = find_possible_tiles_given_neighbours(
                &possible_tiles[x][y],
                (
                    &neighbours[0].map_or(&[] as &[usize], |(x, y)| &possible_tiles[x][y]),
                    &neighbours[1].map_or(&[] as &[usize], |(x, y)| &possible_tiles[x][y]),
                    &neighbours[2].map_or(&[] as &[usize], |(x, y)| &possible_tiles[x][y]),
                    &neighbours[3].map_or(&[] as &[usize], |(x, y)| &possible_tiles[x][y]),
                ),
            );
            if new_possible != possible_tiles[x][y] {
                possible_tiles[x][y] = new_possible;
                to_rearrange.extend(
                    find_neighbours((x, y))
                        .iter()
                        .filter_map(|neighbour| neighbour.map(|(x, y)| (x, y))),
                );
            }
        }
    }

    // Pick random for all
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let tile = possible_tiles[x][y].choose(&mut rand::rng());
            if let Some(tile) = tile {
                map[x][y] = *tile;
            }
        }
    }

    map
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
                // .spawn(Sprite::from_color(
                //     random_color(),
                //     Vec2::new(TILE_SIZE, TILE_SIZE),
                // ))
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
            .filter(|(sprite, pos)| {
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

fn setup(mut commands: Commands, mut window: Single<&mut Window>) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(RESOLUTION_X / 2.0, RESOLUTION_Y / 2.0, 0.0),
    ));
    window.resolution.set(RESOLUTION_X, RESOLUTION_Y);
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
