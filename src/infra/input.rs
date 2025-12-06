use crate::infra::solver::day_to_name;

const YEAR: u64 = 2025;

pub fn read_input(day: u8) -> String {
    let filepath = format!("./input/{}.txt", day_to_name(day));
    std::fs::read_to_string(&filepath).unwrap_or_else(|e| match e.kind() {
        std::io::ErrorKind::NotFound => {
            let mut input = ureq::get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                YEAR, day
            ))
            .header("User-Agent", "https://github.com/pvandommelen/aoc-2025")
            .header(
                "Cookie",
                format!(
                    "session={}",
                    std::env::var("AOC_SESSION_TOKEN")
                        .expect("Missing AOC_SESSION_TOKEN environment variable")
                ),
            )
            .call()
            .unwrap();
            let input = input.body_mut().read_to_string().unwrap();
            let input = input.to_string();
            std::fs::write(&filepath, &input).unwrap();
            input
        }
        _ => panic!("{}", e),
    })
}
