use kalosm::language::*;
use fix::fixed_load_session;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");

        
        let saved_session = file_library::load_chat_session_from_file(filename);
        match saved_session {
          Some(session) => {
              chat_session = fixed_load_session(
            self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                 session
        );
    }
          None => {
    }
}

        let response = match chat_session.add_message(message).await {
            Ok(response) => response,
            Err(_) => String::from("Error generating response"),
        };

        let session_result = chat_session.session();
        match session_result {
           Ok(session) => {
              file_library::save_chat_session_to_file(filename, &session);
    }
        Err(_) => {}
}

        return response;

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                session
                    .history()
                    .iter()
                    .filter(|m| m.role() != MessageType::SystemPrompt)
                    .map(|message| message.content().to_string())
                    .collect()
                // TODO: what should happen here?
            }
        }
    }
}