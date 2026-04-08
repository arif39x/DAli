use pyo3::prelude::*;
use pyo3::types::PyModule;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;

pub enum BridgeRequest {
    GetGitInfo,
}

pub enum BridgeResponse {
    GitInfo(String, usize),
}

pub struct IntelligenceBridge {
    sender: Sender<BridgeRequest>,
    receiver: Receiver<BridgeResponse>,
}

impl IntelligenceBridge {
    pub fn new() -> Self {
        let (req_tx, req_rx) = unbounded::<BridgeRequest>();
        let (res_tx, res_rx) = unbounded::<BridgeResponse>();

        thread::spawn(move || {
            Python::with_gil(|py| {
                let sys = py.import("sys").expect("Failed to import sys");
                let path = sys.getattr("path").expect("Failed to get sys.path");
                let _ = path.call_method1("append", ("intelligence",)).expect("Failed to append to sys.path");

                let git: Py<PyModule> = py.import("git_sense").expect("Failed to import git_sense").into();

                while let Ok(req) = req_rx.recv() {
                    match req {
                        BridgeRequest::GetGitInfo => {
                            let git_bind = git.bind(py);
                            let (branch, modified) = match git_bind.getattr("get_git_status") {
                                Ok(f) => f.call0().and_then(|r| r.extract::<(String, usize)>()).unwrap_or_else(|_| ("Git Error".to_string(), 0)),
                                Err(_) => ("No Git".to_string(), 0),
                            };
                            let _ = res_tx.send(BridgeResponse::GitInfo(branch, modified));
                        }
                    }
                }
            });
        });

        Self { sender: req_tx, receiver: res_rx }
    }

    pub fn request_git_info(&self) {
        let _ = self.sender.send(BridgeRequest::GetGitInfo);
    }

    pub fn try_recv(&self) -> Option<BridgeResponse> {
        self.receiver.try_recv().ok()
    }
}
