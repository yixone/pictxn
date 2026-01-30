use sqlx::SqlitePool;

use crate::repositories::{
    cards::CardsRepository, files::FilesRepository, profiles::ProfilesRepository,
    users::UsersRepository,
};

pub struct RepositoriesContainer {
    cards: CardsRepository,
    files: FilesRepository,

    profiles: ProfilesRepository,
    users: UsersRepository,
}

impl RepositoriesContainer {
    pub fn new(pool: SqlitePool) -> Self {
        RepositoriesContainer {
            cards: CardsRepository::new(pool.clone()),
            files: FilesRepository::new(pool.clone()),
            profiles: ProfilesRepository::new(pool.clone()),
            users: UsersRepository::new(pool),
        }
    }

    pub fn cards(&self) -> &CardsRepository {
        &self.cards
    }

    pub fn files(&self) -> &FilesRepository {
        &self.files
    }

    pub fn profiles(&self) -> &ProfilesRepository {
        &self.profiles
    }

    pub fn users(&self) -> &UsersRepository {
        &self.users
    }
}
