use indexmap::IndexMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct MeasureContext {
    start: Instant,
    measurements: Vec<(&'static str, Instant)>,
}

impl MeasureContext {
    pub fn new() -> Self {
        MeasureContext {
            start: Instant::now(),
            measurements: vec![],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = Vec::with_capacity(capacity);
        MeasureContext {
            start: Instant::now(),
            measurements: capacity,
        }
    }

    #[inline]
    pub fn measure<T>(&mut self, label: &'static str, f: impl FnOnce() -> T) -> T {
        let result = f();
        self.measurements.push((label, Instant::now()));
        result
    }

    pub fn duration(&self) -> Duration {
        self.measurements
            .last()
            .map(|(_, d)| d)
            .unwrap_or(&self.start)
            .duration_since(self.start)
    }

    pub fn measurements(&self) -> impl IntoIterator<Item = (&'static str, Duration)> {
        let mut map = IndexMap::new();
        let mut previous = self.start;
        for (label, instant) in &self.measurements {
            let duration = instant.duration_since(previous);
            *map.entry(*label).or_insert(Duration::ZERO) += duration;
            previous = *instant;
        }
        map
    }
}
impl Default for MeasureContext {
    fn default() -> Self {
        Self::new()
    }
}
