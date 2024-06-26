use derive_more::Display;
use enum_iterator::Sequence;

/// Rotations clockwise starting from the positive y-axis.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Sequence, Display)]
pub enum Rotation {
    D0,
    D90,
    D180,
    D270,
}

impl From<Rotation> for usize {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::D0 => 0,
            Rotation::D90 => 1,
            Rotation::D180 => 2,
            Rotation::D270 => 3,
        }
    }
}
