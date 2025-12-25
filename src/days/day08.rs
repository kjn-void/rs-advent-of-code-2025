use crate::days::Solution;

// -----------------------------------------------------------
// Data types
// -----------------------------------------------------------

#[derive(Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy)]
struct Edge {
    dist2: i64,
    i: usize,
    j: usize,
}

pub struct Day08 {
    points: Vec<Vec3>,
    edges: Vec<Edge>,
}

impl Day08 {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            edges: Vec::new(),
        }
    }

    // -----------------------------------------------------------
    // Distance helpers
    // -----------------------------------------------------------

    fn squared_dist(a: Vec3, b: Vec3) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    fn build_sorted_edges(points: &[Vec3]) -> Vec<Edge> {
        let n = points.len();
        if n < 2 {
            return Vec::new();
        }

        let mut edges = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in i + 1..n {
                edges.push(Edge {
                    dist2: Self::squared_dist(points[i], points[j]),
                    i,
                    j,
                });
            }
        }

        edges.sort_by(|a, b| a.dist2.cmp(&b.dist2));
        edges
    }

    // -----------------------------------------------------------
    // Union-Find
    // -----------------------------------------------------------

    fn run_connections(points: &[Vec3], edges: &[Edge], k: usize) -> Vec<usize> {
        let n = points.len();
        if n == 0 {
            return Vec::new();
        }

        let mut uf = DSU::new(n);
        let limit = k.min(edges.len());

        for idx in 0..limit {
            let e = edges[idx];
            uf.union(e.i, e.j);
        }

        let mut comp_map = std::collections::HashMap::<usize, usize>::new();
        for i in 0..n {
            let r = uf.find(i);
            comp_map.insert(r, uf.size[r]);
        }

        let mut sizes: Vec<usize> = comp_map.values().copied().collect();
        sizes.sort_by(|a, b| b.cmp(a)); // descending
        sizes
    }

    fn run_until_single_circuit(points: &[Vec3], edges: &[Edge]) -> (usize, usize) {
        let n = points.len();
        if n <= 1 {
            return (0, 0);
        }

        let mut uf = DSU::new(n);
        let mut components = n;
        let mut last = (0, 0);

        for e in edges {
            if uf.union(e.i, e.j) {
                components -= 1;
                last = (e.i, e.j);
                if components == 1 {
                    break;
                }
            }
        }

        last
    }
}

// -----------------------------------------------------------
// DSU
// -----------------------------------------------------------

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        true
    }
}

// -----------------------------------------------------------
// Parsing
// -----------------------------------------------------------

fn parse_vec3(line: &str) -> Vec3 {
    let mut it = line.split(',');
    Vec3 {
        x: it.next().unwrap().parse().unwrap(),
        y: it.next().unwrap().parse().unwrap(),
        z: it.next().unwrap().parse().unwrap(),
    }
}

// -----------------------------------------------------------
// Solution impl
// -----------------------------------------------------------

impl Solution for Day08 {
    fn set_input(&mut self, lines: &[String]) {
        self.points.clear();
        for line in lines {
            let s = line.trim();
            if !s.is_empty() {
                self.points.push(parse_vec3(s));
            }
        }
        self.edges = Self::build_sorted_edges(&self.points);
    }

    fn part1(&mut self) -> String {
        let sizes = Self::run_connections(&self.points, &self.edges, 1000);
        if sizes.len() < 3 {
            return "0".to_string();
        }
        (sizes[0] * sizes[1] * sizes[2]).to_string()
    }

    fn part2(&mut self) -> String {
        if self.points.len() < 2 {
            return "0".to_string();
        }
        let (i, j) = Self::run_until_single_circuit(&self.points, &self.edges);
        (self.points[i].x * self.points[j].x).to_string()
    }
}

// -----------------------------------------------------------
// Tests (inline, Go-equivalent)
// -----------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        vec![
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn part1_example() {
        let mut d = Day08::new();
        d.set_input(&example_input());

        let sizes = Day08::run_connections(&d.points, &d.edges, 10);
        let got = sizes[0] * sizes[1] * sizes[2];
        let want = 40;

        assert_eq!(got, want);
    }

    #[test]
    fn part2_example() {
        let mut d = Day08::new();
        d.set_input(&example_input());

        let (i, j) = Day08::run_until_single_circuit(&d.points, &d.edges);
        let got = d.points[i].x * d.points[j].x;
        let want: i64 = 25272;

        assert_eq!(got, want);
    }
}