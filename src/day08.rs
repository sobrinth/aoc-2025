use itertools::Itertools;
use std::collections::HashSet;

type JunctionBox = [i64; 3];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Edge {
    weight: i64,
    i: usize,
    j: usize,
}

struct Kruskal {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

// Kruskal's algorithm for MST
impl Kruskal {
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
            self.parent[i] = self.find_root(self.parent[i]);
        }
        self.parent[i]
    }

    /**
     * union the sets containing x and y
     * returns true if sets were merged, false if already in the same set
     **/
    fn union(&mut self, i: usize, j: usize) -> bool {
        let parent_i = self.find_root(i);
        let parent_j = self.find_root(j);
        if parent_i == parent_j {
            // already in the same set
            return false;
        }

        // attach the smaller tree to the bigger tree
        if self.rank[parent_i] < self.rank[parent_j] {
            // switch parent of set i and update the rank of the set j
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

fn map_junction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|l| {
            let xyz: Vec<i64> = l.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            [xyz[0], xyz[1], xyz[2]]
        })
        .collect_vec()
}

fn calc_edges_sorted(boxes: &[JunctionBox]) -> Vec<Edge> {
    let num_boxes = boxes.len();
    let mut edges = Vec::new();
    // compute the pairwise distance between points (euclidean).
    // No sqrt() as it /should/ not be necessary just to sort
    for i in 0..num_boxes {
        for j in i + 1..num_boxes {
            let dx = boxes[i][0] - boxes[j][0];
            let dy = boxes[i][1] - boxes[j][1];
            let dz = boxes[i][2] - boxes[j][2];
            edges.push(Edge {
                weight: dx * dx + dy * dy + dz * dz,
                i,
                j,
            });
        }
    }
    edges.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));
    edges
}

pub fn part_1(input: &str) -> usize {
    let boxes = map_junction_boxes(input);
    let edges = calc_edges_sorted(&boxes);
    let num_boxes = boxes.len();

    let mut kruskal = Kruskal::new(num_boxes);

    // Stupid workaround for the difference between test data and the real data...
    // The real problem calls for the 1000 smallest edges
    let search_limit = if num_boxes == 20 {
        10
    } else {
        edges.len().min(1000)
    };

    edges.iter().take(search_limit).for_each(|e| {
        kruskal.union(e.i, e.j);
    });

    let mut ranks = Vec::new();
    let mut visited = HashSet::new();

    for i in 0..num_boxes {
        let root = kruskal.find_root(i);
        if visited.insert(root) {
            ranks.push(kruskal.rank[root]);
        }
    }

    // sort descending
    ranks.sort_unstable_by(|a, b| b.cmp(a));

    ranks[0] * ranks[1] * ranks[2]
}

pub fn part_2(input: &str) -> i64 {
    let boxes = map_junction_boxes(input);
    let edges = calc_edges_sorted(&boxes);
    let num_boxes = boxes.len();

    let mut kruskal = Kruskal::new(num_boxes);

    let mut remaining_boxes = num_boxes;

    for edge in edges {
        // Try to union the boxes
        if kruskal.union(edge.i, edge.j) {
            // connection successful -> one less junction box to go
            remaining_boxes -= 1;
            if remaining_boxes == 1 {
                // last connection that is still needed. Multiply the x component of the edge points
                return boxes[edge.i][0] * boxes[edge.j][0];
            }
        }
    }
    unreachable!("oh, oh. I messed up")
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

        assert_eq!(part_2(s), 25272);
    }
}
