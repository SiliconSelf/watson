#![doc = include_str!("../README.md")]

use std::sync::Arc;

use clap::Parser;

/// Where the various kinds of site are represented as structures
mod sites;
/// Generated site data
#[allow(clippy::all)] mod gen;

use gen::SITES;
use once_cell::sync::OnceCell;
use reqwest::Client;
use sites::SiteType;

/// The shared client for Reqwest that all sites will use
pub(crate) static REQWEST_CLIENT: OnceCell<Client> = OnceCell::new();

/// A struct representing the possible CLI configurations of the program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// If all sites should be displayed or only matches
    // #[arg(short, long)]
    // show_all: bool,
    /// Usernames to search for
    names: Vec<String>,
}

/// Asynchronously test for the existence of an account
#[allow(clippy::rc_buffer)]
async fn test_username(name: Arc<String>, site_name: &str, site_data: &SiteType) {
    match site_data.test(&name).await {
        Some(true) => { println!("{site_name}: {name}"); },
        Some(false) => { }
        None => { println!("Error while checking {site_name}") },
    }
}

#[tokio::main]
async fn main() {
    let cli = Args::parse();
    REQWEST_CLIENT.set(reqwest::Client::new()).expect("Cell cannot already be initialized");
    let mut tasks = Vec::new();
    for name in cli.names {
        let name_arc = Arc::new(name);
        for (site_name, site_data) in SITES.entries() {
            let new_arc = name_arc.clone();
            let handle = tokio::task::spawn(test_username(new_arc, site_name, site_data));
            tasks.push(handle);
        }
    }
    for handle in tasks {
        handle.await.unwrap();
    }
}
