# Typst-Embedded-Package

Embed typst packages directly in the binary.

# How to use

1.  Download package archives from <https://typst.app/universe/search/?kind=packages>
1.  Move those archives somewhere in the `src` directory
1.  Include the archives with `include_package!`
1.  Read the content of the archive with `Package::read_archive` (requires the `read-archive` feature)

```rust
// Embed the package located at "/src/typst-packages/preview/cetz-0.3.1.tar.gz"
const CETZ: Package = include_package!("typst-packages" "preview" "cetz" (0, 3, 1));

// Embed multiple packages.
const PACKAGES: [Package; 2] = include_package!(
    "typst-packages"
    [
        "preview" "cetz" (0, 3, 1),
        "preview" "oxifmt" (0, 2, 0),
    ]
);
```
