#[derive(Fail, Debug)]
pub enum WormCellError {
    #[fail(display = "Tried to read a WormReader that wasn't set")]
    ReadNotSet,
    #[fail(display = "Tried to set a WormCell twice")]
    DoubleSet
}

pub type WormCellResult<T> = Result<T, WormCellError>;
