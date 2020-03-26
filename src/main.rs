extern crate qasmsim;

use std::collections::HashMap;
use std::io::{ self, Read };
use std::fs;
use std::path::PathBuf;
use std::fmt::{ self, Write };

use qasmsim::Computation;
use qasmsim::statevector::StateVector;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "qasmsim", about = "A QASM interpreter and quantum simulator in Rust.")]
struct CLI {

  /// QASM program file, read from stdin if not present
  #[structopt(parse(from_os_str))]
  source: Option<PathBuf>,

  /// Output file, stdout if not present
  #[structopt(long)]
  output: Option<PathBuf>,

  /// Prints the state vector of the simulation
  #[structopt(long)]
  statevector: bool,

  /// Prints the probabilities vector of the simulation
  #[structopt(long)]
  probabilities: bool,

  /// Prints times measured for parsing and simulating
  #[structopt(short, long)]
  times: bool

}

fn main() -> io::Result<()> {
  let options = CLI::from_args();
  let source = get_source(&options.source)?;
  match qasmsim::run(&source) {
    Ok(result) => print_result(&result, &options.output, options.statevector, options.probabilities).expect("print result"),
    Err(error) => eprintln!("{}", error)
  }
  Ok(())
}

fn get_source(source: &Option<PathBuf>) -> io::Result<String> {
  if let Some(path) = source {
    fs::read_to_string(path)
  }
  else {
    let mut source = String::new();
    io::stdin().read_to_string(&mut source)?;
    Ok(source)
  }
}

fn print_result(result: &Computation, output: &Option<PathBuf>, statevector: bool, probabilities: bool) -> fmt::Result {
  let mut buffer = String::new();
  print_memory(&mut buffer, &result.memory).expect("can print");
  if statevector {
    print_statevector(&mut buffer, &result.statevector).expect("can print");
  }
  if probabilities {
    print_probabilities(&mut buffer, &result.probabilities).expect("can print");
  }
  Ok(match output {
    Some(path) => fs::write(path, buffer).expect("can write"),
    None => print!("{}", buffer)
  })
}

fn print_memory(buffer: &mut String, memory: &HashMap<String, u64>) -> fmt::Result {
  for (key, value) in memory {
    writeln!(buffer, "{} => {}", key, value)?;
  }
  Ok(())
}

fn print_probabilities(buffer: &mut String, probabilities: &Vec<f64>) -> fmt::Result {
  for (idx, chance) in probabilities.iter().enumerate() {
    writeln!(buffer, "{}: {:.6}%", idx, *chance * 100.0)?;
  }
  Ok(())
}

fn print_statevector(buffer: &mut String, statevector: &StateVector) -> fmt::Result {
  for (idx, amplitude) in statevector.bases.iter().enumerate() {
    writeln!(buffer, "{}: {:.6}", idx, amplitude)?;
  }
  Ok(())
}
