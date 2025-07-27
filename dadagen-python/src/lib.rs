//! Python bindings for dadagen
//! 
//! High-performance data generation from Python
//! using PyO3 bindings.

use pyo3::prelude::*;

#[pyfunction]
fn hello_dadagen(name: &str) -> String {
    format!("Hello, {}! Dadagen Python bindings are ready.", name)
}

#[pymodule]
fn dadagen_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_dadagen, m)?)?;
    Ok(())
}
