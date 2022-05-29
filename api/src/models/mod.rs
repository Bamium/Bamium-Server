use serde::{Serialize, Deserialize};

pub mod message;
pub mod user;
pub mod role;
pub mod channel;

#[derive(Clone)]
pub struct Databases {
    pub mongo: mongodb::Client,
    pub redis: redis::Client,
}

#[derive(Clone)]
pub struct Context {
    pub db: Databases,
    pub userid: Option<i32>,
}

impl juniper::Context for Context {}

// Permissions
#[derive(juniper::GraphQLEnum, Clone, Copy, Serialize, Deserialize)]
enum Permissions {
    Read,
    Write,
    Admin,
}

