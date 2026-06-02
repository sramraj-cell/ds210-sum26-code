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

                let mut chat_session = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                let saved_session = file_library::load_chat_session_from_file(&filename);
                match saved_session {
                    None => {}
                    Some(previous_session) => {
                        chat_session = fixed_load_session(
                            self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                            previous_session
                        );
                    }
                }
                let response = match chat_session.add_message(message).await {
                    Ok(r) => r,
                    Err(_) => String::from("Error generating response"),
                };

                {
                    let session_result = chat_session.session();
                    match session_result {
                        Ok(session) => {
                            file_library::save_chat_session_to_file(&filename, &session);
                        }
                        Err(_) => {}
                    }
                }

                self.cache.put(username, chat_session);

                return response;
            }

            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache!");
                let response = match chat_session.add_message(message).await {
                    Ok(r) => r,
                    Err(_) => String::from("Error generating response"),
                };

                {
                    let session_result = chat_session.session();
                    match session_result {
                        Ok(session) => {
                            file_library::save_chat_session_to_file(&filename, &session);
                        }
                        Err(_) => {}
                    }
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
                let saved_session = file_library::load_chat_session_from_file(&filename);
                match saved_session {
                    None => {
                        return Vec::new();
                    }
                    Some(session) => {
                        let history = session
                            .history()
                            .iter()
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
                }
            }

            Some(chat_session) => {
                println!("get_history: {username} is in the cache!");
                let session_result = chat_session.session();
                match session_result {
                    Err(_) => {
                        return Vec::new();
                    }
                    Ok(session) => {
                        return session
                            .history()
                            .iter()
                            .filter(|m| m.role() != MessageType::SystemPrompt)
                            .map(|m| m.content().to_string())
                            .collect();
                    }
                }
            }
        }
    }
}