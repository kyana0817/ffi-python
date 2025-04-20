mod structs;

use pyo3::prelude::*;
use structs::{PyManualSimple, Simple};

#[pyfunction]
fn manula_simple_struct() -> PyResult<PyManualSimple> {
    Ok(PyManualSimple::new(1, 2))
}

#[pyfunction]
fn simple_struct() -> PyResult<Simple> {
    Ok(Simple::new(1, 2))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(simple_struct, m)?)?;
    m.add_function(wrap_pyfunction!(manula_simple_struct, m)?)?;
    m.add_class::<Simple>()?;
    m.add_class::<PyManualSimple>()?;
    Ok(())
}
