use winnow::Parser;
use winnow::ascii::dec_uint;
use winnow::combinator::alt;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    amount: u32,
}

type Prepared = Vec<Instruction>;

fn prepare(input: &str) -> Prepared {
    input
        .lines()
        .map(|l| {
            (
                alt(("L".value(Direction::Left), "R".value(Direction::Right))),
                dec_uint::<_, _, ()>,
            )
                .map(|(direction, amount)| Instruction { direction, amount })
                .parse(l)
                .unwrap()
        })
        .collect()
}

fn p1(input: &Prepared) -> u32 {
    let mut position: u32 = 50;
    let mut times_pointed_at_zero = 0;
    for instruction in input {
        let amount = instruction.amount % 100;
        match instruction.direction {
            Direction::Left => {
                if position < amount {
                    position += 100;
                }
                position -= amount;
            }
            Direction::Right => {
                position += amount;
                if position >= 100 {
                    position -= 100;
                }
            }
        }
        if position == 0 {
            times_pointed_at_zero += 1;
        }
    }
    times_pointed_at_zero
}

fn p2(input: &Prepared) -> u32 {
    let mut position: u32 = 50;
    let mut times_pointed_at_zero = 0;
    for instruction in input {
        times_pointed_at_zero += instruction.amount / 100;
        let amount = instruction.amount % 100;
        match instruction.direction {
            Direction::Left => {
                if position < amount {
                    if position != 0 {
                        times_pointed_at_zero += 1;
                    }
                    position += 100;
                }
                position -= amount;
            }
            Direction::Right => {
                position += amount;
                if position >= 100 {
                    position -= 100;
                    if position != 0 {
                        times_pointed_at_zero += 1;
                    }
                }
            }
        }
        if position == 0 {
            times_pointed_at_zero += 1;
        }
    }
    times_pointed_at_zero
}

crate::register!(SOLVER, 1, |ctx, input| {
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

    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 6);
    }
}
