use std::env;
use std::fs;
use std::io::ErrorKind;
use std::process;

fn main() {
    let args = env::args();

    let cmd = parse_arguments(args).unwrap_or_else(|err| {
        eprintln!("Failed to run the program: {}", err);
        process::exit(1);
    });

    match cmd {
        Command::Show => show_current_focus(),
        Command::Add(new_todo) => add_todo(&new_todo),
        Command::Done => finish_todo(),
        Command::Skip => skip_current_focus(),
    }
}

enum Command {
    Show,
    Done,
    Skip,
    Add(String),
}

fn parse_arguments(mut args: env::Args) -> Result<Command, &'static str> {
    args.next();

    let action = match args.next() {
        Some(cmd) => cmd,
        None => return Ok(Command::Show),
    };

    let context = args.next();

    match (action.as_str(), context) {
        ("done", _) => Ok(Command::Done),
        ("skip", _) => Ok(Command::Skip),
        ("add", Some(new_todo)) => Ok(Command::Add(new_todo)),
        (_, _) => Err("Invalid arguments"),
    }
}

// file content:
// it just individual todos on each line
//
// todo1
// todo2
// todo3
// ...

fn read_file() -> std::io::Result<String> {
    let file = match fs::read_to_string(".ftd") {
        Ok(file_content) => file_content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => String::from(""),
            _ => return Err(err),
        },
    };

    Ok(file)
}

fn get_current_focus() -> (Option<String>, Option<String>) {
    // PANIC SOURCE refactor later
    let file = read_file().unwrap();
    let mut file_content = file.lines().map(|line_str| line_str.to_string());

    // ignore separator/header
    file_content.next();

    (file_content.next(), file_content.next())
}

fn add_todo(new_todo: &str) {
    // PANIC SOURCE refactor later
    let mut file = read_file().unwrap();

    file.push_str("\n");
    file.push_str(new_todo);

    // PANIC SOURCE refactor later
    fs::write(".ftd", file).unwrap();
}

fn show_current_focus() {
    let (current_focus, next_focus) = get_current_focus();

    match current_focus {
        Some(todo) => {
            println!("Current focus: ");
            println!("{}", todo);
        }
        None => {
            println!("Nothing to do! Good job!");
            return ();
        }
    }

    match next_focus {
        Some(todo) => {
            println!("");
            println!("Next focus: ");
            println!("{}", todo);
        }
        None => return (),
    }
}

fn finish_todo() {
    // PANIC SOURCE refactor later
    let file = read_file()
        .unwrap()
        .lines()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n");

    // PANIC SOURCE refactor later
    fs::write(".ftd", file).unwrap();
}

fn skip_current_focus() {
    // PANIC SOURCE refactor later
    let file_content = read_file().unwrap();
    let mut file_content = file_content.lines();

    let current_focus = match file_content.next() {
        Some(todo) => todo,
        None => return (),
    };

    let mut todos = file_content.collect::<Vec<&str>>();
    todos.push(current_focus);

    let new_file_content = todos.join("\n");

    // PANIC SOURCE refactor later
    fs::write(".ftd", new_file_content).unwrap()
}
