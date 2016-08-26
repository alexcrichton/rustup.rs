extern crate rustup_dist;
extern crate rustup_utils;
#[macro_use]
extern crate error_chain;

extern crate libc;

use std::env;
use std::path::PathBuf;
use std::fmt;

macro_rules! warn {
    ($($arg: tt)*) => ($crate::warn_fmt(format_args!($($arg)*)))
}

fn warn_fmt(_args: fmt::Arguments) {}

error_chain! {
    links {
        rustup_utils::Error, rustup_utils::ErrorKind, Utils;
    }

    foreign_links {
        ::rustup_dist::Error, Temp;
    }
}


fn main() {
    drop(install());
}

fn install() {
    drop(do_pre_install_sanity_checks());
    home_mismatch();

    let install_res: Result<()> = (|| {
        let _env_file = PathBuf::from("a").join("env");
        let _env_str = format!("{:?}\n", shell_export_string());
        drop(rustup_utils::write_file());
        Ok(())
    })();

    if let Err(ref e) = install_res {
        ::report_error(e);
    }
}

fn do_pre_install_sanity_checks() -> Result<()> {
    let multirust_manifest_path
        = PathBuf::from("/usr/local/lib/rustlib/manifest-multirust");
    let rustc_manifest_path
        = PathBuf::from("/usr/local/lib/rustlib/manifest-rustc");
    let uninstaller_path
        = PathBuf::from("/usr/local/lib/rustlib/uninstall.sh");
    let multirust_meta_path
        = env::home_dir().map(|d| d.join(".multirust"));
    let multirust_version_path
        = multirust_meta_path.as_ref().map(|p| p.join("version"));
    let rustup_sh_path
        = env::home_dir().map(|d| d.join(".rustup"));
    let rustup_sh_version_path = rustup_sh_path.as_ref().map(|p| p.join("rustup-version"));

    let multirust_exists =
        multirust_manifest_path.exists() && uninstaller_path.exists();
    let rustc_exists =
        rustc_manifest_path.exists() && uninstaller_path.exists();
    let rustup_sh_exists =
        rustup_sh_version_path.map(|p| p.exists()) == Some(true);
    let old_multirust_meta_exists = if let Some(ref multirust_version_path) = multirust_version_path {
        multirust_version_path.exists() && {
            let version = rustup_utils::read_file();
            let version = version.unwrap_or(String::new());
            let version = version.parse().unwrap_or(0);
            let cutoff_version = 12; // First rustup version

            version < cutoff_version
        }
    } else {
        false
    };

    match (multirust_exists, old_multirust_meta_exists) {
        (true, false) => {
            warn!("it looks like you have an existing installation of multirust");
            warn!("rustup cannot be installed alongside multirust");
            warn!("run `{}` as root to uninstall multirust before installing rustup", uninstaller_path.display());
            return Err("cannot install while multirust is installed".into());
        }
        (false, true) => {
            warn!("it looks like you have existing multirust metadata");
            warn!("rustup cannot be installed alongside multirust");
            warn!("delete `{}` before installing rustup", multirust_meta_path.expect("").display());
            return Err("cannot install while multirust is installed".into());
        }
        (true, true) => {
            warn!("it looks like you have an existing installation of multirust");
            warn!("rustup cannot be installed alongside multirust");
            warn!("run `{}` as root and delete `{}` before installing rustup", uninstaller_path.display(), multirust_meta_path.expect("").display());
            return Err("cannot install while multirust is installed".into());
        }
        (false, false) => ()
    }

    if rustc_exists {
        return Err("cannot install while Rust is installed".into());
    }

    if rustup_sh_exists {
        warn!("it looks like you have existing rustup.sh metadata");
        warn!("rustup cannot be installed while rustup.sh metadata exists");
        warn!("delete `{}` to remove rustup.sh", rustup_sh_path.expect("").display());
        return Err("cannot install while rustup.sh is installed".into());
    }

    Ok(())
}

fn home_mismatch() -> bool {
    extern crate libc as c;

    use std::env;
    use std::ffi::CStr;
    use std::mem;
    use std::ops::Deref;
    use std::ptr;

    // test runner should set this, nothing else
    if env::var("A").as_ref().map(Deref::deref).ok() == Some("yes") {
        return false;
    }
    let mut pwd = unsafe { mem::uninitialized::<c::passwd>() };
    let mut pwdp: *mut c::passwd = ptr::null_mut();
    let mut buf = [0u8; 1024];
    let rv = unsafe { c::getpwuid_r(c::geteuid(), &mut pwd, mem::transmute(&mut buf), buf.len(), &mut pwdp) };
    if rv != 0 || pwdp == ptr::null_mut() {
        return false
    }
    let pw_dir = unsafe { CStr::from_ptr(pwd.pw_dir) }.to_str().ok();
    let env_home = env::var_os("HOME");
    let env_home = env_home.as_ref().map(Deref::deref);
    match (env_home, pw_dir) {
        (None, _) | (_, None) => false,
        (Some(ref eh), Some(ref pd)) => eh != pd
    }
}

fn shell_export_string() -> Result<String> {
    Ok(String::new())
}

fn report_error(e: &Error) {
    warn!("{}", e);

    for e in e.iter().skip(1) {
        warn!("caused by: {}", e);
    }

    if show_backtrace() {
        warn!("backtrace:");
        println!("");
        println!("{:?}", e.backtrace());
    } else {
    }

    fn show_backtrace() -> bool {
        use std::env;
        use std::ops::Deref;

        if env::var("RUST_BACKTRACE").as_ref().map(Deref::deref) == Ok("1") {
            return true;
        }

        for arg in env::args() {
            if arg == "-v" || arg == "--verbose" {
                return true;
            }
        }

        return false;
    }
}
