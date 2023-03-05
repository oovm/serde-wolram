#[derive(Debug, Copy, Clone)]
pub enum ZapError {
    UnknownError
}

pub type ZapResult<T> = std::result::Result<T, ZapError>;
