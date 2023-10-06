# Base type for BitTorrent peer IDs



`tdyne_peer_id` is a newtype for BitTorrent peer IDs, represented as `[u8; 20]`.
It's intentionally kept very minimalist to minimise the possibility of backwards-incompatible
changes.

```toml
[dependencies]
tdyne-peer-id = "1"
```

## Example

```rust
use tdyne_peer_id::{PeerId, BadPeerIdLengthError};

fn main() {
    let byte_array: &[u8; 20] = b"-TR0000-*\x00\x01d7xkqq04n";
    let byte_slice: &[u8] = b"-TR0000-*\x00\x01d7xkqq04n";
    let short_byte_slice: &[u8] = b"-TR0000-";

    // creating a PeerId from an array is simple
    let peer_id = PeerId::from(b"-TR0000-*\x00\x01d7xkqq04n");
    assert_eq!(peer_id.to_string(), "-TR0000-???d7xkqq04n".to_string());

    // you can also create PeerId from a byte slice if its 20 bytes long
    _ = PeerId::try_from(byte_slice).expect("matching lengths");

    // …if it's not, you get an error
    let error = BadPeerIdLengthError(short_byte_slice.len());
    assert_eq!(PeerId::try_from(short_byte_slice).expect_err("lengths don't match"), error);
}
```

## Libraries and projects using `tdyne_peer_id`

* [`tdyne_peer_id_registry`](https://crates.io/crates/tdyne-peer-id-registry), peer ID
  database and parser


[<img src="./.github/logo.svg" align="right" alt="TORRENTDYNE logo" width="40" height="40">](https://torrentdyne.com)

`tdyne_peer_id` is opensourced from [TORRENTDYNE](https://torrentdyne.com), a BitTorrent diagnostic service.



#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>