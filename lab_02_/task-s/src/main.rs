struct Film {
    name: String,
    director: String,
    year: u16,
    country: String,
}

impl Film {
    fn description(&self) -> String {
        format!(
            "Назва: \"{}\"\nРежисер: {}\nРік: {}\nКраїна: {}",
            self.name, self.director, self.year, self.country
        )
    }
}

struct Phone {
    name: String,
    screen_size: f32,
    processor: String,
    ram: u32,
}

impl Phone {
    fn description(&self) -> String {
        format!(
            "Модель: {}\nДіагональ: {:.1}\"\nПроцесор: {}\nОперативна пам’ять: {} ГБ",
            self.name, self.screen_size, self.processor, self.ram
        )
    }
}

struct Car {
    name: String,
    brand: String,
    year: u16,
    run: u32,
}

impl Car {
    fn description(&self) -> String {
        format!(
            "Модель: {}\nБренд: {}\nРік: {}\nПробіг: {} км",
            self.name, self.brand, self.year, self.run
        )
    }
    
}

struct Worker {
    position: String,
    sallary: u32,
    start_date: String,
}

impl Worker {
    fn description(&self) -> String {
        format!(
            "Посада: {}\nЗарплата: {}$\nДата початку роботи: {}",
            self.position, self.sallary, self.start_date
        )
    }
}

struct Estate {
    estate_type: String,
    area: f32,
    owner: String,
    price: u32,
}

impl Estate {
    fn description(&self) -> String {
        format!(
            "Тип: {}\nПлоща: {:.1} м²\nВласник: {}\nЦіна: {}$",
            self.estate_type, self.area, self.owner, self.price
        )
    }
}

fn main() {
    //film
    let my_film = Film {
        name: String::from("1+1"),
        director: String::from("Олівʼє Накаш"),
        year: 2011,
        country: String::from("Франція"),
    };

    println!("Фільм\n{}\n", my_film.description());

    //phone
    let my_phone = Phone {
        name: String::from("Pixel 9"),
        screen_size: 6.3,
        processor: String::from("Tensor G4"),
        ram: 8,
    };

    println!("Телефон\n{}\n", my_phone.description());

    //car
    let my_car = Car {
        name: String::from("Model S"),
        brand: String::from("Tesla"),
        year: 2021,
        run: 1000,
    };

    println!("Машина\n{}\n", my_car.description());

    //worker
    let my_worker = Worker {
        position: String::from("Програміст"),
        sallary: 1000,
        start_date: String::from("01.01.2021"),
    };

    println!("Працівник\n{}\n", my_worker.description());

    //estate
    let my_estate = Estate {
        estate_type: String::from("Квартира"),
        area: 50.0,
        owner: String::from("Джон Іванович"),
        price: 50000,
    };

    println!("Нерухомість\n{}\n", my_estate.description());

    //sort by screen size
    let mut phones = vec![
        Phone {
            name: String::from("Pixel 9"),
            screen_size: 6.3,
            processor: String::from("Tensor G4"),
            ram: 8,
        },
        Phone {
            name: String::from("Iphone 15"),
            screen_size: 6.1,
            processor: String::from("A16 Bionic"),
            ram: 6,
        },
        Phone {
            name: String::from("Galaxy S25"),
            screen_size: 6.7,
            processor: String::from("Exynos 2500"),
            ram: 12,
        },
    ];

    phones.sort_by(|a, b| a.screen_size.partial_cmp(&b.screen_size).unwrap());

    println!("Телефони відсортовані за діагоналлю:\n");
    for phone in &phones {
        println!("{}", phone.description());
        println!("-----------------");
    }
}

