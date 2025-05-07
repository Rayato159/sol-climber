use anchor_lang::prelude::*;

#[error_code]
pub enum MintFailed {
    #[msg("Name is too long")]
    NameIsTooLong,

    #[msg("Symbol is too long")]
    SymbolIsTooLong,

    #[msg("URI is too long")]
    UriIsTooLong,

    #[msg("Metadata creation failed")]
    MetadataCreationFailed,

    #[msg("Master edition creation failed")]
    MasterEditionCreationFailed,
}
