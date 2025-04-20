use pyo3::prelude::*;

#[pyclass]
pub struct Simple {
    #[pyo3(get, set)]
    pub a: i32,
    #[pyo3(get, set)]
    pub b: i32,
}

impl Simple {
    pub fn new(a: i32, b: i32) -> Self {
        Simple { a, b }
    }
}

pub struct ManualSimple {
    pub a: i32,
    pub b: i32,
}

#[pyclass(name = "ManualSimple")]
pub struct PyManualSimple {
    inner: ManualSimple,
}

#[pymethods]
impl PyManualSimple {
    #[new]
    pub fn new(a: i32, b: i32) -> Self {
        PyManualSimple {
            inner: ManualSimple { a, b },
        }
    }

    #[getter]
    fn a(&self) -> PyResult<i32> {
        Ok(self.inner.a)
    }
    #[getter]
    fn b(&self) -> PyResult<i32> {
        Ok(self.inner.b)
    }
}
