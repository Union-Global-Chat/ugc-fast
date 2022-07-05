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

struct BaseProtocol {
    ws: Option<WebSocket<TlsStream<TcpStream>>>,
}

impl BaseProtocol {
    fn new() Self -> {
        BaseProtocol {
            ws: None,
        }
    }
    
    fn connect(&mut self) {
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("ugc.renorari.net:443").unwrap();
        let stream = connector.connect("ugc.renorari.net", stream).unwrap();
        let wsurl = String::from("wss://ugc.renorari.net/v1/gateway");
        let (ws, _res) = tungstenite::client::client(&wsurl, stream).unwrap();
        self.ws = Some(ws);
        self.closed = false;
    }
}

#[pyclass]
struct FastProtocol {
    ws: BaseProtocol,
    pub closed: bool,
    loop_: PyObject,
}

#[pymethods]
impl FastProtocol {
    #[new]
    fn new(loop_: PyObject) -> Self {
        FastProtocol {
            ws: BaseProtocol(),
            closed: true,
            loop_: PyObject,
        }
    }
    
    fn connect(&mut self) -> PyResult<PyObject> {
        let future = self.loop_.call_method0("create_future")?;
        thread::spawn(move || {
            ws.connect();
            let gil = Python::acquire_gil();
            let py = gil.python();
            let set_result = future.getattr(py, "set_result")?;
            loop_.call_method1(py, "call_soon_threadsafe", (set_result, py.None()))?;
        });
        Ok(future)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ugc_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
