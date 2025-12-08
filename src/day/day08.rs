use crate::util::solver::solve_depth_first;
use rustc_hash::FxHashMap;

type Prepared = (Vec<[u32; 3]>, Vec<(usize, usize)>);

fn dist_squared(a: [u32; 3], b: [u32; 3]) -> u64 {
    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| {
            let diff = a.abs_diff(b) as u64;
            diff * diff
        })
        .sum()
}

fn parse(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn prepare(junctions: Vec<[u32; 3]>) -> Prepared {
    let mut distances = junctions
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junctions
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (i, j, dist_squared(*a, *b)))
        })
        .collect::<Vec<_>>();

    distances.sort_unstable_by_key(|(_, _, dist)| *dist);

    let connections = distances.into_iter().map(|(i, j, _)| (i, j)).collect();

    (junctions, connections)
}

fn p1((junctions, connections): &Prepared, connection_count: usize) -> usize {
    let connections = connections.iter().copied().take(connection_count).fold(
        vec![vec![]; junctions.len()],
        |mut acc, (a, b)| {
            acc[a].push(b);
            acc[b].push(a);
            acc
        },
    );

    let mut reached: Vec<Option<usize>> = vec![None; junctions.len()];

    for i in 0..junctions.len() {
        if reached[i].is_some() {
            continue;
        }

        solve_depth_first(
            |stack, s| {
                reached[s] = Some(i);
                for b in &connections[s] {
                    if reached[*b].is_some() {
                        continue;
                    }
                    stack.push(*b);
                }
            },
            vec![i],
        );
    }

    let mut counts = FxHashMap::default();
    for num in reached.into_iter().map(|reached| reached.unwrap()) {
        *counts.entry(num).or_default() += 1;
    }

    let mut counts = counts.into_values().collect::<Vec<usize>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    counts[0] * counts[1] * counts[2]
}

fn p2((junctions, connections): &Prepared) -> u64 {
    let mut junction_to_circuit = Vec::from_iter(0..junctions.len());
    let mut circuit_to_junctions = Vec::from_iter((0..junctions.len()).map(|i| vec![i]));
    let mut number_of_circuits = junctions.len();
    for (a, b) in connections.iter().copied() {
        let x = junction_to_circuit[a];
        let y = junction_to_circuit[b];
        if x != y {
            // Different networks, combine.
            number_of_circuits -= 1;
            if number_of_circuits == 1 {
                return junctions[a][0] as u64 * junctions[b][0] as u64;
            }
            for other in &circuit_to_junctions[y] {
                junction_to_circuit[*other] = x;
            }
            let [x, y] = circuit_to_junctions.get_disjoint_mut([x, y]).unwrap();
            x.extend(y.drain(..));
        }
    }
    panic!("No solution found");
}

crate::register!(SOLVER, 8, |ctx, input| {
    let parsed = ctx.measure("parse", || parse(input));
    let prepared = ctx.measure("prepare", || prepare(parsed));
    (
        ctx.measure("part1", || p1(&prepared, 1000)),
        ctx.measure("part2", || p2(&prepared)),
    )
        .into()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
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
425,690,689";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(parse(EXAMPLE_INPUT)), 10), 40);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(parse(EXAMPLE_INPUT))), 25272);
    }
}
