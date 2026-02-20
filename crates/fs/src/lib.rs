pub mod hash;
pub mod provider;
pub mod stream;

pub mod fs;

const MAX_FILE_SIZE: u64 = 128 * 1024 * 1024;
const BUFF_WRITER_CAPACITY: usize = 64 * 1024;
