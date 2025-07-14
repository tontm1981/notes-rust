use std::error::Error;
use crate::domain::interfaces::usecase::UsecaseInterface;

pub struct LogoutUsecase {}

impl UsecaseInterface for LogoutUsecase {
    fn execute<T, U>(&self, input: T) -> Result<U, Box<dyn Error>> {
        todo!()
    }
}