#![allow(non_snake_case)]
use rand::Rng;
use serde::Deserialize;
use http::StatusCode;
use warp::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use Filter;

#[derive(Hash, Eq, PartialEq)]
struct Usage {
    num_times_used: u32,
    next_use_allowed: u64,
    allowed: bool,
    endpoint: Endpoint,
}

impl Usage {
    fn new(name: String, req_before_cooldown: u16, cooldown_time: u16) -> Self {
        Self {
            num_times_used: 0,
            next_use_allowed: 0,
            allowed: true,
            endpoint: Endpoint::new(name, req_before_cooldown, cooldown_time),
        }
    }
}

#[derive(Deserialize)]
struct Data<T: std::marker::Send> {
    authentication: u128,
    data: T,
}

impl From<Role> for String {
    fn from(data: Role) -> String {
        match data {
            Role::Standered => String::from("Standard"),
            Role::Admin => String::from("Admin"),
            Role::None => String::from("None"),
        }
    }
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq)]
enum Role {
    Standered,
    Admin,
    None,
}

#[derive(Hash, Eq, PartialEq)]
struct Endpoint {
    name: String,
    req_before_cooldown: u16,
    cooldown_time: u16,
}

impl Endpoint {
    const fn new(
        name: String,
        req_before_cooldown: u16,
        cooldown_time: u16,
    ) -> Self {
        Self {
            name,
            req_before_cooldown,
            cooldown_time,
        }
    }
}

#[derive(Clone)]
struct Ids {
    id_list: Arc<RwLock<HashMap<u128, Role>>>,
    usage_list: Arc<RwLock<HashMap<u128, HashMap<String, Usage>>>>,
    api_3_data: Arc<RwLock<Vec<i32>>>,
}

