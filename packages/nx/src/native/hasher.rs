use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::trace;
use xxhash_rust::xxh3;

pub fn hash(content: &[u8]) -> String {
    xxh3::xxh3_64(content).to_string()
}

#[napi]
pub fn hash_array(input: Vec<String>) -> String {
    let joined = input.join(",");
    let content = joined.as_bytes();
    hash(content)
}

#[napi]
pub fn hash_file(file: String) -> Option<String> {
    hash_file_path(file)
}

#[inline]
pub fn hash_file_path<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    let Ok(file) = File::open(path) else {
        trace!("could not open file: {path:?}");
        return None;
    };

    let mut buffer = BufReader::new(file);
    let Ok(content) = buffer.fill_buf() else {
        trace!("could not read file: {path:?}");
        return None;
    };

    Some(hash(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;

    ///
    /// Setup a temporary directory to do testing in
    ///
    fn setup_fs() -> TempDir {
        let temp = TempDir::new().unwrap();
        temp.child("test.txt").write_str("content").unwrap();
        temp.child("foo.txt").write_str("content1").unwrap();
        temp.child("bar.txt").write_str("content2").unwrap();
        temp.child("baz")
            .child("qux.txt")
            .write_str("content@qux")
            .unwrap();
        temp.child("node_modules")
            .child("node-module-dep")
            .write_str("content")
            .unwrap();
        temp
    }

    #[test]
    fn it_hashes_a_file() {
        // handle non existent files
        let content = hash_file("".into());
        assert!(content.is_none());

        let temp_dir = setup_fs();

        let test_file_path = temp_dir.display().to_string() + "/test.txt";
        let content = hash_file(test_file_path);

        assert_eq!(content.unwrap(), "6193209363630369380");
    }
}
