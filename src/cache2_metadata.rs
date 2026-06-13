//Last 4 bytes of the every firefox's cache file is the byte number where the metadata starts.
//There is NO DOCUMENTATION about this in firefox source, wow, thanks mozzila

use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct CacheKey<'a> {
    partition_key: Option<&'a str>,
    url: &'a str,
}

pub fn read_metadata(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;

    let file_size = file.seek(SeekFrom::End(0))?;

    // Firefox stores metadata start offset in last 4 bytes.
    file.seek(SeekFrom::End(-4))?;

    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;

    let metadata_start = u32::from_be_bytes(buf) as u64;

    if metadata_start > file_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "metadata offset beyond file size",
        ));
    }

    let metadata_len = file_size - metadata_start - 4;

    file.seek(SeekFrom::Start(metadata_start))?;

    let mut metadata = vec![0u8; metadata_len as usize];
    file.read_exact(&mut metadata)?;

    Ok(metadata)
}

pub fn parse_cache_key(metadata: &[u8]) -> Option<CacheKey<'_>> {
    const PREFIX: &[u8] = b"partitionKey=";

    let start = metadata.windows(PREFIX.len()).position(|w| w == PREFIX)?;

    let start = start + PREFIX.len();

    let end = metadata[start..]
        .iter()
        .position(|&b| b == 0)
        .map(|p| start + p)?;

    let text = std::str::from_utf8(&metadata[start..end]).ok()?;

    let (partition_key, url) = text.split_once(",:")?;

    Some(CacheKey {
        partition_key: Some(partition_key),
        url,
    })
}

pub fn filename_from_url(url: &str) -> &str {
    url.rsplit('/').next().unwrap_or(url)
}
/*
fn main() -> io::Result<()> {
    let path = env::args()
        .nth(1)
        .expect("Usage: firefox_cached_file_metadata_parser <cache_file>");

    let metadata = read_metadata(&path)?;

    match parse_cache_key(&metadata) {
        Some(key) => {
            println!("Partition Key: {:?}", key.partition_key);
            println!("URL: {}", key.url);
            println!("Filename: {}", filename_from_url(key.url));
        }
        None => {
            println!("No cache key found");
        }
    }

    Ok(())
}
*/
