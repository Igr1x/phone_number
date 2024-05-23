use pgrx::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

mod implementation;

#[derive(
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    PostgresType,
    Serialize,
    Deserialize,
    PostgresEq,
    PostgresHash,
)]
#[inoutfuncs]
pub struct PhoneNumber {
    country_code: u16,
    city_code: u16,
    first_code: u16,
    second_code: u16,
    third_code: u16,
}

#[pg_extern]
fn create_random_number() -> PhoneNumber {
    PhoneNumber {
        country_code: rand::thread_rng().gen_range(1..9),
        city_code: rand::thread_rng().gen_range(1..999),
        first_code: rand::thread_rng().gen_range(1..999),
        second_code: rand::thread_rng().gen_range(1..99),
        third_code: rand::thread_rng().gen_range(1..99),
    }
}
