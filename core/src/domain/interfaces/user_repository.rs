use crate::domain::entities::User;

pub trait FindUserRepositoryInterface {
    fn find_by_username(&self, username: String) -> Option<User>;
}

pub trait CreateUserRepositoryInterface {
    fn create_user(&self, username: String, password: String) -> User;
}

pub trait UpdateUserRepositoryInterface {
    fn update_user(&self, username: String, password: String) -> User;
}

pub trait DeleteUserRepositoryInterface {
    fn delete_user(&self, username: String) -> User;
}