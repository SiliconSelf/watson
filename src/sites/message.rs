use super::Site;

/// A site that will indicate the lack of an account with a message in the
/// response body
pub(crate) struct MessageSite {
    /// The site's url
    pub(crate) url: &'static str,
}

impl Site for MessageSite {
    async fn test(&self, username: &str) -> Option<bool> {
        todo!();
    }
}
