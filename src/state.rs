use crate::posts::Posts;

#[derive(Clone)]
pub struct AppState {
    pub posts: Posts,
}

pub trait InitState {
    fn init_state() -> Self;
}

impl InitState for AppState {
    fn init_state() -> Self {
        AppState {
            posts: Posts::init_state(),
        }
    }
}
