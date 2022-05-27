//mod message;
//mod user;
//mod role;
//mod channel;

#[derive(Clone)]
pub struct Databases {
    pub db: mongodb::Client,
}

#[derive(Clone)]
pub struct Context {
    pub db: Databases,
    pub userid: Option<i32>,
}

impl juniper::Context for Context {}

// Permissions
#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum Permissions {
    Read,
    Write,
    Admin,
}

