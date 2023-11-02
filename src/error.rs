#[derive(Debug)]
pub enum Error {
    NotEnoughResources,
}

pub type Result<T> = std::result::Result<T, Error>;
