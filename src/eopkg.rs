pub mod files;
pub mod metadata;

pub use self::files::Files;
pub use self::metadata::Pisi;

use roxmltree::Document;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::Read;
use zip::read::ZipArchive;

#[derive(Debug)]
pub struct EopkgPackage {
    metadata: Pisi,
    files: Files,
}

impl EopkgPackage {
    pub fn from_file(file: &mut File) -> io::Result<Self> {
        let mut arc = ZipArchive::new(file)?;

        let metadata: Pisi = {
            let mut data_file = arc.by_name("metadata.xml")?;
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let doc: Document = Document::parse(&data).expect("Error parsing metadata xml");
            let root_element = doc.root_element();
            root_element.try_into().expect("Error parsing metadata xml")
        };

        let files: Files = {
            let mut data_file = arc.by_name("files.xml")?;
            let mut data = String::new();
            data_file.read_to_string(&mut data)?;
            let doc: Document = Document::parse(&data).expect("Error parsing metadata xml");
            let root_element = doc.root_element();
            root_element.try_into().expect("Error parsing metadata xml")
        };

        Ok(Self { metadata, files })
    }
}
