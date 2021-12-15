use serde::Deserialize;

const BASE_URL: &str = "https://api.github.com";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unable to build url: {0}")]
    BuildURL(String),

    #[error("GitHub API error: {0}")]
    GitHubApi(#[from] reqwest::Error),
}

#[derive(Deserialize, Clone, Debug)]
struct GitObject {
    sha: String,

    #[serde(rename = "type")]
    obj_type: String,

    #[allow(dead_code)]
    url: String,
}

#[derive(Deserialize, Clone, Debug)]
struct Reference {
    #[serde(rename = "ref")]
    #[allow(dead_code)]
    git_ref: String,

    #[allow(dead_code)]
    node_id: String,

    #[allow(dead_code)]
    url: String,
    object: GitObject,
}

pub struct GitHub {
    api: reqwest::blocking::Client,
}

impl GitHub {
    // todo pass http client into this; start with blocking reqwest; later turn into async
    pub fn new(api: reqwest::blocking::Client) -> Self {
        Self { api }
    }

    pub fn resolve(&self, repository: &str, version: &str) -> Result<Option<String>, Error> {
        if let Some(c) = self.resolve_tag(repository, version)? {
            return Ok(Some(c));
        }

        if let Some(c) = self.resolve_branch(repository, version)? {
            return Ok(Some(c));
        }

        Ok(None)
    }

    fn resolve_tag(&self, repository: &str, version: &str) -> Result<Option<String>, Error> {
        self.resolve_by_type("tags", repository, version)
    }

    fn resolve_branch(&self, repository: &str, version: &str) -> Result<Option<String>, Error> {
        self.resolve_by_type("heads", repository, version)
    }

    fn resolve_by_type(
        &self,
        ref_type: &str,
        repository: &str,
        version: &str,
    ) -> Result<Option<String>, Error> {
        let url = reqwest::Url::parse(&format!(
            // "per_page" is documented in the api specs, but doesn't always take effect
            "{}/repos/{}/git/matching-refs/{}/{}?per_page=1",
            BASE_URL, repository, ref_type, version
        ))
        .map_err(|err| Error::BuildURL(format!("{}", err)))?;

        let mut response = self.api.get(url).send()?.json::<Vec<Reference>>()?;

        if response.is_empty() {
            return Ok(None);
        }

        // results seem to come in in reverse order; the last entry being the latest one
        // so we'll reverse the vector, if more than one result is returned
        // the api specs don't mention any sorting capability
        if response.len() > 1 {
            response.reverse();
        }

        let reference = response.first().expect("this cannot fail");

        match reference.object.obj_type.as_str() {
            "commit" | "tag" => return Ok(Some(reference.object.sha.clone())),
            _ => Ok(None)
        }
    }
}
