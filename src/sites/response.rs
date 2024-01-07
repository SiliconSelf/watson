use super::Site;

/// A representation of a site that returns a redirect as an error.
pub(crate) struct ResponseSite;

impl Site for ResponseSite {
    async fn test(&self, _username: &str) -> Option<String> {
        Some(String::new())
    }
}
