use crate::domain::entities::User;
use crate::domain::interfaces::user_repository::UpdateUserRepositoryInterface;

pub struct UpdateUserUsecase {
    updateRepository: dyn UpdateUserRepositoryInterface,
}

impl UpdateUserRepositoryInterface for UpdateUserUsecase {
    fn update_user(&self, username: String, password: String) -> User {
        todo!()
    }
}