use crate::domain::entities::User;
use crate::domain::interfaces::user_repository::CreateUserRepositoryInterface;

pub struct CreateUserUsecase {
    createRepository: dyn CreateUserRepositoryInterface,
}

impl CreateUserRepositoryInterface for CreateUserUsecase {
    fn create_user(&self, username: String, password: String) -> User {
        todo!()
    }
}
