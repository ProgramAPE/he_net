mod de;
mod ser;
mod error;
/// Variable length sequences in LCS are limited to max length of 2^31 - 1.
pub const MAX_SEQUENCE_LENGTH: usize = (1 << 31) - 1;

/// Maximal allowed depth of LCS data, counting only structs and enums.
pub const MAX_CONTAINER_DEPTH: usize = 500;

pub use de::{from_bytes, from_bytes_seed};
pub use error::{Error, Result};
pub use ser::{is_human_readable, serialize_into, serialized_size, to_bytes};