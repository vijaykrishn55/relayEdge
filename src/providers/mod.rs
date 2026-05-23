// providers

pub trait Provider{

        fn name(&self) -> &str;

        fn is_available(&self) -> bool;

        fn complete(&self, prompt: &str) -> Result<String, String>;
}
pub mod groq;