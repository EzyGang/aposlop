mod python;
mod rust;
mod typescript;

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tree_sitter::{Language, Query};

const NORMALIZATION_CAPTURES: &[&str] = &[
    "block",
    "anonymize.identifier",
    "anonymize.literal",
    "ignore",
];
const METRIC_CAPTURES: &[&str] = &["complexity"];
static PROVIDERS: [&dyn LanguageSupport; 3] =
    [&rust::SUPPORT, &python::SUPPORT, &typescript::SUPPORT];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LanguageId {
    Rust,
    Python,
    TypeScript,
}

pub(crate) trait LanguageSupport: Sync {
    fn id(&self) -> LanguageId;
    fn extensions(&self) -> &'static [&'static str];
    fn grammar(&self, extension: &str) -> Option<Language>;
    fn normalization_query(&self) -> &'static str;
    fn metrics_query(&self) -> &'static str;
}

pub(crate) struct CompiledLanguage {
    pub(crate) id: LanguageId,
    pub(crate) grammar: Language,
    pub(crate) normalization: Arc<Query>,
    pub(crate) metrics: Arc<Query>,
}

pub(crate) struct LanguageRegistry {
    languages: HashMap<&'static str, CompiledLanguage>,
}

#[derive(Debug, Error)]
pub(crate) enum LanguageError {
    #[error("provider {language} does not supply a grammar for .{extension}")]
    MissingGrammar {
        language: &'static str,
        extension: &'static str,
    },
    #[error("failed to compile {kind} query for .{extension}: {source}")]
    Query {
        extension: &'static str,
        kind: &'static str,
        #[source]
        source: tree_sitter::QueryError,
    },
    #[error("unsupported capture @{capture} in {kind} query for .{extension}")]
    Capture {
        extension: &'static str,
        kind: &'static str,
        capture: String,
    },
}

impl LanguageId {
    #[must_use]
    pub(crate) const fn key(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }
}

impl LanguageRegistry {
    pub(crate) fn compile() -> Result<Self, LanguageError> {
        let mut languages = HashMap::with_capacity(4);
        for provider in PROVIDERS {
            for &extension in provider.extensions() {
                let grammar = provider
                    .grammar(extension)
                    .ok_or(LanguageError::MissingGrammar {
                        language: provider.id().key(),
                        extension,
                    })?;
                let normalization = compile_query(
                    &grammar,
                    provider.normalization_query(),
                    extension,
                    "normalization",
                    NORMALIZATION_CAPTURES,
                )?;
                let metrics = compile_query(
                    &grammar,
                    provider.metrics_query(),
                    extension,
                    "metrics",
                    METRIC_CAPTURES,
                )?;
                languages.insert(
                    extension,
                    CompiledLanguage {
                        id: provider.id(),
                        grammar,
                        normalization: Arc::new(normalization),
                        metrics: Arc::new(metrics),
                    },
                );
            }
        }
        Ok(Self { languages })
    }

    #[must_use]
    pub(crate) fn get(&self, extension: &str) -> Option<&CompiledLanguage> {
        self.languages.get(extension)
    }
}

fn compile_query(
    grammar: &Language,
    source: &str,
    extension: &'static str,
    kind: &'static str,
    captures: &[&str],
) -> Result<Query, LanguageError> {
    let query = Query::new(grammar, source).map_err(|source| LanguageError::Query {
        extension,
        kind,
        source,
    })?;
    for capture in query.capture_names() {
        if !captures.contains(capture) {
            return Err(LanguageError::Capture {
                extension,
                kind,
                capture: (*capture).to_owned(),
            });
        }
    }
    Ok(query)
}
