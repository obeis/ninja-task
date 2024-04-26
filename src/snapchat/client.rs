use super::segment::SegmentService;

pub struct SnapChat {
    token: String,
}

impl SnapChat {
    pub async fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn segment(&self) -> SegmentService {
        SegmentService::new(&self.token).await
    }
}
