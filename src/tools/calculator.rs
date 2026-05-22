use super::Tool;

pub struct CalculatorTool;

impl Tool for CalculatorTool {
        fn name(&self) -> &str {
                "calculator"
        }

        fn execute(&self, query: &str) -> String {
                format!("calculator received : {}", query)
        }
}