use std::collections::HashMap;
use std::fmt::{self, Write};

use serde_json::{json, Value};

use crate::statevector::StateVector;
use crate::{Execution, ExecutionTimes, Histogram};

use crate::options::Options;

/// Writes the `msg` in the `buffer`
pub fn print<W>(buffer: &mut W, result: &Execution, options: &Options)
where
    W: Write,
{
    do_print(buffer, result, options).expect("writes in stdout");
}

/// Writes the `msg` in the `buffer`
fn do_print<W>(buffer: &mut W, result: &Execution, options: &Options) -> fmt::Result
where
    W: Write,
{
    let mut output = json!({});
    if options.shots.is_some() {
        let stats = result.stats().as_ref().expect("there is some histogram");
        println!("{:?}", stats.is_empty());
        if !stats.is_empty() {
            if options.mode == "aggregation" {
                print_stats(&mut output, stats)?;
            } else if options.mode == "min" || options.mode == "max" {
                print_minmax(&mut output, stats, options)?;
            } else {
                panic!("Invalid mode");
            }
        } else if !result
            .sequences()
            .as_ref()
            .expect("there is some sequences")
            .is_empty()
        {
            if options.mode == "sequence" {
                print_sequence(
                    &mut output,
                    result
                        .sequences()
                        .as_ref()
                        .expect("there is some sequences"),
                )?;
            } else {
                panic!("Invalid mode");
            }
        }
    }

    if (options.statevector || options.probabilities) && options.shots.is_none() {
        print_state(
            &mut output,
            result.statevector(),
            result.probabilities(),
            options,
        )?;
    }

    if options.times {
        print_times(&mut output, result.times())?;
    }

    let output_str = serde_json::to_string_pretty(&output).expect("json pretty print");
    write!(buffer, "{}", output_str)
}

fn print_memory(
    value: &mut Value,
    memory: &HashMap<String, (u64, usize, usize)>,
    options: &Options,
) -> fmt::Result {
    let histogram = HashMap::from_iter(
        memory
            .iter()
            .map(|(key, value)| (key.clone(), (vec![(value.0, 1)], value.1))),
    );
    print_memory_summary(value, &histogram, options, true)
}

fn print_histogram(value: &mut Value, histogram: &Histogram, options: &Options) -> fmt::Result {
    print_memory_summary(value, histogram, options, false)
}

fn print_memory_summary(
    value: &mut Value,
    histogram: &Histogram,
    options: &Options,
    omit_count: bool,
) -> fmt::Result {
    let mut json = json!({});

    let binary = options.binary;
    let hexadecimal = options.hexadecimal;
    let integer = if binary || hexadecimal {
        options.integer
    } else {
        true
    };

    for (key, hist) in histogram {
        json[key] = json!({});
        for (idx, (value, count)) in hist.0.iter().enumerate() {
            json[key][format!("{}", idx)] = json!({});
            json[key][format!("{}", idx)]["Register length"] = json!(hist.1);
            if integer {
                json[key][format!("{}", idx)]["Int value"] = json!(value);
            }
            if hexadecimal {
                json[key][format!("{}", idx)]["Hex value"] = json!(format!("0x{:x}", value));
            }
            if binary {
                json[key][format!("{}", idx)]["Bin value"] =
                    json!(format!("0b{:0width$b}", value, width = hist.1));
            }
            if !omit_count {
                json[key][format!("{}", idx)]["Count"] = json!(count);
            }
        }
    }

    value["Memory"] = json;

    Ok(())
}

fn print_stats(value: &mut Value, stats: &HashMap<String, usize>) -> fmt::Result {
    let json = json!(stats);

    value["Memory"] = json;

    Ok(())
}

fn print_sequence(value: &mut Value, sequences: &Vec<String>) -> fmt::Result {
    let json = json!(sequences);

    value["Sequences"] = json;

    Ok(())
}

fn print_minmax(
    value: &mut Value,
    stats: &HashMap<String, usize>,
    options: &Options,
) -> fmt::Result {
    if options.mode == "max" {
        let max_state = stats.iter().max_by_key(|(_, &v)| v).unwrap();
        value["Memory"] = json!({max_state.0: max_state.1});
    } else {
        let min_state = stats.iter().min_by_key(|(_, &v)| v).unwrap();
        value["Memory"] = json!({min_state.0: min_state.1});
    }
    Ok(())
}

fn print_state(
    value: &mut Value,
    statevector: &StateVector,
    probabilities: &[f64],
    options: &Options,
) -> fmt::Result {
    assert!(
        options.statevector || options.probabilities,
        "at least one of probabibilities or statevector should be provided"
    );

    let mut json = json!({});

    let amplitudes_and_probabilities = statevector
        .as_complex_bases()
        .iter()
        .zip(probabilities)
        .enumerate();
    for (idx, (amplitude, probability)) in amplitudes_and_probabilities {
        json[format!("{}", idx)] = json!({});
        if options.statevector {
            json[format!("{}", idx)]["Real"] = json!(format!("{:.6}", amplitude.re));
            json[format!("{}", idx)]["Imaginary"] = json!(format!("{:.6}", amplitude.im));
        }
        if options.probabilities {
            json[format!("{}", idx)]["Probability"] = json!(format!("{:.6}", probability));
        }
    }

    if options.statevector {
        let format_vec: Vec<String> = statevector
            .expectation_values()
            .iter()
            .map(|v| format!("{:.6}", v))
            .collect();
        value["Expectations"] = json!(format_vec);
    }

    value["State"] = json;

    Ok(())
}

fn print_times(value: &mut Value, times: &ExecutionTimes) -> fmt::Result {
    let json = json!({
        "Parsing": times.parsing_time(),
        "Simulation": times.simulation_time(),
    });

    value["Times"] = json;

    Ok(())
}
