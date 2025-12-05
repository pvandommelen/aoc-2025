use std::ops::RangeInclusive;

struct Prepared {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn prepare(input: &str) -> Prepared {
    let (fresh, ingredients) = input.split_once("\n\n").unwrap();
    let fresh = fresh
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            start..=end
        })
        .collect();
    let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();
    Prepared { fresh, ingredients }
}

fn p1(input: &Prepared) -> usize {
    input
        .ingredients
        .iter()
        .filter(|id| input.fresh.iter().any(|range| range.contains(id)))
        .count()
}

fn p2(input: &Prepared) -> u64 {
    let mut fresh = input.fresh.clone();
    fresh.sort_unstable_by_key(|range| *range.start());

    let mut fresh_count = 0;
    let mut current_end = 0;
    for r in fresh {
        if *r.end() < current_end {
            continue;
        }
        fresh_count += *r.end() + 1 - current_end.max(*r.start());
        current_end = *r.end() + 1;
    }

    fresh_count
}

crate::register!(SOLVER, 5, |ctx, input| {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || p1(&input)),
        ctx.measure("part2", || p2(&input)),
    )
        .into()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 14);
    }
}
