//! This module contains the definition of the command line options.

/// Output options.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Options {
    /// Verbosity of the output.
    pub verbose: bool,

    /// Prints the binary representation of the values.
    pub binary: bool,

    /// Prints the hexadecimal representation of the values.
    pub hexadecimal: bool,

    /// Prints the interger representation of the values. Default option.
    pub integer: bool,

    /// Prints the state vector of the simulation. Ignored if shots is set.
    pub statevector: bool,

    /// Prints the probabilities vector of the simulation. Ignored if shots is set.
    pub probabilities: bool,

    /// Prints times measured for parsing and simulating.
    pub times: bool,

    /// Specify the number of simulations.
    pub shots: Option<usize>,

    /// Show gate-related information.
    pub info: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            verbose: false,
            binary: false,
            hexadecimal: false,
            integer: true,
            statevector: false,
            probabilities: false,
            times: false,
            shots: None,
            info: None,
        }
    }
}
