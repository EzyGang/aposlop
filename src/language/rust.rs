use std::borrow::Cow;

use tree_sitter::Language;

use super::{LanguageId, LanguageSupport};

pub(super) static SUPPORT: RustSupport = RustSupport;

pub(super) struct RustSupport;

impl LanguageSupport for RustSupport {
    fn id(&self) -> LanguageId {
        LanguageId::Rust
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["rs"]
    }

    fn grammar(&self, extension: &str) -> Option<Language> {
        (extension == "rs").then(|| tree_sitter_rust::LANGUAGE.into())
    }

    fn normalization_query(&self, _extension: &str) -> Cow<'static, str> {
        Cow::Borrowed(include_str!("../queries/rust/normalize.scm"))
    }

    fn metrics_query(&self) -> &'static str {
        include_str!("../queries/rust/metrics.scm")
    }
}
