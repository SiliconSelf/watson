//! Generates functionality at compile time for searching various sites

use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};

/// The kind of error a site will return if a user doesn't exist
#[derive(Serialize, Deserialize)]
enum ErrorType {
    /// 404 Status Code
    #[serde(rename = "status_code")]
    StatusCode,
    /// Text message in page body
    #[serde(rename = "message")]
    Message,
    /// Response URL
    #[serde(rename = "response_url")]
    ResponseUrl,
}

/// A representation of a deserialized site from sites.json
#[derive(Serialize, Deserialize)]
struct Site {
    /// Kind of error that will occur if a profile does not exist
    #[serde(rename = "errorType")]
    error_type: ErrorType,
    /// Message that will be returned by the site if a profile does not exist
    #[serde(rename = "errorMsg")]
    error_msg: Option<String>,
    /// Url to check for a profile at
    url: String,
    #[serde(rename = "urlMain")]
    /// URL of the main page of the site
    url_main: String,
    /// Regex check to skip testing usernames that won't be allowed on the
    /// given site anyways
    #[serde(rename = "regexCheck")]
    regex_check: Option<String>,
    /// An example of a claimed username for generating tests
    username_claimed: Option<String>,
    /// An example of an unclaimed username for generating tests
    username_unclaimed: Option<String>,
    /// If the site is NSFW
    #[serde(rename = "isNSFW")]
    is_nsfw: Option<bool>,
    /// Where a ResponseUrl site will redirect on error
    #[serde(rename = "errorUrl")]
    error_url: Option<String>,
}

fn main() {
    let sites_file_string = std::fs::read_to_string("sites.json")
        .expect("Failed to read sites.json");
    let data: HashMap<String, Site> =
    serde_json::from_str(&sites_file_string)     .expect("Failed to
    deserialize sites.json"); let mut file_handle =
    std::fs::File::create("src/gen.rs").expect("Failed to open gen.rs");

    let mut sites = phf_codegen::Map::new();
    for (site, data) in data {
        match data.error_type {
            ErrorType::StatusCode => {
                sites.entry(site, &format!("SiteType::StatusCode(StatusSite {{
                    url: \"{}\"
                }})", data.url));
            },
            // ErrorType::Message => {
            //     sites.entry(site, &format!("SiteType::Message(MessageSite {{
            //         url: \"{}\"
            //     }})", data.url));
            // }
            _ => {},
            // ErrorType::ResponseUrl => {},
        }
    }
    write!(file_handle, "#![allow(clippy::all)]\n");
    write!(file_handle, "use crate::sites::SiteType;\n");
    write!(file_handle, "use crate::sites::status::StatusSite;\n");
    write!(file_handle, "use crate::sites::message::MessageSite;\n");
    write!(file_handle, "pub(crate) static SITES: phf::Map<&'static str, SiteType> = {};",
    sites.build());
}
