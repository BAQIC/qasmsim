#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/qasmsim/1.3.1")]
//! The `qasmsim` library includes a
//! [OPENQASM 2.0](https://github.com/Qiskit/openqasm/blob/master/spec-human/)
//! parser and interpreter, along with a statevector simulator. Compiled with
//! default features, the library presents a `qasmsim` CLI for running programs
//! like the following one:
//!
//! ```qasm
//! // in test.qasm
//! OPENQASM 2.0;
//! include "qelib1.inc";
//! qreg q[2];
//! creg c[2];
//! h q[0];
//! cx q[0], q[1];
//! measure q -> c;
//! ```
//!
//! ```sh
//! $ qasmsim --shots 1024 test.qasm
//! +------+-----------+-------+
//! | Name | Int value | Count |
//! +------+-----------+-------+
//! |    c |         0 |   503 |
//! |      |         3 |   521 |
//! +------+-----------+-------+
//! ```
//!
//! Check the full options with:
//!
//! ```sh
//! $ qasmsim --help
//! qasmsim 1.2.0
//! A QASM interpreter and quantum simulator in Rust.
//!
//! USAGE:
//!     qasmsim [FLAGS] [OPTIONS] [source]
//!
//! FLAGS:
//!     -b, --binary           Prints the binary representation of the values
//!     -h, --help             Prints help information
//!     -x, --hexadecimal      Prints the hexadecimal representation of the values
//!     -i, --integer          Prints the interger representation of the values. Default option
//!         --probabilities    Prints the probabilities vector of the simulation. Ignored if shots is set
//!         --statevector      Prints the state vector of the simulation. Ignored if shots is set
//!     -t, --times            Prints times measured for parsing and simulating
//!     -V, --version          Prints version information
//!     -v                     Verbosity of the output
//!
//! OPTIONS:
//!         --info <info>      Show gate-related information
//!         --out <out>        Output files prefix, print in the stdout if not present. The output format of each file is
//!                            CSV. At most, three files are created with the names out.memory.csv, out.state.csv and
//!                            out.times.csv
//!         --shots <shots>    Specify the number of simulations
//!
//! ARGS:
//!     <source>    QASM program file, read from stdin if not present
//! ```
#[macro_use]
pub mod error;
pub mod grammar;
pub mod options;
pub mod output;
pub mod statevector;

pub use crate::{
    arch::native::{
        get_gate_info, parse_and_link, run, simulate, simulate_with_shots, Execution,
        ExecutionTimes,
    },
    error::QasmSimError,
    interpreter::{Computation, Histogram},
    semantics::QasmType,
};

mod api;
mod arch;
mod complex;
mod interpreter;
mod linker;
mod qe;
mod random;
mod semantics;
