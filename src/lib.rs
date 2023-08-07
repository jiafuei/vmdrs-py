extern crate blas_src;
use pyo3::{prelude::*,  exceptions::PyValueError};
use numpy::{ToPyArray, PyReadonlyArray1};
use vmd_rs::vmd;

#[pyfunction]
fn VMD(py: Python<'_>, input: PyReadonlyArray1<f64>, alpha: i32, tau: i32, K: usize, DC: i32, init: i32, tol: f64) -> PyResult<(PyObject, PyObject, PyObject)> {
    let res = vmd(input.as_slice().unwrap(), alpha, tau, K, DC, init, tol);
    match res {
        Ok((u, u_hat, omega)) => {
            Ok((u.to_pyarray(py).to_owned().into(), u_hat.to_pyarray(py).to_owned().into(), omega.to_pyarray(py).to_owned().into()))
        },
        Err(e) => Err(PyValueError::new_err(e.to_string()))
    }
}

// A Python module implemented in Rust.
#[pymodule]
fn vmdrs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(VMD, m)?)?;
    Ok(())
}
