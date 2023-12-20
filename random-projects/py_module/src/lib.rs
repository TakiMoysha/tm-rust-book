use pyo3::prelude::*;
use serde_json::Value;

#[pyclass]
pub struct TmJsonCache {
    pub key: String,
    pub value: Value,
}

#[pymethods]
impl TmJsonCache {
    #[new]
    fn new(key: String, value: String) -> Self {
        Self { key, value: serde_json::from_str(&value).unwrap() }
    }
}


#[pyfunction]
fn set_json(key: String, value: String) -> PyResult<String> {
    Ok(format!("{key}:{value}").to_string())
}

#[pyfunction]
fn get_json(key: String) -> PyResult<String> {
    Ok(format!("{key}:build-in").to_string())
}

#[pymodule]
fn tm_json_cache(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_json, m)?)?;
    m.add_function(wrap_pyfunction!(get_json, m)?)?;
    Ok(())
}
