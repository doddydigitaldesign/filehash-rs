use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::{Error, Read},
    path::Path,
};

/// ```rust
/// // A simple example of a program that queries the user
/// // for a file to hash and prints the result.
/// # use std::error::Error;
/// # use filehash_rs::filehash;
/// fn main() -> Result<(), Box<dyn Error>> {
///    let mut input_string = String::new();
///    println!("Path to the file:");
///
///    std::io::stdin().read_line(&mut input_string)?;
/// #  input_string = String::from("./mock/textfile.txt");
///
///    let result = filehash(&mut input_string);
///
///    match result {
///        Ok(value) => {
///            println!("{}", value);
///            return Ok(());
///        }
///        Err(err) => {
///            println!("Error: {}", err);
///            return Err(Box::new(err));
///        }
///    }
/// }
/// ```

pub fn filehash(file_path: &mut String) -> Result<u64, Error> {
    let tmp = file_path.trim();
    let path_input = Path::new(&tmp);

    let path_canonicalized = path_input.canonicalize()?;
    let path_os_string = path_canonicalized.as_os_str();
    let mut buffer: Vec<u8> = Vec::new();

    let mut f = File::open(path_os_string)?;
    f.read_to_end(&mut buffer)?;

    let mut hasher = DefaultHasher::new();
    buffer.hash(&mut hasher);
    let finished_hash = hasher.finish();
    return Ok(finished_hash);
}

#[cfg(test)]
mod tests {
    use super::filehash;

    #[test]
    fn it_parses_png_correctly() {
        let mut mock_file = String::from("./mock/imagefile.png");
        let result = filehash(&mut mock_file);
        assert_eq!(result.unwrap(), 16483649873581620343u64);
    }

    #[test]
    fn it_parses_txt_correctly() {
        let mut mock_file = String::from("./mock/textfile.txt");
        let result = filehash(&mut mock_file);
        assert_eq!(result.unwrap(), 17875805651733834705u64);
    }

    #[test]
    fn it_parses_json_correctly() {
        let mut mock_file = String::from("./mock/jsonfile.json");
        let result = filehash(&mut mock_file);
        assert_eq!(result.unwrap(), 17809943400347874249u64);
    }

    #[test]
    fn it_parses_wmv_correctly() {
        let mut mock_file = String::from("./mock/BEAMextract_final_revB.wmv");
        let result = filehash(&mut mock_file);
        assert_eq!(result.unwrap(), 17778933049671139411u64);
    }
}
