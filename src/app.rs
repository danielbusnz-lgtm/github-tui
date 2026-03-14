use crate::github::{User, Repo, RepoContent};

pub enum Screen {
    Profile,
    Repos,
    Files,
}

pub struct App {
    pub screen: Screen,
    pub user: Option<User>,
    pub repos: Vec<Repo>,
    pub contents: Vec<RepoContent>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            screen: Screen::Profile,
            user: None,
            repos: Vec::new(),
            contents: Vec::new(),
            selected: 0,
        }
    }
}
