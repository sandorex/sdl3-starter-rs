//! Contains SSL certificate and logic for it

use std::{ffi::CString, path::Path, str::FromStr};

const SSL_CERTIFICATE: &[u8] = include_bytes!("cacert.pem");

/// Enables builtin SSL certificate, writes the certificate to the path provided make sure the path
/// is actually safe and private to prevent someone from tampering with it
///
/// This is a fallback solution, please try using system certificates first
pub fn enable_builtin_ssl_certificate<P: AsRef<Path>>(safe_path: P) {
    let path = safe_path.as_ref().join("cacert.pem");

    // TODO this will overwrite the certificate each time but does it matter? its 200kB
    // write the certificate
    std::fs::write(&path, SSL_CERTIFICATE)
        .expect(&format!("could not write certificate to {path:?}"));

    let path_c = CString::from_str(&path.to_string_lossy())
        .expect(&format!("could not convert path {path:?} into CString"));

    if let Err(err) = unsafe { git2::opts::set_ssl_cert_file(path_c) } {
        panic!("could not set ssl certificate file to {path:?}: {err}");
    }
}
