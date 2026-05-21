#[derive(Debug, Clone)]
pub struct Message {
        pub role : String,
        pub content : String,
}

// context manager 
#[derive(Debug, Clone)]
pub struct ContextManager {
        messages: Vec<Message>,
        max_messages: usize,
}

impl ContextManager {
        pub fn new(max_messages: usize) -> Self {
                ContextManager{
                        messages: Vec::new(),
                        max_messages,
                }
        }

        // adding a message to history 
        pub fn add_message(&mut self, role: &str, content: &str) {
                let message = Message{
                        role: role.to_string(),
                        content: content.to_string(),
                };
                self.messages.push(message);

                //remove the oldest message 
                if self.messages.len() > self.max_messages {
                        self.messages.remove(0);
                }
        }

        // get the context
        pub fn get_messages(&self) -> &Vec<Message> {
                &self.messages
        }

        // current context size
        pub fn len(&self) -> usize {
                self.messages.len()
        }

}