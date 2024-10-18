use std::fs::File;
use std::io::{self};
use std::process;
use serde::{Serialize, Deserialize};
use serde_json::{to_writer, from_reader};

// Estrutura do animal
#[derive(Serialize, Deserialize, Debug)]
struct InfoAnimal {
    tipo: String,
    nome: String,
}

// Função para ler a entrada do usuário
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}

// Função para salvar os animais no arquivo JSON
fn save_to_json(db: &Vec<InfoAnimal>) {
    let file = File::create("animais.json").expect("Erro ao criar arquivo");
    to_writer(file, db).expect("Erro ao salvar dados em JSON");
}

// Função para carregar os animais do arquivo JSON
fn load_from_json() -> Vec<InfoAnimal> {
    if let Ok(file) = File::open("animais.json") {
        from_reader(file).expect("Erro ao carregar dados do JSON")
    } else {
        Vec::new() // Retorna um vetor vazio se o arquivo não existir
    }
}

// Função para listar todos os animais
fn listar_animais(db: &Vec<InfoAnimal>) {
    if db.is_empty() {
        println!("Nenhum animal cadastrado.");
    } else {
        for (index, animal) in db.iter().enumerate() {
            println!("{}: Tipo: {}, Nome: {}", index, animal.tipo, animal.nome);
        }
    }
}

// Função para incluir um novo animal
fn incluir_animal(db: &mut Vec<InfoAnimal>) {
    println!("Digite o tipo do animal:");
    let tipo = read_input();
    println!("Digite o nome do animal:");
    let nome = read_input();

    let novo_animal = InfoAnimal { tipo, nome };
    db.push(novo_animal);
    save_to_json(db);
    println!("Animal incluído com sucesso.");
}

// Função para editar um animal
fn editar_animal(db: &mut Vec<InfoAnimal>) {
    listar_animais(db);
    println!("Digite o índice do animal a ser editado:");
    let index: usize = read_input().parse().expect("Por favor, insira um número válido");

    if index < db.len() {
        println!("Digite o novo tipo do animal:");
        let tipo = read_input();
        println!("Digite o novo nome do animal:");
        let nome = read_input();

        db[index] = InfoAnimal { tipo, nome };
        save_to_json(db);
        println!("Animal editado com sucesso.");
    } else {
        println!("Índice inválido.");
    }
}

// Função para excluir um animal
fn excluir_animal(db: &mut Vec<InfoAnimal>) {
    listar_animais(db);
    println!("Digite o índice do animal a ser excluído:");
    let index: usize = read_input().parse().expect("Por favor, insira um número válido");

    if index < db.len() {
        db.remove(index);
        save_to_json(db);
        println!("Animal excluído com sucesso.");
    } else {
        println!("Índice inválido.");
    }
}

// Função principal do menu
fn main() {
    let mut db = load_from_json();
    
    loop {
        println!("\nMenu:");
        println!("1. Listar animais");
        println!("2. Incluir animal");
        println!("3. Editar animal");
        println!("4. Excluir animal");
        println!("5. Sair");

        let escolha = read_input();

        match escolha.as_str() {
            "1" => listar_animais(&db),
            "2" => incluir_animal(&mut db),
            "3" => editar_animal(&mut db),
            "4" => excluir_animal(&mut db),
            "5" => {
                println!("Saindo...");
                process::exit(0);
            }
            //Caso o Usuário digite alguma opção diferente dos números
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}
