#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ffi channel recv failed: `{0}`")]
    FfiChannelRecvFailed(String),
}

impl From<Error> for (u32, String, String) {
    fn from(err: Error) -> Self {
        let (code, typ, message) = match err {
            Error::FfiChannelRecvFailed(_) => {
                (201, "ffi channel recv failed".to_string(), err.to_string())
            }
        };
        (code, typ, message)
    }
}
