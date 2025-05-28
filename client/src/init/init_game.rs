use std::fs::File;
use std::io::Read;

pub fn initialize_game() -> String {
    let field: String = open_file("asset/field.txt");
    let mut home: Vec<String> = vec![];
    let mut away: Vec<String> = vec![];
    let mut input: String = String::new();

    for i in 1..24 {
        let home_player: String = open_file(&format!("asset/home/{}.txt", i));
        let away_player: String = open_file(&format!("asset/away/{}.txt", i));
        home.push(home_player);
        away.push(away_player);
    }

    input.push_str("init\n");
    input.push_str(&field);
    input.push_str("\n");
    input.push_str(&home.join("\n"));
    input.push_str("\n");
    input.push_str(&away.join("\n"));
    println!("Send input: {}", input);
    return input;
}

pub fn open_file(file_path: &str) -> String {
    // Open the file at the specified path
    let mut file = File::open(file_path).expect("Failed to open file");
    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    // Replace all occurrences of \n with _
    contents = contents.replace('\n', "_");
    return contents;
}