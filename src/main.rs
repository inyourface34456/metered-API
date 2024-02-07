#![allow(non_snake_case)]
use rand::Rng;
use serde::Deserialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use warp::Filter;

#[derive(Hash, Eq, PartialEq)]
struct Usage {
    num_times_used: u32,
    next_use_allowed: u128,
    endpoint: Endpoint,
}

impl Usage {
    fn new(name: String, req_before_cooldown: u16, lim_time: u16, cooldown_time: u16) -> Self {
        Self {
            num_times_used: 0,
            next_use_allowed: 0,
            endpoint: Endpoint::new(name, req_before_cooldown, lim_time, cooldown_time),
        }
    }
}

impl From<Role> for String {
    fn from(data: Role) -> String {
        match data {
            Role::Standered => String::from("Standard"),
            Role::Admin => String::from("Admin"),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
enum Role {
    Standered,
    Admin,
}

#[derive(Clone)]
struct Ids {
    id_list: Arc<RwLock<HashMap<u128, Role>>>,
    usage_list: Arc<RwLock<HashMap<u128, HashMap<String, Usage>>>>,
}

#[derive(Hash, Eq, PartialEq)]
struct Endpoint {
    name: String,
    req_before_cooldown: u16,
    lim_time: u16,
    cooldown_time: u16,
}

impl Endpoint {
    const fn new(
        name: String,
        req_before_cooldown: u16,
        lim_time: u16,
        cooldown_time: u16,
    ) -> Self {
        Self {
            name,
            req_before_cooldown,
            lim_time,
            cooldown_time,
        }
    }
}

impl Ids {
    // you can make A number of hits on ENDPOINT in B time, if you go over, you have to wait C mins
    const API_LIMITS: [(&'static str, u16, u16, u16); 2] =
        [("/api_1", 10, 1, 1), ("/api_2", 1, 10, 100)];

    fn new() -> Self {
        Self {
            id_list: Arc::new(RwLock::new(HashMap::new())),
            usage_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn gen_start_hashmap(role: Role) -> HashMap<String, Usage> {
        let mut map = HashMap::new();

        match role {
            Role::Standered => {
                for i in Self::API_LIMITS {
                    map.insert(
                        String::from(i.0),
                        Usage::new(String::from(i.0), i.1, i.2, i.3),
                    );
                }
            }
            Role::Admin => {
                for i in Self::API_LIMITS {
                    map.insert(
                        String::from(i.0),
                        Usage::new(String::from(i.0), u16::MAX, 0, 0),
                    );
                }
            }
        }

        map
    }

    fn gen_new_id(&self, role: Role) -> u128 {
        let mut rng = rand::thread_rng();
        let mut correct_id: u128 = 0;

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
        let mut role: Role = Role::Standered;

        match self.id_list.read() {
            Ok(map) => match map.get(&user) {
                Some(dat) => role = *dat,
                None => unreachable!(),
            },
            Err(_) => {}
        }

        match self.usage_list.write() {
            Ok(mut map) => match map.get_mut(&user) {
                Some(dat) => match dat.get_mut(endpoint) {
                    Some(dat) => {
                        let current_time = get_unix_epoch();

                        if dat.num_times_used >= dat.endpoint.req_before_cooldown.into() {
                            dat.next_use_allowed = current_time + dat.next_use_allowed * 60 * 1000;
                            allowed = false;
                            dat.num_times_used = 0;
                        } else if dat.next_use_allowed > current_time {
                            allowed = false;
                        } else {
                            allowed = true;
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
}

async fn api_1_hit(user: u128, arg_2: Ids) -> Result<impl warp::Reply, warp::Rejection> {
    let result = arg_2.register_hit(user, "/api_1");

    if result.0 {
        Ok(warp::reply::json(&String::from(result.1)))
    } else {
        Ok(warp::reply::json(&String::from("failed")))
    }
}

async fn api_2_hit(user: u128, arg_2: Ids) -> Result<impl warp::Reply, warp::Rejection> {
    let result = arg_2.register_hit(user, "/api_2");

    if result.0 {
        Ok(warp::reply::json(&String::from(result.1)))
    } else {
        Ok(warp::reply::json(&String::from("failed")))
    }
}

async fn get_id(arg_1: Role, arg_2: Ids) -> Result<warp::reply::Html<String>, warp::Rejection> {
    Ok(warp::reply::html(arg_2.gen_new_id(arg_1).to_string()))
}

#[tokio::main]
async fn main() {
    let ids = Ids::new();
    let ids_filter = warp::any().map(move || ids.clone());

    let api_1 = warp::post()
        .and(warp::path("api_1"))
        .and(warp::path::end())
        .and(json_body())
        .and(ids_filter.clone())
        .and_then(api_1_hit);

    let api_2 = warp::post()
        .and(warp::path("api_2"))
        .and(warp::path::end())
        .and(json_body())
        .and(ids_filter.clone())
        .and_then(api_2_hit);

    let hello_get = warp::post()
        .and(warp::path("get_id"))
        .and(warp::path::end())
        .and(role_json())
        .and(ids_filter.clone())
        .and_then(get_id);

    let routes = warp::post().and(hello_get.or(api_1).or(api_2));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn get_unix_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn json_body() -> impl Filter<Extract = (u128,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn role_json() -> impl Filter<Extract = (Role,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
