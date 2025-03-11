use super::{Leetcode, LIMIT};
use reqwest_graphql::GraphQLError;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

const RECENT_SUBMISSION_REQUEST: &str = r#"
query recentAcSubmissions($username: String!, $limit: Int!) {
  recentAcSubmissionList(username: $username, limit: $limit) {
    title
    titleSlug
    timestamp
  }
}
"#;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentAcSubmission {
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        info!("Fetching recent submissions for user: {}", username);

        let vars = RecentSubmissionVars {
            username: username.to_string(),
            limit: LIMIT.to_string(),
        };

        match self
            .gql_client
            .query_with_vars::<RecentAcSubmissionResp, RecentSubmissionVars>(
                RECENT_SUBMISSION_REQUEST,
                vars,
            )
            .await
        {
            Ok(data) => {
                info!(
                    "Successfully retrieved {} submissions for user: {}",
                    data.recent_ac_submission_list.len(),
                    username
                );
                Ok(data)
            }
            Err(e) => {
                error!("Error fetching submissions for user {}: {}", username, e);
                Err(e)
            }
        }
    }
}
