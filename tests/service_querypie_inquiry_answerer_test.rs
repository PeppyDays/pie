use std::sync::RwLock;

use fake::{faker::internet::en::SafeEmail, Fake};
use uuid::Uuid;

use pie::service::querypie::{
    gateway::QueryPieGateway,
    inquiry::{QueryPieInquiry, QueryPieInquiryAnswerer},
    model::User,
};

#[rstest::rstest]
#[tokio::test]
async fn sut_finds_user_by_email_correctly(user: User) {
    // Arrange
    let gateway = QueryPieGatewayFake::new(user.clone());
    let inquiry = QueryPieInquiry::FindUser {
        email: user.email().to_string(),
    };
    let sut = QueryPieInquiryAnswerer::new(gateway);

    // Act
    let inquired_user = sut.answer(inquiry).await.unwrap();
    let actual = inquired_user.email();

    // Assert
    let expected = user.email();
    assert_eq!(actual, expected);
}

struct QueryPieGatewayFake {
    user: RwLock<User>,
}

impl QueryPieGatewayFake {
    fn new(user: User) -> Self {
        Self {
            user: RwLock::new(user),
        }
    }

    fn user(&self) -> User {
        self.user.read().unwrap().clone()
    }

    fn replace(&self, user: User) {
        *self.user.write().unwrap() = user;
    }
}

#[async_trait::async_trait]
impl QueryPieGateway for QueryPieGatewayFake {
    async fn find_user_by_email(&self, email: &str) -> Result<User, anyhow::Error> {
        let existing_user = self.user();
        match existing_user.email() == email {
            true => Ok(User::new(
                existing_user.uuid().to_owned(),
                existing_user.email().to_owned(),
                existing_user.expired(),
                existing_user.locked(),
            )),
            false => Err(anyhow::anyhow!("User not found")),
        }
    }

    async fn activate_user(&self, user: &User, _: Option<&str>) -> Result<(), anyhow::Error> {
        let existing_user = self.user();
        match existing_user.email() == user.email() {
            true => {
                let activated_user = User::new(
                    user.uuid().to_owned(),
                    user.email().to_owned(),
                    false,
                    false,
                );
                self.replace(activated_user);
                Ok(())
            }
            false => Err(anyhow::anyhow!("User not found")),
        }
    }
}

#[rstest::fixture]
fn user(
    #[default(SafeEmail().fake())] email: String,
    #[default(false)] expired: bool,
    #[default(false)] locked: bool,
) -> User {
    User::new(Uuid::new_v4().to_string(), email, expired, locked)
}
