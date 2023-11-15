use lambda_runtime::{service_fn,Error,LambdaEvent};
use serde::{Deserialize,Serialize};
use serde_json::{json, Value};
use std::fs::File;


///mod add;

/// project strtucture from https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/basic-error-handling/src/main.rs
/// check dynamo impl
#[derive(Deserialize)]
struct Request {
    event_type: EventType,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[derive(Deserialize, Eq, PartialEq)]
enum EventType {
    Response,
    ExternalError,
    SimpleError,
    CustomError,
    Panic,
}

#[derive(Debug, Serialize)]
struct CustomError {
    is_authenticated: bool,
    req_id: String,
    msg: String,
}

impl std::error::Error for CustomError {

}

/// WTF is that!!!
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = json!(self).to_string();
        write!(f, "{}", err_as_json)
    }

}


//use self::{
//    routes::add
//};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true)
        .init();

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    println!("Hello, world!");
    Ok(())
}

pub(crate) async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, ctx) = event.into_parts();

    match  serde_json::from_value::<Request>(event)?.event_type {
        EventType::SimpleError =>{
            return Err(Box::new(simple_error::SimpleError::new("basic error as request"))); // bastante truculenta la forma de declarar errores
        }
        EventType::CustomError => {
            let cust_err = CustomError{
                is_authenticated: ctx.identity.is_some(),
                req_id: ctx.request_id,
                msg: "eres un soperutano".into()
            };
        return Err(Box::new(cust_err));
        }
        EventType::ExternalError => {
            let _file = File::open("lo_que_sea_archivo.txt");

            unreachable!();
        }

        EventType::Panic => {
            panic!();
        }

        EventType::Response => {
            let resp = Response {
                req_id: ctx.request_id,
                msg: "Ok".into()
            };
            return Ok(json!(resp));
        }
        
    }
}
