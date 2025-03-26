use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn create_file(filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "Ім'я,Вік,Місто,Професія,Країна")?;
    writeln!(file, "Іван,30,Київ,Інженер,Україна")?;
    writeln!(file, "Марія,25,Львів,Вчитель,Україна")?;
    writeln!(file, "Петро,35,Харків,Художник,Україна")?;
    writeln!(file, "Ольга,28,Одеса,Лікар,Україна")?;
    writeln!(file, "Андрій,40,Дніпро,Менеджер,Україна")?;
    writeln!(file, "Наталія,32,Вінниця,Розробник,Україна")?;
    writeln!(file, "Юрій,27,Житомир,Кухар,Україна")?;
    writeln!(file, "Софія,38,Чернівці,Дизайнер,Україна")?;
    writeln!(file, "Віктор,45,Тернопіль,Пілот,Україна")?;
    writeln!(file, "Людмила,29,Полтава,Письменник,Україна")?;
    Ok(())
}

fn create_dir(dir_name: &str) -> std::io::Result<()> {
    let dir = Path::new(dir_name);
    if !dir.exists() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

fn copy_file(source: &str, destination: &str) -> std::io::Result<()> {
    if Path::new(source).exists() {
        fs::copy(source, destination)?;
    } else {
        println!("File '{}' is not exist.", source);
    }
    Ok(())
}

fn move_file(source: &str, destination: &Path) -> std::io::Result<()> {
    fs::rename(source, destination)?;
    Ok(())
}

fn copy_dir(source: &Path, destination: &Path) -> std::io::Result<()> {
    if !source.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Source is not a dir",
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

fn find_files(dir: &Path) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                    if extension == "csv" && metadata.len() > 100 {
                        println!("length {}, {}", metadata.len(), path.display());
                    }
                }
            }
        }
    }
    Ok(())
}

fn remove_file(filename: &str) -> std::io::Result<()> {
    fs::remove_file(filename)?;
    println!("File '{}' removed.", filename);
    Ok(())
}

fn remove_directory(dir_name: &str) -> std::io::Result<()> {
    fs::remove_dir_all(dir_name)?;
    println!("Directory '{}' removed.", dir_name);
    Ok(())
}

fn remove_directories(dir_names: &[&str]) -> std::io::Result<()> {
    for dir_name in dir_names {
        fs::remove_dir_all(dir_name)?;
        println!("Directory '{}' removed.", dir_name);
    }
    Ok(())
}

fn list_directory_contents(dir_name: &str) -> std::io::Result<()> {
    println!("Contents of directory '{}':", dir_name);
    for entry in fs::read_dir(dir_name)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

fn print_file_properties(filename: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(filename)?;
    println!("File properties for '{}':", filename);
    println!("Size: {} bytes", metadata.len());
    println!("Created: {:?}", metadata.created()?);
    println!("Modified: {:?}", metadata.modified()?);
    Ok(())
}

fn print_directory_properties(dir_name: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(dir_name)?;
    println!("Directory properties for '{}':", dir_name);
    println!("Created: {:?}", metadata.created()?);
    println!("Modified: {:?}", metadata.modified()?);
    Ok(())
}

fn main() -> std::io::Result<()> {
    create_file("data.csv")?;
    create_dir("lab_data")?;
    copy_file("data.csv", "data_copy.csv")?;
    move_file("data_copy.csv", Path::new("lab_data").join("data_copy.csv").as_path())?;
    copy_dir(Path::new("lab_data"), Path::new("lab_data_copy"))?;
    find_files(Path::new("lab_data"))?;

    for i in 1..4 {
        create_dir(&format!("{}", i))?;
    }

    remove_file("data.csv")?;
    remove_directory("lab_data_copy")?;
    remove_directories(&["1", "2", "3"])?;

    list_directory_contents("lab_data")?;
    print_file_properties("lab_data/data_copy.csv")?;
    print_directory_properties("lab_data")?;

    Ok(())
}