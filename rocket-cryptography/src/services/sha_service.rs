use sha2::{Sha256, Digest};
use crate::models::{ShaRequest, ShaResponse};

use diesel::prelude::*;
use std::time::Instant;
use crate::config::database::establish_connection;
use crate::models::hash::{Hash, NewHash};
use crate::schema::hashes::dsl::*;

pub struct ShaService;

impl ShaService {
    pub fn hash(input: ShaRequest) -> ShaResponse {
        let mut hasher = Sha256::new();
        hasher.update(input.input.as_bytes());
        let result = hasher.finalize();
        ShaResponse {
            hash: format!("{:x}", result),
        }
    }

    pub fn find_hex_digits(num_digits: usize) -> i32 {
        let max_digits = 7;
        let num_digits = if num_digits > max_digits { max_digits } else { num_digits };

        let mut conn = establish_connection();
        
        let mut hasher = Sha256::new();
        let target = "0".repeat(num_digits);

        let start_time = Instant::now();
        let mut nonce = 0;

        loop {
            hasher.update(nonce.to_string().as_bytes());
            let result = hasher.finalize_reset();
            let hex_result = format!("{:x}", result);
            
            if hex_result.starts_with(&target) {
                break;
            }

            nonce += 1;
        }

        let duration = start_time.elapsed().as_millis() as i32;

        let new_hash = NewHash {
            time_of_computing: duration,
            number_of_digits: num_digits as i32,
        };

        diesel::insert_into(hashes)
            .values(&new_hash)
            .execute(&mut conn)
            .expect("Error saving new hash");

        duration
    }

    pub fn get_hashes_by_digits(digits: i32) -> Vec<Hash> {
        let mut conn = establish_connection();
        hashes.filter(number_of_digits.eq(digits))
              .order(time_of_computing.asc())
              .load::<Hash>(&mut conn)
              .expect("Error loading hashes")
    }
}
