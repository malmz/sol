use roxmltree::Node;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Files {
    files: Vec<File>,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Files {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        if node.has_tag_name("Files")
            && node
                .children()
                .filter(Node::is_element)
                .all(|c| c.has_tag_name("File"))
        {
            let files: Vec<Result<File, String>> = node
                .children()
                .filter(Node::is_element)
                .map(TryInto::try_into)
                .collect();

            if !files.is_empty() {
                if let Some(err) = files.iter().find_map(|n| match n {
                    Err(err) => Some(err),
                    Ok(_) => None,
                }) {
                    return Err(err.clone());
                }

                let files = files.into_iter().map(Result::unwrap).collect();

                Ok(Self { files })
            } else {
                Err("File node not found".to_owned())
            }
        } else {
            Err("Files node not found".to_owned())
        }
    }
}

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    file_type: Filetype,
    uid: u32,
    gid: u32,
    mode: u32,
    hash: String,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for File {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let path: PathBuf = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Path"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Path node not found".to_owned())?
            .into();

        let file_type: Filetype = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Type"))
            .ok_or_else(|| "Type node not found".to_owned())
            .and_then(TryInto::try_into)?;

        let uid: u32 = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Uid"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Uid node not found".to_owned())?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let gid: u32 = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Gid"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Gid node not found".to_owned())?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let mode: u32 = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Mode"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Mode node not found".to_owned())
            .and_then(|s| u32::from_str_radix(s, 8).map_err(|err| format!("{:?}", err)))?;

        let hash = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Hash"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Hash node not found".to_owned())?
            .to_owned();

        Ok(Self {
            path,
            file_type,
            uid,
            gid,
            mode,
            hash,
        })
    }
}

#[derive(Debug)]
pub enum Filetype {
    Executable,
    Library,
    Data,
    Man,
    Doc,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Filetype {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let file_type_str = node.text().ok_or_else(|| "Type node not found")?;

        Ok(match file_type_str {
            "executable" => Filetype::Executable,
            "library" => Filetype::Library,
            "data" => Filetype::Data,
            "man" => Filetype::Man,
            "doc" => Filetype::Doc,
            err => return Err(format!("Unknown file type {}", err)),
        })
    }
}