impl Ids {
    // You can make A hits until you have to wait B mins
    const API_LIMITS: [(&'static str, u16, u16); 4] = [
        ("/short_wait", 10, 1),
        ("/long_wait", 1, 100),
        ("/add_to_list", 60, 1),
        ("/echo", 60, 1),
    ];

    fn new() -> Self {
        Self {
            id_list: Arc::new(RwLock::new(HashMap::new())),
            usage_list: Arc::new(RwLock::new(HashMap::new())),
            api_3_data: Arc::new(RwLock::new(vec![])),
        }
    }

    fn gen_start_hashmap(role: Role) -> HashMap<String, Usage> {
        let mut map = HashMap::new();

        match role {
            Role::Standered => {
                for i in Self::API_LIMITS {
                    map.insert(
                        String::from(i.0),
                        Usage::new(String::from(i.0), i.1, i.2),
                    );
                }
            }
            Role::Admin => {
                for i in Self::API_LIMITS {
                    map.insert(
                        String::from(i.0),
                        Usage::new(String::from(i.0), u16::MAX, 0,),
                    );
                }
            }
            Role::None => {}
        }

        map
    }

    fn gen_new_id(&self, role: Role) -> u128 {
        let mut rng = rand::thread_rng();
        let mut correct_id: u128 = 0;

        if role == Role::None {
            return 0;
        }

        match self.id_list.write() {
            Ok(mut map) => {
                loop {
                    let id: u128 = rng.gen();
                    if let Entry::Vacant(e) = map.entry(id) {
                        e.insert(role);
                        correct_id = id;
                        break;
                    }
                }

                match self.usage_list.write() {
                    Ok(mut map) => {
                        map.insert(correct_id, Self::gen_start_hashmap(role));
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
        correct_id
    }

    fn register_hit(&self, user: u128, endpoint: &str) -> (bool, Role) {
        let mut allowed: bool = true;
        let mut role: Role = Role::None;

        match self.id_list.read() {
            Ok(map) => match map.get(&user) {
                Some(dat) => role = *dat,
                None => {}
            },
            Err(_) => {}
        }

        match self.usage_list.write() {
            Ok(mut map) => match map.get_mut(&user) {
                Some(dat) => match dat.get_mut(endpoint) {
                    Some(dat) => {
                        let current_time = get_unix_epoch();

                        if dat.next_use_allowed > current_time {
                            allowed = false;
                            dat.allowed = false;
                        } else if dat.num_times_used >= dat.endpoint.req_before_cooldown.into() {
                            dat.next_use_allowed = current_time + <u16 as Into<u64>>::into(dat.endpoint.cooldown_time * 60);
                            allowed = false;
                            dat.allowed = false;
                            dat.num_times_used = 0;
                        } else {
                            allowed = true;
                            dat.allowed = true;
                            dat.num_times_used += 1;
                        }
                    }
                    None => allowed = false,
                },
                None => allowed = false,
            },
            Err(_) => {}
        }

        (allowed, role)
    }

    fn time_until_next_allowed_hit(&self, user: u128, endpoint: &str) -> Option<u64> {
        match self.usage_list.read() {
            Ok(dat) => {
                match dat.get(&user) {
                    Some(dat) => {
                        match dat.get(&endpoint.to_string()) {
                            Some(dat) => {
                                let current_time = get_unix_epoch();

                                if dat.next_use_allowed > current_time {
                                    Some(dat.next_use_allowed-current_time)
                                } else {
                                    Some(0)
                                }
                            }
                            None => None
                        }
                    }
                    None => None
                }
            },
            Err(_) => None
        }
    }

    fn num_hits_untill_timeout(&self, user: u128, endpoint: &str) -> Option<u32> {
        match self.usage_list.read() {
            Ok(dat) => {
                match dat.get(&user) {
                    Some(dat) => {
                        match dat.get(&endpoint.to_string()) {
                            Some(dat) => {
                                if dat.allowed {
                                    Some(dat.endpoint.req_before_cooldown as u32 - dat.num_times_used)
                                } else {
                                    Some(0)
                                }
                            }
                            None => None
                        }
                    }
                    None => None
                }
            },
            Err(_) => None
        }
    }
}

async fn short_wait_hit(user: u128, arg_2: Ids) -> Result<impl Reply, Rejection> {
    let result = arg_2.register_hit(user, "/short_wait");

    if result.0 {
        Ok(reply::json(&String::from(result.1)))
    } else {
        Ok(reply::json(&String::from("failed")))
    }
}

async fn long_wait_hit(user: u128, arg_2: Ids) -> Result<impl Reply, Rejection> {
    let result = arg_2.register_hit(user, "/long_wait");

    if result.0 {
        Ok(reply::json(&String::from(result.1)))
    } else {
        Ok(reply::json(&String::from("failed")))
    }
}

async fn get_id(arg_1: Role, arg_2: Ids) -> Result<reply::Html<String>, Rejection> {
    if arg_1 == Role::None {
        return Ok(reply::html(String::from("failed")));
    }

    Ok(reply::html(arg_2.gen_new_id(arg_1).to_string()))
}

async fn add_to_list_hit(data: Data<i32>, ids: Ids) -> Result<impl Reply, Rejection> {
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

async fn echo_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.register_hit(data.authentication, "/echo").0;

    if result {
        return Ok(reply::with_status(reply::json(&data.data), StatusCode::OK))
    } else {
        return Ok(reply::with_status(reply::json(&"retelimated".to_string()), StatusCode::TOO_MANY_REQUESTS))
    }
}

async fn next_allowed_request_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.time_until_next_allowed_hit(data.authentication, &data.data);

    if let Some(num) = result {
        return Ok(reply::with_status(format!("{} seconds", num), StatusCode::OK))
    } else {
        return Ok(reply::with_status("failed".to_string(), StatusCode::FORBIDDEN))
    }
}

async fn until_limit_hit(data: Data<String>, ids: Ids) -> Result<impl Reply, Rejection> {
    let result = ids.num_hits_untill_timeout(data.authentication, &data.data);

    if let Some(num) = result {
        return Ok(reply::with_status(format!("{} requests left", num), StatusCode::OK))
    } else {
        return Ok(reply::with_status("failed".to_string(), StatusCode::FORBIDDEN))
    }
}

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

fn get_unix_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn json_body() -> impl Filter<Extract = (u128,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

fn json_arb_data<T: std::marker::Send + for<'de> Deserialize<'de>>(
) -> impl Filter<Extract = (Data<T>,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

fn role_json() -> impl Filter<Extract = (Role,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}
