use aoc_2025::infra::input::read_input;
use aoc_2025::infra::measure::MeasureContext;
use aoc_2025::infra::solution::SolutionTuple;
use aoc_2025::infra::solver::match_solvers;
use clap::Parser;
use std::hint::black_box;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day
    day: Option<String>,
    #[arg(short, long, default_value = "1")]
    repeat: u32,
    #[arg(short, long, default_value = "0")]
    warmup: u32,
}

fn main() {
    let args = Args::parse();
    assert!(args.repeat > 0);

    let solvers = match_solvers(args.day.as_ref());

    let inputs: Vec<_> = solvers.iter().map(|s| read_input(s.day)).collect();

    let mut total_duration = Duration::default();
    for (solver, input) in solvers.into_iter().zip(inputs) {
        let name = solver.name();
        {
            let mut ctx = MeasureContext::new();
            for _ in 0..args.warmup {
                black_box((solver.solve)(&mut ctx, black_box(&input)));
            }
        }

        let mut ctx = MeasureContext::with_capacity(3 * args.repeat as usize);
        let solution = (solver.solve)(&mut ctx, black_box(&input));
        for _ in 0..args.repeat - 1 {
            assert_eq!(&(solver.solve)(&mut ctx, black_box(&input)), &solution);
        }

        let SolutionTuple(p1, p2) = solution;

        println!("{}/part1: {}", name, p1);
        println!("{}/part2: {}", name, p2);

        let per_iter_duration = ctx.duration() / args.repeat;
        println!(
            "{}/time: {:?}{}",
            name,
            per_iter_duration,
            if ctx.measurements().into_iter().next().is_none() {
                "".to_string()
            } else {
                format!(
                    " ({})",
                    ctx.measurements()
                        .into_iter()
                        .map(|(label, duration)| {
                            format!("{}: {:?}", label, duration / args.repeat)
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        );
        total_duration += per_iter_duration;
    }

    if args.day.is_none() {
        println!("Total time: {:?}", total_duration);
    }
}
