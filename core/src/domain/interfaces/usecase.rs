use std::error::Error;

pub trait UsecaseInterface {
    fn execute<T, U>(&self, input: T) -> Result<U, Box<dyn Error>>;
}