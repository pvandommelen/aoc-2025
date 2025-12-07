use crate::util::grid::Grid;
use crate::util::position::{Direction, Position};
use crate::util::solver::solve_depth_first;
use rustc_hash::FxHashSet;

type Prepared = (Grid<bool>, usize);

fn prepare(input: &str) -> Prepared {
    let grid = Grid::from_rows(
        input
            .lines()
            .step_by(2)
            .map(|l| l.as_bytes().iter().map(|c| *c == b'^')),
    );
    let starting_pos = input.as_bytes().iter().position(|c| *c == b'S').unwrap();
    (grid, starting_pos)
}

fn p1((grid, starting_pos): &Prepared) -> usize {
    let pos = Position(0, *starting_pos);

    let mut split = FxHashSet::default();

    solve_depth_first(
        |stack, p| {
            let Some(next_splitter) = p
                .positions(&grid.dimensions, &Direction::Down)
                .filter(|p| *grid.get(&p))
                .next()
            else {
                return;
            };

            let inserted = split.insert(next_splitter);
            if !inserted {
                return;
            }

            if let Some(left) = next_splitter.checked_moved(&grid.dimensions, &Direction::Left) {
                stack.push(left);
            }
            if let Some(right) = next_splitter.checked_moved(&grid.dimensions, &Direction::Right) {
                stack.push(right);
            }
        },
        vec![pos],
    );

    split.len()
}

fn p2((grid, _starting_pos): &Prepared) -> usize {
    let mut cache = vec![1; grid.dimensions.width()];
    for row in grid.rows().rev() {
        for (i, c) in row
            .iter()
            .enumerate()
            .take(grid.dimensions.width() - 1)
            .skip(1)
        {
            if *c {
                let left = cache[i - 1];
                let right = cache[i + 1];
                cache[i] = left + right;
            }
        }
    }

    cache.into_iter().max().unwrap()
}

crate::register!(SOLVER, 7, |ctx, input| {
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

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 40);
    }
}
