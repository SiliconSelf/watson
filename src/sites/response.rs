use super::Site;

pub(crate) struct ResponseSite {}

impl Site for ResponseSite {
    async fn test(&self, username: &str) -> Option<String> {
        Some(String::new())
    }
}
