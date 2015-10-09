use actions::Action;
use notify::Event;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub struct CommandAction {
    command_line: String
}

impl CommandAction {
    pub fn new(command_line: String) -> CommandAction {
        CommandAction {
            command_line: command_line
        }
    }

    pub fn get_command_line(&self, event: &Event) -> String {
        let path = match event.path {
            None => "",
            Some(ref path) => path.to_str().unwrap_or("")
        };

        self.command_line.replace("{:p}", path)
    }

    pub fn get_command(&self, event: &Event) -> Command {
        let command_line = self.get_command_line(event);
        let mut cmd_pieces = command_line.split(" ");
        let mut command = Command::new(cmd_pieces.next().unwrap());
        for piece in cmd_pieces {
            command.arg(piece);
        }

        command
    }
}

impl Action for CommandAction {
    fn handle_change(&self, event: &Event) {
        let mut command = self.get_command(event);
        let command_result = command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        if command_result.is_err() {
            println!("Could not execute command: {:?}", self.command_line);
            return;
        }
        
        let output = command_result.unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));

        if !output.stderr.is_empty() {
            let write_result = writeln!(
                &mut io::stderr(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            match write_result {
                Err(x) => println!("Error: Unable to write to stderr: {}", x),
                Ok(_) => {}
            }
        }
    }
}
