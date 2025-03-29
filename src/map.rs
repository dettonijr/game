pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 50;
pub const TILE_SIZE: f32 = 32.0;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum TileKind {
    Water,
    Grass,
    Forest,
    Road,
    Crossroad,
    Roadturn,
    Roadend,
}

use TileKind::*;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Tile {
    kind: TileKind,
    top: (TileKind, TileKind),
    bottom: (TileKind, TileKind),
    left: (TileKind, TileKind),
    right: (TileKind, TileKind),
}

struct Neighbours {
    top: Option<(usize, usize)>,
    bottom: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
}

// Map from index in the texture atlas to the Tile info
type TileIndexes = std::collections::HashMap<usize, Tile>;

pub fn generate_map() -> Vec<Vec<usize>> {
    let tile_indexes = make_tile_indexes();
    let all_indexes = tile_indexes.keys().copied().collect::<Vec<usize>>();

    let mut possible_tiles =
        vec![vec![all_indexes.clone(); MAP_WIDTH as usize]; MAP_HEIGHT as usize];

    while let Some((x, y)) = get_lowest_entropy_tile(&possible_tiles) {
        // collapse one
        collapse(&tile_indexes, &mut possible_tiles[x][y]);

        let neighbours = find_neighbours((x, y));
        let mut to_rearrange: Vec<(usize, usize)> = neighbours_to_vec(&neighbours);

        while let Some((x, y)) = to_rearrange.pop() {
            let new_possible =
                find_possible_tiles_given_neighbours(&tile_indexes, &possible_tiles, (x, y));
            if new_possible != possible_tiles[x][y] {
                possible_tiles[x][y] = new_possible;

                let neighbours = find_neighbours((x, y));
                to_rearrange.extend(neighbours_to_vec(&neighbours));
            }
        }
    }

    possible_tiles
        .iter()
        .map(|row| row.iter().map(|tile| *tile.first().unwrap()).collect())
        .collect()
}

fn collapse(
    tile_indexes: &TileIndexes,
    possible_tiles: &mut Vec<usize>) {
    // map probability to each tile
    let probabilities = possible_tiles.iter()
        .map(|tile| tile_indexes.get(tile).unwrap().kind)
        .map(|tile| match tile {
            Water => 1.,
            Grass => 1.,
            Forest => 1.,
            Road => 5.,
            Crossroad => 0.0,
            Roadturn => 1.0,
            Roadend => 0.0,
        })
        .collect::<Vec<f32>>();

    let sum: f32 = probabilities.iter().sum();

    let mut random = rand::random_range(0.0..sum);

    for (i, probability) in probabilities.iter().enumerate() {
        random -= probability;
        if random <= 0.0 {
            *possible_tiles = vec![possible_tiles[i]];

            // if picked roadend print for debug
            let kind = tile_indexes.get(possible_tiles.first().unwrap()).unwrap().kind;
            if kind == Roadend {
                println!("Roadend picked {:?}", possible_tiles);
            } 

            return;
        }
    }
}

