mod query;
mod tokenize;

use std::fs;
use std::path::PathBuf;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tree_sitter::Parser;

use crate::ingest::{FileIdentity, SourceFile};
use crate::language::LanguageRegistry;

use self::query::{block_nodes, contains_invalid};
use self::tokenize::build_block;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct AnalyzedFile {
    pub(crate) identity: FileIdentity,
    pub(crate) line_count: usize,
    pub(crate) blocks: Vec<AnalyzedBlock>,
    pub(crate) diagnostics: Vec<AnalysisDiagnostic>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct AnalyzedBlock {
    pub(crate) location: SourceLocation,
    pub(crate) start_byte: usize,
    pub(crate) end_byte: usize,
    pub(crate) line_count: usize,
    pub(crate) named_node_count: usize,
    pub(crate) exact: Vec<u8>,
    pub(crate) normalized: Vec<u8>,
    pub(crate) exact_hash: u64,
    pub(crate) normalized_hash: u64,
    pub(crate) shingles: Vec<u64>,
    pub(crate) complexity: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) struct SourceLocation {
    pub(crate) path: PathBuf,
    pub(crate) start_line: usize,
    pub(crate) end_line: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct AnalysisDiagnostic {
    pub(crate) path: PathBuf,
    pub(crate) kind: AnalysisDiagnosticKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum AnalysisDiagnosticKind {
    #[serde(rename = "read")]
    Read(String),
    #[serde(rename = "parse")]
    Parse(String),
    #[serde(rename = "partial_parse")]
    PartialParse,
}

#[derive(Debug, Error)]
pub(crate) enum AnalysisError {
    #[error("no language provider exists for .{0}")]
    MissingProvider(String),
    #[error("failed to initialize parser for .{extension}: {source}")]
    Parser {
        extension: String,
        #[source]
        source: tree_sitter::LanguageError,
    },
}

pub(crate) fn analyze(
    files: Vec<SourceFile>,
    registry: &LanguageRegistry,
) -> Result<Vec<AnalyzedFile>, AnalysisError> {
    let analyzed: Vec<_> = files
        .into_par_iter()
        .map_init(Parser::new, |parser, file| {
            analyze_file(parser, file, registry)
        })
        .collect();
    let mut analyzed: Vec<_> = analyzed.into_iter().collect::<Result<_, _>>()?;

    analyzed.sort_unstable_by(|left, right| left.identity.path.cmp(&right.identity.path));
    Ok(analyzed)
}

fn analyze_file(
    parser: &mut Parser,
    file: SourceFile,
    registry: &LanguageRegistry,
) -> Result<AnalyzedFile, AnalysisError> {
    let extension = file.identity.extension();
    let provider = registry
        .get(extension)
        .ok_or_else(|| AnalysisError::MissingProvider(extension.to_owned()))?;

    parser
        .set_language(&provider.grammar)
        .map_err(|source| AnalysisError::Parser {
            extension: extension.to_owned(),
            source,
        })?;

    let source = match fs::read(&file.read_path) {
        Ok(source) => source,
        Err(error) => {
            return Ok(file_diagnostic(
                file,
                0,
                AnalysisDiagnosticKind::Read(error.to_string()),
            ));
        }
    };
    let line_count = source_line_count(&source);
    let Some(tree) = parser.parse(&source, None) else {
        return Ok(file_diagnostic(
            file,
            line_count,
            AnalysisDiagnosticKind::Parse("parser returned no syntax tree".to_owned()),
        ));
    };

    let root = tree.root_node();
    let mut diagnostics = Vec::new();
    let partial = root.has_error();
    let mut blocks = Vec::new();

    for block in block_nodes(provider, root, &source) {
        if contains_invalid(block) {
            continue;
        }
        blocks.push(build_block(provider, block, &source, &file.identity.path));
    }

    blocks.sort_unstable_by_key(|block| (block.start_byte, block.end_byte));
    blocks.dedup_by_key(|block| (block.start_byte, block.end_byte));

    if partial {
        diagnostics.push(AnalysisDiagnostic {
            path: file.identity.path.clone(),
            kind: AnalysisDiagnosticKind::PartialParse,
        });
    }

    Ok(AnalyzedFile {
        identity: file.identity,
        line_count,
        blocks,
        diagnostics,
    })
}

fn file_diagnostic(
    file: SourceFile,
    line_count: usize,
    kind: AnalysisDiagnosticKind,
) -> AnalyzedFile {
    AnalyzedFile {
        diagnostics: vec![AnalysisDiagnostic {
            path: file.identity.path.clone(),
            kind,
        }],
        identity: file.identity,
        line_count,
        blocks: Vec::new(),
    }
}

fn source_line_count(source: &[u8]) -> usize {
    source.iter().filter(|&&byte| byte == b'\n').count()
        + usize::from(!source.is_empty() && source.last() != Some(&b'\n'))
}
