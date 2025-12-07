use std::ops::{BitAnd, BitOrAssign, BitXorAssign};
use std::simd::prelude::*;

fn p1(input: &str) -> u32 {
    // lane count of 64 is fastest but breaks for the example.
    const LANE_COUNT: usize = 32;
    const MAX_WIDTH: usize = 144;

    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let stride = 2 * (width + 1);
    assert!(stride >= LANE_COUNT);
    assert!(MAX_WIDTH >= width);

    let splitter = Simd::<u8, LANE_COUNT>::splat(b'^');

    let mut beams = [0u64; MAX_WIDTH.div_ceil(LANE_COUNT)];
    for i in [width / 2 - 3, width / 2 - 1, width / 2 + 1, width / 2 + 3] {
        beams[i / LANE_COUNT] |= 1 << (i % LANE_COUNT);
    }

    let mut split_count = 6;
    for j in 4..input.len() / stride {
        let row_offset = j * stride;
        let chunks = input[row_offset..row_offset + width.div_ceil(LANE_COUNT) * LANE_COUNT]
            .as_chunks::<LANE_COUNT>()
            .0;

        let mut eq_chunks = [0u64; MAX_WIDTH.div_ceil(LANE_COUNT)];

        for (chunk_idx, chunk) in chunks.into_iter().enumerate() {
            let simd_row = Simd::<u8, LANE_COUNT>::from_array(*chunk);
            let eq = simd_row.simd_eq(splitter).to_bitmask();
            let eq = eq.bitand(beams[chunk_idx]);
            split_count += eq.count_ones();
            beams[chunk_idx].bitxor_assign(eq);
            beams[chunk_idx].bitor_assign(eq << 1 | eq >> 1);

            eq_chunks[chunk_idx] = eq;
        }

        for chunk_idx in 0..eq_chunks.len() - 1 {
            beams[chunk_idx + 1].bitor_assign(eq_chunks[chunk_idx] >> (LANE_COUNT - 1));
            beams[chunk_idx].bitor_assign(eq_chunks[chunk_idx + 1] << (LANE_COUNT - 1));
        }
    }

    split_count
}

fn p2(input: &str) -> u64 {
    const LANE_COUNT: usize = 8;

    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let stride = 2 * (width + 1);

    let splitter = Simd::<u8, LANE_COUNT>::splat(b'^');

    let mut cache = vec![Simd::<u64, LANE_COUNT>::splat(1); width.div_ceil(LANE_COUNT)];
    for j in 0..(input.len() / stride - 3) {
        let row_offset = input.len() - stride - j * stride;
        let chunks = input[row_offset..row_offset + width.div_ceil(LANE_COUNT) * LANE_COUNT]
            .as_chunks::<LANE_COUNT>()
            .0;

        let chunk_offset = (j + 1) / LANE_COUNT;

        for (chunk_idx, chunk) in chunks
            .into_iter()
            .enumerate()
            .take(chunks.len() - chunk_offset)
            .skip(chunk_offset)
        {
            let summed = cache[chunk_idx].shift_elements_right::<1>(
                chunk_idx
                    .checked_sub(1)
                    .map(|i| cache[i][LANE_COUNT - 1])
                    .unwrap_or(0),
            ) + cache[chunk_idx].shift_elements_left::<1>(
                cache.get(chunk_idx + 1).map(|chunk| chunk[0]).unwrap_or(0),
            );

            let simd_row = Simd::<u8, LANE_COUNT>::from_array(*chunk);
            let eq = simd_row.simd_eq(splitter).cast();
            cache[chunk_idx] = eq.select(summed, cache[chunk_idx]);
        }
    }

    cache[(width / 2 - 2) / LANE_COUNT][(width / 2 - 2) % LANE_COUNT]
        + 2 * cache[(width / 2) / LANE_COUNT][(width / 2) % LANE_COUNT]
        + cache[(width / 2 + 2) / LANE_COUNT][(width / 2 + 2) % LANE_COUNT]
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
