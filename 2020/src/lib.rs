use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn read<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut result = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut result)?;

    Ok(result)
}
