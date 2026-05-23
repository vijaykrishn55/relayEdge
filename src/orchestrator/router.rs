use crate::providers::Provider;

#[derive(Debug)]
pub enum RelayEdgeError {

        NoProviderAvailable,
        ProviderError(String),
        ToolNotFound(String),
}

impl std::fmt::Display for RelayEdgeError {
        fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result{
                match self{
                        RelayEdgeError::NoProviderAvailable => write!(f, "no llm provider is currently available"),
                        RelayEdgeError::ProviderError(msg)=> write!(f, "provider error:{}",msg),
                        RelayEdgeError::ToolNotFound(name)=> write!(f, "tool not found: {}", name),
                }
        }
}

pub struct ModelRouter{
        providers: Vec<Box<dyn Provider + Send + Sync>>,
}

impl ModelRouter{
        pub fn new() -> Self{
                ModelRouter{
                        providers:Vec::new(),
                }
        }

        pub fn add_provider(&mut self, provider: Box<dyn Provider + Send +Sync>){
                self.providers.push(provider);
        }

        pub fn route(&self, prompt:&str) -> Result<String, RelayEdgeError>{
                for provider in &self.providers{
                        if provider.is_available(){
                                match provider.complete(prompt){
                                        Ok(response) =>{
                                                println!("[router] used provider: {}", provider.name());
                                                return Ok(response);
                                        }
                                        Err(e)=>{
                                                println!("[router] {} failed:{}, trying nxt...", provider.name(), e );
                                                continue;
                                        }
                                }
                        }
                }
                Err(RelayEdgeError::NoProviderAvailable)
        }

        pub fn provider_count(&self) -> usize{
                self.providers.len()
        }
}