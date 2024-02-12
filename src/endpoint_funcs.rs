use crate::data::Data2;
use crate::{Ids, Role};
use http::StatusCode;
use warp::*;

pub async fn short_wait_hit(user: u128, arg_2: Ids) -> Result<impl Reply, Rejection> {
    let result = arg_2.register_hit(user, "/short_wait");

    if result.0 {
        Ok(reply::json(&String::from(result.1)))
    } else {
        Ok(reply::json(&String::from("failed")))
    }
}

pub async fn long_wait_hit(user: u128, arg_2: Ids) -> Result<impl Reply, Rejection> {
    let result = arg_2.register_hit(user, "/long_wait");

    if result.0 {
        Ok(reply::json(&String::from(result.1)))
    } else {
        Ok(reply::json(&String::from("failed")))
    }
}

pub async fn get_id(arg_1: Role, arg_2: Ids) -> Result<reply::Html<String>, Rejection> {
    if let Some(id) = arg_2.gen_new_id(arg_1) {
        Ok(reply::html(id.to_string()))
    } else {
        Ok(reply::html("failed".to_string()))
    }
}

pub async fn add_to_list_hit(user:u128, data: i32, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(user, "/add_to_list");

    if result.0 {
        match ids.api_3_data.write() {
            Ok(mut dat) => {
                dat.push(data);
                return Ok(reply::json(&dat.clone()));
            }
            Err(_) => return Ok(reply::json(&String::from("failed"))),
        }
    } else {
        return Ok(reply::json(&String::from("failed")));
    }
}

pub async fn echo_hit(user: u128, data: String, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(user, "/echo").0;

    if result {
        return Ok(reply::with_status(reply::json(&data), StatusCode::OK));
    }

    Ok(reply::with_status(
        reply::json(&"retelimated".to_string()),
        StatusCode::TOO_MANY_REQUESTS,
    ))
}

pub async fn next_allowed_request_hit(
    user: u128,
    data: String,
    ids: Ids,
) -> Result<impl Reply, Rejection> {
    let result = ids.time_until_next_allowed_hit(user, &data);

    if let Some(num) = result {
        return Ok(reply::with_status(
            format!("{} seconds", num),
            StatusCode::OK,
        ));
    }

    Ok(reply::with_status(
        "failed".to_string(),
        StatusCode::FORBIDDEN,
    ))
}

pub async fn until_limit_hit(user: u128, data: String, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.num_hits_untill_timeout(user, &data);

    if let Some(num) = result {
        return Ok(reply::with_status(
            format!("{} requests left", num),
            StatusCode::OK,
        ));
    }

    Ok(reply::with_status(
        "failed".to_string(),
        StatusCode::FORBIDDEN,
    ))
}

pub async fn add_KV_pair_hit(
    user: u128,
    data: Data2<String, String>,
    ids: Ids,
) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(user, "/add_KV_pair").0;

    if result {
        match ids.str_dict_data.write() {
            Ok(mut map) => match map.insert(data.data1, data.data2) {
                Some(_) => Ok(reply::with_status(
                    reply::json(&map.clone()),
                    StatusCode::OK,
                )),
                None => Ok(reply::with_status(
                    reply::json(&map.clone()),
                    StatusCode::OK,
                )),
            },
            Err(_) => Ok(reply::with_status(
                reply::json(&"try again".to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    } else {
        Ok(reply::with_status(
            reply::json(&"retelimated".to_string()),
            StatusCode::TOO_MANY_REQUESTS,
        ))
    }
}