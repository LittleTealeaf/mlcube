use crate::puzzle::ApplyActionError;

#[derive(Debug)]
pub enum RecordActionError {
    ApplyActionError(ApplyActionError),
}

impl From<ApplyActionError> for RecordActionError {
    fn from(value: ApplyActionError) -> Self {
        Self::ApplyActionError(value)
    }
}

pub enum SampleReplayError {
    EmptyReplay,
}
