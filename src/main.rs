use std::{fs, io};

fn get_user_message() -> String {
    let mut message = String::new();
    println!("This will be attached to the file");
    io::stdin()
        .read_line(&mut message)
        .expect("Unable to read line");
    message.trim().to_owned()
}

fn get_file() -> Result<String, io::Error> {
    println!("Add path to file you want to edit. If such file doesn't exist, it will be created");

    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;

    Ok(file_path.trim().to_owned())
}

fn ask_user_for_messages(path: &String) {
    let finish_command: String = ".exit".to_owned();

    loop {
        let last_message: String;
        last_message = get_user_message();
        if last_message == finish_command {
            println!("Bye!");
            return;
        }

        attach_to_file(path, last_message.trim().to_owned())
    }
}

fn get_file_content(path: &String) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn attach_to_file(path: &String, content: String) {
    match get_file_content(path) {
        Ok(current_content) => {
            let upd_content = format!("{current_content}\n{content}");
            fs::write(&path, upd_content).expect("Cannot write {content} to file {file}");
        }
        Err(_) => fs::write(&path, content).expect("Cannot write {content} to file {file}"),
    }
}

fn print_file(path: &String) {
    match get_file_content(path) {
        Ok(current_content) => {
            println!("File {path} now contains: \n\n{current_content}");
        }
        Err(_) => (),
    }
}

fn main() {
    let file_path = get_file().expect("Unable to create file");
    ask_user_for_messages(&file_path);
    print_file(&file_path);
}
