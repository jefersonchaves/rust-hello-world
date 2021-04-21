use std::io;

mod phone_catalog;

fn main() {

    // let country_code = 3;
    // match country_code {
    //     1 => println!("ok"),
    //     z @ 2...6 => println!("fine, value = {}", z),
    //     _ => println!("not ok")
    // }
    // return;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    let phone_catalog = phone_catalog::PhoneCatalog::new();
    phone_catalog.add(
        phone_catalog::Entry{name: String::from("Jeferson"), phone: String::from("555")}
    );
    phone_catalog.print();
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_main() {
        main();
    }
}
