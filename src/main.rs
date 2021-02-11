extern crate dotenv;
use dotenv::dotenv;

use std::env;

fn main() {

    dotenv().ok();

    let dep = env::var("DEPLOY").unwrap_or(String::from("development"));

    match dep.as_ref() {
        "development" => println!("Deploying on development"),
        "production"  => println!("Deploying on production"),
        _             => return,
    }
}
