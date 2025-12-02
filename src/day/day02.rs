use std::ops::RangeInclusive;
use winnow::Parser;
use winnow::ascii::dec_uint;
use winnow::combinator::separated_pair;

type Prepared = Vec<RangeInclusive<u64>>;

fn prepare(input: &str) -> Prepared {
    input
        .split(',')
        .map(|l| {
            separated_pair(dec_uint::<_, _, ()>, '-', dec_uint)
                .map(|(a, b)| a..=b)
                .parse(l)
                .unwrap()
        })
        .collect()
}

fn p1(input: &Prepared) -> u64 {
    fn valid(num: u64) -> bool {
        let s = num.to_string();
        if s.len() % 2 == 1 {
            // odd length, can't be two numbers repeated twice.
            return true;
        }

        if s.as_bytes()[..s.len() / 2] == s.as_bytes()[s.len() / 2..] {
            return false;
        }

        true
    }

    input
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|num| !valid(*num))
        .sum()
}

fn p2(input: &Prepared) -> u64 {
    fn valid(num: u64) -> bool {
        let s = num.to_string();
        let b = s.as_bytes();

        'next_slice_length: for slice_length in 1..=b.len() / 2 {
            if b.len() % slice_length != 0 {
                // odd length, can't be numbers repeated with length slice_length.
                continue;
            }

            let slice = &b[..slice_length];
            for i in (slice_length..b.len()).step_by(slice_length) {
                if slice != &b[i..i + slice_length] {
                    continue 'next_slice_length;
                }
            }
            return false;
        }

        true
    }

    input
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|num| !valid(*num))
        .sum()
}

crate::register!(SOLVER, 2, |ctx, input| {
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

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 1227775554);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 4174379265);
    }
}
