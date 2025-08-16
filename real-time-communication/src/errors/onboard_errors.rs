#[derive(Debug)]
pub enum OnboardErrors {
    InvalidCommand(&'static str),
}
