mod security;
mod orchestrator;
mod memory;
mod eventbus;
mod tools;
mod channel;
mod providers;

use orchestrator::context::ContextManager;
use orchestrator::router::ModelRouter;
use tools::registry::ToolRegistry;
use tools::calculator::CalculatorTool;
use providers::groq::GroqProvider;
use tracing_subscriber::registry;


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

    // model router
    let mut router = ModelRouter::new();

    router.add_provider(Box::new(GroqProvider::new(String::from(""))));
    println!("providers registered: {}\n", router.provider_count());

    match router.route("explain attention mechanisms"){
        Ok(response)=>println!("response:{}", response),
        Err(e)=> println!("routing failed:{}",e),
    }

    match run_pipeline("calulate 10 + 5", &registry){
        Ok(result)=> println!("\n pipeline result:{}", result),
        Err(e)=>println!("\npipeline failed: {}", e)
    }
    
}
fn run_pipeline(task: &str, registry: &ToolRegistry) -> Result<String, orchestrator::router::RelayEdgeError> {
    use orchestrator::router::RelayEdgeError;

    let tool = registry
        .get("calculator")
        .ok_or(RelayEdgeError::ToolNotFound(String::from("calculator")))?;

    let result = tool.execute(task);
    Ok(result)
}