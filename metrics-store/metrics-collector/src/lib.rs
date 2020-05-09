use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::{SystemTime,Duration};
use serde::{Serialize,Deserialize};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_stream_context(|context_id, root_context_id| -> Box<dyn StreamContext> {
        Box::new(TCPMetrics::new())
    });
}

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
struct TCPMetrics {
    data_downstream: usize,
    data_upstream: usize,
    #[serde(skip_serializing)]
    time: SystemTime,
    latency: u128,
    qid: Option<u32>
}


impl TCPMetrics {
    fn new() -> Self {
        return Self {
            data_downstream : 0,
            data_upstream : 0,
            time: SystemTime::UNIX_EPOCH,
            latency: 0,
            qid: None
        }
    }
}

impl StreamContext for TCPMetrics {
    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        self.data_downstream += _data_size;
        Action::Continue
    }
    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        self.data_upstream += _data_size;
        Action::Continue
    }
    fn on_downstream_close(&mut self, _peer_type: PeerType) {
        if let Ok(curr_time) = proxy_wasm::hostcalls::get_current_time() {
            if let Ok(dur) = curr_time.duration_since(self.time) {
                self.latency = dur.as_micros()
            }
        }
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("{:?}", self).as_str());
        
        if self.qid == None {
            if let Ok(x) = proxy_wasm::hostcalls::resolve_shared_queue("singleton", "q1") {
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("Queue identified : {:?}", x).as_str());
                self.qid = x;

                let target: Option<TCPMetrics>  = Some(self.clone());
                let encoded: Vec<u8> = bincode::serialize(&target).unwrap();
                let res = proxy_wasm::hostcalls::enqueue_shared_queue(self.qid.unwrap(), Some(&encoded));

                proxy_wasm::hostcalls::log(LogLevel::Info, format!("Enqueue result : {:?}", res).as_str());
            }
        }
        
    }
    fn on_upstream_close(&mut self, _peer_type: PeerType) {
        if let Ok(curr_time) = proxy_wasm::hostcalls::get_current_time() {
            self.time = curr_time;
        }
    }
}

impl Context for TCPMetrics {}
impl RootContext for TCPMetrics {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        proxy_wasm::hostcalls::log(LogLevel::Debug, "FILTER VM instantiated");
        self.set_tick_period(Duration::from_secs(2));
        true
    }
    fn on_tick(&mut self) {
    }
}