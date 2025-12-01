type Prepared = Vec<u64>;

fn prepare(input: &str) -> Prepared {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn p1(input: &Prepared) -> u64 {
    input.iter().sum()
}

fn p2(input: &Prepared) -> u64 {
    input.iter().sum()
}

crate::register!(SOLVER, 0, |ctx, input| {
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

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 0);
    }
}
