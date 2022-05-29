use actix_web::{web, post, Error, HttpResponse};

use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{api::schema::Query, models::{Context, Databases}};



pub async fn authenticate(dbs: &Databases, username: &String, email: &String, pubkey: &String) -> i32 {
    1
}