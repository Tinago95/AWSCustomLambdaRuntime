use lambda_runtime::{Context, Error, service_fn, run, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[tokio::main]
async fn main () ->Result<(), Error>{
    let handler = service_fn(handler);
   run(handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    last_name: String

}

#[derive(Serialize)]
struct Output {
    message: String,
    request_id:String
}

async fn handler (event: LambdaEvent<Event>) -> Result<Output, Error> {
let message:String = format!("Gidday {} {}", event.payload.first_name, event.payload.last_name);
Ok(Output { message, request_id: event.context.request_id })
}