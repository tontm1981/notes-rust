use std::error::Error;
use crate::domain::usecase::UsecaseInterface;
use crate::domain::user_repository::FindUserRepositoryInterface;

pub struct FindUserUsecase {
    findRepository: dyn FindUserRepositoryInterface,
}

impl UsecaseInterface for FindUserUsecase {
    fn execute<T, U>(&self, input: T) -> Result<U, Box<dyn Error>> {
        todo!()
    }
}