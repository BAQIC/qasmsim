//! This module contains the definition of the command line options.

/// Output format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Format {
    /// Tabular format.
    Tabular,

    /// JSON format.
    Json,
}

/// Output options.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Options {
    /// Output format.
    pub format: Format,

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

    /// Specify the mode of return value
    pub mode: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            format: Format::Tabular,
            binary: true,
            hexadecimal: true,
            integer: true,
            statevector: true,
            probabilities: true,
            times: false,
            shots: None,
            mode: "aggregation".to_string(),
        }
    }
}
