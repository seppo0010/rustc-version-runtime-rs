extern crate rustc_version;
extern crate semver;

use std::fs::File;
use std::io::Write;
use std::{env, path};

use semver::Identifier;
use rustc_version::{version_meta, Channel};

fn identifier_to_source(id: &Identifier) -> String {
    match *id {
        Identifier::Numeric(ref n) => format!("semver::Identifier::Numeric({})", n),
        Identifier::AlphaNumeric(ref n) => format!("semver::Identifier::AlphaNumeric({:?}.to_owned())", n),
    }
}

fn identifiers_to_source(ids: &Vec<Identifier>) -> String {
    let mut r = "vec![".as_bytes().to_vec();
    for id in ids {
        write!(r, "{}, ", identifier_to_source(id)).unwrap();
    }
    write!(r, "]").unwrap();
    String::from_utf8(r).unwrap()
}

fn main() {
    let mut path = path::PathBuf::from(env::var_os("OUT_DIR").unwrap());
    path.push("version.rs");
    let mut f = File::create(&path).unwrap();

    let version = version_meta().expect("Failed to read rustc version.");

    write!(f, "
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {{
                VersionMeta {{
                    semver: Version {{
                        major: {major},
                        minor: {minor},
                        patch: {patch},
                        pre: {pre},
                        build: {build},
                    }},
                    host: \"{host}\".to_owned(),
                    short_version_string: \"{short_version_string}\".to_owned(),
                    commit_hash: {commit_hash},
                    commit_date: {commit_date},
                    build_date: {build_date},
                    channel: Channel::{channel},
                }}
            }}
            ",
            major = version.semver.major,
            minor = version.semver.minor,
            patch = version.semver.patch,
            pre = identifiers_to_source(&version.semver.pre),
            build = identifiers_to_source(&version.semver.build),
            commit_hash = version.commit_hash.map(|h| format!("Some(\"{}\".to_owned())", h)).unwrap_or("None".to_owned()),
            commit_date = version.commit_date.map(|h| format!("Some(\"{}\".to_owned())", h)).unwrap_or("None".to_owned()),
            build_date = version.build_date.map(|h| format!("Some(\"{}\".to_owned())", h)).unwrap_or("None".to_owned()),
            host = version.host,
            short_version_string = version.short_version_string,
            channel = match version.channel {
                Channel::Dev => "Dev",
                Channel::Nightly => "Nightly",
                Channel::Beta => "Beta",
                Channel::Stable => "Stable",
            }
            ).unwrap();
}
