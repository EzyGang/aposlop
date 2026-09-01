use tree_sitter::Language;

use super::{LanguageId, LanguageSupport};

pub(super) static SUPPORT: PythonSupport = PythonSupport;

pub(super) struct PythonSupport;

impl LanguageSupport for PythonSupport {
    fn id(&self) -> LanguageId {
        LanguageId::Python
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["py"]
    }

    fn grammar(&self, extension: &str) -> Option<Language> {
        (extension == "py").then(|| tree_sitter_python::LANGUAGE.into())
    }

    fn normalization_query(&self) -> &'static str {
        include_str!("../queries/python/normalize.scm")
    }

    fn metrics_query(&self) -> &'static str {
        include_str!("../queries/python/metrics.scm")
    }
}
