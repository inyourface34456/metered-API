#![allow(non_snake_case)]
use warp::Filter;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::Rng;

#[derive(Clone)]
struct Ids {
    id_list: Arc<RwLock<Vec<u64>>>,
    usage_list: Arc<RwLock<HashMap<u64, HashMap<String, u32>>>>
}

impl Ids {
    fn new() -> Self {
        Self {
            id_list: Arc::new(RwLock::new(vec![])),
            usage_list: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    fn gen_new_id(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let correct_id: u64;
    
        loop {
            if let Ok(mut vec) = self.id_list.try_write() {
                loop {
                    let id: u64 = rng.gen();
                    if !vec.contains(&id) {
                        vec.push(id);
                        correct_id = id;
                        break
                    }

                    loop {
                        if let Ok(mut map) = self.usage_list.try_write() {
                            //if map.keys().collect::<Vec<_>>().contains()
                            break
                        }
                    }
                }
                break
            }
        }
        correct_id
    }
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let ids = Ids::new();
    // let ids_filter = warp::path("get_id").map(|| );

    let hello = warp::path("get_id").map(move || ids.gen_new_id().to_string());


    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}