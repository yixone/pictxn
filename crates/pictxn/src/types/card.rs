use std::process;

use chrono::Utc;
use rand::{Rng, rng};

/// Unique numeric identifier of the card
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub struct CardId(pub i64);

impl CardId {
    pub fn generate() -> Self {
        const TS_MASK: i64 = (1i64 << 42) - 1;
        const PID_MASK: i64 = (1 << 8) - 1;
        const RN_MASK: i64 = (1 << 13) - 1;

        const TS_OFFSET: i64 = 1750000000000;

        let ts = (Utc::now().timestamp_millis() - TS_OFFSET) & TS_MASK;
        let pid = process::id() as i64 & PID_MASK;
        let rn = (rng().random::<u32>() as i64) & RN_MASK;

        let id = (ts << (8 + 13)) | (pid << 13) | rn;

        CardId(id)
    }
}
