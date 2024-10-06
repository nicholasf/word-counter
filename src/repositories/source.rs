
use crate::entities::source::Source;

pub trait SourceRepository {
    pub fn write(&self, &source: Source) -> bool;
}

pub struct SourceRepositoryImpl {}


