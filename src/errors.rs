#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Cannot encrypt an already encrypted file")]
    AlreadyEncrypted,
    #[error("Failed to decrypt the file")]
    DecryptError(#[from] age::DecryptError),
    #[error("Failed to encrypt the file")]
    EncryptError(#[from] age::EncryptError),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error), // Add more error variants as needed
}

pub type AppResult<T> = Result<T, AppError>;
