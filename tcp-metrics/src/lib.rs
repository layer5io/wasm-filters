use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_stream_context(|_, _| -> Box<dyn StreamContext> {
        Box::new(TCPMetrics {  })
    });
}

struct TCPMetrics {
}

impl Context for TCPMetrics {}
impl RootContext for TCPMetrics {}

impl StreamContext for TCPMetrics {
    fn on_downstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        let s: String = format!("size : {}, eos: {}", _data_size, _end_of_stream).to_owned();
        let s_slice: &str = &s[..];
        proxy_wasm::hostcalls::log(LogLevel::Debug, s_slice);
        Action::Continue
    }
}