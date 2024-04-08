#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {
        stringify!($func)
    };
}

#[macro_use]
mod ai_function;
mod apis;
mod models;
mod utils;

use utils::command_line_utils::get_user_reponse;
use models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() {
    let usr_req: String = get_user_reponse("What website are we building today?");

    let mut manage_agent: ManagingAgent = ManagingAgent::new(usr_req)
        .await
        .expect("Error creating agent");

    manage_agent.execute_project().await;

    // dbg!(manage_agent);



  
}