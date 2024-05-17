use std::collections::{BTreeMap, BTreeSet};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
}

impl std::cmp::PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl std::cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y != other.y {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Cell;

#[derive(Default)]
pub struct Grid {
    cells: BTreeMap<Coord, Cell>,
}

impl Grid {
    pub fn cycle(&mut self, coord: Coord) {
        if self.cells.remove(&coord).is_none() {
            self.cells.insert(coord, Default::default());
        };
    }

    pub fn get(&self, (a, b): (Coord, Coord)) -> impl Iterator<Item = (Coord, Cell)> + '_ {
        self.cells.range(a..=b).map(|(&coord, &cell)| (coord, cell))
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
    }

    pub fn multistep(&mut self, n: u128) {
        for _ in 0..n {
            self.step();
        }
    }
}
