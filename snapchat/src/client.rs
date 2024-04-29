use super::{auth::AuthService, segment::SegmentService, user::UserService};

pub struct SnapChat<'a> {
    token: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    refresh_token: &'a str,
}

impl<'a> SnapChat<'a> {
    pub async fn new(
        token: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
        refresh_token: &'a str,
    ) -> Self {
        Self {
            token,
            client_id,
            client_secret,
            refresh_token,
        }
    }

    pub async fn auth(&self) -> AuthService {
        AuthService::new(self.client_id, self.client_secret, self.refresh_token).await
    }

    pub async fn segment(&self) -> SegmentService {
        SegmentService::new(self.token).await
    }

    pub async fn user(&self) -> UserService {
        UserService::new(self.token).await
    }
}
