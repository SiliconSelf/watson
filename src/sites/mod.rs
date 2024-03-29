/// Message response sites
pub(crate) mod message;
/// Response sites
pub(crate) mod response;
/// Status error sites
pub(crate) mod status;

use message::MessageSite;
use response::ResponseSite;
use status::StatusSite;

/// The kind of site a page is
pub(crate) enum SiteType {
    /// 404 Status Code
    StatusCode(StatusSite),
    /// Text message in page body
    Message(MessageSite),
    /// Response URL
    _ResponseUrl(ResponseSite),
}

/// The methods a site must implement
pub(crate) trait Site {
    /// Test if a username exists at the given site
    async fn test(&self, username: &str) -> Option<String>;
}

impl SiteType {
    /// Test if a username exists on the given platform represented by &self<T>
    pub(crate) async fn test(&self, username: &str) -> Option<String> {
        match self {
            SiteType::StatusCode(site) => site.test(username).await,
            SiteType::Message(site) => site.test(username).await,
            SiteType::_ResponseUrl(site) => site.test(username).await,
        }
    }
}
