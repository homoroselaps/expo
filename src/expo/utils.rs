use std::str::FromStr;

pub fn to_i64(s: &str) -> i64 {
    FromStr::from_str(s).unwrap()
}

pub fn vec_to_i64(v: Vec<char>) -> i64 {
    to_i64(v.into_iter().collect::<String>().as_ref())
}
