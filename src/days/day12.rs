use crate::days::Solution;
use rayon::prelude::*;

type Cell = (i32, i32);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Shape {
    cells: Vec<Cell>,
}

#[derive(Clone)]
struct Region {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

#[derive(Default)]
pub struct Day12 {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    // ------------------------------------------------------------
    // Geometry helpers
    // ------------------------------------------------------------

    // Takes shape cells, shifts them to the origin, sorts them, and returns a canonical cell list.
    fn normalize(cells: &[Cell]) -> Vec<Cell> {
        let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
        let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
        let mut normalized: Vec<_> = cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();
        normalized.sort();
        normalized
    }

    // Takes one shape, generates all rotations/flips, deduplicates them, and returns unique orientations.
    fn orientations(shape: &Shape) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut rotated = shape.cells.clone();

        for _ in 0..4 {
            rotated = rotated.iter().map(|(x, y)| (*y, -*x)).collect();
            for flip in [false, true] {
                let cells: Vec<Cell> = if flip {
                    rotated.iter().map(|(x, y)| (-x, *y)).collect()
                } else {
                    rotated.clone()
                };
                orientations.push(Shape {
                    cells: Self::normalize(&cells),
                });
            }
        }

        orientations.sort();
        orientations.dedup();
        orientations
    }

    // Takes an oriented shape and region size, enumerates valid placements, and returns occupied cell indices.
    fn placements(shape: &Shape, width: usize, height: usize) -> Vec<Vec<usize>> {
        let mut placements = Vec::new();
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let mut cells = Vec::new();
                let mut fits = true;
                for (dx, dy) in &shape.cells {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        fits = false;
                        break;
                    }
                    cells.push(ny as usize * width + nx as usize);
                }
                if fits {
                    placements.push(cells);
                }
            }
        }
        placements
    }

    // ------------------------------------------------------------
    // Solver
    // ------------------------------------------------------------

    // Takes a region and available shapes, backtracks over placements, and returns whether all presents fit.
    fn can_pack(region: &Region, shapes: &[Shape]) -> bool {
        let board_size = region.width * region.height;
        let mut occupied = vec![false; board_size];
        let mut remaining_counts = region.shape_counts.clone();

        let mut placements_by_shape: Vec<Vec<Vec<usize>>> = Vec::new();
        for shape in shapes {
            let mut placements = Vec::new();
            for orientation in Self::orientations(shape) {
                placements.extend(Self::placements(&orientation, region.width, region.height));
            }
            placements_by_shape.push(placements);
        }

        // Chooses the most constrained remaining shape, tries valid placements, and returns success/failure.
        fn dfs(
            occupied: &mut [bool],
            remaining_counts: &mut [usize],
            placements_by_shape: &[Vec<Vec<usize>>],
        ) -> bool {
            // Choose the most constrained shape (fewest valid placements)
            let mut best_shape = None;
            let mut fewest_valid_placements = usize::MAX;

            for (shape_index, &remaining) in remaining_counts.iter().enumerate() {
                if remaining == 0 {
                    continue;
                }
                let valid_count = placements_by_shape[shape_index]
                    .iter()
                    .filter(|placement| placement.iter().all(|&idx| !occupied[idx]))
                    .count();

                if valid_count == 0 {
                    return false; // dead end
                }
                if valid_count < fewest_valid_placements {
                    fewest_valid_placements = valid_count;
                    best_shape = Some(shape_index);
                }
            }

            // No shapes left → success
            let shape_index = match best_shape {
                Some(shape_index) => shape_index,
                None => return true,
            };

            // Try all valid placements for that shape
            for placement in &placements_by_shape[shape_index] {
                if placement.iter().all(|&idx| !occupied[idx]) {
                    for &idx in placement {
                        occupied[idx] = true;
                    }
                    remaining_counts[shape_index] -= 1;

                    if dfs(occupied, remaining_counts, placements_by_shape) {
                        return true;
                    }

                    remaining_counts[shape_index] += 1;
                    for &idx in placement {
                        occupied[idx] = false;
                    }
                }
            }
            false
        }

        dfs(&mut occupied, &mut remaining_counts, &placements_by_shape)
    }
}

// ------------------------------------------------------------
// Parsing
// ------------------------------------------------------------

// Takes the mixed shape/region input, parses shape cells and region counts, and fills the provided vectors.
fn parse_day12(lines: &[String], shapes: &mut Vec<Shape>, regions: &mut Vec<Region>) {
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].as_str();

        // ----------------------------
        // Shape block: "N:"
        // ----------------------------
        if line.ends_with(':') && line[..line.len() - 1].chars().all(|c| c.is_ascii_digit()) {
            i += 1;
            let mut cells = Vec::new();
            let mut y = 0;

            while i < lines.len() {
                let l = lines[i].as_str();

                // Stop if next shape or region starts
                if l.ends_with(':') || l.contains('x') {
                    break;
                }

                for (x, c) in l.chars().enumerate() {
                    if c == '#' {
                        cells.push((x as i32, y));
                    }
                }
                y += 1;
                i += 1;
            }

            shapes.push(Shape {
                cells: Day12::normalize(&cells),
            });
        }
        // ----------------------------
        // Region: "WxH: c c c c c c"
        // ----------------------------
        else if line.contains('x') && line.contains(':') {
            let parts: Vec<_> = line.split(':').collect();
            let dims: Vec<_> = parts[0].split('x').collect();

            let width = dims[0].parse::<usize>().unwrap();
            let height = dims[1].parse::<usize>().unwrap();
            let shape_counts = parts[1]
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            regions.push(Region {
                width,
                height,
                shape_counts,
            });
            i += 1;
        } else {
            i += 1;
        }
    }
}

// ------------------------------------------------------------
// Trait impl
// ------------------------------------------------------------

const SMALL_BOARD_MAX_AREA12: usize = 15 * 15;

impl Solution for Day12 {
    // Takes the full puzzle input, parses present shapes and target regions, and stores both.
    fn set_input(&mut self, lines: &[String]) {
        self.shapes.clear();
        self.regions.clear();
        parse_day12(lines, &mut self.shapes, &mut self.regions);
    }

    // Checks each region for fit feasibility in parallel and returns how many regions can fit their presents.
    fn part1(&mut self) -> String {
        let shapes = &self.shapes;

        self.regions
            .par_iter()
            .filter(|region| {
                let needed_area: usize = shapes
                    .iter()
                    .zip(&region.shape_counts)
                    .map(|(shape, &count)| shape.cells.len() * count)
                    .sum();

                let board_area = region.width * region.height;

                if needed_area > board_area {
                    return false;
                }
                if board_area > SMALL_BOARD_MAX_AREA12 {
                    // Same heuristic as your Go solution:
                    // "Large boards: assume area is sufficient."
                    return true;
                }

                Day12::can_pack(region, shapes)
            })
            .count()
            .to_string()
    }

    // Day 12 has no implemented second part in this codebase and returns a placeholder.
    fn part2(&mut self) -> String {
        "N/A".to_string()
    }
}

// ------------------------------------------------------------
// Tests (inline, Go-equivalent)
// ------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    const DAY12_EXAMPLE: &str = r#"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    fn split_lines(s: &str) -> Vec<String> {
        s.lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day12::new();
        d.set_input(&split_lines(DAY12_EXAMPLE));

        let got = d.part1();
        let want = "2";

        assert_eq!(got, want);
    }
}
