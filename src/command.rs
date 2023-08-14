use colored::*;

pub fn handle_command(
    command: String,
    options: Vec<String>,
    todos: &mut Vec<Todo>,
) -> Result<(), &'static str> {
    match command.as_str() {
        "add" | "a" => {
            if options.is_empty() {
                return Err("The command for add is 'todo add <todo>'");
            }
            add_todo(options.join(" "), todos);
        }
        "edit" | "e" => {
            if options.is_empty() {
                return Err("The command for edit is 'todo edit <index> <new_todo>'");
            }
            let idx = parse_index(&options, todos)?;
            edit_todo(idx, options[1..].join(" "), todos);
        }
        "delete" | "d" => {
            if options.is_empty() || options.len() > 1 {
                return Err("The command for delete is 'todo delete <index>'");
            }
            let idx = parse_index(&options, todos)?;
            delete_todo(idx, todos);
        }
        "list" | "l" => {
            if !options.is_empty() {
                return Err("The command for list is 'todo list'");
            }
            list_todo(todos);
        }
        "toggle" | "t" => {
            if options.is_empty() || options.len() > 1 {
                return Err("The command for toggle is 'todo toggle <index>'");
            }
            let idx = parse_index(&options, todos)?;
            toggle_todo(idx, todos);
        }
        _ => return Err("Invalid command!"),
    }
    Ok(())
}

fn parse_index(options: &Vec<String>, todos: &Vec<Todo>) -> Result<usize, &'static str> {
    let idx = options[0]
        .parse::<usize>()
        .map_err(|_| "Index out of bounds!")?;
    if idx <= 0 || idx > todos.len() {
        return Err("Index out of bounds!");
    }
    Ok(idx)
}

pub struct Todo {
    todo: String,
    is_done: bool,
}

impl Todo {
    pub fn new(todo: String) -> Todo {
        Todo {
            todo,
            is_done: false,
        }
    }
}

fn add_todo(todo: String, todos: &mut Vec<Todo>) {
    println!("{}{}{}", "Todo '".blue(), todo.blue(), "' added!".blue());
    todos.push(Todo::new(todo));
}

fn edit_todo(idx: usize, new_todo: String, todos: &mut Vec<Todo>) {
    println!(
        "{}{}{}{}{}",
        "Todo '".blue(),
        todos[idx - 1].todo.blue(),
        "' edited to '".blue(),
        new_todo.blue(),
        "'!".blue()
    );
    todos[idx - 1] = Todo::new(new_todo);
}

fn delete_todo(idx: usize, todos: &mut Vec<Todo>) {
    println!(
        "{}{}{}",
        "Todo '".blue(),
        todos[idx - 1].todo.blue(),
        "' deleted!".blue()
    );
    todos.remove(idx - 1);
}

fn list_todo(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("You have no todos!");
    }
    for (idx, todo) in todos.iter().enumerate() {
        let is_done = if todo.is_done {
            "✓".green()
        } else {
            "☐".white()
        };
        println!("{}. {} {}", idx + 1, todo.todo, is_done);
    }
}

fn toggle_todo(idx: usize, todos: &mut Vec<Todo>) {
    if todos[idx - 1].is_done == false {
        println!(
            "{}{}{}",
            "Todo '".green(),
            todos[idx - 1].todo.green(),
            "' is completed!".green()
        );
    } else {
        println!(
            "{}{}{}",
            "Todo '".red(),
            todos[idx - 1].todo.red(),
            "' is marked as incomplete again!".red()
        );
    }
    todos[idx - 1].is_done = !todos[idx - 1].is_done;
}
