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
    fn extract_args(&self, context: &Context) -> Vec<RuntimeValue> {
        let params = form_urlencoded::parse(context.query.unwrap().as_bytes());
        let mut vec = Vec::new();
        for p in params.into_iter() {
            vec.push(RuntimeValue::I32(p.1.parse::<i32>().unwrap()));
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
        result: Result<ActionOutcome, ActionError>,
    ) -> Response {
        let msg = match result.unwrap() {
            ActionOutcome::Returned { values } => format!(
                "module: {}, function: {}, returned {:#}",
                context.module_path, context.function_name, values[0]
            ),
            ActionOutcome::Trapped { message } => format!(
                "Trap from within function: {}", message),
        };
        let body = msg.to_string().into_bytes();
        return Response::new()
            .with_header(ContentLength(body.len() as u64))
            .with_body(body);
    }
}

fn main() {
    wasm_executor::start(|| Box::new(ReqHandler {}), || Box::new(ResHandler {}));
}
