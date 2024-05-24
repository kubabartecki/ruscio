use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::hashes;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = hashes)]
pub struct Hash {
    pub id: i32,
    pub time_of_computing: i32,
    pub number_of_digits: i32,
}

#[derive(Insertable)]
#[diesel(table_name = hashes)]
pub struct NewHash {
    pub time_of_computing: i32,
    pub number_of_digits: i32,
}

#[derive(Deserialize)]
pub struct HashRequest {
    pub num_digits: usize,
}
