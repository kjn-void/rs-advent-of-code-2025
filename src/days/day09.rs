use crate::days::Solution;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Default)]
pub struct Day09 {
    red_tiles: Vec<Point>,
    boundary: Vec<Segment>,
    horizontal_edges: Vec<Segment>,
    vertical_edges: Vec<Segment>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    // ----------------------------------------------------------
    // Part 1
    // ----------------------------------------------------------

    // Takes red tile coordinates, checks all corner pairs, and returns the largest inclusive rectangle area.
    fn max_area_inclusive(red_tiles: &[Point]) -> i64 {
        let point_count = red_tiles.len();
        let mut best: i64 = 0;

        for i in 0..point_count {
            for j in i + 1..point_count {
                let width = (red_tiles[i].x - red_tiles[j].x).abs() as i64 + 1;
                let height = (red_tiles[i].y - red_tiles[j].y).abs() as i64 + 1;
                best = best.max(width * height);
            }
        }
        best
    }

    // ----------------------------------------------------------
    // Geometry helpers
    // ----------------------------------------------------------

    // Converts the ordered red-tile boundary into horizontal and vertical segments used by containment checks.
    fn build_edges(&mut self) {
        let point_count = self.red_tiles.len();
        self.boundary.clear();
        self.horizontal_edges.clear();
        self.vertical_edges.clear();

        for i in 0..point_count {
            let a = self.red_tiles[i];
            let b = self.red_tiles[(i + 1) % point_count];

            if a.y == b.y {
                let (x1, x2) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
                let edge = Segment {
                    x1,
                    y1: a.y,
                    x2,
                    y2: a.y,
                };
                self.boundary.push(edge);
                self.horizontal_edges.push(edge);
            } else {
                let (y1, y2) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
                let edge = Segment {
                    x1: a.x,
                    y1,
                    x2: a.x,
                    y2,
                };
                self.boundary.push(edge);
                self.vertical_edges.push(edge);
            }
        }
    }

    // Takes a point, checks boundary membership first, and returns whether it is inside or on the polygon.
    fn point_inside_or_on(&self, point: Point) -> bool {
        for edge in &self.horizontal_edges {
            if point.y == edge.y1 && point.x >= edge.x1 && point.x <= edge.x2 {
                return true;
            }
        }
        for edge in &self.vertical_edges {
            if point.x == edge.x1 && point.y >= edge.y1 && point.y <= edge.y2 {
                return true;
            }
        }

        self.point_inside_polygon(point)
    }

    // Takes a point, performs an integer ray-crossing test, and returns whether it is inside the polygon.
    fn point_inside_polygon(&self, point: Point) -> bool {
        let mut inside = false;

        for edge in &self.vertical_edges {
            // Vertical edge at x = e.x1, spanning [y1, y2)
            if point.y >= edge.y1 && point.y < edge.y2 && point.x < edge.x1 {
                inside = !inside;
            }
        }

        inside
    }

    // Takes rectangle bounds, tests boundary intersections through its interior, and returns whether it is cut.
    fn rectangle_cut_by_polygon(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        if x1 == x2 || y1 == y2 {
            return false;
        }

        for edge in &self.horizontal_edges {
            let y = edge.y1;
            if y <= y1 || y >= y2 {
                continue;
            }
            if edge.x1.max(x1) < edge.x2.min(x2) {
                return true;
            }
        }
        for edge in &self.vertical_edges {
            let x = edge.x1;
            if x <= x1 || x >= x2 {
                continue;
            }
            if edge.y1.max(y1) < edge.y2.min(y2) {
                return true;
            }
        }
        false
    }
}

impl Solution for Day09 {
    // Takes coordinate lines, parses the ordered red tiles, and clears derived polygon edges.
    fn set_input(&mut self, lines: &[String]) {
        self.red_tiles.clear();
        self.boundary.clear();
        self.horizontal_edges.clear();
        self.vertical_edges.clear();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(',');
            let x: i32 = parts.next().unwrap().parse().unwrap();
            let y: i32 = parts.next().unwrap().parse().unwrap();
            self.red_tiles.push(Point { x, y });
        }
    }

    // Finds the largest rectangle from any red-tile corner pair and returns its area.
    fn part1(&mut self) -> String {
        Self::max_area_inclusive(&self.red_tiles).to_string()
    }

    // Finds the largest rectangle contained in the boundary polygon and returns its area.
    fn part2(&mut self) -> String {
        if self.red_tiles.len() < 2 {
            return "0".to_string();
        }

        if self.boundary.is_empty() {
            self.build_edges();
        }

        let point_count = self.red_tiles.len();
        let mut best: i64 = 0;

        for i in 0..point_count {
            let a = self.red_tiles[i];
            for j in i + 1..point_count {
                let b = self.red_tiles[j];

                let x1 = a.x.min(b.x);
                let x2 = a.x.max(b.x);
                let y1 = a.y.min(b.y);
                let y2 = a.y.max(b.y);

                let area = (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64;
                if area <= best {
                    continue;
                }

                if self.rectangle_cut_by_polygon(x1, y1, x2, y2) {
                    continue;
                }

                let opposite_corner_a = Point { x: x1, y: y2 };
                let opposite_corner_b = Point { x: x2, y: y1 };

                if !self.point_inside_or_on(opposite_corner_a)
                    || !self.point_inside_or_on(opposite_corner_b)
                {
                    continue;
                }

                best = area;
            }
        }

        best.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::Solution;

    fn example_input() -> Vec<String> {
        vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day09::new();
        d.set_input(&example_input());
        assert_eq!(d.part1(), "50");
    }

    #[test]
    fn part2_example() {
        let mut d = Day09::new();
        d.set_input(&example_input());
        assert_eq!(d.part2(), "24");
    }
}
