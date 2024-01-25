use crate::Execution;
use crate::{options, output};
use std::fmt;

/// print gate info.
pub fn print_info(
    docstring: &str,
    name: &str,
    real_params: &[String],
    quantum_params: &[String],
) -> fmt::Result {
    println!(
        "gate {}{} {}",
        name,
        match real_params.len() {
            0 => String::from(""),
            _ => format!("({})", real_params.join(", ")),
        },
        quantum_params.join(" ")
    );
    println!("{}", docstring);
    Ok(())
}

/// print result.
pub fn print_result(result: &Execution, options: &options::Options) -> String {
    let mut output = String::new();
    output::tabular::print(&mut output, result, options);

    output
}
