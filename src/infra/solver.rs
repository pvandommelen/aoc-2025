use crate::infra::measure::MeasureContext;
use crate::infra::solution::SolutionTuple;
use linkme::distributed_slice;

pub struct Solver {
    pub day: u8,
    pub solve: fn(&mut MeasureContext, &str) -> SolutionTuple,
}

pub fn day_to_name(day: u8) -> String {
    format!("day{:02}", day)
}

impl Solver {
    pub const fn new(day: u8, solve: fn(&mut MeasureContext, &str) -> SolutionTuple) -> Self {
        Solver { day, solve }
    }

    pub fn name(&self) -> String {
        day_to_name(self.day)
    }
}

#[distributed_slice]
pub static SOLVERS: [Solver];

#[macro_export]
macro_rules! register {
    ($id:ident, $day:literal, $solve:expr) => {
        #[::linkme::distributed_slice($crate::infra::solver::SOLVERS)]
        static $id: $crate::infra::solver::Solver =
            $crate::infra::solver::Solver::new($day, $solve);
    };
}

pub fn match_solvers<S: AsRef<str>>(s: Option<S>) -> Vec<&'static Solver> {
    let mut solvers: Vec<&Solver> = match s {
        Some(d) => SOLVERS
            .into_iter()
            .filter(|s| s.name().contains(d.as_ref()))
            .collect(),
        None => SOLVERS.into_iter().collect(),
    };
    solvers.sort_unstable_by_key(|s| s.day);
    solvers
}
