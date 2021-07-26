mod config;
mod search;

pub use search::repos;
pub use config::Settings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_repos() -> anyhow::Result<()> {
        let settings = Settings::from_env()?;
        for path in search::repos(&settings) {
            println!("{:}", path.to_string_lossy());
        }
    
        Ok(())
    }
}