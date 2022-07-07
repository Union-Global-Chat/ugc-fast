use pyo3::prelude::*;
use native_tls::{TlsConnector, TlsStream};
use tungstenite::protocol::{WebSocket, Message};
use reqwest::Method;

use std::net::TcpStream;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

struct BaseProtocol {
    ws: Option<WebSocket<TlsStream<TcpStream>>>,
}

impl BaseProtocol {
    fn new() -> Self {
        Self {
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
    }

    fn send_message(&mut self, message: String) {
        match self.ws {
            Some(ref mut ws) => {
                let _ = ws.write_message(Message::Text(message));
            },
            None => {
                println!("No websocket connection");
            }
        }
    }

    fn send_binary(&mut self, message: Vec<u8>) {
        match self.ws {
            Some(ref mut ws) => {
                let _ = ws.write_message(Message::Binary(message));
            },
            None => {
                println!("No websocket connection");
            }
        }
    }
}

#[pyclass]
struct BaseFastProtocol {
    ws: BaseProtocol,
    pub closed: bool,
}

#[pyclass]
struct FastSession {
    client: reqwest::Client,
}

#[pymethods]
impl BaseFastProtocol {
    #[new]
    fn new() -> Self {
        Self {
            ws: BaseProtocol::new(),
            closed: true,
        }
    }
    
    fn connect(&mut self) -> PyResult<()> {
        self.ws.connect();
        self.closed = false;
        Ok(())
    }

    fn send_message(&mut self, message: String) -> PyResult<()> {
        self.ws.send_message(message);
        Ok(())
    }

    fn send_binary(&mut self, message: Vec<u8>) -> PyResult<()> {
        self.ws.send_binary(message);
        Ok(())
    }
}

#[pymethods]
impl FastSession {
    #[new]
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ugc_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BaseFastProtocol>()?;
    m.add_class::<FastSession>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
