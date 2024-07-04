use thiserror::Error;

#[derive(Error, Debug)]
pub enum N2NError {
    #[error("no supernode")]
    SnNumIsZero,
    #[error("conf.community_name is null")]
    CommunityNameNull,
    #[error("ptr is null")]
    PtrNull,
    #[error("i32 is negative when transform to c_int")]
    I32NegativeToCInt,
    #[error("c_char length {0} shorter than slice length {1}")]
    CCharLenTooLong(usize, usize),
    #[error("Unknown error")]
    UnKnown,
}
