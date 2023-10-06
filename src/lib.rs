#![warn(missing_docs)]

//! # Base type for BitTorrent peer IDs in Rust
//!
//! `tdyne_peer_id` is a newtype for BitTorrent peer IDs, represented as `[u8; 20]`.
//! It's intentionally kept very minimalist to minimise the possibility of backwards-incompatible
//! changes.
//!
//! Example:
//!
//! ```
//! use tdyne_peer_id::{PeerId, BadPeerIdLengthError};
//!
//! let byte_array: &[u8; 20] = b"-TR0000-*\x00\x01d7xkqq04n";
//! let byte_slice: &[u8] = b"-TR0000-*\x00\x01d7xkqq04n";
//! let short_byte_slice: &[u8] = b"-TR0000-";
//!
//! // creating a PeerId from an array is simple
//! let peer_id = PeerId::from(b"-TR0000-*\x00\x01d7xkqq04n");
//! assert_eq!(peer_id.to_string(), "-TR0000-???d7xkqq04n".to_string());
//!
//! // you can also create PeerId from a byte slice if its 20 bytes long
//! _ = PeerId::try_from(byte_slice).expect("matching lengths");
//!
//! // …if it's not, you get an error
//! let error = BadPeerIdLengthError(short_byte_slice.len());
//! assert_eq!(PeerId::try_from(short_byte_slice).expect_err("lengths don't match"), error);
//! ```
//!
//! ## Libraries and projects using `tdyne_peer_id`
//! * [`tdyne_peer_id_registry`](https://crates.io/crates/tdyne-peer-id-registry), peer ID
//!   database and parser


mod errors;

pub use crate::errors::BadPeerIdLengthError;
use std::borrow::Cow;
use std::fmt;


/// Represents an unparsed peer ID. It's just a thin wrapper over `[u8; 20]`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PeerId(pub [u8; 20]);

impl From<[u8; 20]> for PeerId {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<&[u8; 20]> for PeerId {
    fn from(value: &[u8; 20]) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<[u8; 20]> for PeerId {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl TryFrom<&[u8]> for PeerId {
    type Error = BadPeerIdLengthError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(Self)
            .map_err(|_| BadPeerIdLengthError(value.len()))
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_safe())
    }
}

impl PeerId {
    /// Renders the [`PeerId`] into a [`Cow<'_, str>`] with every character outside base64 range
    /// (`0-9`, `a-z`, `A-Z`, `-`, `.`) transformed into ASCII `?`. Most clients only use those
    /// characters in their peer IDs, so this representation is good enough, while being completely
    /// safe to show in any environment without escaping.
    ///
    /// Returns [`Cow<'_, str>`] despite always allocating the string at the moment in anticipation
    /// of a future optimisation.
    ///
    /// Reused in the [`Display`] implementation.
    ///
    /// [`Cow<'_, str>`]: std::borrow::Cow
    /// [`Display`]: std::fmt::Display
    ///
    /// ```
    /// # use tdyne_peer_id::PeerId;
    /// let peer_id = PeerId::from(b"-TR0000-*\x00\x01d7xkqq04n");
    /// assert_eq!(peer_id.to_safe(), "-TR0000-???d7xkqq04n");
    /// ```
    pub fn to_safe(&self) -> Cow<'_, str> {
        // todo: don't allocate on the happy path
        String::from_utf8_lossy(&self.0)
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' => c,
                _ => '?',
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn length_error() {
        let ok_vec = vec![0u8; 20];
        assert!(PeerId::try_from(ok_vec.as_slice()).is_ok());

        let bad_vec = vec![0u8; 21];
        let e = PeerId::try_from(bad_vec.as_slice()).unwrap_err();
        assert_eq!(e.0, 21);
        assert!(e.to_string().contains("21"));
    }

    #[test]
    fn to_safe() {
        let bytes = b"-TR0072-abvd7xkqq04n";
        let peer_id = PeerId::from(bytes);
        assert_eq!(&peer_id.to_safe(), str::from_utf8(bytes).unwrap());

        let bytes = b"-TR0072-*\x00\x01d7xkqq04n";
        let safe = "-TR0072-???d7xkqq04n";
        let peer_id = PeerId::from(bytes);
        assert_eq!(&peer_id.to_safe(), safe);
    }
}
