//! Output encoding for emitted webhook descriptions.

/// Output encoding for [`super::Webhook::write`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitFormat {
    /// JSON document.
    Json,
    /// KDL source (preferred in-repo representation).
    Kdl,
}
