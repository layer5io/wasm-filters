use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::{SystemTime,Duration};
use serde::{Deserialize};

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

#[derive(Debug,Deserialize)]
struct TCPMetrics {
    data_downstream: usize,
    data_upstream: usize,
    latency: u128,
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
        self.set_tick_period(Duration::from_secs(2));
        if let Ok(qid) = proxy_wasm::hostcalls::register_shared_queue(QNAME) {
            self.qid = Some(qid);
        }
        true
    }

    fn on_tick(&mut self) {
        if let Ok(res) = proxy_wasm::hostcalls::dequeue_shared_queue(self.qid.unwrap()) {
            if let Some(bytes) = res {
                let pkt: Option<TCPMetrics> = bincode::deserialize(&bytes).unwrap();
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("Packet Recieved : {:?}", pkt).as_str());
                let pkt = pkt.unwrap();
                let body = format!("{} | {} | {}", pkt.data_downstream, pkt.data_upstream, pkt.latency);
                let body = Some(body.as_bytes());
                let x = self.dispatch_http_call(
                    "wasm_upstream",
                    vec![
                        (":method", "GET"),
                        (":path", "/store"),
                        (":authority", "wasm_upstream"),
                        ],
                        body,
                    vec![],
                    Duration::from_secs(5),
                );
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("Send stat upstream : {:?}", x).as_str());
            }
        }
    }
}
