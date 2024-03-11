use std::collections::HashMap;

use crate::statevector::StateVector;

/// Map classical registers with values and number of outcomes.
/// register name -> (Vector of (value, count), register size)
pub type Histogram = HashMap<String, (Vec<(u64, usize)>, usize)>;

/// Represent the result of a simulation.
///
/// API functions such as [`simulate()`] or [`simulate_with_shots()`] return
/// `Computation` instances.
///
/// # Examples:
///
/// See [`simulate()`] or [`simulate_with_shots()`] for an example of generating
/// a `Computation` instance.
///
/// [`simulate()`]: ./fn.simulate.html
/// [`simulate_with_shots()`]: ./fn.simulate_with_shots.html
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Computation {
    statevector: StateVector,
    memory: HashMap<String, (u64, usize, usize)>,
    probabilities: Vec<f64>,
    histogram: Option<Histogram>,
    stats: Option<HashMap<String, usize>>,
}

impl Computation {
    /// Create a new computation.
    ///
    /// Probabilities are computed from the state-vector.
    pub fn new(
        memory: HashMap<String, (u64, usize, usize)>,
        statevector: StateVector,
        histogram: Option<Histogram>,
        stats: Option<HashMap<String, usize>>,
    ) -> Self {
        Computation {
            probabilities: statevector.probabilities(),
            statevector,
            memory,
            histogram,
            stats,
        }
    }

    /// Return the statevector of the quantum system.
    pub fn statevector(&self) -> &StateVector {
        &self.statevector
    }

    /// Return an associative map with classical names and the classical outcomes.
    pub fn memory(&self) -> &HashMap<String, (u64, usize, usize)> {
        &self.memory
    }

    /// Return the probabilities associated with the state-vector.
    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }

    /// Return the histogram when simulating with several shots.
    pub fn histogram(&self) -> &Option<Histogram> {
        &self.histogram
    }

    /// Return the statistics when simulating with several shots.
    pub fn stats(&self) -> &Option<HashMap<String, usize>> {
        &self.stats
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HistogramBuilder {
    pub histogram: Histogram,
    pub stats: HashMap<String, usize>,
}

impl HistogramBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, memory: &HashMap<String, (u64, usize, usize)>) {
        for (key, current_value) in memory {
            if !self.histogram.contains_key(key) {
                self.histogram
                    .insert(key.clone(), (Vec::new(), current_value.1));
            }
            let values = &mut self.histogram.get_mut(key).expect("get values for key").0;
            match values.binary_search_by_key(&current_value.0, |(v, _)| *v) {
                Err(idx) => values.insert(idx, (current_value.0, 1)),
                Ok(found) => values[found].1 += 1,
            }
        }

        let mut memory_vec = memory.into_iter().collect::<Vec<_>>();
        memory_vec.sort_by(|x, y| y.1 .2.cmp(&x.1 .2));
        let mut binary = String::new();
        for (_, current_value) in memory_vec {
            binary.push_str(&format!(
                "{:0width$b}",
                current_value.0,
                width = current_value.1
            ));
        }
        *self.stats.entry(binary).or_insert(0) += 1;
    }

    pub fn histogram(self) -> Histogram {
        self.histogram
    }

    pub fn stats(self) -> HashMap<String, usize> {
        self.stats
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_histogram_builder_empty_histogram() {
        let builder = HistogramBuilder::new();
        let histogram = builder.histogram();
        assert_eq!(histogram, HashMap::new());
    }

    #[test]
    fn test_histogram_builder_one_update() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (1, 1, 1))]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![("a".into(), (vec![(1, 1)], 1))])
        );
    }

    #[test]
    fn test_histogram_builder_couple_of_updates() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (1, 1, 1))]));
        builder.update(&HashMap::from_iter(vec![("a".into(), (1, 1, 1))]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![("a".into(), (vec![(1, 2)], 1))])
        );
    }

    #[test]
    fn test_histogram_builder_couple_of_registers() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (1, 1, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (1, 1, 2))]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![
                ("a".into(), (vec![(1, 1)], 1)),
                ("b".into(), (vec![(1, 1)], 1))
            ])
        );
    }

    #[test]
    fn test_histogram_builder_different_values() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (5, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (4, 3, 2))]));
        builder.update(&HashMap::from_iter(vec![("a".into(), (3, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (2, 3, 2))]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![
                ("a".into(), (vec![(3, 1), (5, 1)], 3)),
                ("b".into(), (vec![(2, 1), (4, 1)], 3))
            ])
        );
    }

    #[test]
    fn test_histogram_builder_different_repeated_values() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (5, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (4, 3, 2))]));
        builder.update(&HashMap::from_iter(vec![("a".into(), (5, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (2, 3, 2))]));
        let histogram = builder.histogram();
        assert_eq!(
            histogram,
            HashMap::from_iter(vec![
                ("a".into(), (vec![(5, 2)], 3)),
                ("b".into(), (vec![(2, 1), (4, 1)], 3))
            ])
        );
    }

    #[test]
    fn test_histogram_builder_stats_different_repeated_values() {
        let mut builder = HistogramBuilder::new();
        builder.update(&HashMap::from_iter(vec![("a".into(), (5, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (4, 3, 2))]));
        builder.update(&HashMap::from_iter(vec![("a".into(), (5, 3, 1))]));
        builder.update(&HashMap::from_iter(vec![("b".into(), (2, 3, 2))]));
        let stats = builder.stats();
        assert_eq!(
            stats,
            HashMap::from_iter(vec![
                ("101".to_string(), 2),
                ("100".to_string(), 1),
                ("010".to_string(), 1)
            ])
        );
    }
}
