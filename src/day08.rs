use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Edge {
    dist: i64,
    i: usize,
    j: usize,
}

// Disjoint set union for Kruskal's algorithm
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

// based on the german wikipedia article for Kruskal's algorithm
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect_vec(),
            rank: vec![1; n],
        }
    }

    /**
     * find the root of the set containing x
     **/
    fn find_root(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            // path compression
            self.parent[i] = self.find_root(self.parent[i]);
        }
        self.parent[i]
    }

    /**
     * union the sets containing x and y
     * returns true if sets were merged, false if already in the same set
     **/
    fn union(&mut self, i: usize, j: usize) -> bool {
        // find root of set i
        let parent_i = self.find_root(i);
        // find root of set j
        let parent_j = self.find_root(j);
        if parent_i == parent_j {
            // already in the same set
            return false;
        }

        // attach the smaller tree to the bigger tree
        if self.rank[parent_i] < self.rank[parent_j] {
            // switch parent of set i and update the rank of set j
            self.parent[parent_i] = parent_j;
            self.rank[parent_j] += self.rank[parent_i];
        } else {
            // same but reverse
            self.parent[parent_j] = parent_i;
            self.rank[parent_i] += self.rank[parent_j];
        }
        true
    }
}

pub fn part_1(_input: &str) -> usize {
    let points = _input
        .lines()
        .map(|l| {
            let xyz: Vec<i64> = l.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            [xyz[0], xyz[1], xyz[2]]
        })
        .collect_vec();

    let edges = calc_edges_sorted(&points);
    let num_points = points.len();

    let mut union_find = UnionFind::new(num_points);

    // Stupid workaround for the difference between testdata & real data...
    // The real problem calls for the 1000 smallest edges
    let search_limit = if num_points == 20 {
        10
    } else {
        edges.len().min(1000)
    };

    edges.iter().take(search_limit).for_each(|e| {
        union_find.union(e.i as usize, e.j as usize);
    });

    let mut ranks = Vec::new();
    let mut visited = HashSet::new();

    for i in 0..num_points {
        let root = union_find.find_root(i);
        if visited.insert(root) {
            ranks.push(union_find.rank[root]);
        }
    }

    // sort descending
    ranks.sort_unstable_by(|a, b| b.cmp(a));

    ranks[0] * ranks[1] * ranks[2]
}

pub fn part_2(_input: &str) -> i32 {
    0
}

fn calc_edges_sorted(points: &[[i64; 3]]) -> Vec<Edge> {
    let num_points = points.len();
    let mut edges = Vec::new();
    // compute the pairwise distance between points (euclidean).
    // No sqrt() as it /should/ not be needed just to sort..
    for i in 0..num_points {
        for j in i + 1..num_points {
            let dx = points[i][0] - points[j][0];
            let dy = points[i][1] - points[j][1];
            let dz = points[i][2] - points[j][2];
            edges.push(Edge {
                dist: dx * dx + dy * dy + dz * dz,
                i,
                j,
            });
        }
    }
    edges.sort_unstable_by(|a, b| a.dist.cmp(&b.dist));
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        assert_eq!(part_1(s), 40);
    }
    #[test]
    fn test_part_2() {
        let s = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        assert_eq!(part_2(s), 0);
    }
}
