use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::{SystemTime,Duration};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_stream_context(|context_id, root_context_id| -> Box<dyn StreamContext> {
        Box::new(TCPMetrics::new())
    });
}


struct TCPMetrics {
}

impl Context for TCPMetrics {
}

impl TCPMetrics {
    fn new() -> Self {
        return Self {
        }
    }
}

impl StreamContext for TCPMetrics {
    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        if let Some(data) = self.get_downstream_data(0, _data_size) {
            if let Ok(packet) = std::str::from_utf8(&data) {
                proxy_wasm::hostcalls::log(LogLevel::Debug, packet);
            }
        }
        Action::Continue
    }
}

impl RootContext for TCPMetrics {}
