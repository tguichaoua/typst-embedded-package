use typst_embedded_package::{include_package, Package};

const CETZ: Package = include_package!("typst-packages" "preview" "cetz" (0, 3, 1));

const PACKAGES: [Package; 2] = include_package!(
    "typst-packages"
    [
        "preview" "cetz" (0, 3, 1),
        "preview" "oxifmt" (0, 2, 0),
    ]
);

#[test]
fn include_package() {
    debug_assert_eq!(CETZ.namespace, "preview");
    debug_assert_eq!(CETZ.name, "cetz");
    debug_assert_eq!(CETZ.version.major, 0);
    debug_assert_eq!(CETZ.version.minor, 3);
    debug_assert_eq!(CETZ.version.patch, 1);

    debug_assert_eq!(PACKAGES[0].namespace, "preview");
    debug_assert_eq!(PACKAGES[0].name, "cetz");
    debug_assert_eq!(PACKAGES[0].version.major, 0);
    debug_assert_eq!(PACKAGES[0].version.minor, 3);
    debug_assert_eq!(PACKAGES[0].version.patch, 1);

    debug_assert_eq!(PACKAGES[1].namespace, "preview");
    debug_assert_eq!(PACKAGES[1].name, "oxifmt");
    debug_assert_eq!(PACKAGES[1].version.major, 0);
    debug_assert_eq!(PACKAGES[1].version.minor, 2);
    debug_assert_eq!(PACKAGES[1].version.patch, 0);
}
