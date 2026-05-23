use super::Provider;

pub struct GroqProvider{
        pub api_key: String,
        pub model: String,
}

impl GroqProvider{
        pub fn new(api_key:String)-> Self{
                GroqProvider{
                        api_key,
                        model: String::from("llaama-4-scout"),
                }
        }
}

impl Provider for GroqProvider{
        fn name(&self)-> &str {
                "groq"
        }

        fn is_available(&self)->bool{
                !self.api_key.is_empty()
        }

        fn complete(&self, prompt:&str)-> Result<String, String>{

                if self.api_key.is_empty(){
                        Err(String::from("groq api key not set"))
                }else{
                        Ok(format!("[groq/{}] response to : {}", self.model, prompt))
                }
        }
}
