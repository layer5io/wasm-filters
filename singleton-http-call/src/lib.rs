use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::{SystemTime,Duration};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|root_context_id| -> Box<dyn RootContext> {
        Box::new(SingletonService::new())
    });
}

#[derive(Debug)]
struct SingletonService {}


impl SingletonService {
    fn new() -> Self {
        return Self {
        }
    }
}

impl Context for SingletonService {
    fn on_http_call_response(&mut self, _token_id: u32, _num_headers: usize, _body_size: usize, _num_trailers: usize) {
        if let Some(body) = self.get_http_call_response_body(0, _body_size) {
            if let Ok(body) = std::str::from_utf8(&body) {
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("HTTP Call Response : {:?}", body).as_str());
            }
        }
    }
}
impl RootContext for SingletonService {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        proxy_wasm::hostcalls::log(LogLevel::Debug, "VM instantiated");
        self.set_tick_period(Duration::from_secs(5));
        true
    }

    fn on_tick(&mut self) {
        let x = self.dispatch_http_call(
            "wasm_upstream",
            vec![
                (":method", "GET"),
                (":path", "/auth"),
                (":authority", "wasm_upstream"),
                ],
                None,
            vec![],
            Duration::from_secs(5),
        );
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("Request Status {:?}", x).as_str());
    }
}
