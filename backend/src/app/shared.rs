#[derive(juniper::GraphQLInputObject)]
pub struct Credentials {
    username: String,
    password: String,
}
