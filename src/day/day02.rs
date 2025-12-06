use std::ops::RangeInclusive;
use winnow::Parser;
use winnow::ascii::dec_uint;
use winnow::combinator::separated_pair;

type Prepared = Vec<RangeInclusive<u64>>;

fn prepare(input: &str) -> Prepared {
    input
        .trim()
        .split(',')
        .map(|l| {
            separated_pair(dec_uint::<_, u64, ()>, '-', dec_uint)
                .map(|(a, b)| a..=b)
                .parse(l)
                .unwrap()
        })
        // Make sure each range entry has the same number of digits as start and end.
        .flat_map(|range| {
            let mut next_start = *range.start();
            let mut ranges = vec![];
            for log in range.start().ilog10()..range.end().ilog10() {
                let boundary = 10u64.pow(log + 1);
                ranges.push(next_start..=(boundary - 1));
                next_start = boundary;
            }
            ranges.push(next_start..=*range.end());
            ranges
        })
        // Assert previous step.
        .inspect(|range| {
            let a = range.start().ilog10();
            let b = range.end().ilog10();
            assert_eq!(a, b);
        })
        .collect()
}

fn p1(input: &Prepared) -> u64 {
    /// Check if the range is entirely valid by checking if the length is odd.
    fn range_valid(range: &RangeInclusive<u64>) -> bool {
        range.start().ilog10() % 2 == 0
    }

    fn valid(num: u64, factor: u64) -> bool {
        if num / factor == num % factor {
            return false;
        }

        true
    }

    input
        .iter()
        .filter(|range| !range_valid(range))
        .flat_map(|range| {
            let log = range.start().ilog10();
            let factor = 10u64.pow((log + 1) / 2);
            range
                .clone()
                .into_iter()
                .filter(move |num| !valid(*num, factor))
        })
        .sum()
}

fn p2(input: &Prepared) -> u64 {
    fn valid(num: u64) -> bool {
        let len = num.ilog10() as usize + 1;

        'next_slice_length: for slice_length in 1..=len / 2 {
            if len % slice_length != 0 {
                // odd length, can't be numbers repeated with length slice_length.
                continue;
            }

            let factor = 10u64.pow(slice_length as u32);

            let slice = num % factor;
            let mut remainder = num / factor;
            while remainder != 0 {
                if slice != remainder % factor {
                    continue 'next_slice_length;
                }
                remainder /= factor;
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
