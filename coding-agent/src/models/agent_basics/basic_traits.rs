use super::basic_agent::AgentState;
use crate::models::generals::llm::Message;

pub  trait BasicTrait {
    fn new(Objective:String,position:String)->Self;
    fn update_state(&mut self,new_state:AgentState);
    fn get_objective(&self) -> &String;
    fn get_state(&self) -> &AgentState;
    fn get_position(&self) -> &String;
    fn get_memory(&self) -> &Vec<Message>;
}
