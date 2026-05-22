// tools
pub mod registry;
pub mod calculator;


pub trait Tool {
        fn name(&self) -> &str;

        fn execute(&self, query: &str)-> String;
}