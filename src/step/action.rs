use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    pub static ref ACTION_USES_RE: Regex =
        Regex::new(r"(?i).*uses[\W]?: [\W]?(?P<repo>[a-z0-9-/]+)@(?P<version>[a-z0-9-/.]+)[\W]?",)
            .unwrap();
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not build action from line")]
    BuildLine,
}

#[derive(Debug, PartialEq, Default)]
pub struct Action {
    /// The repository in {owner}/{name} format
    pub repository: String,

    /// The version used; can be a partial tag, complete tag, commit sha or branch
    pub version: String,

    /// The commit sha we resolved to
    pub sha: Option<String>,
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = ACTION_USES_RE.captures(s).ok_or(Error::BuildLine)?;

        let res = Action {
            repository: caps["repo"].to_string(),
            version: caps["version"].to_string(),
            ..Default::default()
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(
        "        uses: actions/checkout@ec3a7ce113134d7a93b817d10a8272cb61118579 # v2.4.0",
        "actions/checkout",
        "ec3a7ce113134d7a93b817d10a8272cb61118579"
    )]
    #[case(" - uses: actions/checkout@v2", "actions/checkout", "v2")]
    #[case(" - 'uses': actions/checkout@v2", "actions/checkout", "v2")]
    #[case(" - uses: 'actions/checkout@v2'", "actions/checkout", "v2")]
    #[case(" - uses: \"actions/checkout@v2\"", "actions/checkout", "v2")]
    #[case("uses: actions/checkout@1", "actions/checkout", "1")]
    #[case("uses: actions/checkout@1.0.0", "actions/checkout", "1.0.0")]
    #[case("uses: actions/checkout@1.0-beta42", "actions/checkout", "1.0-beta42")]
    fn from_str(#[case] input: &str, #[case] repository: &str, #[case] version: &str) {
        let res = Action::from_str(input).unwrap();
        assert_eq!(repository, res.repository);
        assert_eq!(version, res.version);
    }
}
