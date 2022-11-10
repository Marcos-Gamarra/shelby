use std::io::Write;
use std::process::Command;

fn main() {
    loop {
        //prompt
        print!(" > ");
        std::io::stdout().flush().unwrap();

        //read input from user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //splits the user input by whitespaces into args.
        //args.remove(0) removes the first element from args and returns it,
        //so command now will hold the name of the command and args only the arguments.
        let mut args: Vec<String> = input.trim().split(" ").map(|arg| arg.to_string()).collect();
        let command = args.remove(0);

        //executes the command as child process
        let child = Command::new(&command).args(args).spawn();

        //if the child executes correctly wait until it stops, else
        //print error message
        if let Ok(mut child) = child {
            child.wait().unwrap();
        } else {
            println!("Error while trying to execute command: {}", command);
        }
    }
}
