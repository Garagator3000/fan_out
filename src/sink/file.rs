use std::io::{Write, BufWriter};
use std::fs::File;
use log::debug;

pub struct FileSink {
    files: Vec<BufWriter<File>>,
}

pub fn create_file_sink(pattern: &str, path: &str, shards: u8) -> std::io::Result<FileSink> {
    let mut files = Vec::with_capacity(usize::from(shards));

    std::fs::create_dir_all(path)?;

    for i in 0..shards {
        let filename = pattern.replace("{}", &i.to_string());
        let filepath = std::path::Path::new(path).join(filename);
        let file = std::fs::File::create(filepath)?;
        files.push(BufWriter::new(file));
    }

    Ok(FileSink { files })
}

impl FileSink {
    pub fn flush(&mut self) -> std::io::Result<()> {
        for f in &mut self.files {
            f.flush()?;
        }
        Ok(())
    }
}

impl super::Sink for FileSink {
    fn write(&mut self, shard: usize, data: &[u8]) -> std::io::Result<()> {
        let writer = self
        .files
        .get_mut(shard)
        .ok_or_else(||{
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid shard")
        })?;

        writer.write_all(data)?;
        debug!("data wrote to shard {}", shard);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::sink::{Sink, file::create_file_sink};

    #[test]
    fn create_all_files() {
        let dir = tempfile::tempdir().unwrap();

        let _ = create_file_sink("shard_{}.log", dir.path().to_str().unwrap(), 3);

        for i in 0..3 {
            let path = dir.path().join(format!("shard_{i}.log"));
            assert!(path.exists())
        }
    }

    #[test]
    fn writes_to_correct_shard() {
        let dir = tempfile::tempdir().unwrap();

        let mut sink = create_file_sink(
            "shard_{}.log",
            dir.path().to_str().unwrap(),
            2,
        ).unwrap();

        sink.write(0, b"hello").unwrap();
        sink.write(1, b"world").unwrap();

        let _ = sink.flush();

        let s0 = fs::read_to_string(dir.path().join("shard_0.log")).unwrap();
        let s1 = fs::read_to_string(dir.path().join("shard_1.log")).unwrap();

        assert_eq!(s0, "hello");
        assert_eq!(s1, "world");
    }

    #[test]
    fn invalid_shard_returns_error() {
        let dir = tempfile::tempdir().unwrap();

        let mut sink = create_file_sink(
            "shard_{}.log",
            dir.path().to_str().unwrap(),
            2,
        ).unwrap();

        let res = sink.write(10, b"data");

        assert!(res.is_err());
    }
}
