use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::{SystemTime,Duration};

const QNAME: &str = "q1";

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|root_context_id| -> Box<dyn RootContext> {
        Box::new(SingletonService::new())
    });
}

#[derive(Debug)]
struct SingletonService {
    qid: Option<u32>
}


impl SingletonService {
    fn new() -> Self {
        return Self {
            qid: None
        }
    }
}

impl Context for SingletonService {
    
}

impl RootContext for SingletonService {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        proxy_wasm::hostcalls::log(LogLevel::Debug, "VM instantiated");
        self.set_tick_period(Duration::from_secs(5));
        if let Ok(qid) = proxy_wasm::hostcalls::register_shared_queue(QNAME) {
            self.qid = Some(qid);
        }
        true
    }

    fn on_tick(&mut self) {
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("Request Status").as_str());
    }
}
