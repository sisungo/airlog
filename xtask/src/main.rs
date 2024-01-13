mod dist;
mod env;

pub type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    let mut args = std::env::args();
    args.next();

    let task = args.next();

    match task.as_deref() {
        Some("dist") => dist::main()?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application
"
    )
}