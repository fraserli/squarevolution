use std::collections::{BTreeMap, BTreeSet};

const CHUNK_SIZE: i32 = 128;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn to_f32(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }

    fn neighbours(&self) -> [Self; 8] {
        let Self { x, y } = *self;
        [
            Coord { x: x - 1, y: y - 1 },
            Coord { x, y: y - 1 },
            Coord { x: x + 1, y: y - 1 },
            Coord { x: x - 1, y },
            Coord { x: x + 1, y },
            Coord { x: x - 1, y: y + 1 },
            Coord { x, y: y + 1 },
            Coord { x: x + 1, y: y + 1 },
        ]
    }

    fn chunk(&self) -> (i32, i32) {
        (self.x / CHUNK_SIZE, self.y / CHUNK_SIZE)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Cell;

#[derive(Default)]
pub struct Grid {
    cells: BTreeMap<Coord, Cell>,
    active_chunks: BTreeSet<(i32, i32)>,
}

impl Grid {
    pub fn cycle(&mut self, coord: Coord) {
        self.active_chunks.insert(coord.chunk());

        if self.cells.remove(&coord).is_none() {
            self.cells.insert(coord, Default::default());
        };
    }

    pub fn get(&self, coord: Coord) -> Option<Cell> {
        if self.active_chunks.contains(&coord.chunk()) {
            self.cells.get(&coord).copied()
        } else {
            None
        }
    }

    pub fn step(&mut self) {
        let cells: BTreeSet<Coord> = self
            .cells
            .iter()
            .flat_map(|(coord, _)| coord.neighbours())
            .collect();

        let cells: BTreeMap<Coord, Cell> = cells
            .into_iter()
            .filter_map(|coord| {
                let neighbours = coord
                    .neighbours()
                    .into_iter()
                    .filter_map(|c| self.cells.get(&c));
                let n = neighbours.count();
                match (self.cells.get(&coord), n) {
                    (Some(c), 2 | 3) => Some((coord, *c)),
                    (None, 3) => Some((coord, Default::default())),
                    _ => None,
                }
            })
            .collect();

        self.cells = cells;

        self.active_chunks = self.cells.keys().map(Coord::chunk).collect();
    }

    pub fn multistep(&mut self, n: u128) {
        for _ in 0..n {
            self.step();
        }
    }
}
