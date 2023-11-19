use super::{Leetcode, LIMIT};
use reqwest_graphql::GraphQLError;
use serde::{Deserialize, Serialize};

const USER_REQUEST: &str = r#"
query getUserProfile($username: String!) {
  matchedUser(username: $username) {
    username
    profile {
      userAvatar
    }
  }
}
"#;

#[derive(Debug, Deserialize)]
pub struct Data {
    #[serde(rename = "matchedUser")]
    pub matched_user: MatchedUser,
}

#[derive(Debug, Deserialize)]
pub struct MatchedUser {
    pub username: String,
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    #[serde(rename = "userAvatar")]
    pub user_avatar: String,
}

#[derive(Serialize)]
struct UserVars {
    username: String,
    limit: String,
}

impl Leetcode<'_> {
    pub async fn get_user(&self, username: &String) -> Result<Data, GraphQLError> {
        let vars = UserVars {
            username: username.to_string(),
            limit: LIMIT.to_string(),
        };

        let data = self
            .gql_client
            .query_with_vars::<Data, UserVars>(USER_REQUEST, vars)
            .await?;
        Ok(data)
    }
}
