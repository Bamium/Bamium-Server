use super::Context;

#[derive(juniper::GraphQLEnum, Clone, Copy)]
enum ChannelType {
    text,
    voice,
}

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