extern crate ahrs_fusion;

use pyo3::prelude::*;
use ahrs_fusion::NxpFusion;

#[pymodule]
fn ahrs_fusion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NxpFusion>()?;

    Ok(())
}
