//Last 4 bytes of the every firefox's cache file is the byte number where the metadata starts.
//I am so smart that it took me 4 days to realise that...

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

fn read_last_i32_be(path: &str) -> io::Result<i32> {
    let mut file = File::open(path)?;

    file.seek(SeekFrom::End(-4))?;

    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;

    Ok(i32::from_be_bytes(buffer))
}

fn read_metadata(path: &str) -> io::Result<String> {
	let mut file = File::open(path)?;
	let file_size = file.seek(SeekFrom::End(0))?;
	//Get metadata beggining byte number
    file.seek(SeekFrom::End(-4))?;
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    let metadata_start = i32::from_be_bytes(buffer) as u64;
    let metadata_len = file_size - metadata_start - 4;
    file.seek(SeekFrom::Start(metadata_start))?;
    
    let mut metadata_raw = vec![0u8; metadata_len as usize];
	file.read_exact(&mut metadata_raw)?;
	
	let metadata_str = String::from_utf8_lossy(&metadata_raw).into_owned();
	
	Ok(metadata_str)
}

fn main() -> io::Result<()> {
    let path = "/home/jakub/.cache/librewolf/bgvrjjel.default-default/cache2/entries/1FEB142DDC5E0455B6006F32987B4E2FDEA3DCA9";
    //let value = read_last_i32_be(path)?;

	let metadata = read_metadata(path);
	println!("{}" , metadata?);
    Ok(())
}
