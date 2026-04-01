use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::fs;

pub struct IntelligenceBridge {
    python_code: String,
}

impl IntelligenceBridge {
    pub fn new() -> Self {
        let python_code = fs::read_to_string("intelligence/senses.py")
            .unwrap_or_else(|_| "def calculate_indentation(ctx): return 0\ndef get_status_message(): return 'Bridge Offline'".to_string());
        
        Self { python_code }
    }

    pub fn calculate_indent(&self, buffer_context: &str) -> PyResult<usize> {
        Python::with_gil(|py| {
            let code = std::ffi::CString::new(self.python_code.as_str()).unwrap();
            let file_name = std::ffi::CString::new("senses.py").unwrap();
            let module_name = std::ffi::CString::new("senses").unwrap();
            
            let senses = PyModule::from_code(py, &code, &file_name, &module_name)?;
            let calculate_indentation = senses.getattr("calculate_indentation")?;
            let result: usize = calculate_indentation.call1((buffer_context,))?.extract()?;
            Ok(result)
        })
    }

    pub fn get_status_message(&self) -> String {
        Python::with_gil(|py| {
            let code = std::ffi::CString::new(self.python_code.as_str()).unwrap();
            let file_name = std::ffi::CString::new("senses.py").unwrap();
            let module_name = std::ffi::CString::new("senses").unwrap();
            
            let senses = match PyModule::from_code(py, &code, &file_name, &module_name) {
                Ok(m) => m,
                Err(_) => return "Python Error".to_string(),
            };
            
            let get_status_message = match senses.getattr("get_status_message") {
                Ok(f) => f,
                Err(_) => return "Status Func Missing".to_string(),
            };
            
            get_status_message.call0().and_then(|r| r.extract::<String>()).unwrap_or_else(|_| "Bridge Error".to_string())
        })
    }
}
