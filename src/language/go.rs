use std::borrow::Cow;

use tree_sitter::Language;

use super::{LanguageId, LanguageSupport};

pub(super) static SUPPORT: GoSupport = GoSupport;

pub(super) struct GoSupport;

impl LanguageSupport for GoSupport {
    fn id(&self) -> LanguageId {
        LanguageId::Go
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["go"]
    }

    fn grammar(&self, extension: &str) -> Option<Language> {
        (extension == "go").then(|| tree_sitter_go::LANGUAGE.into())
    }

    fn normalization_query(&self, _extension: &str) -> Cow<'static, str> {
        Cow::Borrowed(include_str!("../queries/go/normalize.scm"))
    }

    fn metrics_query(&self) -> &'static str {
        include_str!("../queries/go/metrics.scm")
    }
}
