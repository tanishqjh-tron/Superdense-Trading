mod order_book;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

// This #[pyclass] macro tells the compiler to expose this struct to Python
#[pyclass]
pub struct LimitOrderBook {
    // You will add your BTreeMaps and VecDeques here later
    pub is_active: bool,
}

// This #[pymethods] macro exposes these Rust functions so Python can call them
#[pymethods]
impl LimitOrderBook {
    #[new]
    pub fn new() -> Self {
        LimitOrderBook { is_active: true }
    }

    pub fn status(&self) -> String {
        "Superdense Engine: ONLINE. Ready for logic.".to_string()
    }
}

// This is the FFI Bridge. The function name MUST match your Cargo.toml lib name.
#[pymodule]
fn superdense_lob(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register the class with the module
    m.add_class::<LimitOrderBook>()?;
    Ok(())
}