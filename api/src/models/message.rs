use super::Context;



struct Message {
    id: i32,
    content: String,
    sender_id: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Message {
    fn id(&self) -> i32 {
        self.id
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn sender_id(&self) -> i32 {
        self.sender_id
    }
}