use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
pub struct Model {
    timestamp_table_name: String,
    dynamodb_client: Client,
}
impl Model {
    pub fn new(timestamp_table_name: String, dynamodb_client: Client) -> Self {
        Self {
            timestamp_table_name,
            dynamodb_client,
        }
    }
    pub async fn add_or_update_timestamp(
        &self,
        user: &String,
        timestamp: &String,
    ) -> Result<(), Error> {
        let user_av = AttributeValue::S(user.to_string());
        let timestamp_av = AttributeValue::S(timestamp.to_string());
        let _ = self
            .dynamodb_client
            .put_item()
            .table_name(&self.timestamp_table_name)
            .item("User", user_av)
            .item("Timestamp", timestamp_av)
            .send()
            .await?;
        Ok(())
    }
    pub async fn get_latest_timestamp(&self, user: &str) -> Result<String, Error> {
        let user_av = AttributeValue::S(user.to_string());
        let resp = self
            .dynamodb_client
            .get_item()
            .table_name(&self.timestamp_table_name)
            .key("User", user_av)
            .send()
            .await?;

        // If timestamp is not found "" is returned instead of a error. As this might be the first
        // time user was added
        Ok(resp
            .item()
            .and_then(|item| item.get("Timestamp"))
            .and_then(|user| user.as_s().ok())
            .map(|user_s| user_s.to_string())
            .unwrap_or_else(|| "".to_string()))
    }
}
