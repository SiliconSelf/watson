# `watson`

Search for accounts across social media platforms by username. This is dramatically faster than the inspiring project [Sherlock](https://github.com/sherlock-project/sherlock) because it uses tokio to search in parallel.

## Installation

```sh
git clone https://github.com/SiliconSelf/watson.git
cargo install --path watson/
```

## Usage

To use, simply add the usernames you want to search for as the program arguments.

```other
Does awesome things

Usage: watson [NAMES]...

Arguments:
  [NAMES]...  Usernames to search for

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Why Not Use Sherlock?

I made this program because I got sick of how slow Sherlock is when I was searching for usernames that weren't already squatted on a site I use.

## Major Issues

I am using a snapshot in time from the Sherlock repository as the source of my data. This kind of tool by nature is extremely fragile as even slight changes in target websites. Because of it's status as a small side project of mine and not a high profile OSINT tool, watson's data most likely will not be kept up to date nearly as well as sherlock.
