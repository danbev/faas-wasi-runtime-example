extern crate url;
extern crate wasm_executor;

use wasm_executor::{Context, RequestExtractor, ResponseHandler, WasmResponse};
use wasmtime::{Val, Trap};

struct ReqHandler {}

impl RequestExtractor for ReqHandler {
    fn extract_args(&self, context: &Context) -> Vec<Val> {
        println!("CloudEvent: {:?}", context);
        let mut vec = Vec::new();
        vec.push(Val::I32(4));
        vec.push(Val::I32(14));
        return vec;
    }
}

struct ResHandler {}
impl ResponseHandler for ResHandler {
    fn create_response(
        &self,
        context: &Context,
        result: Result<Box<[Val]>, Trap>,
    ) -> WasmResponse {
        let msg = match result {
            Ok(values) => format!(
                "module: {}, function: {}, returned {:?}",
                context.module_path, context.function_name, values[0]
            ),
            Err(e) => format!("Trap from within function: {}", e.message()),
        };
        println!("WASM Response: {:#?}", msg);
        let body = msg.to_string().into_bytes();
        WasmResponse { body, headers: None}
    }
}

fn main() {
    wasm_executor::start(|| Box::new(ReqHandler {}), || Box::new(ResHandler {}));
}
