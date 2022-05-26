use std::{pin::Pin, time::Duration};

use futures::Stream;
use juniper::{graphql_object, FieldError, graphql_subscription, graphql_value, RootNode};

use crate::models::Context;

//Query
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn test(int: i32) -> i32 {
        int
    }
}


//Mutation
pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn test(int: i32) -> i32 {
        int
    }
}

type I32Stream = Pin<Box<dyn Stream<Item = Result<i32, FieldError>> + Send>>;

//Subscription
pub struct Subscription;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn test() -> I32Stream {
        let mut counter = 0;
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        let stream = async_stream::stream! {
            counter += 1;
            loop {
                interval.tick().await;
                if counter == 2 {
                    yield Err(FieldError::new(
                        "some field error from handler",
                        graphql_value!("some additional string"),
                    ))
                } else {
                    yield Ok(1)
                }
            }
        };

        Box::pin(stream)
    }
}


// GraphQL schema
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}