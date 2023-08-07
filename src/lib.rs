extern crate blas_src;
use numpy::{PyReadonlyArray1, ToPyArray};
use pyo3::{exceptions::PyValueError, prelude::*};
use vmd_rs::vmd;

///
/// Input and Parameters:
/// ---------------------
/// f       - the time domain signal (1D) to be decomposed
/// alpha   - the balancing parameter of the data-fidelity constraint
/// tau     - time-step of the dual ascent ( pick 0 for noise-slack )
/// K       - the number of modes to be recovered
/// DC      - true if the first mode is put and kept at DC (0-freq)
/// init    - 0 = all omegas start at 0
///                    1 = all omegas start uniformly distributed
///                   2 = all omegas initialized randomly
/// tol     - tolerance of convergence criterion; typically around 1e-6

/// Output:
/// -------
/// u       - the collection of decomposed modes
/// u_hat   - spectra of the modes
/// omega   - estimated mode center-frequencies
/// 
#[allow(non_snake_case, clippy::too_many_arguments)]
#[pyfunction]
fn VMD(
    py: Python<'_>,
    input: PyReadonlyArray1<f64>,
    alpha: i32,
    tau: i32,
    K: usize,
    DC: i32,
    init: i32,
    tol: f64,
) -> PyResult<(PyObject, PyObject, PyObject)> {
    let res = vmd(input.as_slice().unwrap(), alpha, tau, K, DC, init, tol);
    match res {
        Ok((u, u_hat, omega)) => Ok((
            u.to_pyarray(py).to_owned().into(),
            u_hat.to_pyarray(py).to_owned().into(),
            omega.to_pyarray(py).to_owned().into(),
        )),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

// A Python module implemented in Rust.
#[pymodule]
fn vmdrs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(VMD, m)?)?;
    Ok(())
}
