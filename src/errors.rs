#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Cannot encrypt an already encrypted file")]
    AlreadyEncrypted,
    // Add more error variants as needed
}