fn find_possible_tiles_given_neighbours(
    tile_indexes: &TileIndexes,
    possible_tiles: &Vec<Vec<Vec<usize>>>,
    (x, y): (usize, usize),
) -> Vec<usize> {
    // TODO Simplify this function
    let neighbours = find_neighbours((x, y));

    let top_neighbours_edges = neighbours.top.map_or(vec![], |(x, y)| {
        possible_tiles[x][y]
            .iter()
            .map(|tile_index| tile_indexes.get(tile_index).unwrap().bottom)
            .collect::<Vec<_>>()
    });

    let bottom_neighbours_edges = neighbours.bottom.map_or(vec![], |(x, y)| {
        possible_tiles[x][y]
            .iter()
            .map(|tile_index| tile_indexes.get(tile_index).unwrap().top)
            .collect::<Vec<_>>()
    });

    let left_neighbours_edges = neighbours.left.map_or(vec![], |(x, y)| {
        possible_tiles[x][y]
            .iter()
            .map(|tile_index| tile_indexes.get(tile_index).unwrap().right)
            .collect::<Vec<_>>()
    });

    let right_neighbours_edges = neighbours.right.map_or(vec![], |(x, y)| {
        possible_tiles[x][y]
            .iter()
            .map(|tile_index| tile_indexes.get(tile_index).unwrap().left)
            .collect::<Vec<_>>()
    });

    if (top_neighbours_edges.len() == 0 && bottom_neighbours_edges.len() == 0)
        || (left_neighbours_edges.len() == 0 && right_neighbours_edges.len() == 0)
    {
        return vec![50];
    }

    let new_possible: Vec<usize> = possible_tiles[x][y]
        .iter()
        .filter(|tile_index| {
            let tile = tile_indexes.get(tile_index).unwrap();
            let top = &tile.top;
            let bottom = &tile.bottom;
            let left = &tile.left;
            let right = &tile.right;

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

fn find_neighbours((x, y): (usize, usize)) -> Neighbours {
    Neighbours {
        top: (y < MAP_HEIGHT - 1).then(|| (x, y + 1)),
        bottom: (y > 0).then(|| (x, y - 1)),
        left: (x > 0).then(|| (x - 1, y)),
        right: (x < MAP_WIDTH - 1).then(|| (x + 1, y)),
    }
}

fn neighbours_to_vec(neighbours: &Neighbours) -> Vec<(usize, usize)> {
    [
        neighbours.top,
        neighbours.bottom,
        neighbours.left,
        neighbours.right,
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn make_tile_indexes() -> TileIndexes {
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
                kind: Roadturn,
                top: (Grass, Grass),
                bottom: (Road, Road),
                left: (Grass, Grass),
                right: (Road, Road),
            },
        ),
        (
            33,
            Tile {
                kind: Crossroad,
                top: (Grass, Grass),
                bottom: (Road, Road),
                left: (Road, Road),
                right: (Road, Road),
            },
        ),
        (
            34,
            Tile {
                kind: Roadturn,
                top: (Grass, Grass),
                bottom: (Road, Road),
                left: (Road, Road),
                right: (Grass, Grass),
            },
        ),
        (
            42,
            Tile {
                kind: Crossroad,
                top: (Road, Road),
                bottom: (Road, Road),
                left: (Grass, Grass),
                right: (Road, Road),
            },
        ),
        (
            43,
            Tile {
                kind: Crossroad,
                top: (Road, Road),
                bottom: (Road, Road),
                left: (Road, Road),
                right: (Road, Road),
            },
        ),
        (
            44,
            Tile {
                kind: Crossroad,
                top: (Road, Road),
                bottom: (Road, Road),
                left: (Road, Road),
                right: (Grass, Grass),
            },
        ),
        (
            52,
            Tile {
                kind: Roadturn,
                top: (Road, Road),
                bottom: (Grass, Grass),
                left: (Grass, Grass),
                right: (Road, Road),
            },
        ),
        (
            53,
            Tile {
                kind: Crossroad,
                top: (Road, Road),
                bottom: (Grass, Grass),
                left: (Road, Road),
                right: (Road, Road),
            },
        ),
        (
            54,
            Tile {
                kind: Roadturn,
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
                kind: Roadend,
                top: (Grass, Grass),
                bottom: (Grass, Grass),
                left: (Road, Road),
                right: (Grass, Grass),
            },
        ),
        (
            46,
            Tile {
                kind: Roadend,
                top: (Grass, Grass),
                bottom: (Road, Road),
                left: (Grass, Grass),
                right: (Grass, Grass),
            },
        ),
        (
            55,
            Tile {
                kind: Roadend,
                top: (Grass, Grass),
                bottom: (Grass, Grass),
                left: (Grass, Grass),
                right: (Road, Road),
            },
        ),
        (
            56,
            Tile {
                kind: Roadend,
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
