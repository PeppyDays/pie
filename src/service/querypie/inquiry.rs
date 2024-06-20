use crate::service::querypie::{gateway::QueryPieGateway, model::User};

pub enum QueryPieInquiry {
    FindUser { email: String },
}

pub struct QueryPieInquiryAnswerer<G: QueryPieGateway> {
    gateway: G,
}

impl<G: QueryPieGateway> QueryPieInquiryAnswerer<G> {
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    pub async fn answer(&self, inquiry: QueryPieInquiry) -> Result<User, anyhow::Error> {
        match inquiry {
            QueryPieInquiry::FindUser { email } => {
                let user = self.gateway.find_user_by_email(&email).await?;
                Ok(user)
            }
        }
    }
}
