fn number(input: &[u8], i: &mut usize) -> u16 {
    let mut amount = 0;
    while *i < input.len() {
        let c = input[*i];
        *i += 1;
        match c {
            b'0'..=b'9' => amount = amount * 10 + (c - b'0') as u16,
            _ => break,
        }
    }
    amount
}

fn prepare(input: &str) -> impl IntoIterator<Item = i16> {
    let input = input.as_bytes();
    let mut i = 0;

    std::iter::from_fn(move || {
        if i >= input.len() {
            return None;
        }
        let sign = (input[i] & 2) as i16 - 1;
        i += 1;

        let amount = number(input, &mut i);
        Some(sign * amount as i16)
    })
}

fn both(input: impl IntoIterator<Item = i16>) -> (u32, u32) {
    let mut position: i16 = 50;
    let mut times_pointed_at_zero_exactly = 0;
    let mut times_passed_zero = 0;

    for amount in input {
        times_passed_zero += amount.abs() / 100;
        let amount = amount % 100;
        if amount < 0 {
            if position + amount < 0 {
                if position != 0 {
                    times_passed_zero += 1;
                }
                position += 100;
            }
            position += amount;
        } else {
            position += amount;
            if position >= 100 {
                if position != 100 {
                    times_passed_zero += 1;
                }
                position -= 100;
            }
        }

        if position == 0 {
            times_pointed_at_zero_exactly += 1;
        }
    }
    (
        times_pointed_at_zero_exactly,
        times_passed_zero as u32 + times_pointed_at_zero_exactly,
    )
}

crate::register!(SOLVER, 1, |ctx, input| {
    ctx.measure("all", || both(prepare(input))).into()
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

    fn p1(input: impl IntoIterator<Item = i16>) -> u32 {
        both(input).0
    }

    fn p2(input: impl IntoIterator<Item = i16>) -> u32 {
        both(input).1
    }

    #[test]
    fn example_part1() {
        assert_eq!(p1(prepare(EXAMPLE_INPUT)), 3);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(prepare(EXAMPLE_INPUT)), 6);
    }
}
