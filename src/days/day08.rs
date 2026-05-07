use crate::days::Solution;

// -----------------------------------------------------------
// Data types
// -----------------------------------------------------------

#[derive(Clone, Copy)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy)]
struct Edge {
    distance_sq: i64,
    a: usize,
    b: usize,
}

#[derive(Default)]
pub struct Day08 {
    junctions: Vec<Point3>,
    edges: Vec<Edge>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    // -----------------------------------------------------------
    // Distance helpers
    // -----------------------------------------------------------

    // Takes two junction coordinates and returns their squared Euclidean distance for ordering edges.
    fn squared_dist(a: Point3, b: Point3) -> i64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        let dz = a.z - b.z;
        dx * dx + dy * dy + dz * dz
    }

    // Takes all junctions, builds every pairwise connection edge, and returns them sorted by distance.
    fn build_sorted_edges(junctions: &[Point3]) -> Vec<Edge> {
        let junction_count = junctions.len();
        if junction_count < 2 {
            return Vec::new();
        }

        let mut edges = Vec::with_capacity(junction_count * (junction_count - 1) / 2);
        for a in 0..junction_count - 1 {
            for b in a + 1..junction_count {
                edges.push(Edge {
                    distance_sq: Self::squared_dist(junctions[a], junctions[b]),
                    a,
                    b,
                });
            }
        }

        radix_sort_edges(&mut edges);
        edges
    }

    // -----------------------------------------------------------
    // Union-Find
    // -----------------------------------------------------------

    // Takes sorted edges and a connection count, unions that many closest pairs, and returns circuit sizes.
    fn circuit_sizes_after_connections(
        junctions: &[Point3],
        edges: &[Edge],
        connection_count: usize,
    ) -> Vec<usize> {
        let junction_count = junctions.len();
        if junction_count == 0 {
            return Vec::new();
        }

        let mut circuits = Dsu::new(junction_count);
        let limit = connection_count.min(edges.len());

        for edge in edges.iter().take(limit) {
            circuits.union(edge.a, edge.b);
        }

        let mut seen_roots = vec![false; junction_count];
        let mut sizes = Vec::with_capacity(junction_count);
        for index in 0..junction_count {
            let root = circuits.find(index);
            if !seen_roots[root] {
                seen_roots[root] = true;
                sizes.push(circuits.size[root]);
            }
        }

        sizes.sort_by(|a, b| b.cmp(a)); // descending
        sizes
    }

    // Takes sorted edges, connects until one circuit remains, and returns the final edge's endpoint indices.
    fn final_connection(junctions: &[Point3], edges: &[Edge]) -> (usize, usize) {
        let junction_count = junctions.len();
        if junction_count <= 1 {
            return (0, 0);
        }

        let mut circuits = Dsu::new(junction_count);
        let mut component_count = junction_count;
        let mut last_connection = (0, 0);

        for edge in edges {
            if circuits.union(edge.a, edge.b) {
                component_count -= 1;
                last_connection = (edge.a, edge.b);
                if component_count == 1 {
                    break;
                }
            }
        }

        last_connection
    }
}

// Takes mutable distance edges, sorts them by squared distance with radix passes, and returns in-place.
fn radix_sort_edges(edges: &mut [Edge]) {
    if edges.len() < 2 {
        return;
    }

    const RADIX_BITS: usize = 16;
    const BUCKETS: usize = 1 << RADIX_BITS;
    const MASK: u64 = (BUCKETS as u64) - 1;

    let mut src = edges.to_vec();
    let mut dst = vec![edges[0]; edges.len()];
    let mut counts = vec![0usize; BUCKETS];

    for shift in (0..64).step_by(RADIX_BITS) {
        counts.fill(0);
        for edge in src.iter() {
            counts[((edge.distance_sq as u64 >> shift) & MASK) as usize] += 1;
        }

        let mut sum = 0;
        for count in counts.iter_mut() {
            let current = *count;
            *count = sum;
            sum += current;
        }

        for edge in src.iter().copied() {
            let bucket = ((edge.distance_sq as u64 >> shift) & MASK) as usize;
            dst[counts[bucket]] = edge;
            counts[bucket] += 1;
        }

        std::mem::swap(&mut src, &mut dst);
    }

    edges.copy_from_slice(&src);
}

// -----------------------------------------------------------
// Disjoint set union
// -----------------------------------------------------------

struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    // Creates singleton sets for the requested node count and returns the union-find structure.
    fn new(node_count: usize) -> Self {
        let mut parent = Vec::with_capacity(node_count);
        let mut size = Vec::with_capacity(node_count);
        for node in 0..node_count {
            parent.push(node);
            size.push(1);
        }
        Self { parent, size }
    }

    // Takes a node, compresses its parent path, and returns the root representative.
    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    // Takes two nodes, joins their sets if distinct, and returns whether a merge happened.
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

// Takes an X,Y,Z input line, parses each coordinate, and returns a 3D point.
fn parse_point3(line: &str) -> Point3 {
    let mut it = line.split(',');
    Point3 {
        x: it.next().unwrap().parse().unwrap(),
        y: it.next().unwrap().parse().unwrap(),
        z: it.next().unwrap().parse().unwrap(),
    }
}

// -----------------------------------------------------------
// Solution impl
// -----------------------------------------------------------

impl Solution for Day08 {
    // Takes junction coordinate lines, parses them, and precomputes sorted connection edges.
    fn set_input(&mut self, lines: &[String]) {
        self.junctions.clear();
        for line in lines {
            let s = line.trim();
            if !s.is_empty() {
                self.junctions.push(parse_point3(s));
            }
        }
        self.edges = Self::build_sorted_edges(&self.junctions);
    }

    // Connects the 1000 closest pairs and returns the product of the three largest circuit sizes.
    fn part1(&mut self) -> String {
        let sizes = Self::circuit_sizes_after_connections(&self.junctions, &self.edges, 1000);
        if sizes.len() < 3 {
            return "0".to_string();
        }
        (sizes[0] * sizes[1] * sizes[2]).to_string()
    }

    // Connects until all junctions share one circuit and returns the puzzle's final endpoint product.
    fn part2(&mut self) -> String {
        if self.junctions.len() < 2 {
            return "0".to_string();
        }
        let (a, b) = Self::final_connection(&self.junctions, &self.edges);
        (self.junctions[a].x * self.junctions[b].x).to_string()
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

        let sizes = Day08::circuit_sizes_after_connections(&d.junctions, &d.edges, 10);
        let got = sizes[0] * sizes[1] * sizes[2];
        let want = 40;

        assert_eq!(got, want);
    }

    #[test]
    fn part2_example() {
        let mut d = Day08::new();
        d.set_input(&example_input());

        let (a, b) = Day08::final_connection(&d.junctions, &d.edges);
        let got = d.junctions[a].x * d.junctions[b].x;
        let want: i64 = 25272;

        assert_eq!(got, want);
    }
}
