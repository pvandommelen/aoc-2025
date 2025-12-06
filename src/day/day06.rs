type Prepared<'data> = Vec<&'data [u8]>;

fn prepare(input: &str) -> Prepared<'_> {
    input.as_bytes().split(|c| *c == b'\n').collect()
}

fn p1(input: &Prepared) -> u64 {
    let lines = &input[0..input.len() - 1];
    let last_line = input[input.len() - 1];

    let mut total_sum = 0;
    let mut col = 0;
    loop {
        // First column also contains the operator.
        if col >= last_line.len() {
            break;
        }
        let local_operator = last_line[col];

        let mut max_idx = 0;
        let col_numbers = lines.iter().map(|line| {
            let mut idx = col;
            while line[idx] == b' ' {
                idx += 1;
            }
            let (col_num, col_num_len) = atoi_simd::parse_prefix_pos::<u64>(&line[idx..]).unwrap();
            max_idx = max_idx.max(idx + col_num_len + 1);
            col_num
        });
        let local_result = match local_operator {
            b'+' => col_numbers.sum::<u64>(),
            b'*' => col_numbers.reduce(|acc, col_num| acc * col_num).unwrap(),
            _ => panic!("found char for operator `{}`", char::from(local_operator)),
        };
        total_sum += local_result;

        assert_ne!(col, max_idx);
        col = max_idx;
    }

    total_sum
}

fn p2(input: &Prepared) -> u64 {
    let lines = &input[0..input.len() - 1];
    let last_line = input[input.len() - 1];

    let mut total_sum = 0;
    let mut col = 0;
    loop {
        // First column also contains the operator.
        if col >= last_line.len() {
            break;
        }
        let local_operator = last_line[col];

        let col_end = last_line[col + 1..]
            .iter()
            .position(|c| *c != b' ')
            .map(|idx| col + idx)
            .unwrap_or(last_line.len());

        let col_numbers = (col..).map_while(|col| {
            lines
                .iter()
                .filter_map(|line| match line.get(col) {
                    Some(&num) if matches!(num, b'0'..=b'9') => Some((num - b'0') as u64),
                    _ => None,
                })
                .reduce(|acc, num| acc * 10 + num)
        });

        let local_result = match local_operator {
            b'+' => col_numbers.sum::<u64>(),
            b'*' => col_numbers.reduce(|acc, col_num| acc * col_num).unwrap(),
            _ => panic!("found char for operator `{}`", char::from(local_operator)),
        };
        total_sum += local_result;

        col = col_end + 1;
    }

    total_sum
}

crate::register!(SOLVER, 6, |ctx, input| {
    let input = ctx.measure("prepare", || prepare(&input));
    (
        ctx.measure("part1", || p1(&input)),
        ctx.measure("part2", || p2(&input)),
    )
        .into()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn example_part1() {
        assert_eq!(p1(&prepare(EXAMPLE_INPUT)), 4277556);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(&prepare(EXAMPLE_INPUT)), 3263827);
    }
}
