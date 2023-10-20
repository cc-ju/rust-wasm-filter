use log::info;
use proxy_wasm::{
    traits::{Context, StreamContext},
    types::Action,
};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_stream_context(|context_id, root_context_id| -> Box<dyn StreamContext> {
        
        //let prop = proxy_wasm::hostcalls::get_property(vec!["connection", "mtl"]);
        info!("RUST-WASM: new filter for stream content {}", context_id);
        Box::new(MyFilter::new(context_id))
    });
}

struct MyFilter {
    context_id: u32,
}

impl MyFilter {
    fn new(context_id: u32) -> Self {
        return Self {
            context_id,
        };
    }
}

impl Context for MyFilter {}

/*impl HttpContext for MyFilter {
    fn on_http_request_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> proxy_wasm::types::Action {
        for (name, value) in &self.get_http_request_headers() {
            trace!("In WASM : #{} -> {}: {}", self.context_id, name, value);
        }

        match self.get_http_request_header("secret") {
            Some(secret) if !secret.is_empty() => {
                self.resume_http_request();
                Action::Continue
            }
            _ => {
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden.\n"),
                );
                Action::Pause
            }
        }
    }
}*/

impl StreamContext for MyFilter {
    fn on_new_connection(&mut self) -> Action {
        /*if let Ok(curr_time) = proxy_wasm::hostcalls::get_current_time() {
            
        }
        if let Ok(mtls) = proxy_wasm::hostcalls::get_property(vec!["connection", "mtls"]) {
            
        }*/
        
        info!("RUST-WASM: new connection");
        
        Action::Continue
    }
    
    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        /*if let Some(data) = self.get_downstream_data(0, _data_size) {
            

        }*/
        info!("RUST-WASM: upstream data");

        Action::Continue
    }
}
