use rand::Rng;
use std::fs;

fn send_mail(address: &str, message: &str) {}

fn main() {
    let mut rng = rand::thread_rng();

    let mut file_path = std::env::current_dir().unwrap();
    file_path.push("config");
    file_path.push("data.json");
    let file_contents: String =
        fs::read_to_string(&file_path).expect(&format!("File {} not found", file_path.display()));
    let parsed_contents: json::JsonValue = json::parse(&file_contents).unwrap();

    for (email_address, messages) in parsed_contents.entries() {
        let message_index = rng.gen_range(0..messages.len());
        let chosen_message = &messages[message_index];
        println!("to: {}\nchosen message: {}", email_address, chosen_message);
    }
}
