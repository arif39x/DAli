use pyo3::prelude::*;
use pyo3::types::PyModule;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;

pub enum BridgeRequest {
    CalculateIndent {
        window: String,
        relative_line: usize,
    },
    GetGitInfo,
    ExpandSnippet {
        trigger: String,
    },
}

pub enum BridgeResponse {
    Indent(usize),
    GitInfo(String, usize),
    Snippet(Option<String>),
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

                let analysis: Py<PyModule> = py.import("analysis").expect("Failed to import analysis").into();
                let git: Py<PyModule> = py.import("git_sense").expect("Failed to import git_sense").into();
                let snippets: Py<PyModule> = py.import("snippets").expect("Failed to import snippets").into();

                while let Ok(req) = req_rx.recv() {
                    match req {
                        BridgeRequest::CalculateIndent { window, relative_line } => {
                            let result: usize = analysis.bind(py)
                                .getattr("calculate_indentation").unwrap()
                                .call1((window, relative_line)).unwrap()
                                .extract().unwrap_or(0);
                            let _ = res_tx.send(BridgeResponse::Indent(result));
                        }
                        BridgeRequest::GetGitInfo => {
                            let git_bind = git.bind(py);
                            let (branch, modified) = match git_bind.getattr("get_git_status") {
                                Ok(f) => f.call0().and_then(|r| r.extract::<(String, usize)>()).unwrap_or_else(|_| ("Git Error".to_string(), 0)),
                                Err(_) => ("No Git".to_string(), 0),
                            };
                            let _ = res_tx.send(BridgeResponse::GitInfo(branch, modified));
                        }
                        BridgeRequest::ExpandSnippet { trigger } => {
                            let result: Option<String> = snippets.bind(py)
                                .getattr("expand_snippet").ok()
                                .and_then(|f| f.call1((trigger,)).ok())
                                .and_then(|r| r.extract::<Option<String>>().ok())
                                .flatten();
                            let _ = res_tx.send(BridgeResponse::Snippet(result));
                        }
                    }
                }
            });
        });

        Self { sender: req_tx, receiver: res_rx }
    }

    pub fn request_indent(&self, buffer: &crate::buffer::GapBuffer, center_line: usize) {
        let (prefix, suffix) = buffer.get_chunks_str();
        let lines: Vec<&str> = prefix.lines().chain(suffix.lines()).collect();
        let start = center_line.saturating_sub(25);
        let end = std::cmp::min(lines.len(), center_line + 25);
        let window = lines[start..end].join("\n");
        let relative_line = center_line - start;

        let _ = self.sender.send(BridgeRequest::CalculateIndent { window, relative_line });
    }

    pub fn request_git_info(&self) {
        let _ = self.sender.send(BridgeRequest::GetGitInfo);
    }

    pub fn request_snippet(&self, trigger: &str) {
        let _ = self.sender.send(BridgeRequest::ExpandSnippet { trigger: trigger.to_string() });
    }

    pub fn try_recv(&self) -> Option<BridgeResponse> {
        self.receiver.try_recv().ok()
    }
}
