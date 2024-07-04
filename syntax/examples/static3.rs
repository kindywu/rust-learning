#![allow(unused)]

fn main() {
    // let linux = "linux".to_string();
    // let pfs = PosixFileSystem { name: &linux };
    let pfs = PosixFileSystem { name: "linux" };
    test_fs(&pfs);
}

trait FileSystem: Send + Sync + 'static {}

// struct PosixFileSystem {
//     name: String,
// }
// impl FileSystem for PosixFileSystem {}

// struct PosixFileSystem<'a> {
//     name: &'a str,
// }
// impl<'a> FileSystem for PosixFileSystem<'a> {}

struct PosixFileSystem<'a> {
    name: &'a str,
}

// impl FileSystem for PosixFileSystem<'static> {}

impl<'a: 'static> FileSystem for PosixFileSystem<'a> {}

fn test_fs<F: FileSystem>(fs: &F) {}
