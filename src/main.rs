#![allow(non_snake_case)]
mod data;
mod endpoint;
mod endpoint_funcs;
mod ids;
mod role;
mod usage;
mod utils;

use endpoint::Endpoint;
use endpoint_funcs::*;
use ids::Ids;
use role::Role;
use usage::Usage;
use utils::*;
use warp::*;
use Filter;


#[tokio::main]
async fn main() {
    let ids = Ids::new();
    let ids_filter = any().map(move || ids.clone());

    let short_wait = post()
        .and(path("short_wait"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
        .and(ids_filter.clone())
        .and_then(short_wait_hit);

    let long_wait = post()
        .and(path("long_wait"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
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
        .and(header::<u128>("Authentication"))
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(add_to_list_hit);

    let echo = post()
        .and(path("echo"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(echo_hit);

    let next_allowed_request = post()
        .and(path("next_allowed_request"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(next_allowed_request_hit);

    let until_limit = post()
        .and(path("until_limit"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
        .and(json_arb_data())
        .and(ids_filter.clone())
        .and_then(until_limit_hit);

    let add_KV_pair = post()
        .and(path("add_KV_pair"))
        .and(path::end())
        .and(header::<u128>("Authentication"))
        .and(json_arb_data2())
        .and(ids_filter.clone())
        .and_then(add_KV_pair_hit);

    let routes = post().and(
        get_id
            .or(short_wait)
            .or(long_wait)
            .or(add_to_list)
            .or(echo)
            .or(next_allowed_request)
            .or(until_limit)
            .or(add_KV_pair)
    );

    serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
