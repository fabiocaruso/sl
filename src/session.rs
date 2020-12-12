use super::User;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct Session {
    user: Arc<Mutex<Option<User>>>,
}

impl Session {
    
    pub fn new() -> Self {
        Self { user: Arc::new(Mutex::new(None)) }
    }

    fn aquire(&self) -> MutexGuard<'_, Option<User>> {
        if let Ok(u) = self.user.lock() {
            u
        } else {
            panic!("Session tried to lock a poisoned mutex")
        }
    }

    pub fn set_user(&self, user: User) {
        let mut u = self.aquire();
        *u = Some(user);
    }

    pub fn get_user(&self) -> Option<User> {
        (*self.aquire()).clone()
    }

}
