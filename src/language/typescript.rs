use tree_sitter::Language;

use super::{LanguageId, LanguageSupport};

pub(super) static SUPPORT: TypeScriptSupport = TypeScriptSupport;

pub(super) struct TypeScriptSupport;

impl LanguageSupport for TypeScriptSupport {
    fn id(&self) -> LanguageId {
        LanguageId::TypeScript
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ts", "tsx"]
    }

    fn grammar(&self, extension: &str) -> Option<Language> {
        match extension {
            "ts" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            "tsx" => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
            _ => None,
        }
    }

    fn normalization_query(&self) -> &'static str {
        include_str!("../queries/typescript/normalize.scm")
    }

    fn metrics_query(&self) -> &'static str {
        include_str!("../queries/typescript/metrics.scm")
    }
}
