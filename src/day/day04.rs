use crate::util::grid::Grid;
use std::iter::{once, repeat};

type Prepared = Grid<u8>;

fn prepare(input: &str) -> Prepared {
    let width = input.lines().next().unwrap().as_bytes().len();
    let empty_row = repeat(0).take(width + 2).collect::<Vec<_>>();
    Grid::from_rows(
        once(empty_row.clone())
            .chain(input.lines().map(|line| {
                once(0)
                    .chain(
                        line.as_bytes()
                            .iter()
                            .map(|c| if *c == b'@' { 1 } else { 0 }),
                    )
                    .chain(once(0))
                    .collect::<Vec<_>>()
            }))
            .chain(once(empty_row)),
    )
}

fn p1(input: &Prepared) -> u64 {
    input
        .iter_windows3_where(|b| *b == 1)
        .filter(|w| w.iter().sum::<u8>() < 5)
        .count() as u64
}

fn p2(input: &Prepared) -> u64 {
    let grid = input
        .rows()
        .map(|row| row.iter().copied().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut counts = grid.clone();
    for j in 1..grid.len() - 1 {
        for i in 1..grid[j].len() - 1 {
            let a = grid[j - 1][i - 1..i + 2].iter().copied().sum::<u8>();
            let b = grid[j][i - 1..i + 2].iter().copied().sum::<u8>();
            let c = grid[j + 1][i - 1..i + 2].iter().copied().sum::<u8>();
            let sum: u8 = a + b + c;
            counts[j][i] += sum * 2;
        }
    }

    let mut removed = 0;
    loop {
        let mut any_removed = false;
        for j in 1..counts.len() - 1 {
            for i in 1..counts[j].len() - 1 {
                if counts[j][i] & 1 == 1 && counts[j][i] < 10 {
                    removed += 1;
                    any_removed = true;
                    counts[j][i] ^= 1;
                    for num in counts[j - 1][i - 1..i + 2].iter_mut() {
                        *num = num.saturating_sub(2);
                    }
                    for num in counts[j][i - 1..i + 2].iter_mut() {
                        *num = num.saturating_sub(2);
                    }
                    for num in counts[j + 1][i - 1..i + 2].iter_mut() {
                        *num = num.saturating_sub(2);
                    }
                }
            }
        }
        if !any_removed {
            break;
        }
    }
    removed
}

crate::register!(SOLVER, 4, |ctx, input| {
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

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 43);
    }
}
