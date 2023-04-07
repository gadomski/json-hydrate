use ::json_hydrate::{Error, Hydrate};
use pyo3::{create_exception, exceptions::PyException, prelude::*};
use serde_json::Value;

create_exception!(json_hydrate, HydrationError, PyException);

/// Hydrate Python dictionaries and arrays using a base object.
///
/// # Examples
///
/// ```python
/// item = {"a": "minimal", "item": {"meaning": 42}}
/// base = {"b": "c", "item": {"everything": true} }
/// item = json_hydrate.hydrate(item, base)
/// assert item == {"a": "minimal", "item": {"meaning": 42, "everything": true}, "b": "c"}
/// ```
#[pyfunction]
fn hydrate(item: &PyAny, base: &PyAny) -> PyResult<Py<PyAny>> {
    let mut serde_item: Value = pythonize::depythonize(item)?;
    let serde_base: Value = pythonize::depythonize(base)?;
    serde_item.hydrate(serde_base).map_err(JsonHydrateError)?;
    pythonize::pythonize(item.py(), &serde_item).map_err(PyErr::from)
}

struct JsonHydrateError(Error);

impl From<JsonHydrateError> for PyErr {
    fn from(value: JsonHydrateError) -> Self {
        HydrationError::new_err(value.0.to_string())
    }
}

/// Hydrate Python dictionaries and arrays using a base object.
#[pymodule]
fn json_hydrate(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hydrate, m)?)?;
    m.add("HydrationError", py.get_type::<HydrationError>())?;
    Ok(())
}
