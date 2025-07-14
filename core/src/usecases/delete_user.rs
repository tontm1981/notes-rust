use crate::domain::entities::User;
use crate::domain::interfaces::user_repository::DeleteUserRepositoryInterface;

pub struct DeleteUserUsecase {
    deleteRepository: dyn DeleteUserRepositoryInterface,
}

impl DeleteUserRepositoryInterface for DeleteUserUsecase {
    fn delete_user(&self, username: String) -> User {
        todo!()
    }
}