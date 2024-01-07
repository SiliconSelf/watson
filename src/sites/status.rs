use super::Site;
use crate::REQWEST_CLIENT;

/// A site that fails with a status code
pub(crate) struct StatusSite {
    /// The url to test for users at
    pub(crate) url: &'static str,
}

impl Site for StatusSite {
    async fn test(&self, username: &str) -> Option<bool> {
        let request_url = self.url.replace("{}", username);
        let Ok(response) = REQWEST_CLIENT
            .get()
            .expect("Client not defined for {self.url}")
            .head(request_url)
            .send()
            .await
        else {
            return None;
        };
        Some(response.status().is_success())
    }
}
