#![cfg(test)]

extern crate qasmsim;

use std::f64::consts::FRAC_1_SQRT_2;

use qasmsim::statevector::{assert_approx_eq, Complex, StateVector};

#[test]
fn endianess() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) r[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
        ]),
    )
}

#[test]
fn call_custom_gate_on_qubit() {
    let source = "
  OPENQASM 2.0;
  gate h q {
    U(pi/2, 0, pi) q;
  }
  qreg q[1];
  h q[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn call_custom_gate_on_register() {
    let source = "
  OPENQASM 2.0;
  gate h q {
    U(pi/2, 0, pi) q;
  }
  qreg q[2];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
        ]),
    )
}

#[test]
fn call_custom_gate_inside_custom_gate() {
    let source = "
  OPENQASM 2.0;
  gate u2(phi, lambda) q {
    U(pi/2, phi, lambda) q;
  }
  gate h q {
    u2(0, pi) q;
  }
  qreg q[2];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
            Complex::from(0.5),
        ]),
    )
}

#[test]
fn test_one_register_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[2];
  U (pi/2, 0, pi) q[0];
  CX q[0], q[1];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_two_registers_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) q[0];
  CX q[0], r[0];
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_no_indices_bell_circuit() {
    let source = "
  OPENQASM 2.0;
  qreg q[1];
  qreg r[1];
  U (pi/2, 0, pi) q;
  CX q, r;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![
            Complex::from(FRAC_1_SQRT_2),
            Complex::from(0.0),
            Complex::from(0.0),
            Complex::from(FRAC_1_SQRT_2),
        ]),
    )
}

#[test]
fn test_no_indices_superposition() {
    let source = "
  OPENQASM 2.0;
  qreg q[4];
  U (pi/2, 0, pi) q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![Complex::from(0.25); 16]),
    )
}

#[test]
fn test_quantum_experience_header_is_included() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[4];
  h q;
  ";
    assert_approx_eq(
        qasmsim::run(source, None).unwrap().statevector(),
        &StateVector::from_complex_bases(vec![Complex::from(0.25); 16]),
    )
}

#[test]
fn test_measurements() {
    let subtests = vec![
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     measure q -> c;
     ",
            0b00_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q[0];
     measure q -> c;
     ",
            0b01_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q[1];
     measure q -> c;
     ",
            0b10_u64,
        ),
        (
            "
     OPENQASM 2.0;
     include \"qelib1.inc\";
     qreg q[2];
     creg c[2];
     x q;
     measure q -> c;
     ",
            0b11_u64,
        ),
    ];
    for (index, (source, expected_result)) in subtests.iter().enumerate() {
        let result = &qasmsim::run(source, None).unwrap();
        println!("Using source sample #{}", index);
        assert_eq!(
            *result.memory().get("c").unwrap(),
            (*expected_result, 2, 68)
        );
    }
}

#[test]
fn test_all_classical_memory_is_displayed() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[2];
  creg c[2];
  creg d[2];
  creg e[2];
  x q;
  measure q -> c;
  ";
    let result = &qasmsim::run(source, None).unwrap();
    assert_eq!(result.memory().len(), 3);
    assert_eq!(*result.memory().get("c").unwrap(), (0b11, 2, 56));
    assert_eq!(*result.memory().get("d").unwrap(), (0b0, 2, 69));
    assert_eq!(*result.memory().get("e").unwrap(), (0b0, 2, 82));
}

#[test]
fn test_conditional() {
    let source = "
  OPENQASM 2.0;
  include \"qelib1.inc\";
  qreg q[2];
  creg c[2];
  creg d[2];
  x q[1];
  measure q[1] -> c[1];
  if (c==2) x q;
  measure q -> d;
  ";
    let result = &qasmsim::run(source, None).unwrap();
    assert_eq!(result.memory().len(), 2);
    assert_eq!(*result.memory().get("c").unwrap(), (0b10, 2, 56));
    assert_eq!(*result.memory().get("d").unwrap(), (0b01, 2, 69));
}

