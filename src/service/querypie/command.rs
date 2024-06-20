use crate::service::querypie::gateway::QueryPieGateway;

pub enum QueryPieCommand {
    ActivateUser { email: String },
}

pub struct QueryPieCommandExecutor<G: QueryPieGateway> {
    gateway: G,
}

impl<G: QueryPieGateway> QueryPieCommandExecutor<G> {
    pub fn new(gateway: G) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, command: QueryPieCommand) -> Result<(), anyhow::Error> {
        match command {
            QueryPieCommand::ActivateUser { email } => {
                let user = self.gateway.find_user_by_email(&email).await?;
                self.gateway
                    .activate_user(&user, Some("Requested by user"))
                    .await?;
                Ok(())
            }
        }
    }
}
