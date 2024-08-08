#![cfg(not(target_arch = "wasm32"))]

use std::collections::HashMap;
use std::convert;

use crate::{api, statevector::StateVector};

use crate::error::QasmSimError;
use crate::interpreter::{Computation, Histogram};

pub use api::get_gate_info;
pub use api::parse_and_link;
pub use api::simulate;
pub use api::simulate_with_shots;

macro_rules! measure {
    ($block:expr) => {{
        use std::time::Instant;
        let measurement = Instant::now();
        let result = $block;
        let elapsed = measurement.elapsed().as_millis();
        (result, elapsed)
    }};
}

/// Register the milliseconds spent in parsing the program and simulating.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct ExecutionTimes {
    parsing_time: u128,
    simulation_time: u128,
}

impl ExecutionTimes {
    /// Create new times statistics. Parameters are expressed in milliseconds.
    pub fn new(parsing_time: u128, simulation_time: u128) -> Self {
        ExecutionTimes {
            parsing_time,
            simulation_time,
        }
    }

    /// Return the time spent in parsing the program and converting it to an AST.
    pub fn simulation_time(&self) -> u128 {
        self.simulation_time
    }

    /// Return the time spent in simulating the program.
    pub fn parsing_time(&self) -> u128 {
        self.parsing_time
    }
}

impl From<&[u128; 2]> for ExecutionTimes {
    fn from(pair: &[u128; 2]) -> Self {
        ExecutionTimes::new(pair[0], pair[1])
    }
}

impl From<(u128, u128)> for ExecutionTimes {
    fn from(pair: (u128, u128)) -> Self {
        ExecutionTimes::new(pair.0, pair.1)
    }
}

/// Represent a complete execution of a program, from parsing to simulating.
///
/// This structure is similar to [`Computation`] although this also includes
/// [time statistics] regarding parsing and execution times.
///
/// # Examples
///
/// See the [`run()`] function for a complete example.
///
/// [`run()`]: ./fn.run.html
/// [`Computation`]: ./struct.Computation.html
/// [time statistics]: ./struct.ExecutionTimes.html
#[derive(Debug, Clone, PartialEq)]

pub struct Execution {
    statevector: StateVector,
    probabilities: Vec<f64>,
    memory: HashMap<String, (u64, usize, usize)>,
    histogram: Option<Histogram>,
    times: ExecutionTimes,
    stats: Option<HashMap<String, usize>>,
}

impl Execution {
    /// Create a new `Execution` instance.
    pub fn new(
        statevector: StateVector,
        probabilities: Vec<f64>,
        memory: HashMap<String, (u64, usize, usize)>,
        histogram: Option<Histogram>,
        times: ExecutionTimes,
        stats: Option<HashMap<String, usize>>,
    ) -> Self {
        Execution {
            statevector,
            probabilities,
            memory,
            histogram,
            times,
            stats,
        }
    }

    /// Return the statevector of the quantum system.
    pub fn statevector(&self) -> &StateVector {
        &self.statevector
    }

    /// Return the probabilities associated with the state-vector.
    pub fn probabilities(&self) -> &Vec<f64> {
        &self.probabilities
    }

    /// Return an associative map with classical names and the classical outcomes.
    pub fn memory(&self) -> &HashMap<String, (u64, usize, usize)> {
        &self.memory
    }

    /// Return the histogram when simulating with several shots.
    pub fn histogram(&self) -> &Option<Histogram> {
        &self.histogram
    }

    /// Return the time spent in parsing and performing the simulation.
    pub fn times(&self) -> &ExecutionTimes {
        &self.times
    }

    /// Return the statistics of the simulation.
    pub fn stats(&self) -> &Option<HashMap<String, usize>> {
        &self.stats
    }
}

impl convert::From<(Computation, u128, u128)> for Execution {
    fn from(value: (Computation, u128, u128)) -> Self {
        let (computation, parsing_time, simulation_time) = value;
        Execution {
            statevector: computation.statevector().clone(),
            probabilities: computation.probabilities().to_vec(),
            memory: computation.memory().clone(),
            histogram: computation.histogram().clone(),
            times: ExecutionTimes {
                parsing_time,
                simulation_time,
            },
            stats: computation.stats().clone(),
        }
    }
}

/// Parse and simulate the `input` OPENQASM program with optional `shots`.
///
/// # Errors
///
/// The function can fail if the source code presents an error or something
/// unexpected happens during the simulation. In this case, an `Err` variant
/// wrapping a value of [`QasmSimError`] is returned.
///
/// [`QasmSimError`]: ./error/enum.QasmSimError.html
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use qasmsim::run;
///
/// let execution = run(r#"
/// OPENQASM 2.0;
/// include "qelib1.inc";
/// qreg q[2];
/// "#, None)?;
/// # use qasmsim::QasmSimError;
/// # Ok::<(), QasmSimError>(())
/// ```
pub fn run(input: &str, shots: Option<usize>) -> api::Result<'_, Execution> {
    let (linked, parsing_time) = measure!({ parse_and_link(input) });
    let (out, simulation_time) = measure!({
        match shots {
            None => simulate(&linked?),
            Some(shots) => simulate_with_shots(&linked?, shots),
        }
    });
    let out = out.map_err(|err| QasmSimError::from((input, err)));
    Ok(Execution::from((out?, parsing_time, simulation_time)))
}
