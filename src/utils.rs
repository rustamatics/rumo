
use zip;
use std;
use std::io;
use std::fs;
use std::path::Path;
use std::io::{Error};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn mkdirp(path: &str) -> Result<(), Error> {
    if ! Path::new(path).exists() {
        fs::create_dir(path)
    } else {
        Ok(())
    }
}

pub fn unzip_shell(data: &'static [u8], dst: String) -> bool {
    let reader = std::io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len()
    {
        let mut file = archive.by_index(i).unwrap();
        let sanitized_filename = sanitize_filename(file.name());
        let dst = format!("{}/{}", dst, sanitized_filename.to_str().unwrap());
        let outpath = Path::new(&dst[..]);

        create_directory(outpath.parent().unwrap_or(std::path::Path::new("")), None);

        let perms = convert_permissions(file.unix_mode());

        if (&*file.name()).ends_with("/") {
            create_directory(&outpath, perms);
        }
        else {
            write_file(&mut file, &outpath, perms);
        }
    }

    return true;
}

#[cfg(unix)]
fn convert_permissions(mode: Option<u32>) -> Option<fs::Permissions>
{
    match mode {
        Some(mode) => Some(fs::Permissions::from_mode(mode)),
        None => None,
    }
}
#[cfg(not(unix))]
fn convert_permissions(_mode: Option<u32>) -> Option<fs::Permissions>
{
    None
}

fn write_file(file: &mut zip::read::ZipFile, outpath: &std::path::Path, perms: Option<fs::Permissions>)
{
    let mut outfile = fs::File::create(&outpath).unwrap();
    io::copy(file, &mut outfile).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

fn create_directory(outpath: &std::path::Path, perms: Option<fs::Permissions>)
{
    fs::create_dir_all(&outpath).unwrap();
    if let Some(perms) = perms {
        fs::set_permissions(outpath, perms).unwrap();
    }
}

fn sanitize_filename(filename: &str) -> std::path::PathBuf
{
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    std::path::Path::new(no_null_filename)
        .components()
        .filter(|component| match *component {
            std::path::Component::Normal(..) => true,
            _ => false
        })
        .fold(std::path::PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        })
}
