use crossterm::{style::{Color, ResetColor,SetForegroundColor},ExecutableCommand,};
use std::io::{stdin,stdout};


#[derive(PartialEq,Debug)]
pub enum printCommand{
    AICall,
    UnitTest,
    Issue,
}

impl printCommand{
    pub fn print_agent_message(&self,agent_role:&str,agent_statement:&str){
        let mut stdout: std::io::Stdout =stdout();

        let statement_color = match self {
            Self::AICall =>Color::Cyan,
            Self::Issue=>Color::Red,
            Self::UnitTest => Color::Magenta,
        };

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();

        print!("Agent {}: ",agent_role);

        //Reset the color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}",agent_statement);


        //Reset the color
        stdout.execute(ResetColor).unwrap();
    }   
}


//get user input form the command line
pub fn get_user_reponse(question:&str) -> String {
    let mut stdout :std::io::Stdout = stdout();

    //print the question in the specific color 
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();

    println!("");
    println!("{}",question);
    
    //reset the color
    stdout.execute(ResetColor).unwrap();

    //Read the user inoput 
    let mut user_repsosne:String =String::new();
    stdin()
    .read_line(&mut user_repsosne)
    .expect("Failed to read");

    // Trim white space and return 
    return user_repsosne.trim().to_string();

}

// Get user response that code is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the question in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("");
        print!("WARNING: You are about to run code written entirely by AI. ");
        println!("Review your code and confirm you wish to continue.");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Present Options with different colors
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::DarkRed)).unwrap();
        println!("[2] Lets stop this project");

        // Reset Color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut human_response: String = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response: String = human_response.trim().to_lowercase();

        // Match response
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select '1' or '2'")
            }
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_print_agent_msg(){
        printCommand::AICall
        .print_agent_message("Managing agent", "Testing stuff to check if they are working");
    }
}