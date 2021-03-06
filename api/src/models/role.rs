use crate::models::Permissions;

use serde::{Serialize, Deserialize};

use super::Context;

#[derive(Serialize, Deserialize)]
pub struct Role {
    id: i32,
    name: String,
    color: String,
    permissions: Permissions,
}

#[juniper::graphql_object(Context = Context)]
impl Role {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self) -> &str {
        &self.color
    }

    fn permissions(&self) -> Permissions {
        self.permissions
    }
}