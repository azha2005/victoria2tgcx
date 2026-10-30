//! Preserve a complete per-process trace when two game instances share a folder.
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};
pub struct Log {
    canonical: File,
    legacy: Option<File>,
}
impl Log {
    pub fn create(dir: &Path, pid: u32) -> io::Result<Self> {
        Ok(Self {
            canonical: File::create(dir.join(format!("tgcx_volunteers_{pid}.log")))?,
            legacy: File::create(dir.join("tgcx_volunteers.log")).ok(),
        })
    }
}
impl Write for Log {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.canonical.write_all(bytes)?;
        if let Some(legacy) = self.legacy.as_mut() {
            let _ = legacy.write_all(bytes);
        }
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.canonical.flush()?;
        if let Some(legacy) = self.legacy.as_mut() {
            let _ = legacy.flush();
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_legacy_file_cannot_corrupt_independent_process_traces() {
        let dir = std::env::temp_dir().join(format!("tgcx-log-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut a = Log::create(&dir, 1).unwrap();
        let mut b = Log::create(&dir, 2).unwrap();
        for n in 0..20 {
            writeln!(a, "peer1 {n}").unwrap();
            writeln!(b, "peer2 {n}").unwrap();
        }
        a.flush().unwrap();
        b.flush().unwrap();
        drop(a);
        drop(b);
        for peer in 1..=2 {
            assert_eq!(
                std::fs::read_to_string(dir.join(format!("tgcx_volunteers_{peer}.log"))).unwrap(),
                (0..20)
                    .map(|n| format!("peer{peer} {n}\n"))
                    .collect::<String>()
            );
        }
        for name in [
            "tgcx_volunteers_1.log",
            "tgcx_volunteers_2.log",
            "tgcx_volunteers.log",
        ] {
            std::fs::remove_file(dir.join(name)).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }
}
