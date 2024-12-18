mod state;

use crate::app::state::{get_previous_state_path, get_state_path, save_state};
use crate::repo::Repo;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

// Struct to hold application state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializableState {
    pub repos: HashMap<String, Repo>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub repos: Arc<Mutex<HashMap<String, Repo>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            repos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn save_state(&self) {
        save_state(self.get_serializable());
    }
    pub fn restore_state(&mut self) {
        let mut state_path: String = get_state_path();
        if state_path.is_empty() {
            state_path = get_previous_state_path();
        }

        if Path::new(state_path.as_str()).exists() {
            let content = fs::read_to_string(&state_path).unwrap();
            let restored = serde_json::from_str::<SerializableState>(&content).unwrap();
            self.deserialize_restore(restored);
        }

        if let Ok(s) = self.repos.lock() {
            if s.len() > 0 {
                println!(
                    "Restored state:\n      repo: {}\n      path: {}\n",
                    s.len(),
                    state_path,
                );
            }
        }
    }
    pub fn get_serializable(&self) -> SerializableState {
        SerializableState {
            repos: self.repos.lock().unwrap().to_owned(),
        }
    }
    pub fn deserialize_restore(&mut self, state: SerializableState) {
        self.repos.lock().unwrap().clone_from(&state.repos)
    }

    // Add a new repository
    pub fn add_repo(&self, name: String, repo: Repo) {
        self.repos.lock().unwrap().insert(name, repo);
    }
}
