use std::io::{Error, Result};
use std::path::{Path, PathBuf};

pub fn get_home() -> Result<PathBuf> {
    if let Ok(home) = std::env::var("TLDR_CACHE_DIR") {
        return Ok(home.into());
    }
    if let Ok(home) = std::env::var("HOME") {
        return Ok(home.into());
    }

    let pid = unsafe { libc::getpwuid(libc::getuid()) };
    if !pid.is_null() {
        let home = unsafe { std::ffi::CStr::from_ptr((*pid).pw_dir) };
        return Ok(home.to_string_lossy().to_string().into());
    }

    Err(Error::new(
        std::io::ErrorKind::NotFound,
        "Home directory not found",
    ))
}

pub fn get_platform() -> &'static str {
    let mut sys;
    unsafe {
        sys = std::mem::zeroed();
        libc::uname(&mut sys);
    }
    let os = unsafe { std::ffi::CStr::from_ptr(sys.sysname.as_ptr()) };
    let os = os.to_string_lossy().to_string();
    match os.as_str() {
        "Linux" => "linux",
        "Darwin" => "osx",
        "SunOS" => "sunos",
        "Windows" => "windows",
        _ => "common",
    }
}

pub fn download_file<P: AsRef<Path>>(url: &str, path: P) -> Result<()> {
    let mut resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                format!("Error downloading file {url}"),
            ))
        }
    };
    let mut file = std::fs::File::create(path)?;
    std::io::copy(&mut resp, &mut file)?;
    Ok(())
}

// pub fn download_content(url: &str) -> Result<String> {
//     match reqwest::blocking::get(url) {
//         Ok(resp) => match resp.text() {
//             Ok(text) => Ok(text),
//             Err(_) => Err(Error::new(std::io::ErrorKind::Other, format!("Error downloading {url}"))),
//         },
//         Err(_) => Err(Error::new(std::io::ErrorKind::Other, format!("Error downloading {url}"))),
//     }
// }

pub fn unzip_file<P: AsRef<Path>>(src: P, dest: P) -> Result<()> {
    let file = std::fs::File::open(src)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(dest)?;
    Ok(())
}
