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
pub struct CoplanarError();

impl fmt::Display for CoplanarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CoplanarError")
    }
}
impl error::Error for CoplanarError {}

#[derive(Debug)]
pub struct NotCoplanarError();

impl fmt::Display for NotCoplanarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotCoplanarError")
    }
}
impl error::Error for NotCoplanarError {}

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
    CoplanarError(CoplanarError),
    NotCoplanarError(NotCoplanarError),
    SuperpositionError(SuperpositionError),
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::ParallelError(ref e) => e.fmt(f),
            Self::NotParallelError(ref e) => e.fmt(f),
            Self::VerticalError(ref e) => e.fmt(f),
            Self::NotVertincalError(ref e) => e.fmt(f),
            Self::IncludedError(ref e) => e.fmt(f),
            Self::NotIncludedError(ref e) => e.fmt(f),
            Self::CoplanarError(ref e) => e.fmt(f),
            Self::NotCoplanarError(ref e) => e.fmt(f),
            Self::SuperpositionError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for PositionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            Self::ParallelError(ref e) => Some(e),
            Self::NotParallelError(ref e) => Some(e),
            Self::VerticalError(ref e) => Some(e),
            Self::NotVertincalError(ref e) => Some(e),
            Self::IncludedError(ref e) => Some(e),
            Self::NotIncludedError(ref e) => Some(e),
            Self::CoplanarError(ref e) => Some(e),
            Self::NotCoplanarError(ref e) => Some(e),
            Self::SuperpositionError(ref e) => Some(e),
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
impl From<CoplanarError> for PositionError {
    fn from(value: CoplanarError) -> Self {
        Self::CoplanarError(value)
    }
}
impl From<NotCoplanarError> for PositionError {
    fn from(value: NotCoplanarError) -> Self {
        Self::NotCoplanarError(value)
    }
}
impl From<SuperpositionError> for PositionError {
    fn from(value: SuperpositionError) -> Self {
        Self::SuperpositionError(value)
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