use colored::Colorize;
use todo_list::check_args;
use todo_list::command;
use todo_list::constant::INPUT_ERROR_MESSAGE;

fn main() {
    println!("Welcome to your todo list!");
    let mut todos: Vec<command::Todo> = Vec::new();
    loop {
        let (command, options) = match check_args() {
            Ok((command, options)) => (command, options),
            Err(_) => {
                eprintln!("{}", INPUT_ERROR_MESSAGE.bright_red());
                continue;
            }
        };

        if let Err(e) = command::handle_command(command, options, &mut todos) {
            eprintln!("{}", e.bright_red());
            continue;
        };
    }
}
