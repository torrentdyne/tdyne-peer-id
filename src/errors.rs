use std::fmt;

/// Returned when provided byte slice length is not equal to 20 bytes. Includes the
/// length of the offending slice.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BadPeerIdLengthError(
    /// the length of the problematic byte slice
    pub usize
);

impl fmt::Display for BadPeerIdLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid Peer Id length, expected a 20 bytes long slice, got {} bytes",
            self.0
        )
    }
}

impl std::error::Error for BadPeerIdLengthError {}