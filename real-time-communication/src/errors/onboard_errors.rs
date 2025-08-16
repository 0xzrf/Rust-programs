#[derive(Debug)]
pub enum OnboardErrors {
    InvalidCommand(&'static str),
    CreateErrors(&'static str),
    JoinErrors(&'static str),
}

#[derive(Debug)]
pub enum CreateErrors {
    RoomNotCreated(&'static str),
}

#[derive(Debug)]
pub enum JoinErrors {
    RoomNotJoined(&'static str),
}

impl From<CreateErrors> for OnboardErrors {
    fn from(value: CreateErrors) -> Self {
        match value {
            CreateErrors::RoomNotCreated(str) => Self::CreateErrors(str),
        }
    }
}

impl From<JoinErrors> for OnboardErrors {
    fn from(value: JoinErrors) -> Self {
        match value {
            JoinErrors::RoomNotJoined(str) => Self::JoinErrors(str),
        }
    }
}
