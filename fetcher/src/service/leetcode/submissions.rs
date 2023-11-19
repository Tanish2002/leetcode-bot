use super::{Leetcode, LIMIT};
use reqwest_graphql::GraphQLError;
use serde::{Deserialize, Serialize};

const RECENT_SUBMISSION_REQUEST: &str = r#"
query recentAcSubmissions($username: String!, $limit: Int!) {
  recentAcSubmissionList(username: $username, limit: $limit) {
    title
    titleSlug
    timestamp
  }
}
"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentAcSubmission {
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "data")]
pub struct RecentAcSubmissionResp {
    #[serde(rename = "recentAcSubmissionList")]
    pub recent_ac_submission_list: Vec<RecentAcSubmission>,
}

#[derive(Serialize)]
struct RecentSubmissionVars {
    username: String,
    limit: String,
}

impl Leetcode<'_> {
    pub async fn get_recent_submissions(
        &self,
        username: &String,
    ) -> Result<RecentAcSubmissionResp, GraphQLError> {
        let vars = RecentSubmissionVars {
            username: username.to_string(),
            limit: LIMIT.to_string(),
        };

        let data = self
            .gql_client
            .query_with_vars::<RecentAcSubmissionResp, RecentSubmissionVars>(
                RECENT_SUBMISSION_REQUEST,
                vars,
            )
            .await?;
        Ok(data)
    }
}
