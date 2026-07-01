//There's no rust library that would parse it for me, fuck my faggy ass again I guess...
//I should probably turn those cache parsers into real rust crates,
//maybe it will help other people with their projects instead of going trough the same hell as I do

//the-real-index is little endian
//it contains list of cached entries with lenght of their metadata i guess?

//use std::fs::File;
//use std::io::{self, Read, Seek, SeekFrom};

//fn find(data: &[u8], needle: &[u8]) -> Option<usize> {
//    data.windows(needle.len())
//        .position(|window| window == needle)
//}
//
//fn find_double_null(data: &[u8]) -> Option<usize> {
//    data.windows(2)
//        .position(|window| window == [0, 0])
//}
//
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let data = fs::read(
//        "/home/jakub/.cache/chromium/Default/Cache/Cache_Data/1b554cc6b572dd47_0"
//    )?;
//
//    if let Some(pos) = find(&data, b"HTTP/1.1") {
//        println!("HTTP headers found at offset {}", pos);
//
//        let end = find_double_null(&data[pos..])
//            .unwrap_or(data.len() - pos);
//
//        let headers = &data[pos..pos + end];
//
//        for part in headers.split(|b| *b == 0) {
//            if !part.is_empty() {
//                println!("{}", String::from_utf8_lossy(part));
//            }
//        }
//    } else {
//        println!("No HTTP headers found");
//    }
//
//    Ok(())
//}
fn main() {}
