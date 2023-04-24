use std::fs;
use rand::Rng;

fn send_mail(address: &str, message: &str) {

}

fn main() {
    let mut rng = rand::thread_rng();

    let file_path: &str = "src/data.json";
    let file_contents: String =
        fs::read_to_string(file_path).expect(&format!("File {} not found", file_path));
    let parsed_contents: json::JsonValue = json::parse(&file_contents).unwrap();

    for (email_address, messages) in parsed_contents.entries() {
        let message_index = rng.gen_range(0..messages.len());
        let chosen_message = &messages[message_index];
        println!("to: {}\nchosen message: {}", email_address, chosen_message);
    }
}
