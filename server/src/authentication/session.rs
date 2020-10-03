use cookie::{Cookie, CookieJar, Key};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::{Seeder, SipHasher};
use std::env;

pub fn session(user_id: String) -> String {
    let mut jar = CookieJar::new();

    let secret_key_base = env::var("SECRET_KEY_BASE").unwrap();
    let mut rng: Pcg64 = Seeder::from(secret_key_base).make_rng();
    let mut master_key = [0u8; 32];
    rng.fill_bytes(&mut master_key);
    let key = Key::derive_from(&master_key);

    let new_cookie = Cookie::build("user_id", user_id)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    jar.private(&key).add(new_cookie);
    let cookie_header: Vec<String> = jar.iter().map(|cookie| cookie.to_string()).collect();

    cookie_header.join("; ")
}
