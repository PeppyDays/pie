use anyhow::Context;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::service::querypie::model::User;

#[async_trait::async_trait]
pub trait QueryPieGateway {
    async fn find_user_by_email(&self, email: &str) -> Result<User, anyhow::Error>;
    async fn activate_user(&self, user: &User, comment: Option<&str>) -> Result<(), anyhow::Error>;
}

struct QueryPieDefaultGateway {
    client: reqwest::Client,
    host: reqwest::Url,
    api_key: String,
}

impl QueryPieDefaultGateway {
    fn new(client: reqwest::Client, host: reqwest::Url, api_key: String) -> Self {
        Self {
            client,
            host,
            api_key,
        }
    }
}

#[async_trait::async_trait]
impl QueryPieGateway for QueryPieDefaultGateway {
    async fn find_user_by_email(&self, email: &str) -> Result<User, anyhow::Error> {
        let response = self
            .client
            .get("/api/external/users")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &self.api_key)
            .query(&[("email", email), ("pageSize", "1")])
            .send()
            .await
            .context("Failed to make request with QueryPie")?
            .error_for_status()
            .context("Failed to get user by email from QueryPie")?;

        let user = serde_json::from_value::<Vec<User>>(
            response
                .json::<serde_json::Value>()
                .await
                .context("Failed to jsonify response body")?
                .get("list")
                .context("Failed to deserialise response body to user")?
                .clone(),
        )
        .context("Failed to deserialise response body to user")?
        .into_iter()
        .nth(0)
        .ok_or(anyhow::anyhow!("User not found"))?;

        Ok(user)
    }

    async fn activate_user(&self, user: &User, comment: Option<&str>) -> Result<(), anyhow::Error> {
        let _ = self
            .client
            .post("/api/external/users/activate")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, &self.api_key)
            .json(&serde_json::json!(
                {"uuid": user.uuid(), "reasonForChangeStatus": comment.unwrap_or("requested by user")}
            ))
            .send()
            .await
            .context("Failed to make request with QueryPie")?
            .error_for_status()
            .context("Failed to activate user with QueryPie")?;

        Ok(())
    }
}
