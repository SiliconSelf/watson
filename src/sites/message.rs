use super::Site;

pub(crate) struct MessageSite {
    pub(crate) url: &'static str
}

impl Site for MessageSite {
    async fn test(&self, username: &str) -> Option<bool> {
        todo!();
    }
}