use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub role: i64,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionInfo {
    pub source_screen: Option<String>,
    pub category1_code: Option<String>,
    pub modal_state: Option<String>,
}

#[derive(Default)]
pub struct SessionState {
    pub current_user: Mutex<Option<User>>,
    pub session_info: Mutex<SessionInfo>,
}

impl SessionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_user(&self, user: User) {
        *self.current_user.lock().unwrap() = Some(user);
    }

    pub fn get_user(&self) -> Option<User> {
        self.current_user.lock().unwrap().clone()
    }

    pub fn clear_user(&self) {
        *self.current_user.lock().unwrap() = None;
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_user.lock().unwrap().is_some()
    }

    pub fn set_source_screen(&self, source_screen: String) {
        self.session_info.lock().unwrap().source_screen = Some(source_screen);
    }

    pub fn get_source_screen(&self) -> Option<String> {
        self.session_info.lock().unwrap().source_screen.clone()
    }

    pub fn clear_source_screen(&self) {
        self.session_info.lock().unwrap().source_screen = None;
    }

    pub fn set_category1_code(&self, category1_code: String) {
        self.session_info.lock().unwrap().category1_code = Some(category1_code);
    }

    pub fn get_category1_code(&self) -> Option<String> {
        self.session_info.lock().unwrap().category1_code.clone()
    }

    pub fn clear_category1_code(&self) {
        self.session_info.lock().unwrap().category1_code = None;
    }

    pub fn set_modal_state(&self, modal_state: String) {
        self.session_info.lock().unwrap().modal_state = Some(modal_state);
    }

    pub fn get_modal_state(&self) -> Option<String> {
        self.session_info.lock().unwrap().modal_state.clone()
    }

    pub fn clear_modal_state(&self) {
        self.session_info.lock().unwrap().modal_state = None;
    }

    pub fn clear_all(&self) {
        self.clear_user();
        *self.session_info.lock().unwrap() = SessionInfo::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_initialization() {
        let state = SessionState::new();
        assert!(!state.is_authenticated());
        assert_eq!(state.get_user(), None);
        assert_eq!(state.get_source_screen(), None);
        assert_eq!(state.get_category1_code(), None);
    }

    #[test]
    fn test_set_and_get_user() {
        let state = SessionState::new();
        let user = User {
            user_id: 1,
            name: "test_user".to_string(),
            role: 0,
        };

        state.set_user(user.clone());
        assert!(state.is_authenticated());
        
        let retrieved_user = state.get_user().unwrap();
        assert_eq!(retrieved_user.user_id, 1);
        assert_eq!(retrieved_user.name, "test_user");
        assert_eq!(retrieved_user.role, 0);
    }

    #[test]
    fn test_clear_user() {
        let state = SessionState::new();
        let user = User {
            user_id: 1,
            name: "test_user".to_string(),
            role: 0,
        };

        state.set_user(user);
        assert!(state.is_authenticated());

        state.clear_user();
        assert!(!state.is_authenticated());
        assert_eq!(state.get_user(), None);
    }

    #[test]
    fn test_set_and_get_source_screen() {
        let state = SessionState::new();
        
        state.set_source_screen("shop_mgmt".to_string());
        assert_eq!(state.get_source_screen(), Some("shop_mgmt".to_string()));
    }

    #[test]
    fn test_clear_source_screen() {
        let state = SessionState::new();
        
        state.set_source_screen("shop_mgmt".to_string());
        assert_eq!(state.get_source_screen(), Some("shop_mgmt".to_string()));

        state.clear_source_screen();
        assert_eq!(state.get_source_screen(), None);
    }

    #[test]
    fn test_set_and_get_category1_code() {
        let state = SessionState::new();
        
        state.set_category1_code("INCOME".to_string());
        assert_eq!(state.get_category1_code(), Some("INCOME".to_string()));
    }

    #[test]
    fn test_clear_category1_code() {
        let state = SessionState::new();
        
        state.set_category1_code("EXPENSE".to_string());
        assert_eq!(state.get_category1_code(), Some("EXPENSE".to_string()));

        state.clear_category1_code();
        assert_eq!(state.get_category1_code(), None);
    }

    #[test]
    fn test_clear_all() {
        let state = SessionState::new();
        
        let user = User {
            user_id: 1,
            name: "test_user".to_string(),
            role: 0,
        };
        state.set_user(user);
        state.set_source_screen("shop_mgmt".to_string());
        state.set_category1_code("INCOME".to_string());

        assert!(state.is_authenticated());
        assert!(state.get_source_screen().is_some());
        assert!(state.get_category1_code().is_some());

        state.clear_all();
        
        assert!(!state.is_authenticated());
        assert_eq!(state.get_user(), None);
        assert_eq!(state.get_source_screen(), None);
        assert_eq!(state.get_category1_code(), None);
    }

    #[test]
    fn test_multiple_session_operations() {
        let state = SessionState::new();
        
        // Set initial values
        let user1 = User {
            user_id: 1,
            name: "user1".to_string(),
            role: 0,
        };
        state.set_user(user1);
        state.set_source_screen("screen1".to_string());
        state.set_category1_code("CODE1".to_string());

        // Update values
        let user2 = User {
            user_id: 2,
            name: "user2".to_string(),
            role: 1,
        };
        state.set_user(user2);
        state.set_source_screen("screen2".to_string());
        state.set_category1_code("CODE2".to_string());

        // Verify updated values
        let retrieved_user = state.get_user().unwrap();
        assert_eq!(retrieved_user.user_id, 2);
        assert_eq!(retrieved_user.name, "user2");
        assert_eq!(retrieved_user.role, 1);
        assert_eq!(state.get_source_screen(), Some("screen2".to_string()));
        assert_eq!(state.get_category1_code(), Some("CODE2".to_string()));
    }
}
