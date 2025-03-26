use std::fs;
use std::io;
use std::path::Path;

fn copy_dir(source: &Path, destination: &Path) -> io::Result<()> {
    if !source.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Source is not a directory",
        ));
    }
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

fn search(dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let data = entry.metadata()?;
        let path = entry.path();

        if data.is_file() {
            if let Some(ex) = path.extension() {
                if ex == "csv" && data.len() > 1 {
                    println!("{} length {}", path.display(), data.len());
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    fs::write("data.csv", "lab 3 data")?;

    for i in 1..4 {
        let dir = format!("{}", i);
        let dir_path = Path::new(&dir);
        if !dir_path.exists() {
            fs::create_dir(dir_path)?;
        }
        fs::write(dir_path.join("empty.csv"), "")?;
    }

    fs::copy("data.csv", "data_copy.csv")?;

    let source = Path::new("1");
    let destination = Path::new("1_copy");
    copy_dir(source, destination)?;

    println!("Press 'y' to search:");
    if read_user_input()? == "y" {
        search(Path::new("."))?;
    }

    println!("Press 'y' to delete files and directories:");
    if read_user_input()? == "y" {
        fs::remove_file("data_copy.csv")?;
        fs::remove_dir_all("1_copy")?;

        let dirs_to_delete = vec![Path::new("2"), Path::new("3")];

        for dir_path in dirs_to_delete {
            if dir_path.exists() {
                fs::remove_dir_all(dir_path)?;
            }
        }
    }

    println!("Press 'y' to show directory content:");
    if read_user_input()? == "y" {
        let cont_path = Path::new("1");

        if cont_path.is_dir() {
            println!("Content of directory:");
            for entry in fs::read_dir(cont_path)? {
                let entry = entry?;
                let path = entry.path();
                println!(" {}", path.display());
            }
        }
    }

    println!("Press 'y' to show file and directory properties:");
    if read_user_input()? == "y" {
        let file_prop_path = Path::new("data.csv");
        let file_meta = fs::metadata(file_prop_path)?;
        println!("Size: {} bytes", file_meta.len());
        println!("Type: {:?}", file_meta.file_type());
        println!("Permissions: {:?}", file_meta.permissions());
        println!("Modified: {:?}", file_meta.modified());

        let dir_prop_path = Path::new("1");
        let dir_meta = fs::metadata(dir_prop_path)?;
        if dir_meta.is_dir() {
            println!("Size: {} bytes", dir_meta.len());
            println!("Type: {:?}", dir_meta.file_type());
            println!("Permissions: {:?}", dir_meta.permissions());
            println!("Modified: {:?}", dir_meta.modified());
        }
    }

    println!("Press 'y' to delete all files, created by this programm:");
    if read_user_input()? == "y"{
        fs::remove_file("data.csv")?;
        fs::remove_dir_all("1")?;
    }

    Ok(())
}

fn read_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}