use super::Site;
use crate::REQWEST_CLIENT;

/// A site that will indicate the lack of an account with a message in the
/// response body
pub(crate) struct MessageSite {
    /// The site's url
    pub(crate) url: &'static str,
    /// The error message to look for in the response body
    pub(crate) error_message: &'static str,
}

impl Site for MessageSite {
    async fn test(&self, username: &str) -> Option<String> {
        let request_url = self.url.replace("{}", username);
        let Ok(response) = REQWEST_CLIENT
            .get()
            .expect("Client not defined for {self.url}")
            .get(&request_url)
            .send()
            .await
        else {
            return None;
        };
        let text = response.text().await.expect("");
        if text.contains(self.error_message) {
            None
        } else {
            Some(request_url)
        }
    }
}
