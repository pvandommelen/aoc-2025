fn p1(input: &str) -> usize {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let stride = 2 * (width + 1);
    let height = input.len() / stride;

    let mut beams = vec![false; width];
    beams[width / 2 - 3] = true;
    beams[width / 2 - 1] = true;
    beams[width / 2 + 1] = true;
    beams[width / 2 + 3] = true;
    let mut split_count = 6;
    for j in 4..input.len() / stride {
        let row_offset = j * stride;
        let row = &input[row_offset..row_offset + width];
        for i in height - j..row.len() + j - height {
            if row[i] == b'^' && beams[i] {
                beams[i] = false;
                split_count += 1;
                beams[i - 1] = true;
                beams[i + 1] = true;
            }
        }
    }

    split_count
}

fn p2(input: &str) -> usize {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let stride = 2 * (width + 1);

    let mut cache = vec![1; width];
    for j in 0..(input.len() / stride - 3) {
        let row_offset = input.len() - stride - j * stride;
        let row = &input[row_offset..row_offset + width];
        for i in (j + 1..row.len() - j - 1).step_by(2) {
            if row[i] == b'^' {
                cache[i] = cache[i - 1] + cache[i + 1];
            }
        }
    }

    cache[width / 2 - 2] + 2 * cache[width / 2] + cache[width / 2 + 2]
}

crate::register!(SOLVER, 7, |ctx, input| {
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
...............
";

    #[test]
    fn example_part1() {
        assert_eq!(p1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn example_part2() {
        assert_eq!(p2(EXAMPLE_INPUT), 40);
    }
}
