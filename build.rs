extern crate rustc_version;
extern crate semver;

use std::fs::File;
use std::io::Write;

use semver::Identifier;
use rustc_version::{version_meta, Channel};

fn identifier_to_source(id: &Identifier) -> String {
    match *id {
        Identifier::Numeric(ref n) => format!("Identifier::Numeric({})", n),
        Identifier::AlphaNumeric(ref n) => format!("Identifier::AlphaNumeric({:?}.to_owned())", n),
    }
}

fn identifiers_to_source(ids: &Vec<Identifier>) -> String {
    let mut r = "vec![".as_bytes().to_vec();
    for id in ids {
        write!(r, "{}", identifier_to_source(id)).unwrap();
    }
    write!(r, "]").unwrap();
    String::from_utf8(r).unwrap()
}

fn main() {
    let mut f = File::create("src/version.rs").unwrap();

    write!(f, "
            use rustc_version::{{Channel, VersionMeta}};
            use semver::{{Identifier, Version}};
            ").unwrap();
    let version = version_meta();

    write!(f, "
            /// Returns the `rustc` SemVer version and additional metadata¬
            /// like the git short hash and build date.¬
            pub fn version_meta() -> VersionMeta {{
                VersionMeta {{
                    semver: Version {{
                        major: {major},
                        minor: {minor},
                        patch: {patch},
                        pre: {pre},
                        build: {build},
                    }},
                    git_short_hash: \"{hash}\".to_owned(),
                    date: \"{date}\".to_owned(),
                    channel: Channel::{channel},
                }}
            }}
            ",
            major = version.semver.major,
            minor = version.semver.minor,
            patch = version.semver.patch,
            pre = identifiers_to_source(&version.semver.pre),
            build = identifiers_to_source(&version.semver.build),
            hash = version.git_short_hash,
            date = version.date,
            channel = match version.channel {
                Channel::Dev => "Dev",
                Channel::Nightly => "Nightly",
                Channel::Beta => "Beta",
                Channel::Stable => "Stable",
            }
            ).unwrap();
}
