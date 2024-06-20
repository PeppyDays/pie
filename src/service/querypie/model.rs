#[derive(Clone, Debug, serde::Deserialize)]
pub struct User {
    uuid: String,
    email: String,
    expired: bool,
    locked: bool,
}

impl User {
    pub fn new(uuid: String, email: String, expired: bool, locked: bool) -> Self {
        Self {
            uuid,
            email,
            expired,
            locked,
        }
    }

    pub fn uuid(&self) -> &str {
        self.uuid.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn expired(&self) -> bool {
        self.expired
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn is_active(&self) -> bool {
        !self.expired && !self.locked
    }
}
