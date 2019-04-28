use chrono::NaiveDate;
use roxmltree::Node;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Pisi {
    source: Source,
    package: Package,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Pisi {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let source: Source = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Source"))
            .ok_or_else(|| "Source node not found".to_owned())
            .and_then(TryInto::try_into)?;

        let package: Package = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Package"))
            .ok_or_else(|| "Package node not found".to_owned())
            .and_then(TryInto::try_into)?;

        Ok(Self { source, package })
    }
}

#[derive(Debug)]
pub struct Source {
    name: String,
    packager: User,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Source {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let name = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Name"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Name node not found".to_owned())?
            .to_owned();

        let packager = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Packager"))
            .ok_or_else(|| "Packager node not found".to_owned())
            .and_then(TryInto::try_into)?;

        Ok(Self { name, packager })
    }
}

#[derive(Debug)]
pub struct Package {
    name: String,
    summary: String,
    description: String,
    part_of: String,
    licenses: Vec<String>,
    runtime_dependencies: Vec<Dependency>,
    history: Vec<Update>,
    build_host: String,
    distribution: String,
    distribution_release: String,
    arch: String,
    installed_size: u64,
    package_format: String,
    source: Source,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Package {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let name = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Name"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Name node not found".to_owned())?
            .to_owned();

        let summary = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Summary"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Summary node not found".to_owned())?
            .to_owned();

        let description = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Description"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Description node not found".to_owned())?
            .to_owned();

        let part_of = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("PartOf"))
            .and_then(|n| n.text())
            .ok_or_else(|| "PartOf node not found".to_owned())?
            .to_owned();

        let licenses: Vec<String> = node
            .children()
            .filter(Node::is_element)
            .filter(|c| c.has_tag_name("License"))
            .filter_map(|n| n.text())
            .map(ToOwned::to_owned)
            .collect();

        if licenses.is_empty() {
            return Err("License node not found".to_owned());
        }

        let runtime_dependencies: Vec<Dependency> = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("RuntimeDependencies"))
            .ok_or_else(|| "RuntimeDependencies node not found")?
            .children()
            .filter(Node::is_element)
            .filter(|c| c.has_tag_name("Dependency"))
            .filter_map(|n| match Dependency::try_from(n) {
                Ok(v) => Some(v),
                Err(err) => {
                    eprintln!("{}", err);
                    None
                }
            })
            .collect();

        let history: Vec<Update> = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("History"))
            .ok_or_else(|| "History node not found")?
            .children()
            .filter(Node::is_element)
            .filter(|c| c.has_tag_name("Update"))
            .filter_map(|n| match Update::try_from(n) {
                Ok(v) => Some(v),
                Err(err) => {
                    eprintln!("{}", err);
                    None
                }
            })
            .collect();

        let build_host = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("BuildHost"))
            .and_then(|n| n.text())
            .ok_or_else(|| "BuildHost node not found".to_owned())?
            .to_owned();

        let distribution = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Distribution"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Distribution node not found".to_owned())?
            .to_owned();

        let distribution_release = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("DistributionRelease"))
            .and_then(|n| n.text())
            .ok_or_else(|| "DistributionRelease node not found".to_owned())?
            .to_owned();

        let arch = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Architecture"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Architecture node not found".to_owned())?
            .to_owned();

        let installed_size = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("InstalledSize"))
            .and_then(|n| n.text())
            .ok_or_else(|| "InstalledSize node not found".to_owned())?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let package_format = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("PackageFormat"))
            .and_then(|n| n.text())
            .ok_or_else(|| "PackageFormat node not found".to_owned())?
            .to_owned();

        let source: Source = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Source"))
            .ok_or_else(|| "Source node not found".to_owned())
            .and_then(TryInto::try_into)?;

        Ok(Self {
            name,
            summary,
            description,
            part_of,
            licenses,
            runtime_dependencies,
            history,
            build_host,
            distribution,
            distribution_release,
            arch,
            installed_size,
            package_format,
            source,
        })
    }
}

#[derive(Debug)]
pub struct User {
    name: String,
    email: String,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for User {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let name = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Name"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Name node not found".to_owned())?
            .to_owned();

        let email = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Email"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Email node not found".to_owned())?
            .to_owned();

        Ok(Self { name, email })
    }
}

#[derive(Debug)]
pub struct Dependency {
    release: u32,
    name: String,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Dependency {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let release = node
            .attribute("releaseFrom")
            .ok_or_else(|| "Dependency missing \"releaseFrom\" attribute")?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let name = node
            .text()
            .ok_or_else(|| "Dependency missing name")?
            .to_owned();

        Ok(Self { release, name })
    }
}

#[derive(Debug)]
pub struct Update {
    release: u32,
    date: NaiveDate,
    version: String,
    comment: String,
    packager: User,
}

impl<'a, 'd: 'a> TryFrom<Node<'a, 'd>> for Update {
    type Error = String;
    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let release = node
            .attribute("release")
            .ok_or_else(|| "Update missing \"release\" attribute")?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let date: NaiveDate = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Date"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Date node not found".to_owned())?
            .parse()
            .map_err(|err| format!("{:?}", err))?;

        let version = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Version"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Version node not found".to_owned())?
            .to_owned();

        let comment = node
            .children()
            .filter(Node::is_element)
            .find(|c| c.has_tag_name("Comment"))
            .and_then(|n| n.text())
            .ok_or_else(|| "Comment node not found".to_owned())?
            .to_owned();

        let packager: User = node.try_into()?;

        Ok(Self {
            release,
            date,
            version,
            comment,
            packager,
        })
    }
}