#[test]
fn test_print_json_1() {
    let source = "
    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[2];
    creg c[2];
    x q;
    measure q -> c;
    ";

    let option = qasmsim::options::Options {
        format: qasmsim::options::Format::Json,
        shots: None,
        times: false,
        ..Default::default()
    };

    let result = qasmsim::run(source, option.shots).unwrap();
    let output = qasmsim::print_result(&result, &option);
    assert_eq!(
        output,
        r#"{
  "Expectations": [
    "1.000000",
    "1.000000"
  ],
  "State": {
    "0": {
      "Imaginary": "0.000000",
      "Probability": "0.000000",
      "Real": "0.000000"
    },
    "1": {
      "Imaginary": "0.000000",
      "Probability": "0.000000",
      "Real": "0.000000"
    },
    "2": {
      "Imaginary": "0.000000",
      "Probability": "0.000000",
      "Real": "0.000000"
    },
    "3": {
      "Imaginary": "0.000000",
      "Probability": "1.000000",
      "Real": "1.000000"
    }
  }
}"#
    );
}

#[test]
fn test_print_json_2() {
    let source = "
    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[2];
    creg c[2];
    h q;
    ";

    let option = qasmsim::options::Options {
        format: qasmsim::options::Format::Json,
        shots: None,
        times: false,
        ..Default::default()
    };

    let result = qasmsim::run(source, option.shots).unwrap();
    let output = qasmsim::print_result(&result, &option);
    assert_eq!(
        output,
        r#"{
  "Expectations": [
    "0.000000",
    "0.000000"
  ],
  "State": {
    "0": {
      "Imaginary": "0.000000",
      "Probability": "0.250000",
      "Real": "0.500000"
    },
    "1": {
      "Imaginary": "0.000000",
      "Probability": "0.250000",
      "Real": "0.500000"
    },
    "2": {
      "Imaginary": "0.000000",
      "Probability": "0.250000",
      "Real": "0.500000"
    },
    "3": {
      "Imaginary": "0.000000",
      "Probability": "0.250000",
      "Real": "0.500000"
    }
  }
}"#
    );
}

#[test]
fn test_print_json_shots() {
    let source = "
    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[2];
    creg c[2];
    creg c1[2];
    x q[0];
    measure q -> c;
    ";

    let option = qasmsim::options::Options {
        format: qasmsim::options::Format::Json,
        shots: Some(1000),
        times: false,
        ..Default::default()
    };

    let result = qasmsim::run(source, option.shots).unwrap();
    let output = qasmsim::print_result(&result, &option);
    assert_eq!(
        output,
        r#"{
  "Memory": {
    "0001": 1000
  }
}"#
    )
}

#[test]
fn test_print_json_shots_sequence() {
    let source = "
    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[2];
    creg c[2];
    creg c1[2];
    x q[0];
    measure q -> c;
    ";

    let option = qasmsim::options::Options {
        format: qasmsim::options::Format::Json,
        shots: Some(5),
        times: false,
        mode: "sequence".to_string(),
        ..Default::default()
    };

    let result = qasmsim::run_mode(source, option.shots, option.mode.clone()).unwrap();
    println!("{:?}", result);
    let output = qasmsim::print_result(&result, &option);
    assert_eq!(
        output,
        r#"{
  "Sequences": [
    "0001",
    "0001",
    "0001",
    "0001",
    "0001"
  ]
}"#
    )
}

// TODO: add min and max test
// #[test]
// fn test_print_json_shots_max() {
//     let source = "
//     OPENQASM 2.0;
//     include \"qelib1.inc\";
//     qreg q[2];
//     creg c[2];
//     creg c1[2];
//     h q[0];
//     ry(1/4) q[0];
//     measure q -> c;
//     ";

//     let option = qasmsim::options::Options {
//         format: qasmsim::options::Format::Json,
//         shots: Some(1000),
//         times: false,
//         mode: "max".to_string(),
//         ..Default::default()
//     };

//     let result = qasmsim::run_mode(source, option.shots, option.mode.clone()).unwrap();
//     let output = qasmsim::print_result(&result, &option);
//     assert_eq!(
//         output,
//         r#"{
//   "Memory": {
//     "0000": xxx
//   }
// }"#
//     )
// }
