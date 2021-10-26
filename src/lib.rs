#![warn(missing_docs)]

//! Simple library for getting the version information of a `rustc`
//! compiler in runtime.
//!
//! The goal of this crate is to provide debug information to developers,
//! not to branch application logic based on compiler version. Please, don't
//! do that.
//!
//! # Example
//!
//! ```rust
//! extern crate rustc_version_runtime;
//!
//! println!("This was compiled using {:?}", rustc_version_runtime::version());
//! ```

extern crate rustc_version;
extern crate semver;
use semver::VersionReq;
pub use semver::Identifier;

pub use rustc_version::{Version, VersionMeta, Channel};
mod version {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
}
pub use version::version_meta;

/// Returns the `rustc` SemVer version.
pub fn version() -> Version {
    version_meta().semver
}

/// Check wether the `rustc` version matches the given SemVer
/// version requirement.
pub fn version_matches(req: &str) -> bool {
    // There is some issue checking requirements for pre-releases
    // https://github.com/steveklabnik/semver/issues/172
    // I believe users of this crate would expect 1.31.0-nightly to be greater than 1.30 and
    // equal to 1.31.0. This might not be the case, but I cannot see why.
    let mut v = version();
    v.pre = vec![];
    VersionReq::parse(req).unwrap().matches(&v)
}

#[test]
fn smoketest() {
    let v = version();
    assert!(v.major >= 1);
    assert!(v.minor >= 2);

    let v = version_meta();
    assert!(v.semver.major >= 1);
    assert!(v.semver.minor >= 2);

    assert!(version_matches(">= 1.2.0"));
}
