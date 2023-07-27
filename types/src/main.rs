#[derive(Debug)]
enum IpAddr {
    V4(String, i32),
    V6(String),
}

#[derive(Debug)]
struct User {
    access_count: u32,
    email: String,
    username: String,
    active: bool,
}

impl User {
    // associated function: User::new(...)
    fn new(username: String, email: String) -> User {
        User {
            active: true,
            username, // mesmo nome de variável e parâmetro, logo não precisa repetir 2x
            email,
            access_count: 1,
        }
    }

    fn reset_access(&mut self) {
        self.access_count = 0;
    }

    fn print_info(&self) {
        let status = match self.active {
            true => "está ativo",
            false => "não está ativo",
        };
        let vezes = match self.access_count {
            1 => "vez",
            _ => "vezes",
        };
        println!(
            "O usuário {} acessou a conta {} {} e {}.",
            self.username, self.access_count, vezes, status
        )
    }

    fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // pattern matching pode se adaptar ao valor de cada variante
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let lb = String::from("::1");

    let home = IpAddr::V4(String::from("127.0.0.1"), 30);

    let loopback = IpAddr::V6(lb);

    println!("{:#?} {:#?}", home, loopback);

    let mut rob = User::new(String::from("Roberto"), String::from("rob@ufs.com.br"));
    rob.print_info();
    rob.reset_access();
    rob.change_email(String::from("robsao@gmail.com"));
    println!("{:#?}", rob);

    value_in_cents(Coin::Quarter(UsState::Alaska));
}
