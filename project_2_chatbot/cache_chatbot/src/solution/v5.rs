use std::num::NonZero;
use kalosm::language::*;
use file_chatbot::solution::file_library;
use fix::fixed_load_session;
use lru::LruCache;

pub struct ChatbotV5 {
    model: Llama,
    cache: LruCache<String, Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: LruCache::new(NonZero::new(2).unwrap()),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?

                let mut chat_session = self.model.chat()
                    .with_system_prompt("The assistant will act like a pirate");

                if let Some(session) = file_library::load_chat_session_from_file(&filename) {
                    chat_session = fixed_load_session(
                        self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                        session
                    );
                }

                let response = match chat_session.add_message(message).await {
                    Ok(r) => r,
                    Err(_) => String::from("Error generating response"),
                };

                if let Ok(session) = chat_session.session() {
                    file_library::save_chat_session_to_file(&filename, &session);
                }
                self.cache.put(username, chat_session);

                return response;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?

                let response = match chat_session.add_message(message).await {
                    Ok(r) => r,
                    Err(_) => String::from("Error generating response"),
                };

                if let Ok(session) = chat_session.session() {
                    file_library::save_chat_session_to_file(&filename, &session);
                }
                return response;
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.

                let session = file_library::load_chat_session_from_file(&filename);
                if session.is_none() {
                    return Vec::new();
                }
                let session = session.unwrap();

                let history = session.history().iter()
                    .filter(|m| m.role() != MessageType::SystemPrompt)
                    .map(|m| m.content().to_string())
                    .collect();

                let chat_session = fixed_load_session(
                    self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                    session
                );
                self.cache.put(username, chat_session);

                return history;
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.

                let session = chat_session.session();
                if session.is_err() {
                    return Vec::new();
                }
                return session.unwrap().history().iter()
                    .filter(|m| m.role() != MessageType::SystemPrompt)
                    .map(|m| m.content().to_string())
                    .collect();
            }
        }
    }
}