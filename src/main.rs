mod resolve;
mod step;

use crate::resolve::github::GitHub;
use crate::step::Action;
use rayon::prelude::*;
use regex::Regex;
use reqwest::header;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

#[derive(structopt::StructOpt, Debug)]
#[structopt(
    name = "actions-digest",
    about = "Resolve the tagged steps in your GitHub Action workflows to commit sha references",
    global_settings = &[structopt::clap::AppSettings::ColoredHelp]
)]
struct Args {
    #[structopt(
        short = "t",
        long,
        help = "GitHub access token to use for increased rate-limit",
        env = "GITHUB_TOKEN"
    )]
    github_token: Option<String>,

    /// The file to process
    file: PathBuf,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    // While operating on such small files, it is more efficient to read and mutate them in memory.
    // One could also read the target line-by-line while writing each line, processed or not, back
    // to disk into a temporary file. But that would only make sense for very large data-sets.
    let mut content = read_to_string(&args.file)?;

    let mut actions: Vec<Action> = content
        .lines()
        .filter_map(|line| {
            if !step::ACTION_USES_RE.is_match(line) {
                return None;
            }

            // todo remove duplicate matches so the resolver has to do fewer requests

            match Action::from_str(line) {
                Ok(a) => Some(a),
                Err(err) => {
                    eprintln!("error: {} -> {}", line.trim(), err);
                    None
                }
            }
        })
        .collect();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    if let Some(github_token) = &args.github_token {
        let header_value = format!("token {}", github_token);
        let mut auth_value = header::HeaderValue::from_str(&header_value)?;
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
    }

    let http_client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .timeout(Duration::from_secs(10))
        .use_rustls_tls()
        .build()?;

    let resolver = GitHub::new(http_client);

    actions.par_iter_mut().for_each(|action| {
        action.sha = match resolver.resolve(&action.repository, &action.version) {
            Ok(Some(s)) => {
                eprintln!(
                    "resolved: {}@{} -> {}@{}",
                    &action.repository, &action.version, &action.repository, s
                );
                Some(s)
            }
            Ok(None) => None,
            Err(err) => {
                eprintln!(
                    "error: {}@{} -> {}",
                    &action.repository, &action.version, err
                );
                None
            }
        }
    });

    for action in actions.iter() {
        if action.sha.is_none() {
            continue;
        }

        let search = Regex::new(&format!(
            r"(?i)({}@{})\b",
            action.repository, action.version
        ))
        .unwrap();

        content = search
            .replace(
                &content,
                format!(
                    "{}@{}",
                    action.repository,
                    action.sha.as_ref().expect("cannot fail")
                ),
            )
            .to_string()
    }

    print!("{}", content);

    Ok(())
}
