use serde::{Serialize, Deserialize};

use crate::models::role::Role;

use super::Context;

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    pubkey: String,
    status: Option<String>,
    bio: Option<String>,
    roles: Vec<Role>,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn status(&self) -> Option<&str> {
        self.status.as_ref().map(|s| &s[..])
    }

    fn bio(&self) -> Option<&str> {
        self.bio.as_ref().map(|s| &s[..])
    }

    fn roles(&self) -> &Vec<Role> {
        &self.roles
    }
}

