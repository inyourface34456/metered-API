use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data<T> {
    pub authentication: u128,
    pub data: T,
}

#[derive(Deserialize)]
pub struct Data2<K, V> {
    pub authentication: u128,
    pub data1: K,
    pub data2: V,
}