use crossbeam_channel::{unbounded, Receiver, Sender};
use std::process::Command;
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
            while let Ok(req) = req_rx.recv() {
                match req {
                    BridgeRequest::GetGitInfo => {
                        let info = Self::get_git_status_native();
                        let _ = res_tx.send(BridgeResponse::GitInfo(info.0, info.1));
                    }
                }
            }
        });

        Self {
            sender: req_tx,
            receiver: res_rx,
        }
    }

    fn get_git_status_native() -> (String, usize) {
        let branch = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "No Git".to_string());

        let modified = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()
            .map(|output| {
                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|l| !l.is_empty())
                        .count()
                } else {
                    0
                }
            })
            .unwrap_or(0);

        (branch, modified)
    }

    pub fn request_git_info(&self) {
        let _ = self.sender.send(BridgeRequest::GetGitInfo);
    }

    pub fn try_recv(&self) -> Option<BridgeResponse> {
        self.receiver.try_recv().ok()
    }
}
