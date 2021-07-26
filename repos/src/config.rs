use std::env;

const DIR: &'static str = "REPOS_DIR";
const MIN_DEPTH: &'static str = "REPOS_MIN_DEPTH";

pub struct Settings {
    pub base_dir: String,
    pub min_depth: Option<usize>,
}

impl Settings {
    pub fn from_env() -> anyhow::Result<Self> {
        let base_dir = env::var(DIR)?;
        let min_depth = env::var(MIN_DEPTH)
            .map(|s| s.parse::<usize>().expect("Invalid min_depth"))
            .ok();

        Ok(Settings {
            base_dir,
            min_depth,
        })
    }
}
