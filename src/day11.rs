use std::collections::HashMap;

pub fn part_1(input: &str) -> u64 {
    let graph = parse(&input);
    count_paths("you", "out", &graph, &mut HashMap::new())
}

pub fn part_2(input: &str) -> u64 {
    let graph = parse(&input);
    let svr_fft_dac_out = count_paths("svr", "fft", &graph, &mut HashMap::new())
        * count_paths("fft", "dac", &graph, &mut HashMap::new())
        * count_paths("dac", "out", &graph, &mut HashMap::new());

    let svr_dac_fft_out = count_paths("svr", "dac", &graph, &mut HashMap::new())
        * count_paths("dac", "fft", &graph, &mut HashMap::new())
        * count_paths("fft", "out", &graph, &mut HashMap::new());

    svr_dac_fft_out + svr_fft_dac_out
}

fn count_paths<'a>(
    start: &'a str,
    destination: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Recursive DFS with memoization
    if start == destination {
        return 1;
    }
    if let Some(&count) = cache.get(start) {
        return count;
    }

    let mut path_totals = 0;
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            path_totals += count_paths(neighbor, destination, graph, cache);
        }
    }

    cache.insert(start, path_totals);
    path_totals
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    input.lines().for_each(|l| {
        let mut parts = l.split_ascii_whitespace();
        let key = parts.next().unwrap().strip_suffix(':').unwrap();
        graph.insert(key, parts.collect());
    });

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let s = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

        assert_eq!(part_1(s), 5);
    }
    #[test]
    fn test_part_2() {
        let s = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        assert_eq!(part_2(s), 2);
    }
}
