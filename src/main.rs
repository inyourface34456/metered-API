#![allow(non_snake_case)]
mod endpoint_funcs;
mod usage;
mod data;
mod role;
mod endpoint;
mod ids;
mod utils;

use usage::Usage;
use endpoint_funcs::*;
use warp::*;
use ids::Ids;
use data::Data;
use role::Role;
use utils::*;
use endpoint::Endpoint;
use Filter;

#[tokio::main]
async fn main() {
    let ids = Ids::new();
    let ids_filter = any().map(move || ids.clone());

    let short_wait = post()
        .and(path("short_wait"))
        .and(path::end())
        .and(json_body())
        .and(ids_filter.clone())
        .and_then(short_wait_hit);

    let long_wait = post()
        .and(path("long_wait"))
        .and(path::end())
        .and(json_body())
        .and(ids_filter.clone())
        .and_then(long_wait_hit);

    let get_id = post()
        .and(path("get_id"))
        .and(path::end())
        .and(role_json())
        .and(ids_filter.clone())
        .and_then(get_id);

    let add_to_list = post()
        .and(path("add_to_list"))
        .and(path::end())
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(add_to_list_hit);

    let echo = post()
        .and(path("echo"))
        .and(path::end())
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(echo_hit);

    let next_allowed_request = post()
        .and(path("next_allowed_request"))
        .and(path::end())
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(next_allowed_request_hit);

    let until_limit = post()
        .and(path("until_limit"))
        .and(path::end())
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(until_limit_hit);

    let routes = post()
    .and(
        get_id
        .or(short_wait)
        .or(long_wait)
        .or(add_to_list)
        .or(echo)
        .or(next_allowed_request)
        .or(until_limit)
    );

    serve(routes).run(([127, 0, 0, 1], 3030)).await;
}