# qasmsim
> A QASM interpreter and quantum simulator in Rust.

## Prerequisites

Make sure you have [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.
For compiling the WASM version, make sure you have [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
also installed.

## What is missing?

Looking for what's new? See [RELEASE_NOTES](RELEASE_NOTES.md).

Very few is missing, but the most important things are allowing the inclusion
of external libraries throught the `include` directive, and improve the
reporting of errors. Nevertheless,
[including `qelib1.inc`](https://github.com/Qiskit/openqasm/blob/master/examples/generic/qelib1.inc) is possible since it comes with the interpreter.

Planned for future versions is:

 - [ ] Allow including external source.
   - [ ] In the native lib.
   - [ ] In the WASM version.
 - [ ] Add a semantic checker for checking the correctness of the program before runtime.
 - [ ] Handling opaque gates. Right now they are ignored and calling an opaque gate results in Error, like trying to call an undefined gate.

A sample QASM program can be found here:

```qasm
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];
```

The complete specification can be found under the [QASM repository](https://github.com/Qiskit/openqasm/blob/master/spec-human/).

## qasmsim CLI

Install `qasmsim` with:

```sh
$ cargo install --git https://github.com/delapuente/qasmsim
```

And simulate a QASM program with:

```sh
$ qasmsim source.qasm
```

See more options with:

```
$ qasmsim --help
qasmsim 1.1.0
A QASM interpreter and quantum simulator in Rust.

USAGE:
    qasmsim [FLAGS] [OPTIONS] [source]

FLAGS:
    -b, --binary           Prints the binary representation of the values
    -h, --help             Prints help information
    -x, --hexadecimal      Prints the hexadecimal representation of the values
    -i, --integer          Prints the interger representation of the values. Default option
        --probabilities    Prints the probabilities vector of the simulation. Ignored if shots is set
        --statevector      Prints the state vector of the simulation. Ignored if shots is set
    -t, --times            Prints times measured for parsing and simulating
    -V, --version          Prints version information
    -v                     Verbosity of the output

OPTIONS:
        --info <info>      Show gate-related information
        --out <out>        Output files prefix, print in the stdout if not present. The output format of each file is
                           CSV. At most, three files are created with the names out.memory.csv, out.state.csv and
                           out.times.csv
        --shots <shots>    Specify the number of simulations

ARGS:
    <source>    QASM program file, read from stdin if not present
```

## qasmsim library

`qasmsim` is also a library including a QASM parser which generates a QASM AST,
an interpreter, and a quantum state-vector simulator. The command-line tool is
just one of the multiple consumers the library can have. If you want to install
the library functionality only, remove the `default` features when installing:

```sh
$ cargo install --no-default-features
```

## Testing the project

You can refer to unit tests (in the files under the `src` folder) and integration tests (under the `tests` folder) to figure out what is implemented. For passing the tests of the project you can do:

```sh
$ cargo test
```

## WASM version

`qasmsim` can be used in the web if you compile it for Web Assembly. Doing it is easy, simply download the sources, ensure you have `wasm-pack` installed and run:

```sh
$ wasm-pack build -- --features="serde"
```

It will compile your project and pack it inside the `pkg` folder. Now enter the `www` directory, install the dependencies with (you only need run this once):

```sh
$ npm install
```

And start the web server with:

```sh
$ npm start
```

Browse the URL provided by the second command and you'll see a blank page. Go to the developer tools of your browser and try running a small test:

```js
var result = qasmsim.run(`
OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];
`);
```

The module is exported by default as the `qasmsim` object in `window` and implements the following interface:

```ts
interface qasmsim {
  run: (input: string, shots?: number) => Execution,
  simulate: (program: OpenQasmProgram, shots?: number) => Computation,
  parseAndLink: (source: string) => OpenQasmProgram,
  parseProgram: (source: string) => OpenQasmProgram,
  parseLibrary: (source: string) => OpenQasmLibrary,
  parseExpression: (source: string) => Expression,
  parseProgramBody: (source: string) => Statement[],
  parseStatement: (source: string) => Statement
  getGateInfo: (source: string, gateName: string) => GateInfo
}

interface Computation {
  histogram?: Histogram,
  probabilities: Float64Array,
  statevector: { bases: Float64Array, qubitWidth: number },
  memory: Memory
}

interface Execution extends Computation {
  times: ExecutionTimes
}

type Memory = { [key: string]: Array[number] }
type Histogram = { [key: string]: Array[[number, number]] }
type ExecutionTimes = {
  parsing: number,
  simulation: number,
  serialization: number
}
type GateInfo = {
  docstring: string,
  name: string,
  realParameters: Array[string],
  quantumParameters: Array[string]
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE] or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

[LICENSE-APACHE]: LICENSE-APACHE.txt
[LICENSE-MIT]: LICENSE-MIT.txt

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
