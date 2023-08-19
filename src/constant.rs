pub const ERROR_MESSAGE: &str = "The usage is '<command> [options]', type 'help' for more details";
pub const INPUT_ERROR_MESSAGE: &str = "There was an error in your input!";
pub const ADD_ERROR_MESSAGE: &str = "Command <add|a> => 'add <todo>'";
pub const EDIT_ERROR_MESSAGE: &str = "Command <edit|e> => 'edit <index> <new_todo>'";
pub const DELETE_ERROR_MESSAGE: &str = "Command <delete|d> => 'delete <index>'";
pub const LIST_ERROR_MESSAGE: &str = "Command <list|l> => 'list'";
pub const TOGGLE_ERROR_MESSAGE: &str = "Command <toggle|t> => 'toggle <index>'";
pub const HELP_ERROR_MESSAGE: &str = "Command <help|h> => 'help'";
pub const EXIT_ERROR_MESSAGE: &str = "Command <exit|x> => 'exit'";
pub const READ_DB_ERROR_MESSAGE: &str = "There was an error with reading the database file!";
pub const ADD_DB_ERROR_MESSAGE: &str = "There was an error in inserting in your DB!";
pub const EDIT_DB_ERROR_MESSAGE: &str = "There was an error in updating in your DB!";
pub const DELETE_DB_ERROR_MESSAGE: &str = "There was an error in deleting in your DB!";

pub static ERROR_LIST: &'static [&str] = &[
    ADD_ERROR_MESSAGE,
    EDIT_ERROR_MESSAGE,
    DELETE_ERROR_MESSAGE,
    LIST_ERROR_MESSAGE,
    TOGGLE_ERROR_MESSAGE,
    HELP_ERROR_MESSAGE,
    EXIT_ERROR_MESSAGE,
];
