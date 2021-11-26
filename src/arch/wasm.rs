#![cfg(target_arch = "wasm32")]
#![allow(missing_docs)]

#[macro_use]
mod macros;
mod computation;
mod error;

use std::iter::FromIterator;

use console_error_panic_hook;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys;

use crate::api;
use crate::error::QasmSimError;
use crate::grammar::{self, ast};

macro_rules! adapt_parse_functions {
    ($($(#[$attr:meta])* $vis:vis fn $funcname:ident ($param:ident) => $parsefunc:path;)*) => {
        $(
            #[wasm_bindgen]
            #[allow(non_snake_case)]
            $(#[$attr])* $vis fn $funcname(
                $param: &str
            ) -> Result<JsValue, JsValue> {
                $parsefunc(source)
                    .map(|v| serde_wasm_bindgen::to_value(&v).unwrap())
                    .map_err(|err| err.into())
            }
        )*
    };
}

#[wasm_bindgen]
pub fn run(input: &str, shots: Option<usize>) -> Result<JsValue, JsValue> {
    let (linked, parsing_time) = measure!("parsing", { api::parse_and_link(input) });
    let (computation, simulation_time) = measure!("simulation", {
        match shots {
            None => api::simulate(&linked?),
            Some(shots) => api::simulate_with_shots(&linked?, shots),
        }
    });
    let (out, serialization_time) = measure!("serialization", {
        computation
            .map_err(|err| QasmSimError::from((input, err)))?
            .into()
    });
    let times = js_sys::Object::new();
    set!(&times,
        "parsing" => parsing_time,
        "simulation" => simulation_time,
        "serialization" => serialization_time
    );
    set!(&out, "times" => times);
    Ok(out)
}

#[wasm_bindgen]
pub fn simulate(program: JsValue, shots: Option<usize>) -> Result<JsValue, JsValue> {
    let openqasm_program: ast::OpenQasmProgram = serde_wasm_bindgen::from_value(program)?;
    let computation = match shots {
        None => api::simulate(&openqasm_program),
        Some(shots) => api::simulate_with_shots(&openqasm_program, shots),
    };
    computation.map(|v| v.into()).map_err(|err| err.into())
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn getGateInfo(input: &str, gate_name: &str) -> Result<JsValue, JsValue> {
    api::get_gate_info(input, gate_name)
        .map(|(docstring, (name, real_params, quantum_params))| {
            let gate_info = js_sys::Object::new();
            set!(&gate_info,
                "docstring" => docstring,
                "name" => name,
                "realParameters" => js_sys::Array::from_iter(real_params.iter().map(JsValue::from)),
                "quantumParameters" => js_sys::Array::from_iter(quantum_params.iter().map(JsValue::from))
            );
            gate_info.into()
        })
        .map_err(|err| err.into())
}

adapt_parse_functions! {
    pub fn parseAndLink(source) => api::parse_and_link;
    pub fn parseProgram(source) => grammar::parse_program;
    pub fn parseLibrary(source) => grammar::parse_library;
    pub fn parseExpression(source) => grammar::parse_expression;
    pub fn parseProgramBody(source) => grammar::parse_program_body;
    pub fn parseStatement(source) => grammar::parse_statement;
}

#[wasm_bindgen(start)]
pub fn init() {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook))
}
