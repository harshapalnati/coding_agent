use std::{fs, path};

use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

use crate::apis::call_req::call_gpt;
use crate::models::generals::llm::Message;
use crate::utils::command_line_utils::printCommand;

const CODE_TEMPLATE_PATH :&str="C:/Users/Harsha/Desktop/rust/coding-agent/webtemplate/webtemplate/src/code_template.rs";
const EXEC_MAIN_PATH :&str="C:/Users/Harsha/Desktop/rust/coding-agent/webtemplate/webtemplate/src/main.rs";
const API_SCHEMA_PATH:&str = "C:/Users/Harsha/Desktop/rust/coding-agent/Schemas/api_schema.json";
pub const WEB_SERVER_PROJECT_PATH: &str = "C:/Users/Harsha/Desktop/rust/coding-agent/webtemplate/webtemplate";

pub fn extent_ai_function(ai_func:fn(&str) ->&'static str,func_input:&str)->Message 
{
    let ai_function_str = ai_func(func_input);

    //extent the string to encourage pn;y printing the output
    let msg: String = format!("FUNCTION {} INSTRUCTION: You are a function printer. you ONLY print  the results of the functions.
    Nothing else.No commentary. Here is the input to the function:{}.
    Print out  what the function will return.",
    ai_function_str,func_input);

    Message{
        role:"system".to_string(),
        content:msg,
    }
    
}


//perfomr call to llm GPT
pub async fn ai_task_request(msg_context:String,agent_role:&str,agent_operation:&str,function_pass:for<'a> fn(&'a str) -> &'static str) ->String{

    //extent ai function
    let func_msg:Message  =extent_ai_function(function_pass, &msg_context);

    printCommand::AICall.print_agent_message(agent_role, agent_operation);

    //Get LLM response 
    let llm_repsosne_res = call_gpt(vec![func_msg.clone()]).await;

    //Handle succes
    match llm_repsosne_res  {
        Ok(llm_resposne)=>llm_resposne,
        Err(_) => call_gpt(vec![func_msg.clone()])
        .await
        .expect("Failed twice to call Open AI")
    }

       
}   



//Decoded version

pub async fn ai_task_request_decoed<T:DeserializeOwned>
(msg_context:String,
    agent_role:&str,
    agent_operation:&str,
    function_pass:for<'a> fn(&'a str) -> &'static str) ->T{

    let llm_response :  String = ai_task_request(msg_context, agent_role, agent_operation, function_pass).await;
        let decoded_repsosne:T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode ai resposne from serde_json");
        decoded_repsosne
}   

//Check wehtr request url is valid 
pub async fn check_status_code(client:&Client,url :&str)->Result<u16,reqwest::Error>{
    let response:reqwest::Response = client.get(url)
    .send()
    .await?;
    Ok(response.status().as_u16())
}



//Get Code templatew
pub fn read_code_template_content() ->String{
    let path:String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template path")
}

//Save new Backend Code
pub fn save_backend_code(contents:&String){
    let path= String::from(EXEC_MAIN_PATH);
    fs::write(path, contents).expect("Failed to read main file")
}

// Get Exec Main
pub fn read_exec_main_contents() -> String {
    let path: String = String::from(EXEC_MAIN_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}


//Save Json API Endpoint Schema

pub fn save_api_endpoints(api_endpoints:&String){
    let path= String::from(API_SCHEMA_PATH);
    fs::write(path, api_endpoints).expect("Failed to write API ENDPoint to file");
}




#[cfg(test)]
mod  tests {
    use super::*;
    use crate::ai_function::ai_fun_managing_agents::convert_user_input_to_goal;

    #[test]
    fn test_extending_ai_function(){
        let res = extent_ai_function(convert_user_input_to_goal, "input dummy");
        assert_eq!(res.role,"system".to_string());
                dbg!(res);
    }


    #[tokio::test]
    async fn test_ai_task_request(){
        let ai_func_param: String ="Build me a webserver for making todolist api request".to_string();
        let reponse = ai_task_request(ai_func_param, "Managing agent", "Defining user reqiurements",convert_user_input_to_goal)
        .await;
    
        assert!(reponse.len() >20);
    }
}