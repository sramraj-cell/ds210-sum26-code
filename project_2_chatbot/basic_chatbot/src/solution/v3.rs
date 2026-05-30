use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            sessions: HashMap::new(),
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if !self.sessions.contains_key(&username) {
            let session = self.model.chat().with_system_prompt("The assistant will act like a drunk comedian");
            self.sessions.insert(username.clone(), session);    
        }
        let chat_session = self.sessions.get_mut(&username).unwrap();

        match chat_session.add_message(message).await {
            Ok(response) => response,
            Err(_) => String::from("Error generating response"),
        }

        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat_session) = self.sessions.get(&username) {
            let history = chat_session.session().unwrap().history();

            history.iter().map(|message| format!("{:?}", message)).collect()

        } else {
            Vec::new()
        }
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        
    }
}