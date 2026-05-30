use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            chat_session: model
              .chat()
              .with_system_prompt("The assistant will act like a drunk comedian"),
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        match self.chat_session.add_message(message).await {
            Ok(response) => response,
            Err(_) => String::from("Error generating response"),
        }
        // Add your code for chatting with the agent while keeping conversation history here.
    }
}