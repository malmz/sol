mod eopkg;

use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "info")]
    Info {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Info { path } => info(&path).unwrap(),
    }
}

fn info(path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;
    let package = eopkg::EopkgPackage::from_file(&mut file)?;
    println!("{:#?}", package);
    Ok(())
}
