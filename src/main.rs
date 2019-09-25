#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::server::server;

mod config;
mod errors;
pub mod handlers;
mod helpers;
mod models;
mod routes;
mod server;
mod validate;

fn main() -> std::io::Result<()> {
    server()
}
