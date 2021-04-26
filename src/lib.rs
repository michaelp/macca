extern crate wapc_guest as guest;
use wasmcloud_actor_core as actorcore;
use wasmcloud_actor_http_server as http;


use serde_json::json;

use guest::prelude::*;

#[actorcore::init]
pub fn init() {
  http::Handlers::register_handle_request(say_hello);
}

fn say_hello(_req: http::Request) -> HandlerResult<http::Response> {
  
  let result =json!({
    "field1": 1,
    "field2": 1.1,
    "data": [
      _req.query_string
    ]
});
  Ok(http::Response::json(&result, 200, "OK"))
}
