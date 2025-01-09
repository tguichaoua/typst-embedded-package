#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Embed typst packages directly in the binary.
//!
//! # How to use
//!
//! 1. Download package archives from <https://typst.app/universe/search/?kind=packages>
//! 1. Move those archives somewhere in the `src` directory
//! 1. Include the archives with [`include_package!`]
//! 1. Read the content of the archive with [`Package::read_archive`] (requires the `read-archive` feature)
//!
//! ```ignore
//! # use typst_embedded_package::{include_package, Package};
//! // Embed the package located at "/src/typst-packages/preview/cetz-0.3.1.tar.gz"
//! const CETZ: Package = include_package!("typst-packages" "preview" "cetz" (0, 3, 1));
//!
//! // Embed multiple packages.
//! const PACKAGES: [Package; 2] = include_package!(
//!     "typst-packages"
//!     [
//!         "preview" "cetz" (0, 3, 1),
//!         "preview" "oxifmt" (0, 2, 0),
//!     ]
//! );
//! ```

#[cfg(feature = "read-archive")]
mod archive;
#[cfg(feature = "read-archive")]
pub use archive::*;

use typst::diag::EcoString;
use typst::syntax::package::{PackageSpec, PackageVersion};

/// Information about an embedded package.
///
/// Created by the [`include_package!`] macro.
#[derive(Debug, Clone)]
pub struct Package {
    /// The namespace the package lives in.
    pub namespace: &'static str,
    /// The name of the package within its namespace.
    pub name: &'static str,
    /// The package's version.
    pub version: PackageVersion,
    /// The content of the package's tgz archive.
    pub archive: &'static [u8],
}

impl Package {
    /// Returns the [`PackageSpec`] of the package.
    pub fn spec(&self) -> PackageSpec {
        PackageSpec {
            namespace: EcoString::from(self.namespace),
            name: EcoString::from(self.name),
            version: self.version,
        }
    }
}

#[doc(hidden)]
pub mod __private {
    pub use typst::syntax::package::PackageVersion;
}

/// Embed one or more typst package as tgz archive.
///
/// This macro returns an [`Package`] or an array of [`Package`].
///
/// # Usage
///
/// Embed package archive located at `src/{root}/{namespace}/{name}-{X}.{Y}.{Z}.tar.gz`.
/// ```ignore
/// include_package!( {root} {namespace} {name} ({X}, {Y}, {Z}) );
/// include_package!(
///     {root}
///     [
///         {namespace} {name} ({X}, {Y}, {Z}),
///         {namespace} {name} ({X}, {Y}, {Z}),
///         ...
///     ]
/// );
/// ```
#[macro_export]
macro_rules! include_package {
    (
        $root:literal $namespace:literal $name:literal ($major:literal, $minor:literal, $patch:literal)
    ) => {
        $crate::Package {
            namespace: $namespace,
            name: $name,
            version: $crate::__private::PackageVersion {
                major: $major,
                minor: $minor,
                patch: $patch,
            },
            archive: include_bytes!(concat!(
                $root,
                "/",
                $namespace,
                "/",
                $name,
                "-",
                stringify!($major),
                ".",
                stringify!($minor),
                ".",
                stringify!($patch),
                ".tar.gz"
            )),
        }
    };

    (
        $root:literal
        [
            $( $namespace:literal $name:literal ($major:literal, $minor:literal, $patch:literal) ),* $(,)?
        ]
    ) => {
        [
            $(
                $crate::include_package!($root $namespace $name ($major, $minor, $patch) )
            ),*
        ]
    };

}
