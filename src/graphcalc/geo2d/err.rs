use std::{error, fmt};

#[derive(Debug)]
pub struct ParallelError();

impl fmt::Display for ParallelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParallelError")
    }
}
impl error::Error for ParallelError {}

#[derive(Debug)]
pub struct NotParallelError();

impl fmt::Display for NotParallelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotParallelError")
    }
}
impl error::Error for NotParallelError {}

type IntersectError = NotParallelError;

#[derive(Debug)]
pub struct VerticalError();

impl fmt::Display for VerticalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VerticalError")
    }
}
impl error::Error for VerticalError {}

#[derive(Debug)]
pub struct NotVertincalError();

impl fmt::Display for NotVertincalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotVertincalError")
    }
}
impl error::Error for NotVertincalError {}

#[derive(Debug)]
pub struct IncludedError();

impl fmt::Display for IncludedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IncludedError")
    }
}
impl error::Error for IncludedError {}

#[derive(Debug)]
pub struct NotIncludedError();

impl fmt::Display for NotIncludedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotIncludedError")
    }
}
impl error::Error for NotIncludedError {}

#[derive(Debug)]
pub struct SuperpositionError();

impl fmt::Display for SuperpositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InterpositionError")
    }
}
impl error::Error for SuperpositionError {}

#[derive(Debug)]
pub enum PositionError {
    ParallelError(ParallelError),
    NotParallelError(NotParallelError),
    VerticalError(VerticalError),
    NotVertincalError(NotVertincalError),
    IncludedError(IncludedError),
    NotIncludedError(NotIncludedError),
    SuperpositionError(SuperpositionError)
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PositionError::ParallelError(ref e) => e.fmt(f),
            PositionError::NotParallelError(ref e) => e.fmt(f),
            PositionError::VerticalError(ref e) => e.fmt(f),
            PositionError::NotVertincalError(ref e) => e.fmt(f),
            PositionError::IncludedError(ref e) => e.fmt(f),
            PositionError::NotIncludedError(ref e) => e.fmt(f),
            PositionError::SuperpositionError(ref e) => e.fmt(f)
        }
    }
}

impl error::Error for PositionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            PositionError::ParallelError(ref e) => Some(e),
            PositionError::NotParallelError(ref e) => Some(e),
            PositionError::VerticalError(ref e) => Some(e),
            PositionError::NotVertincalError(ref e) => Some(e),
            PositionError::IncludedError(ref e) => Some(e),
            PositionError::NotIncludedError(ref e) => Some(e),
            PositionError::SuperpositionError(ref e) => Some(e)
        }
    }
}

impl From<ParallelError> for PositionError {
    fn from(value: ParallelError) -> Self {
        PositionError::ParallelError(value)
    }
}
impl From<NotParallelError> for PositionError {
    fn from(value: NotParallelError) -> Self {
        PositionError::NotParallelError(value)
    }
}
impl From<VerticalError> for PositionError {
    fn from(value: VerticalError) -> Self {
        PositionError::VerticalError(value)
    }
}
impl From<NotVertincalError> for PositionError {
    fn from(value: NotVertincalError) -> Self {
        PositionError::NotVertincalError(value)
    }
}
impl From<IncludedError> for PositionError {
    fn from(value: IncludedError) -> Self {
        PositionError::IncludedError(value)
    }
}
impl From<NotIncludedError> for PositionError {
    fn from(value: NotIncludedError) -> Self {
        PositionError::NotIncludedError(value)
    }
}
impl From<SuperpositionError> for PositionError {
    fn from(value: SuperpositionError) -> Self {
        PositionError::SuperpositionError(value)
    }
}

#[derive(Debug)]
pub struct InvalidFnArgError();


impl fmt::Display for InvalidFnArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidFnArgError")
    }
}

impl error::Error for InvalidFnArgError {}
