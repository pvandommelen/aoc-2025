use crate::util::grid::Grid;
use std::iter::{once, repeat};

type Prepared = Grid<bool>;

fn prepare(input: &str) -> Prepared {
    let width = input.lines().next().unwrap().as_bytes().len();
    let empty_row = repeat(false).take(width + 2).collect::<Vec<_>>();
    Grid::from_rows(
        once(empty_row.clone())
            .chain(input.lines().map(|line| {
                once(false)
                    .chain(line.as_bytes().iter().map(|c| *c == b'@'))
                    .chain(once(false))
                    .collect::<Vec<_>>()
            }))
            .chain(once(empty_row)),
    )
}

fn p1(input: &Prepared) -> u64 {
    input
        .iter_windows3_where(|b| *b)
        .filter(|w| w.iter().filter(|c| **c == true).count() < 5)
        .count() as u64
}

fn p2(input: &Prepared) -> u64 {
    let mut grid = input.clone();
    let mut removed = 0;
    loop {
        let previous_grid = grid.clone();
        let positions = previous_grid
            .iter_windows3_where(|b| *b)
            .filter(|w| w.iter().filter(|c| **c == true).count() < 5)
            .map(|w| w.position());

        let mut any_removed = false;
        for p in positions {
            removed += 1;
            any_removed = true;
            assert!(grid.remove(&p));
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
