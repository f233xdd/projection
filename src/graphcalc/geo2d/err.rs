use std::{error, fmt,};

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
        match self {
            Self::ParallelError(e) => e.fmt(f),
            Self::NotParallelError(e) => e.fmt(f),
            Self::VerticalError(e) => e.fmt(f),
            Self::NotVertincalError(e) => e.fmt(f),
            Self::IncludedError(e) => e.fmt(f),
            Self::NotIncludedError(e) => e.fmt(f),
            Self::SuperpositionError(e) => e.fmt(f)
        }
    }
}

impl error::Error for PositionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParallelError(e) => Some(e),
            Self::NotParallelError(e) => Some(e),
            Self::VerticalError(e) => Some(e),
            Self::NotVertincalError(e) => Some(e),
            Self::IncludedError(e) => Some(e),
            Self::NotIncludedError(e) => Some(e),
            Self::SuperpositionError(e) => Some(e),
        }
    }
}

impl From<ParallelError> for PositionError {
    fn from(value: ParallelError) -> Self {
        Self::ParallelError(value)
    }
}
impl From<NotParallelError> for PositionError {
    fn from(value: NotParallelError) -> Self {
        Self::NotParallelError(value)
    }
}
impl From<VerticalError> for PositionError {
    fn from(value: VerticalError) -> Self {
        Self::VerticalError(value)
    }
}
impl From<NotVertincalError> for PositionError {
    fn from(value: NotVertincalError) -> Self {
        Self::NotVertincalError(value)
    }
}
impl From<IncludedError> for PositionError {
    fn from(value: IncludedError) -> Self {
        Self::IncludedError(value)
    }
}
impl From<NotIncludedError> for PositionError {
    fn from(value: NotIncludedError) -> Self {
        Self::NotIncludedError(value)
    }
}
impl From<SuperpositionError> for PositionError {
    fn from(value: SuperpositionError) -> Self {
        Self::SuperpositionError(value)
    }
}

#[derive(Debug)]
pub struct MismatchedComponentError();

impl fmt::Display for MismatchedComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MismatchedComponentError")
    }
}

impl error::Error for MismatchedComponentError {}

#[derive(Debug)]
pub struct InvalidFnArgError();

impl fmt::Display for InvalidFnArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidFnArgError")
    }
}

impl error::Error for InvalidFnArgError {}

#[derive(Debug)]
pub enum Geo2DError {
    PositionError(PositionError),
    MismatchedComponentError(MismatchedComponentError),
    InvalidFnArgError(InvalidFnArgError),
}

impl fmt::Display for Geo2DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PositionError(e) => e.fmt(f),
            Self::MismatchedComponentError(e) => e.fmt(f),
            Self::InvalidFnArgError(e) => e.fmt(f),
        }
    }
}

impl error::Error for Geo2DError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::PositionError(e) => Some(e),
            Self::MismatchedComponentError(e) => Some(e),
            Self::InvalidFnArgError(e) => Some(e),
        }
    }
}

impl From<MismatchedComponentError> for Geo2DError {
    fn from(value: MismatchedComponentError) -> Self {
        Self::MismatchedComponentError(value)
    }
}
impl From<InvalidFnArgError> for Geo2DError {
    fn from(value: InvalidFnArgError) -> Self {
        Self::InvalidFnArgError(value)
    }   
}
impl<T: Into<PositionError>> From<T> for Geo2DError {
    fn from(value: T) -> Self {
        Self::PositionError(value.into())
    }
}
