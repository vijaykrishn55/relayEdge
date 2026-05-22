mod security;
mod orchestrator;
mod memory;
mod eventbus;
mod tools;
mod channel;
mod providers;

use orchestrator::context::ContextManager;
use tools::registry::ToolRegistry;
use tools::calculator::CalculatorTool;
use tools::Tool;


fn main(){
    let mut ctx = ContextManager::new(20);
    ctx.add_message("user", "search arxiv for attention mechanisms");
    ctx.add_message("assistant", "found 3 papers on attention mechanisms");

    println!("Messages in the context: {}", ctx.len());

    //tool registry
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(CalculatorTool));
    println!("tools registred: {}", registry.len());

    if let Some(tool)= registry.get("calculator"){
        let result = tool.execute("10 * 5");
        println!("tool result: {}", result);
    }
    
}