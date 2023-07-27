use std::collections::HashMap;
use std::io;
use rand::thread_rng;
use rand::prelude::SliceRandom;

fn gera_temas() -> HashMap<&'static str, Vec<&'static str>> {
    let mut temas = HashMap::new();
    temas.insert("Filme", vec![
        "as branquelas", "titanic", "star wars", "toy story", "forrest gump",
        "harry potter e o prisioneiro de azkaban", "o senhor dos anéis",
    ]);
    temas.insert("Objeto", vec![
        "espátula", "guarda-chuva", "caixão", "bola", "cinto",
    ]);
    temas.insert("Animal", vec![
        "caranguejo", "humano", "arara", "pônei", "cavalo",
        "cobra", "esquilo", "aranha", "sapo",
    ]);

    return temas;
}

// fn escolhe_palavra() -> 

fn main() {
    let temas = gera_temas();
    println!("Bem vind@ ao jogo da forca escrito em Rust!");
    loop {
        println!("Escolha um dos temas:");
        for key in temas.keys() {
            println!("{}", key);
        }
        let mut tema = String::new();
        io::stdin()
            .read_line(&mut tema)
            .expect("Falha ao ler a entrada");

        let tema = tema.trim();

        if !temas.contains_key(tema) {
            println!("Infelizmente, o tema {} não existe =(", tema);
            continue;
        }
        else {
            let palavra = temas.get(tema)
                .unwrap()
                .choose(&mut thread_rng())
                .unwrap();
            break;
        }
    }
    let revelada = palavra;
}
