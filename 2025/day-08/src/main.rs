use glam::I64Vec3;
use itertools::Itertools;

type DistanceSquared = i64;
type Index = usize;

fn main() {
    let input = include_str!("../input.txt").trim();
    let coordinates: Vec<I64Vec3> = parse(input);

    let connections: Vec<(DistanceSquared, Index, Index)> = coordinates
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pair| {
            let (i, vec_a) = pair[0];
            let (j, vec_b) = pair[1];
            (vec_a.distance_squared(*vec_b), i, j)
        })
        .sorted()
        .collect();

    let mut uf = UnionFind::new(coordinates.len());
    for &(_, i, j) in connections.iter().take(1000) {
        uf.union(i, j);
    }

    println!(
        "Part 1. {}",
        uf.sorted_component_sizes()
            .iter()
            .take(3)
            .product::<usize>()
    );

    let mut uf = UnionFind::new(coordinates.len());

    let &(_, i, j) = connections
        .iter()
        .find(|&&(_, i, j)| {
            uf.union(i, j);
            uf.components == 1
        })
        .expect("Failed to solve part 2.");

    println!("Part 2. {}", coordinates[i].x * coordinates[j].x);
}

fn parse(s: &str) -> Vec<I64Vec3> {
    s.lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            I64Vec3::new(x, y, z)
        })
        .collect()
}

struct UnionFind {
    parent: Vec<Index>,
    size: Vec<Index>,
    components: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: Index) -> Index {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: Index, b: Index) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra != rb {
            if self.size[ra] < self.size[rb] {
                std::mem::swap(&mut ra, &mut rb);
            }

            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
            self.components -= 1;
        }
    }

    fn sorted_component_sizes(&self) -> Vec<usize> {
        self.parent
            .iter()
            .unique()
            .map(|&i| self.size[i])
            .sorted()
            .rev()
            .collect()
    }
}
