use marc8::Marc8;
use pyo3::prelude::*;

#[pyclass]
struct MARC8ToUnicode(Marc8);

#[pymethods]
impl MARC8ToUnicode {
    #[new]
    #[pyo3(signature = (g0 = None, g1 = None, quiet = None))]
    fn new(g0: Option<u8>, g1: Option<u8>, quiet: Option<bool>) -> Self {
        Self(Marc8::new(g0, g1, quiet))
    }

    fn translate(&mut self, marc8_string: &[u8]) -> String {
        self.0.convert(marc8_string).unwrap().to_string()
    }
}

#[pymodule]
fn pymarc8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MARC8ToUnicode>()?;
    Ok(())
}
