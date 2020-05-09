use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_http_context(|context_id, root_context_id| -> Box<dyn HttpContext> {
        Box::new(UpstreamCall::new())
    });
}

#[derive(Debug)]
struct UpstreamCall {
}

impl UpstreamCall {
    fn new() -> Self {
        return Self {
        }
    }
}

impl HttpContext for UpstreamCall {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        let token = self.get_http_request_header("token").unwrap_or(String::from(""));
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("Auth header : {:?}", token).as_str());
        let x = self.dispatch_http_call(
            "wasm_upstream",
            vec![
                (":method", "GET"),
                (":path", "/auth"),
                (":authority", "wasm_upstream"),
                ("token", token.as_str()),
                ],
                None,
            vec![],
            Duration::from_secs(5),
        );
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("{:?}", x).as_str());
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        Action::Continue
    }
}

impl Context for UpstreamCall {
    fn on_http_call_response(&mut self, _token_id: u32, _num_headers: usize, _body_size: usize, _num_trailers: usize) {
        if let Some(body) = self.get_http_call_response_body(0, _body_size) {
            if let Ok(body) = std::str::from_utf8(&body) {
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("HTTP Call body : {:?} {:?}", body, body == "Authorized").as_str());
                if body == "Authorized" {
                    self.resume_http_request();
                    return;
                }
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden.\n"),
                );
            }
        }
    }
}
impl RootContext for UpstreamCall {}