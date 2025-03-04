use std::io;
use std::io::Read as _;

use typst::foundations::Bytes;
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};

/// A file from a package's archive.
pub enum File {
    /// A [`Source`] file, i.e. a file with the `.typ` extension.
    Source(Source),
    /// A non-source file.
    File(FileId, Bytes),
}

/// Reads a tgz archive and calls the `inspect` callback for each file.
pub fn inspect_archive(
    spec: PackageSpec,
    tar_gz: impl io::Read,
    mut inspect: impl FnMut(File),
) -> io::Result<()> {
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    let entries = archive.entries()?;

    for entry in entries {
        let mut entry = entry?;
        let path = entry.path()?;

        let is_source_file = path.extension().is_some_and(|ext| ext == "typ");

        let file_id = FileId::new(Some(spec.clone()), VirtualPath::new(path));

        let file = if is_source_file {
            let mut content = String::new();
            entry.read_to_string(&mut content)?;

            File::Source(Source::new(file_id, content))
        } else {
            let mut content = Vec::new();
            entry.read_to_end(&mut content)?;

            File::File(file_id, Bytes::new(content))
        };

        inspect(file)
    }

    Ok(())
}

/// Reads all the files of a tgz archive.
pub fn read_archive(spec: PackageSpec, tar_gz: impl io::Read) -> io::Result<Vec<File>> {
    let mut files = Vec::new();
    inspect_archive(spec, tar_gz, |file| files.push(file))?;
    Ok(files)
}

impl super::Package {
    /// Reads the package's archive and calls `inspect` for each files.
    pub fn inspect_archive(&self, inspect: impl FnMut(File)) -> io::Result<()> {
        inspect_archive(self.spec(), self.archive, inspect)
    }

    /// Reads all the files from the package's archive.
    pub fn read_archive(&self) -> io::Result<Vec<File>> {
        read_archive(self.spec(), self.archive)
    }
}
