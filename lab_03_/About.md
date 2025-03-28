## Пояснення коду Rust (main.rs)

Ця програма на Rust демонструє різні операції з файловою системою, використовуючи модулі `std::fs` та `std::io`. Ось розбір кожної частини коду:

### 1. Імпорт модулів

```rust
use std::fs;
use std::io;
use std::path::Path;
```

-   `std::fs`: Надає функції для взаємодії з файловою системою (читання, запис, створення, видалення файлів та директорій).
-   `std::io`: Надає функціональність вводу/виводу, наприклад, читання зі стандартного вводу.
-   `std::path::Path`: Представляє шляхи до файлів у платформо-незалежний спосіб.

### 2. Функція `read_user_input`

```rust
fn read_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}
```

-   Ця функція зчитує рядок зі стандартного вводу (клавіатури користувача), перетворює його в нижній регістр і повертає.
-   Вона використовує `io::stdin().read_line()` для зчитування вводу.
-   `.trim()` видаляє пробіли на початку та в кінці рядка.
-   Тип `io::Result<String>` вказує на те, що функція може повернути або `String`, або `io::Error`, якщо щось піде не так.

### 3. Функція `copy_dir`

```rust
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
```

-   Ця функція рекурсивно копіює директорію з `source` в `destination`.
-   Спочатку вона перевіряє, чи є вихідний шлях дійсно директорією.
-   `fs::create_dir_all(destination)?` створює цільову директорію та всі необхідні батьківські директорії.
-   Потім вона ітерується по кожному елементу у вихідній директорії, використовуючи `fs::read_dir()`.
-   Якщо елемент є директорією, вона рекурсивно викликає `copy_dir` для копіювання піддиректорії.
-   Якщо елемент є файлом, вона копіює файл, використовуючи `fs::copy()`.

### 4. Функція `search`

```rust
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
```

-   Ця функція шукає файли `.csv` у заданій директорії (`dir`), які мають розмір більше 1 байта.
-   Вона ітерується по кожному елементу в директорії.
-   Для кожного файлу вона перевіряє, чи є розширення файлу "csv" і чи його довжина (в байтах) більше 1.
-   Якщо обидві умови виконуються, вона друкує шлях до файлу та його довжину.

### 5. Функція `main`

```rust
fn main() -> io::Result<()> {
    // ... (інша частина коду функції main) ...
    Ok(())
}
```

Функція `main` організовує операції з файловою системою. Ось покрокове пояснення:

1.  **Створення файлу:**

    ```rust
    fs::write("data.csv", "lab 3 data")?;
    ```

    -   Створює файл з назвою `data.csv` з вмістом "lab 3 data".

2.  **Створення директорій та порожніх файлів:**

    ```rust
    for i in 1..4 {
        let dir = format!("{}", i);
        let dir_path = Path::new(&dir);
        if !dir_path.exists() {
            fs::create_dir(dir_path)?;
        }
        fs::write(dir_path.join("empty.csv"), "")?;
    }
    ```

    -   Створює три директорії з назвами "1", "2" та "3".
    -   Всередині кожної директорії створює порожній файл з назвою `empty.csv`.

3.  **Копіювання файлу:**

    ```rust
    fs::copy("data.csv", "data_copy.csv")?;
    ```

    -   Копіює файл `data.csv` у `data_copy.csv`.

4.  **Копіювання директорії:**

    ```rust
    let source = Path::new("1");
    let destination = Path::new("1_copy");
    copy_dir(source, destination)?;
    ```

    -   Копіює директорію "1" в "1\_copy", використовуючи функцію `copy_dir`.

5.  **Пошук файлів `.csv`:**

    ```rust
    println!("Натисніть 'y' для пошуку:");
    if read_user_input()? == "y" {
        search(Path::new("."))?;
    }
    ```

    -   Пропонує користувачеві натиснути 'y' для ініціювання пошуку файлів `.csv` у поточній директорії.
    -   Якщо користувач вводить 'y', викликається функція `search`.

6.  **Видалення файлів та директорій:**

    ```rust
    println!("Натисніть 'y' для видалення файлів та директорій:");
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
    ```

    -   Пропонує користувачеві натиснути 'y' для видалення певних файлів та директорій.
    -   Якщо користувач вводить 'y', видаляються `data_copy.csv`, директорія "1\_copy", та директорії "2" і "3".

7.  **Показ вмісту директорії:**

    ```rust
    println!("Натисніть 'y' для показу вмісту директорії:");
    if read_user_input()? == "y" {
        let cont_path = Path::new("1");

        if cont_path.is_dir() {
            println!("Вміст директорії:");
            for entry in fs::read_dir(cont_path)? {
                let entry = entry?;
                let path = entry.path();
                println!(" {}", path.display());
            }
        }
    }
    ```

    -   Пропонує користувачеві натиснути 'y' для відображення вмісту директорії "1".
    -   Якщо користувач вводить 'y', зчитується директорія та друкується шлях кожного елемента.

8.  **Показ властивостей файлу та директорії:**

    ```rust
    println!("Натисніть 'y' для показу властивостей файлу та директорії:");
    if read_user_input()? == "y" {
        let file_prop_path = Path::new("data.csv");
        let file_meta = fs::metadata(file_prop_path)?;
        println!("Розмір: {} байт", file_meta.len());
        println!("Тип: {:?}", file_meta.file_type());
        println!("Дозволи: {:?}", file_meta.permissions());
        println!("Змінено: {:?}", file_meta.modified());

        let dir_prop_path = Path::new("1");
        let dir_meta = fs::metadata(dir_prop_path)?;
        if dir_meta.is_dir() {
            println!("Розмір: {} байт", dir_meta.len());
            println!("Тип: {:?}", dir_meta.file_type());
            println!("Дозволи: {:?}", dir_meta.permissions());
            println!("Змінено: {:?}", dir_meta.modified());
        }
    }
    ```

    -   Пропонує користувачеві натиснути 'y' для відображення властивостей (розмір, тип, дозволи, час останньої зміни) файлу `data.csv` та директорії "1".
    -   Якщо користувач вводить 'y', отримуються та друкуються метадані для обох.
9.  **Видалення всіх створених файлів:**

    ```rust
    println!("Натисніть 'y' щоб видалити всі файли, створені цією програмою:");
    if read_user_input()? == "y"{
        fs::remove_file("data.csv")?;
        fs::remove_dir_all("1")?;
    }
    ```

    -   Пропонує користувачеві натиснути 'y' для видалення всіх файлів і каталогів, створених програмою.
    -   Якщо користувач вводить 'y', він видаляє `data.csv` і каталог "1".

### Обробка помилок

Оператор `?` використовується всюди в коді для обробки помилок. Якщо будь-який `io::Result` повертає значення `Err`, функція негайно поверне цю помилку, передаючи її вгору по стеку викликів.

Це вичерпне пояснення має допомогти в розумінні призначення та функціональності кожної частини коду Rust.
