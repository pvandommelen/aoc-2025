type Prepared = Vec<Vec<u8>>;

fn prepare(input: &str) -> Prepared {
    input
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| *c - b'0').collect())
        .collect()
}

fn solve<const N: usize>(input: &Prepared) -> u64 {
    input
        .iter()
        .map(|bank| {
            let mut offset = 0;
            let mut result = 0;
            for i in 0..N {
                let limit = bank.len() + 1 + i - N;

                let max = bank[offset..limit].iter().copied().max().unwrap();
                let max_pos = bank[offset..limit]
                    .iter()
                    .copied()
                    .position(|c| c == max)
                    .unwrap();

                offset += max_pos + 1;
                result = result * 10 + max as u64;
            }

            result
        })
        .sum()
}

crate::register!(SOLVER, 3, |ctx, input| {
    let input = ctx.measure("prepare", || prepare(input));
    (
        ctx.measure("part1", || solve::<2>(&input)),
        ctx.measure("part2", || solve::<12>(&input)),
    )
        .into()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn example_part1() {
        assert_eq!(solve::<2>(&prepare(EXAMPLE_INPUT)), 357);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve::<12>(&prepare(EXAMPLE_INPUT)), 3121910778619);
    }
}
