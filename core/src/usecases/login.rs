use std::error::Error;
use crate::domain::interfaces::usecase::UsecaseInterface;
use crate::domain::interfaces::user_repository::{FindUserRepositoryInterface};

pub struct LoginUsecase {
    findRepository: dyn FindUserRepositoryInterface,
}

pub struct LoginUsecaseInput {
    username: String,
    password: String,
}

impl UsecaseInterface for LoginUsecase {
    fn execute<LoginUsecaseInput, U>(&self, input: LoginUsecaseInput) -> Result<U, Box<dyn Error>> {
        todo!()
    }
}