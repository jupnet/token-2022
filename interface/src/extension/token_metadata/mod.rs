use crate::extension::{Extension, ExtensionType};

// Re-export TokenMetadata so users get it with the Extension trait impl
pub use spl_token_metadata_interface::state::TokenMetadata;

impl Extension for TokenMetadata {
    const TYPE: ExtensionType = ExtensionType::TokenMetadata;
}
