pub(crate) struct UserService<'a> {
    token: &'a str,
}

impl<'a> UserService<'a> {
    pub async fn new(token: &'a str) -> Self {
        Self { token }
    }
}
