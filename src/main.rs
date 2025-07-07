

use std::fs;
use std::io;
use std::path::Path;
use once_cell::sync::Lazy;

static PATH: Lazy<&Path> = Lazy::new(|| Path::new("/home/daner/Desktop/test"));

fn main() -> io::Result<()> {
    if !PATH.is_dir() {
        return Ok(());
    };

    match fs::read_dir(*PATH) {
        Ok(entries) => {
            for entry_result in entries {
                match entry_result {
                    Ok(_entry) => {
                        let entrypath = _entry.path();
                        let actual_dir_path = entrypath.to_str().unwrap();

                        let parent_dir_path = entrypath.parent().unwrap().display();
                        let letter = _entry.file_name().to_string_lossy().chars().next().unwrap_or('_').to_uppercase();
                        let filename = entrypath.file_name().unwrap().to_string_lossy();

                        let new_dir_path = format!("{:?}/{}/{:?}", parent_dir_path, letter, filename);
                        
                        fs::create_dir_all(format!("{:?}/{}", parent_dir_path, letter))?;

                        println!("{}", actual_dir_path);
                        println!("{}", parent_dir_path);
                        println!("{}", letter);
                        println!("{}", filename);
                        println!("{}", new_dir_path);
                        println!("{}", "------------------------------------------------------------");
                        
                        fs::rename(actual_dir_path, new_dir_path)?;
                    },
                    Err(e) => eprintln!("Error reading entry: {}", e),
                };
            };
        },
        Err(e) => eprintln!("Failed to read directory: {}", e),
    };

    Ok(())
}
