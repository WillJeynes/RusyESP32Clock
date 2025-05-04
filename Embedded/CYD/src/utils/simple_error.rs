#[derive(Debug)]
pub struct SimpleError {
    description: &'static str,
}

impl SimpleError {
    pub fn new(description: &'static str) -> Self {
        Self { description }
    }
}

impl core::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SimpleError {}

pub trait ContextExt<T> {
    fn draw_context(self) -> Result<T, SimpleError>;
}

impl<T, E> ContextExt<T> for Result<T, E> {
    fn draw_context(self) -> Result<T, SimpleError> {
        self.map_err(|_| SimpleError::new("Unhandled Draw Ex"))
    }
}

impl<T> ContextExt<T> for Option<T> {
    fn draw_context(self) -> Result<T, SimpleError> {
        match self {
            None => { Err(SimpleError::new("Unhandled Draw Ex")) }
            Some(v) => { Ok(v) }
        }
    }
}