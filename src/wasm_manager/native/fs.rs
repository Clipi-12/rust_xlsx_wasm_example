use std::{io, path::Path};

pub fn save_buffer(path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> io::Result<()> {
    std::fs::write(path, buf)
}
