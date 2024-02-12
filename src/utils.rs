use crate::{Data, Role};
use crate::data::Data2;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use warp::*;

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

pub fn json_arb_data2<K: std::marker::Send + for<'de> Deserialize<'de>, V: std::marker::Send + for<'de> Deserialize<'de>>() -> impl Filter<Extract = (Data2<K, V>,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json::<Data2<K, V>>())
}