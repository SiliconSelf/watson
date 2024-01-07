use std::time::Duration;

use super::Site;
use crate::REQWEST_CLIENT;

/// A site that fails with a status code
pub(crate) struct StatusSite {
    /// The url to test for users at
    pub(crate) url: &'static str,
}

impl Site for StatusSite {
    async fn test(&self, username: &str) -> Option<String> {
        let request_url = self.url.replace("{}", username);
        let response = REQWEST_CLIENT
            .get()
            .unwrap_or_else(|| {
                panic!("Client not defined for {}", &request_url)
            })
            .head(&request_url)
            .timeout(Duration::from_secs(60))
            .send()
            .await
            .expect("Request failed!");
        response.status().is_success().then_some(request_url)
    }
}
