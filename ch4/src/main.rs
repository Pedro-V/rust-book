fn concat_and_print(s: &mut String) -> &mut String {
    // s.push('o');
    println!("{}", s);
    s
}

fn main() {
    let mut s = String::from("Gol");
    let refer = concat_and_print(&mut s);
    println!("{}", refer);
}
