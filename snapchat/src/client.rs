use super::{auth::AuthService, segment::SegmentService, user::UserService};

pub struct SnapChat<'a> {
    token: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
    refresh_token: &'a str,
}

impl<'a> SnapChat<'a> {
    pub async fn new(
        token: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
        redirect_uri: &'a str,
        refresh_token: &'a str,
    ) -> Self {
        Self {
            token,
            client_id,
            client_secret,
            redirect_uri,
            refresh_token,
        }
    }

    pub async fn auth(&self) -> AuthService {
        AuthService::new(
            self.client_id,
            self.client_secret,
            self.redirect_uri,
            self.refresh_token,
        )
        .await
    }

    pub async fn segment(&self) -> SegmentService {
        SegmentService::new(self.token).await
    }

    pub async fn user(&self) -> UserService {
        UserService::new(self.token).await
    }
}
