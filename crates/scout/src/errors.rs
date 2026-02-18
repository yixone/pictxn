use derive_more::From;

#[derive(Debug, From)]
pub enum ScoutError {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
}
