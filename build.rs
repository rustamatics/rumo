extern crate zip;
extern crate walkdir;

use std::io::prelude::*;
use zip::write::FileOptions;

use walkdir::WalkDir;
use std::path::Path;
use std::fs::File;

fn main() {
    zip_it("./shell", "target/shell.zip").unwrap();
}

fn zip_it(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Ok(());
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(src_dir.to_string());

    let it = walkdir.into_iter();

    for dent in it.filter_map(|e| e.ok()) {
        let path = dent.path();
        let name = path.strip_prefix(Path::new(src_dir))
            .unwrap()
            .to_str()
            .unwrap();


        if path.is_file() {
            // println!("adding {:?} as {:?} ...", path, name);
            try!(zip.start_file(name, options));
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            try!(zip.write_all(&*buffer));
        }
    }

    try!(zip.finish());

    Ok(())
}
