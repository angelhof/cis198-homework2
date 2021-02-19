use std::fs::DirEntry;
use std::io::Result;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-find",
    about = "A command line utility for searching for files with regexes."
)]
struct CLI {
    #[structopt(short, long)]
    dirs: Vec<String>,
    #[structopt(short, long)]
    patterns: Vec<String>,
}

// // Only keep the dirs that actually exist
// fn filter_dirs(dirs: Vec<String>) {
//     dirs
// }

#[derive(Debug)]
struct FindFile {
    name: String,
    dir_in: String,
    size_bytes: u64,
}
impl FindFile {
    fn from(path: &Path) -> Option<FindFile> {
        // I am not sure if I should silence the second Option (since it might not be valid Unicode this should be warned somehow).
        let mut components = path.components();
        let name = components.next_back()?.as_os_str().to_str()?;
        let dir_in = components.as_path().to_str()?;
        Some(FindFile {
            name: name.to_owned(),
            dir_in: dir_in.to_owned(),
            size_bytes: 0,
        })
    }
}

fn handle_dir_entry(entry: DirEntry) -> Vec<FindFile> {
    match entry.file_type() {
        Ok(file_type) => {
            if file_type.is_dir() {
                get_dir_files_if_exists(entry.path().as_path())
            } else if file_type.is_file() {
                match FindFile::from(entry.path().as_path()) {
                    Some(file) => Vec::from([file]),
                    None => Vec::new(),
                }
            } else if file_type.is_symlink() {
                eprintln!("Warning: symlink was not followed: {:?}", entry.path());
                Vec::new()
            } else {
                // Should be unreachable
                unimplemented!()
            }
        }
        Err(_) => unimplemented!(),
    }
}

fn handle_dir_entry_if_exists(entry: Result<DirEntry>) -> Vec<FindFile> {
    match entry {
        Ok(entry) => handle_dir_entry(entry),
        // Q: It is fine if the entry cannot be read
        Err(_) => Vec::new(),
    }
}

// Walks a directory,
fn get_dir_files_if_exists(path: &Path) -> Vec<FindFile> {
    match path.read_dir() {
        Ok(read_dir) => read_dir.flat_map(handle_dir_entry_if_exists).collect(),
        // It is fine if the directory cannot be read
        Err(_) => Vec::new(),
    }
}

fn main() {
    let args = CLI::from_args();
    println!("Dirs: {:?}", args.dirs);
    println!("Patterns: {:?}", args.patterns);

    let path = Path::new(".");
    for entry in get_dir_files_if_exists(path) {
        println!("{:?}", entry);
    }
}
