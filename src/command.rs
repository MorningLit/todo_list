use std::process;

use crate::constant::*;
use colored::*;
use rusqlite::{params, Connection};

pub fn handle_command(
    command: String,
    options: Vec<String>,
    todos: &mut Vec<Todo>,
    conn: &Connection,
) -> Result<(), &'static str> {
    match command.as_str() {
        "add" | "a" => {
            if options.is_empty() {
                return Err(ADD_ERROR_MESSAGE);
            }
            if let Err(_) = add_todo(options.join(" "), todos, conn) {
                return Err(ADD_DB_ERROR_MESSAGE);
            }
        }
        "edit" | "e" => {
            if options.is_empty() {
                return Err(EDIT_ERROR_MESSAGE);
            }
            let idx = parse_index(&options, todos)?;
            if let Err(_) = edit_todo(idx, options[1..].join(" "), todos, conn) {
                return Err(EDIT_DB_ERROR_MESSAGE);
            }
        }
        "delete" | "d" => {
            if options.is_empty() || options.len() > 1 {
                return Err(DELETE_ERROR_MESSAGE);
            }
            let idx = parse_index(&options, todos)?;
            if let Err(_) = delete_todo(idx, todos, conn) {
                return Err(DELETE_DB_ERROR_MESSAGE);
            }
        }
        "list" | "l" => {
            if !options.is_empty() {
                return Err(LIST_ERROR_MESSAGE);
            }
            list_todo(todos);
        }
        "toggle" | "t" => {
            if options.is_empty() || options.len() > 1 {
                return Err(TOGGLE_ERROR_MESSAGE);
            }
            let idx = parse_index(&options, todos)?;
            if let Err(_) = toggle_todo(idx, todos, conn) {
                return Err(EDIT_DB_ERROR_MESSAGE);
            }
        }
        "help" | "h" => {
            if !options.is_empty() {
                return Err(HELP_ERROR_MESSAGE);
            }
            display_help();
        }
        "exit" | "x" => {
            if !options.is_empty() {
                return Err(EXIT_ERROR_MESSAGE);
            }
            println!("Exiting program...");
            process::exit(0)
        }
        _ => return Err(ERROR_MESSAGE),
    }
    Ok(())
}

fn parse_index(options: &Vec<String>, todos: &Vec<Todo>) -> Result<usize, &'static str> {
    let idx = options[0].parse::<usize>().map_err(|_| "Invalid index!")?;
    if idx <= 0 || idx > todos.len() {
        return Err("Index out of bounds!");
    }
    Ok(idx)
}

pub struct Todo {
    id: i64,
    todo: String,
    is_done: bool,
}

impl Todo {
    pub fn new(id: i64, todo: String) -> Todo {
        Todo {
            id,
            todo,
            is_done: false,
        }
    }
    pub fn initialise(id: i64, todo: String, is_done: bool) -> Todo {
        Todo { id, todo, is_done }
    }
}

fn add_todo(todo: String, todos: &mut Vec<Todo>, conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT into todo (todo, is_done) VALUES (?1, ?2)",
        params![&todo, false],
    )?;
    println!("{}{}{}", "Todo '".blue(), todo.blue(), "' added!".blue());
    let id = conn.last_insert_rowid();
    todos.push(Todo::new(id, todo));
    Ok(())
}

fn edit_todo(
    idx: usize,
    new_todo: String,
    todos: &mut Vec<Todo>,
    conn: &Connection,
) -> Result<(), rusqlite::Error> {
    let id = todos[idx - 1].id;
    conn.execute(
        "UPDATE todo SET todo = $1 WHERE id = $2",
        params![&new_todo, &id],
    )?;
    println!(
        "{}{}{}{}{}",
        "Todo '".blue(),
        todos[idx - 1].todo.blue(),
        "' edited to '".blue(),
        new_todo.blue(),
        "'!".blue()
    );
    todos[idx - 1] = Todo::initialise(todos[idx - 1].id, new_todo, todos[idx - 1].is_done);
    Ok(())
}

fn delete_todo(
    idx: usize,
    todos: &mut Vec<Todo>,
    conn: &Connection,
) -> Result<(), rusqlite::Error> {
    let id = todos[idx - 1].id;
    conn.execute("DELETE from todo WHERE id = $1", params![&id])?;
    println!(
        "{}{}{}",
        "Todo '".blue(),
        todos[idx - 1].todo.blue(),
        "' deleted!".blue()
    );
    todos.remove(idx - 1);
    Ok(())
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

fn toggle_todo(
    idx: usize,
    todos: &mut Vec<Todo>,
    conn: &Connection,
) -> Result<(), rusqlite::Error> {
    let id = todos[idx - 1].id;
    let updated_is_done = !todos[idx - 1].is_done;
    conn.execute(
        "UPDATE todo SET is_done = $1 WHERE id = $2",
        params![&updated_is_done, &id],
    )?;
    if updated_is_done {
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
    Ok(())
}

fn display_help() {
    for e in ERROR_LIST {
        println!("{}", e);
    }
}
