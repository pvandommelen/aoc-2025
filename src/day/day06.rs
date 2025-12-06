use std::iter::once;

type Prepared<'data, const SIZE: usize> = ([&'data [u8]; SIZE], &'data [u8]);

fn newline_positions<const CHUNK_SIZE: usize>(input: &[u8]) -> impl Iterator<Item = usize> {
    fn create_remaining_chunk_iterator(chunk: &[u8]) -> impl Iterator<Item = usize> {
        chunk
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == b'\n')
            .map(|(i, _)| i)
    }

    let chunks = input.chunks_exact(CHUNK_SIZE);
    let chunks_len = chunks.len();
    let remainder = chunks.remainder();
    chunks
        .into_iter()
        .enumerate()
        .filter(|(_, chunk)| chunk.contains(&b'\n'))
        .flat_map(|(chunk_idx, chunk)| {
            // SAFETY: we just filtered for only chunks that contain at least one match
            let pos = unsafe { chunk.iter().position(|c| *c == b'\n').unwrap_unchecked() };
            let offset = chunk_idx * CHUNK_SIZE + pos;
            once(chunk_idx * CHUNK_SIZE + pos).chain(
                create_remaining_chunk_iterator(&chunk[pos + 1..]).map(move |pos| offset + pos),
            )
        })
        .chain(
            create_remaining_chunk_iterator(remainder)
                .map(move |pos| chunks_len * CHUNK_SIZE + pos),
        )
}

fn prepare<const SIZE: usize>(input: &str) -> Prepared<'_, SIZE> {
    let input = input.as_bytes();

    let mut positions = newline_positions::<128>(&input);
    let mut previous_offset = 0;
    let lines: [&[u8]; SIZE] = std::array::from_fn(|_| {
        let pos = positions.next().unwrap();
        let line = &input[previous_offset..pos];
        previous_offset = pos + 1;
        line
    });

    (lines, &input[previous_offset..input.len()])
}

fn number(input: &[u8], mut i: usize) -> u64 {
    let mut amount = 0;
    while i < input.len() {
        let c = input[i];
        i += 1;
        match c {
            b'0'..=b'9' => amount = amount * 10 + (c - b'0') as u64,
            _ if amount != 0 => break,
            _ => {}
        }
    }
    amount
}

fn p1<const SIZE: usize>((lines, last_line): &Prepared<'_, SIZE>) -> u64 {
    let mut total_sum = 0;
    let mut col = 0;
    while col < last_line.len() {
        // First column also contains the operator.
        let local_operator = last_line[col];

        let col_end = last_line[col + 1..]
            .iter()
            .position(|c| *c == b'+' || *c == b'*')
            .map(|idx| col + idx)
            .unwrap_or(last_line.len());

        let col_numbers = lines.iter().map(|line| {
            let col_num = number(line, col);
            col_num
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

fn p2<const SIZE: usize>((lines, last_line): &Prepared<'_, SIZE>) -> u64 {
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();

    let mut total_sum = 0;
    let mut col = 0;
    while col < last_line.len() {
        // First column also contains the operator.
        let local_operator = last_line[col];

        let col_end = last_line[col + 1..]
            .iter()
            .position(|c| *c == b'+' || *c == b'*')
            .map(|idx| col + idx)
            .unwrap_or(max_len);

        let col_numbers = (col..col_end).map(|col| {
            lines
                .iter()
                .filter_map(|line| match line[col] {
                    num if matches!(num, b'0'..=b'9') => Some((num - b'0') as u64),
                    _ => None,
                })
                .reduce(|acc, num| acc * 10 + num)
                .unwrap()
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
    let input = ctx.measure("prepare", || prepare::<4>(&input));
    (
        ctx.measure("part1", || p1::<4>(&input)),
        ctx.measure("part2", || p2::<4>(&input)),
    )
        .into()
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64\u{0020}
 45 64  387 23\u{0020}
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn example_part1() {
        assert_eq!(p1::<3>(&prepare::<3>(EXAMPLE_INPUT)), 4277556);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2::<3>(&prepare::<3>(EXAMPLE_INPUT)), 3263827);
    }
}
