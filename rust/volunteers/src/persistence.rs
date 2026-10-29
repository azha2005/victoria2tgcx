//! Campaign/save-bound SP checkpoints. File presence alone never activates a
//! mission. The save lifecycle adapter must supply a verified saved-file binding.
use crate::mission::Campaign;
use crate::protocol::{Reader, Writer};
use crate::registry::Registry;
use crate::sha256::Sha256;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const MAGIC: &[u8; 8] = b"TGCXCP01";
const MAX_CHECKPOINT: u64 = 1024 * 1024;
static TEMP_SERIAL: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveBinding {
    pub campaign: Campaign,
    pub save_sha256: String,
}
impl SaveBinding {
    pub fn valid(&self) -> bool {
        self.campaign.0 != [0; 16]
            && self.save_sha256.len() == 64
            && self
                .save_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    }
    pub fn from_saved_file(campaign: Campaign, path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut hash = Sha256::new();
        let mut buf = [0u8; 65536];
        loop {
            let n = file.read(&mut buf)?;
            if n == 0 {
                break;
            }
            hash.update(&buf[..n]);
        }
        let result = Self {
            campaign,
            save_sha256: hash.finish(),
        };
        if !result.valid() {
            return Err(invalid("invalid saved-game binding"));
        }
        Ok(result)
    }
}

pub struct Store {
    directory: PathBuf,
}
impl Store {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }
    fn path(&self, binding: &SaveBinding) -> io::Result<PathBuf> {
        if !binding.valid() {
            return Err(invalid("unbound campaign"));
        }
        Ok(self
            .directory
            .join(format!("tgcx_volunteers_{}.bin", binding.save_sha256)))
    }
    pub fn save(&self, binding: &SaveBinding, registry: &Registry) -> io::Result<PathBuf> {
        let target = self.path(binding)?;
        if registry.campaign != binding.campaign {
            return Err(invalid("campaign mismatch"));
        }
        let payload = registry.encode().map_err(|_| invalid("invalid registry"))?;
        let mut w = Writer(Vec::new());
        w.0.extend_from_slice(MAGIC);
        w.0.extend_from_slice(&binding.campaign.0);
        w.0.extend_from_slice(binding.save_sha256.as_bytes());
        w.u32(payload.len() as u32);
        w.0.extend_from_slice(&payload);
        let mut hash = Sha256::new();
        hash.update(&w.0);
        w.0.extend_from_slice(hash.finish().as_bytes());
        if w.0.len() as u64 > MAX_CHECKPOINT {
            return Err(invalid("checkpoint too large"));
        }
        fs::create_dir_all(&self.directory)?;
        let serial = TEMP_SERIAL.fetch_add(1, Ordering::Relaxed);
        let temp = self.directory.join(format!(
            "tgcx_volunteers_{}.{}.tmp",
            std::process::id(),
            serial
        ));
        // Failed create_new must never remove a file we did not create.
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        let result = (|| {
            file.write_all(&w.0)?;
            file.sync_all()?;
            drop(file);
            replace(&temp, &target)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result?;
        Ok(target)
    }
    pub fn load(&self, binding: &SaveBinding, epoch: u64) -> io::Result<Registry> {
        if epoch == 0 {
            return Err(invalid("unverified load epoch"));
        }
        let path = self.path(binding)?;
        let file = File::open(path)?;
        if file.metadata()?.len() > MAX_CHECKPOINT {
            return Err(invalid("checkpoint too large"));
        }
        let mut bytes = Vec::new();
        file.take(MAX_CHECKPOINT + 1).read_to_end(&mut bytes)?;
        if bytes.len() < 64 || bytes.len() as u64 > MAX_CHECKPOINT {
            return Err(invalid("truncated checkpoint"));
        }
        let (body, checksum) = bytes.split_at(bytes.len() - 64);
        let mut hash = Sha256::new();
        hash.update(body);
        if checksum != hash.finish().as_bytes() {
            return Err(invalid("checkpoint digest mismatch"));
        }
        let mut r = Reader {
            bytes: body,
            cursor: 0,
        };
        if r.take(8).map_err(|_| invalid("header"))? != MAGIC
            || r.take(16).map_err(|_| invalid("campaign"))? != binding.campaign.0
            || r.take(64).map_err(|_| invalid("save binding"))? != binding.save_sha256.as_bytes()
        {
            return Err(invalid("wrong save or campaign"));
        }
        let n = r.u32().map_err(|_| invalid("payload length"))? as usize;
        let payload = r.take(n).map_err(|_| invalid("payload"))?;
        if r.cursor != body.len() {
            return Err(invalid("trailing checkpoint data"));
        }
        Registry::decode(payload, binding.campaign, epoch)
            .map_err(|_| invalid("invalid mission data"))
    }
}
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(windows)]
fn replace(from: &Path, to: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(from: *const u16, to: *const u16, flags: u32) -> i32;
    }
    let from: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let to: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 0x1 | 0x8) } == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
#[cfg(not(windows))]
fn replace(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}
