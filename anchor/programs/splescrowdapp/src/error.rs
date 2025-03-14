use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Data field Cannot Be Empty")]
    DataFieldEmpty,
}
