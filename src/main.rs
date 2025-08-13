mod error;

use error::Result;

fn main() -> Result<()> {
    // load env profile
    dotenv::dotenv()?;

    let files = list_file(".")?;

    println!("Hello, world! {:?}", files);

    Ok(())
}

fn list_file(path: &str) -> Result<Vec<String>> {
    let files = std::fs::read_dir(path)
        .map_err(|_| "error file reading dir...")?
        .filter_map(|re| re.ok())
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    Ok(files)
}
