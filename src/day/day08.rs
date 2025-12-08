use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Prepared = (Vec<[u32; 3]>, BinaryHeap<(Reverse<u64>, usize, usize)>);

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
    let connections = junctions
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            junctions
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(j, b)| (Reverse(dist_squared(*a, *b)), i, j))
        })
        .collect::<BinaryHeap<_>>();

    (junctions, connections)
}

fn both((junctions, connections): Prepared, connection_count: usize) -> (usize, u64) {
    let mut junction_to_circuit = Vec::from_iter(0..junctions.len());
    let mut circuit_to_junctions = Vec::from_iter((0..junctions.len()).map(|i| vec![i]));
    let mut number_of_circuits = junctions.len();

    let mut connections = connections.clone();
    let mut p1 = None;
    let mut i = 0;
    while let Some((_, a, b)) = connections.pop() {
        let x = junction_to_circuit[a];
        let y = junction_to_circuit[b];
        if x != y {
            // Different networks, combine.
            number_of_circuits -= 1;
            if number_of_circuits == 1 {
                return (p1.unwrap(), junctions[a][0] as u64 * junctions[b][0] as u64);
            }
            for other in &circuit_to_junctions[y] {
                junction_to_circuit[*other] = x;
            }
            let [x, y] = circuit_to_junctions.get_disjoint_mut([x, y]).unwrap();
            x.extend(y.drain(..));
        }
        i += 1;
        if i == connection_count {
            let mut counts =
                Vec::from_iter(circuit_to_junctions.iter().map(|junctions| junctions.len()));
            counts.sort_unstable_by(|a, b| b.cmp(a));
            p1 = Some(counts[0] * counts[1] * counts[2]);
        }
    }
    panic!("No solution found");
}

crate::register!(SOLVER, 8, |ctx, input| {
    let parsed = ctx.measure("parse", || parse(input));
    let prepared = ctx.measure("prepare", || prepare(parsed));
    ctx.measure("both", || both(prepared, 1000)).into()
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

    fn p1(prepared: Prepared) -> usize {
        both(prepared, 10).0
    }
    fn p2(prepared: Prepared) -> u64 {
        both(prepared, 10).1
    }

    #[test]
    fn example_part1() {
        assert_eq!(p1(prepare(parse(EXAMPLE_INPUT))), 40);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(prepare(parse(EXAMPLE_INPUT))), 25272);
    }
}
