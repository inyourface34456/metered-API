use crate::Ids;
use crate::Role;
use crate::Data;
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
    if arg_1 == Role::None {
        return Ok(reply::html(String::from("failed")));
    }

    Ok(reply::html(arg_2.gen_new_id(arg_1).to_string()))
}

pub async fn add_to_list_hit(data: Data<i32>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(data.authentication, "/add_to_list");

    if result.0 {
        match ids.api_3_data.write() {
            Ok(mut dat) => {
                dat.push(data.data);
                return Ok(reply::json(&dat.clone()));
            }
            Err(_) => return Ok(reply::json(&String::from("failed"))),
        }
    } else {
        return Ok(reply::json(&String::from("failed")));
    }
}

pub async fn echo_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(data.authentication, "/echo").0;

    if result {
        return Ok(reply::with_status(reply::json(&data.data), StatusCode::OK))
    }
    
    Ok(reply::with_status(reply::json(&"retelimated".to_string()), StatusCode::TOO_MANY_REQUESTS))
}

pub async fn next_allowed_request_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.time_until_next_allowed_hit(data.authentication, &data.data);

    if let Some(num) = result {
        return Ok(reply::with_status(format!("{} seconds", num), StatusCode::OK))
    }

    Ok(reply::with_status("failed".to_string(), StatusCode::FORBIDDEN))
}

pub async fn until_limit_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.num_hits_untill_timeout(data.authentication, &data.data);

    if let Some(num) = result {
        return Ok(reply::with_status(format!("{} requests left", num), StatusCode::OK))
    }

    Ok(reply::with_status("failed".to_string(), StatusCode::FORBIDDEN))
}