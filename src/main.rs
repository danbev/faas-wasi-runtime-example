extern crate url;
extern crate wasm_executor;

use url::form_urlencoded;
use wasm_executor::{Context, RequestExtractor, ResponseHandler, WasmResponse};
use wasmtime::{Val, Trap};

struct ReqHandler {}

impl RequestExtractor for ReqHandler {
    fn extract_args(&self, context: &Context) -> Vec<Val> {
        let params = form_urlencoded::parse(context.query.unwrap().as_bytes());
        let mut vec = Vec::new();
        for p in params.into_iter() {
            vec.push(Val::I32(p.1.parse::<i32>().unwrap()));
        }
        println!("Extracted args for {}: {:?}", context.function_name, vec);
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
        let body = msg.to_string().into_bytes();
        WasmResponse { body, headers: None}
    }
}

fn main() {
    wasm_executor::start(|| Box::new(ReqHandler {}), || Box::new(ResHandler {}));
}
