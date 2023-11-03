
use std::fs::{self};

/**
 *  读取目录下文件名，并写到文件里
 * 
 * 
 */

fn main() -> std::io::Result<()> {
    let mut filenames = vec![];
    for entry in fs::read_dir(".")?{
        let dir = entry?;
        let path = dir.path();

        if path.is_dir(){
            println!("{:?}", path);
        } else {
            let name = path.file_name().unwrap();
            let mut name_str = String::new();

            if let Some(str) = name.to_str() {
                name_str = String::from(str);
            }

            filenames.push(name_str);
        }
    }
    //println!("{:?}", filenames);
    let _ = fs::write("./filenames.txt", filenames.join("\n"));
    Ok(())
}
