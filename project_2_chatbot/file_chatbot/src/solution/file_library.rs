use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    let bytes_result = session.to_bytes();
    match bytes_result {
        Err(_) => {
            eprintln!("Failed to convert session to bytes");
        }
        Ok(bytes) => {
            let write_result = fs::write(filename, bytes);
            match write_result {
                Ok(_) => {
                }
                Err(e) => {
                    eprintln!("Failed to save session to {}: {}", filename, e);
                }
            }
        }
    }
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let read_result = fs::read(filename);
    let bytes = match read_result {
        Ok(bytes) => bytes,
        Err(_) => {
            return None;
        }
    };
    let session_result = LlamaChatSession::from_bytes(&bytes);
    return session_result.ok();
}
