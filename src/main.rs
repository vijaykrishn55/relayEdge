mod security;
mod orchestrator;
mod memory;
mod eventbus;
mod tools;
mod channel;
mod providers;

use orchestrator::context::ContextManager;

fn main(){
    let mut ctx = ContextManager::new(20);
    ctx.add_message("user", "search arxiv for attention mechanisms");
    ctx.add_message("assistant", "found 3 papers on attention mechanisms");

    println!("Messages in the context: {}", ctx.len());

    for msg in ctx.get_messages(){
        println!("[{}]: {}", msg.role, msg.content);
    }
    
}