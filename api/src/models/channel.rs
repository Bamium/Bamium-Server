use super::Context;

use serde::{Serialize, Deserialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Serialize, Deserialize)]
enum ChannelType {
    Text,
    Voice,
}

#[derive(Serialize, Deserialize)]
struct Channel {
    name: String,
    id: i32,
    channel_type: ChannelType,

}

#[juniper::graphql_object(Context = Context)]
impl Channel {
    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn channel_type(&self) -> ChannelType {
        self.channel_type
    }
}