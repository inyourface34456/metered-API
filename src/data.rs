use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct Data2<K, V> {
    pub data1: K,
    pub data2: V,
}
