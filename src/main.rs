#![doc = include_str!("../README.md")]

use std::{sync::Arc, time::Duration};

use clap::Parser;

/// Generated site data
#[allow(clippy::all)]
mod gen;
/// Where the various kinds of site are represented as structures
mod sites;

use gen::SITES;
use once_cell::sync::OnceCell;
use reqwest::Client;
use sites::SiteType;
use tokio::sync::mpsc::UnboundedSender;

/// The shared client for Reqwest that all sites will use
pub(crate) static REQWEST_CLIENT: OnceCell<Client> = OnceCell::new();

/// A struct representing the possible CLI configurations of the program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    // If all sites should be displayed or only matches
    // #[arg(short, long)]
    // show_all: bool,
    /// Usernames to search for
    names: Vec<String>,
}

/// Asynchronously test for the existence of an account
#[allow(clippy::rc_buffer)]
async fn test_username(
    name: Arc<String>,
    site_name: &str,
    site_data: &SiteType,
    sender: UnboundedSender<Option<String>>,
) {
    sender.send(site_data.test(&name).await.map(|url| format!("{site_name}: {url}"))).expect("Sender panics");
}
// Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0
#[tokio::main]
async fn main() {
    let cli = Args::parse();
    REQWEST_CLIENT
        .set(Client::new())
        .expect("Cell cannot already be initialized");
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    for name in cli.names {
        let name_arc = Arc::new(name);
        for (site_name, site_data) in SITES.entries() {
            let new_arc = name_arc.clone();
            tokio::task::spawn(test_username(
                new_arc,
                site_name,
                site_data,
                tx.clone(),
            ));
        }
    }
    
    loop {
        match rx.try_recv() {
            #[allow(clippy::unwrap_used)]
            Ok(value) if value.is_some() => {
                println!("{}", &value.unwrap());
            }
            Ok(value) if value.is_none() => { },
            Ok(_) => {}
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                std::thread::sleep(Duration::from_millis(250));
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => break,
        }
    }
}
