//! A disk's free and whole size, and the bytes a file takes on its disk, on each platform.
use std::fs::Metadata;
use std::path::Path;

/// The free (for this user) and whole size of the disk that holds a path, in bytes.
#[cfg(unix)]
pub fn disk_space(p: &Path) -> Option<(u64, u64)> {
    let c = std::ffi::CString::new(p.as_os_str().as_encoded_bytes()).ok()?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut s) } != 0 { return None; }
    Some((s.f_bavail as u64 * s.f_frsize as u64, s.f_blocks as u64 * s.f_frsize as u64))
}
#[cfg(windows)]
pub fn disk_space(p: &Path) -> Option<(u64, u64)> {
    use std::os::windows::ffi::OsStrExt;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetDiskFreeSpaceExW(dir: *const u16, free_to_caller: *mut u64, total: *mut u64, free: *mut u64) -> i32;
    }
    let wide: Vec<u16> = p.as_os_str().encode_wide().chain(Some(0)).collect();
    let (mut free, mut total, mut all_free) = (0u64, 0u64, 0u64);
    if unsafe { GetDiskFreeSpaceExW(wide.as_ptr(), &mut free, &mut total, &mut all_free) } == 0 { return None; }
    Some((free, total))
}
/// The bytes a file takes on its disk (its blocks; on Windows its length).
pub fn disk_bytes(m: &Metadata) -> u64 {
    #[cfg(unix)]
    { use std::os::unix::fs::MetadataExt; m.blocks() * 512 }
    #[cfg(windows)]
    { m.len() }
}
