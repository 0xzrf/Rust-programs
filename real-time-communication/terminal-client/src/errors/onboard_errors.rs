#[derive(Debug)]
pub enum OnboardErrors {
    CreateErrors(&'static str),
    JoinErrors(&'static str),
    ServerError(&'static str),
}

#[derive(Debug)]
pub enum CreateErrors {
    RoomNotCreated(&'static str),
}

#[derive(Debug)]
pub enum JoinErrors {
    RoomNotJoined(&'static str),
}

#[derive(Debug)]
pub enum ServerConnectionError {
    CouldntConnectServer(&'static str),
}

impl From<CreateErrors> for OnboardErrors {
    fn from(value: CreateErrors) -> Self {
        match value {
            CreateErrors::RoomNotCreated(str) => Self::CreateErrors(str),
        }
    }
}
impl From<ServerConnectionError> for OnboardErrors {
    fn from(value: ServerConnectionError) -> Self {
        match value {
            ServerConnectionError::CouldntConnectServer(str) => Self::CreateErrors(str),
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
