extern crate url;
extern crate wasm_executor;

use hyper::header::ContentLength;
use hyper::server::Response;
use url::form_urlencoded;
use wasmtime_jit::{ActionError, ActionOutcome, RuntimeValue};

use wasm_executor::Context;
use wasm_executor::RequestExtractor;
use wasm_executor::ResponseHandler;

struct ReqHandler {}

impl RequestExtractor for ReqHandler {
    fn extract_args(&self, context: Context) -> Vec<RuntimeValue> {
        let params = form_urlencoded::parse(context.query.unwrap().as_bytes());
        let mut vec = Vec::new();
        for p in params.into_iter() {
            vec.push(RuntimeValue::I32(p.1.parse::<i32>().unwrap()));
        }
        return vec;
    }
}

struct ResHandler {}
impl ResponseHandler for ResHandler {
    fn create_response(
        &self,
        result: Result<ActionOutcome, ActionError>,
        module_path: &str,
        function_name: &str,
    ) -> Response {
        let body = match result.unwrap() {
            ActionOutcome::Returned { values } => format!(
                "module: {}, function: {}, returned {:#}",
                module_path, function_name, values[0]
            )
            .to_string()
            .into_bytes(),
            ActionOutcome::Trapped { message } => format!("Trap from within function: {}", message)
                .to_string()
                .into_bytes(),
        };
        return Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body);
    }
}

fn main() {
    wasm_executor::start(|| Box::new(ReqHandler {}), || Box::new(ResHandler {}));
}
