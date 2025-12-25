use crate::days::Solution;
use rayon::prelude::*;

type Cell = (i32, i32);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Shape {
    cells: Vec<Cell>,
}

#[derive(Clone)]
struct Region {
    w: usize,
    h: usize,
    counts: Vec<usize>,
}

pub struct Day12 {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl Day12 {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            regions: Vec::new(),
        }
    }

    // ------------------------------------------------------------
    // Geometry helpers
    // ------------------------------------------------------------

    fn normalize(cells: &[Cell]) -> Vec<Cell> {
        let min_x = cells.iter().map(|(x, _)| *x).min().unwrap();
        let min_y = cells.iter().map(|(_, y)| *y).min().unwrap();
        let mut out: Vec<_> = cells.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();
        out.sort();
        out
    }

    fn orientations(shape: &Shape) -> Vec<Shape> {
        let mut out = Vec::new();
        let mut cur = shape.cells.clone();

        for _ in 0..4 {
            cur = cur.iter().map(|(x, y)| (*y, -*x)).collect();
            for flip in [false, true] {
                let v: Vec<Cell> = if flip {
                    cur.iter().map(|(x, y)| (-x, *y)).collect()
                } else {
                    cur.clone()
                };
                out.push(Shape {
                    cells: Self::normalize(&v),
                });
            }
        }

        out.sort();
        out.dedup();
        out
    }

    fn placements(shape: &Shape, w: usize, h: usize) -> Vec<Vec<usize>> {
        let mut out = Vec::new();
        for x in 0..w as i32 {
            for y in 0..h as i32 {
                let mut cells = Vec::new();
                let mut ok = true;
                for (dx, dy) in &shape.cells {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= w as i32 || ny >= h as i32 {
                        ok = false;
                        break;
                    }
                    cells.push(ny as usize * w + nx as usize);
                }
                if ok {
                    out.push(cells);
                }
            }
        }
        out
    }

    // ------------------------------------------------------------
    // Solver
    // ------------------------------------------------------------

    fn can_pack(region: &Region, shapes: &[Shape]) -> bool {
        let size = region.w * region.h;
        let mut occ = vec![false; size];
        let mut counts = region.counts.clone();

        let mut placements: Vec<Vec<Vec<usize>>> = Vec::new();
        for s in shapes {
            let mut all = Vec::new();
            for o in Self::orientations(s) {
                all.extend(Self::placements(&o, region.w, region.h));
            }
            placements.push(all);
        }

        fn dfs(occ: &mut [bool], counts: &mut [usize], placements: &[Vec<Vec<usize>>]) -> bool {
            // Choose the most constrained shape (fewest valid placements)
            let mut best_shape = None;
            let mut best_count = usize::MAX;

            for (i, &c) in counts.iter().enumerate() {
                if c == 0 {
                    continue;
                }
                let valid = placements[i]
                    .iter()
                    .filter(|p| p.iter().all(|&idx| !occ[idx]))
                    .count();

                if valid == 0 {
                    return false; // dead end
                }
                if valid < best_count {
                    best_count = valid;
                    best_shape = Some(i);
                }
            }

            // No shapes left → success
            let s = match best_shape {
                Some(v) => v,
                None => return true,
            };

            // Try all valid placements for that shape
            for place in &placements[s] {
                if place.iter().all(|&i| !occ[i]) {
                    for &i in place {
                        occ[i] = true;
                    }
                    counts[s] -= 1;

                    if dfs(occ, counts, placements) {
                        return true;
                    }

                    counts[s] += 1;
                    for &i in place {
                        occ[i] = false;
                    }
                }
            }
            false
        }

        dfs(&mut occ, &mut counts, &placements)
    }
}

// ------------------------------------------------------------
// Parsing
// ------------------------------------------------------------

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

            let w = dims[0].parse::<usize>().unwrap();
            let h = dims[1].parse::<usize>().unwrap();
            let counts = parts[1]
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            regions.push(Region { w, h, counts });
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
    fn set_input(&mut self, lines: &[String]) {
        self.shapes.clear();
        self.regions.clear();
        parse_day12(lines, &mut self.shapes, &mut self.regions);
    }

    fn part1(&mut self) -> String {
        let shapes = &self.shapes;

        self.regions
            .par_iter()
            .filter(|r| {
                let needed_area: usize = shapes
                    .iter()
                    .zip(&r.counts)
                    .map(|(s, &c)| s.cells.len() * c)
                    .sum();

                let board_area = r.w * r.h;

                if needed_area > board_area {
                    return false;
                }
                if board_area > SMALL_BOARD_MAX_AREA12 {
                    // Same heuristic as your Go solution:
                    // "Large boards: assume area is sufficient."
                    return true;
                }

                Day12::can_pack(r, shapes)
            })
            .count()
            .to_string()
    }

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
