use colored::Colorize;
use rusqlite::{Connection, Result};
use todo_list::check_args;
use todo_list::command;
use todo_list::command::Todo;
use todo_list::constant::INPUT_ERROR_MESSAGE;
use todo_list::constant::READ_DB_ERROR_MESSAGE;

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;
    println!("Loading todos...");
    conn.execute(
        "CREATE TABLE if not exists todo (
        id   INTEGER UNIQUE PRIMARY KEY,
        todo TEXT NOT NULL,
        is_done INTEGER NOT NULL
    )",
        (),
    )?;
    let mut todos = Vec::new();
    let mut stmt = conn.prepare("SELECT id, todo, is_done FROM todo")?;
    let rows = stmt.query_map([], |row| {
        Ok(Todo::initialise(row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    for row in rows {
        match row {
            Ok(todo) => {
                todos.push(todo);
            }
            Err(_) => {
                eprintln!("{}", READ_DB_ERROR_MESSAGE.bright_red());
            }
        }
    }
    println!("Welcome to your todo list!");
    loop {
        let (command, options) = match check_args() {
            Ok((command, options)) => (command, options),
            Err(_) => {
                eprintln!("{}", INPUT_ERROR_MESSAGE.bright_red());
                continue;
            }
        };

        if let Err(e) = command::handle_command(command, options, &mut todos, &conn) {
            eprintln!("{}", e.bright_red());
            continue;
        };
    }
}
