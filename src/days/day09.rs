use crate::days::Solution;

#[derive(Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Edge {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    hor: bool,
}

pub struct Day09 {
    reds: Vec<Pt>,
    edges: Vec<Edge>,
}

impl Day09 {
    pub fn new() -> Self {
        Self {
            reds: Vec::new(),
            edges: Vec::new(),
        }
    }

    // ----------------------------------------------------------
    // Part 1
    // ----------------------------------------------------------

    fn max_area_inclusive(points: &[Pt]) -> i64 {
        let n = points.len();
        let mut best: i64 = 0;

        for i in 0..n {
            for j in i + 1..n {
                let dx = (points[i].x - points[j].x).abs() as i64 + 1;
                let dy = (points[i].y - points[j].y).abs() as i64 + 1;

                let area = dx * dy;
                best = best.max(area);
            }
        }
        best
    }
    // ----------------------------------------------------------
    // Part 2 helpers
    // ----------------------------------------------------------

    fn build_edges(&mut self) {
        let n = self.reds.len();
        self.edges.clear();

        for i in 0..n {
            let a = self.reds[i];
            let b = self.reds[(i + 1) % n];

            if a.y == b.y {
                let (x1, x2) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
                self.edges.push(Edge {
                    x1,
                    y1: a.y,
                    x2,
                    y2: a.y,
                    hor: true,
                });
            } else {
                let (y1, y2) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
                self.edges.push(Edge {
                    x1: a.x,
                    y1,
                    x2: a.x,
                    y2,
                    hor: false,
                });
            }
        }
    }

    fn point_inside_or_on(&self, p: Pt) -> bool {
        // Boundary check
        for e in &self.edges {
            if e.hor {
                if p.y == e.y1 && p.x >= e.x1 && p.x <= e.x2 {
                    return true;
                }
            } else {
                if p.x == e.x1 && p.y >= e.y1 && p.y <= e.y2 {
                    return true;
                }
            }
        }
        Self::point_in_polygon_ray_cast(p, &self.reds)
    }

    fn point_in_polygon_ray_cast(p: Pt, poly: &[Pt]) -> bool {
        let mut inside = false;
        let n = poly.len();
        let mut j = n - 1;

        for i in 0..n {
            let pi = poly[i];
            let pj = poly[j];

            if (pi.y > p.y) != (pj.y > p.y) {
                let x_intersect = (pj.x as f64)
                    + ((p.y - pj.y) as f64)
                        * ((pi.x - pj.x) as f64)
                        / ((pi.y - pj.y) as f64);

                if (p.x as f64) < x_intersect {
                    inside = !inside;
                }
            }
            j = i;
        }
        inside
    }

    fn rectangle_cut_by_polygon(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        if x1 == x2 || y1 == y2 {
            return false;
        }

        for e in &self.edges {
            if e.hor {
                let y = e.y1;
                if y <= y1 || y >= y2 {
                    continue;
                }
                if e.x1.max(x1) < e.x2.min(x2) {
                    return true;
                }
            } else {
                let x = e.x1;
                if x <= x1 || x >= x2 {
                    continue;
                }
                if e.y1.max(y1) < e.y2.min(y2) {
                    return true;
                }
            }
        }
        false
    }
}

impl Solution for Day09 {
    fn set_input(&mut self, lines: &[String]) {
        self.reds.clear();
        self.edges.clear();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split(',');
            let x: i32 = parts.next().unwrap().parse().unwrap();
            let y: i32 = parts.next().unwrap().parse().unwrap();
            self.reds.push(Pt { x, y });
        }
    }

    fn part1(&mut self) -> String {
        Self::max_area_inclusive(&self.reds).to_string()
    }

    fn part2(&mut self) -> String {
        if self.reds.len() < 2 {
            return "0".to_string();
        }

        if self.edges.is_empty() {
            self.build_edges();
        }

        let n = self.reds.len();
        let mut best: i64 = 0;

        for i in 0..n {
            let a = self.reds[i];
            for j in i + 1..n {
                let b = self.reds[j];

                let x1 = a.x.min(b.x);
                let x2 = a.x.max(b.x);
                let y1 = a.y.min(b.y);
                let y2 = a.y.max(b.y);

                let area =
                    (x2 - x1 + 1) as i64 * (y2 - y1 + 1) as i64;

                if area <= best {
                    continue;
                }

                let c3 = Pt { x: x1, y: y2 };
                let c4 = Pt { x: x2, y: y1 };

                if !self.point_inside_or_on(c3) || !self.point_inside_or_on(c4) {
                    continue;
                }

                if self.rectangle_cut_by_polygon(x1, y1, x2, y2) {
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
        vec![
            "7,1",
            "11,1",
            "11,7",
            "9,7",
            "9,5",
            "2,5",
            "2,3",
            "7,3",
        ]
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