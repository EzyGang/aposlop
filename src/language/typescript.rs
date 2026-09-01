use std::borrow::Cow;

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

    fn normalization_query(&self, extension: &str) -> Cow<'static, str> {
        const BASE: &str = include_str!("../queries/typescript/normalize.scm");
        if extension != "tsx" {
            return Cow::Borrowed(BASE);
        }
        const TSX: &str = include_str!("../queries/typescript/normalize_tsx.scm");
        let mut query = String::with_capacity(BASE.len() + TSX.len() + 1);
        query.push_str(BASE);
        query.push('\n');
        query.push_str(TSX);
        Cow::Owned(query)
    }

    fn metrics_query(&self) -> &'static str {
        include_str!("../queries/typescript/metrics.scm")
    }
}
