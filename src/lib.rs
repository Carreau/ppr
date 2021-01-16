use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
#[derive(Clone)]
pub struct Point {
    #[pyo3(get, set)]
    x: i64,
    #[pyo3(get, set)]
    y: i64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[pyclass]
struct MyVec {
    #[pyo3(get, set)]
    start: Point,
    #[pyo3(get, set)]
    end: Point,
}

#[pymethods]
impl MyVec {
    #[new]
    fn new(start: Point, end: Point) -> Self {
        MyVec { start, end }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ppr(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<Point>()?;
    m.add_class::<MyVec>()?;

    Ok(())
}
