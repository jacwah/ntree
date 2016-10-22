use std::io;
use std::fs;
use std::path::Path;
use processor::TreeProcessor;

pub fn process<T: TreeProcessor>(dir: &Path, procor: &mut T) -> io::Result<()> {
    let entries = try!(fs::read_dir(dir));
    let entries: Vec<_> = entries.collect();

    procor.open_dir(dir, entries.len());

    for entry in entries {
        let entry = try!(entry);
        let path = entry.path();

        let file_type = try!(entry.file_type());

        if file_type.is_dir() {
            try!(process(&path, procor));
        } else {
            procor.file(&path);
        }
    }

    procor.close_dir();

    Ok(())
}
