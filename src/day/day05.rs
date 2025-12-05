use std::cmp::Reverse;
use std::ops::Range;

struct Prepared {
    fresh: Vec<Range<u64>>,
    ingredients: Vec<u64>,
}

fn parse(input: &str) -> Prepared {
    let input = input.as_bytes();

    let mut i = 0;
    let mut fresh = vec![];
    while input[i] != b'\n' {
        let (start, len) = atoi_simd::parse_prefix_pos(&input[i..]).unwrap();
        i += len + 1;
        let (end, len) = atoi_simd::parse_prefix_pos::<u64>(&input[i..]).unwrap();
        i += len + 1;
        fresh.push(start..end + 1);
    }
    i += 1;

    let mut ingredients = vec![];
    while i < input.len() {
        let (number, len) = atoi_simd::parse_prefix_pos(&input[i..]).unwrap();
        i += len + 1;
        ingredients.push(number);
    }

    Prepared { fresh, ingredients }
}

fn optimize(mut fresh: Vec<Range<u64>>) -> Vec<Range<u64>> {
    fresh.sort_unstable_by_key(|range| (range.start, Reverse(range.end)));

    let mut current_start = 0;
    let mut current_end = 0;
    fresh.retain_mut(|r| {
        if r.end <= current_end {
            return false;
        }
        let mut start = r.start;
        if current_end > start {
            start = current_start;
        }
        *r = start..r.end;

        current_end = r.end;
        current_start = start;
        true
    });

    // Deduplicate keeping last.
    fresh.dedup_by(|next, prev| {
        if next.start == prev.start {
            *prev = next.clone();
            true
        } else {
            false
        }
    });
    fresh
}

fn p1(input: &Prepared) -> usize {
    let ends = input.fresh.iter().map(|r| r.end).collect::<Vec<_>>();

    input
        .ingredients
        .iter()
        .filter(|id| {
            let idx = ends.binary_search(*id).unwrap_or_else(|idx| idx);
            input
                .fresh
                .get(idx)
                .map(|r| r.contains(id))
                .unwrap_or(false)
        })
        .count()
}

fn p2(input: &Prepared) -> u64 {
    input.fresh.iter().map(|r| r.end - r.start).sum()
}

crate::register!(SOLVER, 5, |ctx, input| {
    let mut input = ctx.measure("parse", || parse(input));
    let input = ctx.measure("optimize", || {
        input.fresh = optimize(input.fresh);
        input
    });
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

    fn prepare(input: &str) -> Prepared {
        let mut result = parse(input);
        result.fresh = optimize(result.fresh);
        result
    }

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 14);
    }
}
