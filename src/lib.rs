use pyo3::prelude::*;
use native_tls::{TlsConnector, TlsStream};
use tungstenite::protocol::WebSocket;

use std::thread;
use std::net::TcpStream;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
struct FastProtocol {
    ws: Option<WebSocket<TlsStream<TcpStream>>>,
    pub closed: bool,
    loop_: PyObject,
}

#[pymethods]
impl FastProtocol {
    #[new]
    fn new(loop_: PyObject) Self {
        FastProtocol {
            ws: None,
            closed: true,
            loop_: PyObject,
        }
    }
    
    fn connect(&mut self) {
        let future = self.loop_.call_method0("create_future");
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ugc_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
