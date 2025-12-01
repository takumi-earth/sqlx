use crate::message::{FrontendMessage, FrontendMessageFormat};
use sqlx_core::Error;
use std::num::Saturating;

/// The Flush message does not cause any specific output to be generated,
/// but forces the backend to deliver any data pending in its output buffers.
///
/// A Flush must be sent after any extended-query command except Sync, if the
/// frontend wishes to examine the results of that command before issuing more commands.
#[derive(Debug)]
pub struct Flush;

impl FrontendMessage for Flush {
    const FORMAT: FrontendMessageFormat = FrontendMessageFormat::Flush;

    #[inline(always)]
    fn body_size_hint(&self) -> Saturating<usize> {
        Saturating(0)
    }

    #[inline(always)]
    fn encode_body(&self, _buf: &mut Vec<u8>) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Encode;

    #[test]
    fn encodes_flush_message() {
        let mut buf = Vec::new();
        Flush.encode(&mut buf).expect("encode flush");
        // Format byte + length (4) with zero body.
        assert_eq!(buf, vec![FrontendMessageFormat::Flush as u8, 0, 0, 0, 4]);
    }

    #[test]
    fn format_matches_enum() {
        assert_eq!(Flush::FORMAT as u8, FrontendMessageFormat::Flush as u8);
    }
}
