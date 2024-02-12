use std::time::{UNIX_EPOCH, SystemTime};
use warp::*;
use crate::{Role, Data};
use serde::Deserialize;

pub fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn json_body() -> impl Filter<Extract = (u128,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub fn json_arb_data<T: std::marker::Send + for<'de> Deserialize<'de>>(
) -> impl Filter<Extract = (Data<T>,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub fn role_json() -> impl Filter<Extract = (Role,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}
