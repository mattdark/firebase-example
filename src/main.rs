extern crate cpython;

use cpython::{Python, PyResult, PyModule};

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Slides {
    pub id: String,
    pub file: String,
    pub title: String,
    pub description: String,
    pub theme: String,
    pub url: String,
}

const FIRE_PY: &'static str = include_str!("./python/pyrebase.py");

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let s = example(py).unwrap();
    println!("{:?}", s);
}

fn example(py: Python<'_>) -> PyResult<(Vec<Slides>)> {
    let m = module_from_str(py, "pyrebase", FIRE_PY)?;

    let out: String = m.call(py, "read_data", (2,), None)?.extract(py)?;

    let slides: Vec<Slides> = serde_json::from_str(&out).unwrap();
    
    Ok(slides)
}

/// Import a module from the given file contents.
///
/// This is a wrapper around `PyModule::new` and `Python::run` which simulates
/// the behavior of the builtin function `exec`. `name` will be used as the
/// module's `__name__`, but is not otherwise important (it does not need
/// to match the file's name).
///
/// Note this compiles and executes the module code each time it is called, as it
/// bypasses the regular import mechanism. No entry is added to the cache in `sys.modules`.
fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;
    m.add(py, "__builtins__", py.import("builtins")?)?;

    let m_locals = m.get(py, "__dict__")?.extract(py)?;
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}
