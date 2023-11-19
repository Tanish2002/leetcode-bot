pub mod submissions;
mod user;

use reqwest_graphql::Client;

const API_ENDPOINT: &str = "https://leetcode.com/graphql";

pub const LIMIT: &str = "15";

pub struct Leetcode<'a> {
    gql_client: Client<'a>,
}

impl Leetcode<'_> {
    pub fn new() -> Self {
        let client = Client::new(API_ENDPOINT);
        Self { gql_client: client }
    }
}
