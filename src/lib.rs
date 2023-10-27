use std::mem::transmute;

use proxy_wasm::{
    traits::{Context, StreamContext},
    types::{Action, LogLevel, Bytes},
};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_stream_context(|context_id, _root_context_id| -> Box<dyn StreamContext> {

        proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: new filter for stream context {:?}", context_id).as_str());
        Box::new(MyFilter::new(context_id))
    });
}

#[derive(Debug)]
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


        proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: new connection {:?}", self).as_str());

        Action::Continue
    }

    fn on_upstream_data(&mut self, _data_size: usize, _end_of_stream: bool) -> Action {
        /*if let Some(data) = self.get_downstream_data(0, _data_size) {


        }*/


        let mtls = match proxy_wasm::hostcalls::get_property(vec!["connection", "mtls"]) {
            Ok(mtls_option) => {
                match  mtls_option.unwrap().pop().unwrap() {
                    0x01 => true,
                    _ => false,
                }
            },
            Err(error) => {
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: error {:?}", error).as_str());
                false
            }
        };

        proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: mtls {:?}", mtls).as_str());

        let destination_principal: Option<String> = None;

        let destination_principal = match proxy_wasm::hostcalls::get_property(vec!["connection", "subject_local_certificate"]) {
            Ok(prop_option) => {

                match prop_option {
                    Some(val_vec) => {
                        match String::from_utf8(val_vec) {
                            Ok(val) => val,
                            Err(error) => {
                                proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: error {:?}", error).as_str());
                                String::new()
                            }
                        }
                    }
                    None => String::from("No Option")
                }
            },
            Err(error) => {
                proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: error {:?}", error).as_str());
                String::from("Error")
            }
        };
        proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: destination_principal {:?}", destination_principal).as_str());



        proxy_wasm::hostcalls::log(LogLevel::Info, format!("RUST-WASM: upstream data {:?}", self).as_str());

        Action::Continue
    }
}
