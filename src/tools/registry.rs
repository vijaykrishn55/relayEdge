// stores all available tools 
use std::collections::HashMap;
use super::Tool;

pub struct ToolRegistry{
        tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
        pub fn new() ->Self {
                ToolRegistry {
                        tools: HashMap::new(),
                }
        }

        pub fn register(&mut self, tool: Box<dyn Tool + 'static + Send>){
                let name = tool.name().to_string();
                self.tools.insert(name, tool);
        }
        // look up a tool by name
        pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
                self.tools.get(name)
        }
        // how many there 
        pub fn len(&self) -> usize {
                self.tools.len()
        }
}