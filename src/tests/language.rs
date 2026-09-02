use tree_sitter::Parser;

use crate::language::{LanguageId, LanguageRegistry};
type TestResult = anyhow::Result<()>;

#[test]
fn supported_extensions_select_expected_providers() -> TestResult {
    let registry = LanguageRegistry::compile()?;

    assert_eq!(
        registry.get("go").map(|value| value.id),
        Some(LanguageId::Go)
    );
    assert_eq!(
        registry.get("rs").map(|value| value.id),
        Some(LanguageId::Rust)
    );
    assert_eq!(
        registry.get("py").map(|value| value.id),
        Some(LanguageId::Python)
    );
    assert_eq!(
        registry.get("ts").map(|value| value.id),
        Some(LanguageId::TypeScript)
    );
    assert_eq!(
        registry.get("tsx").map(|value| value.id),
        Some(LanguageId::TypeScript)
    );
    assert!(registry.get("jsx").is_none());
    Ok(())
}

#[test]
fn every_embedded_query_compiles_for_each_grammar() -> TestResult {
    let registry = LanguageRegistry::compile()?;

    for extension in ["go", "rs", "py", "ts", "tsx"] {
        let Some(compiled) = registry.get(extension) else {
            anyhow::bail!("missing provider for .{extension}");
        };
        assert!(!compiled.normalization.capture_names().is_empty());
        assert_eq!(compiled.metrics.capture_names(), &["complexity"]);
    }
    Ok(())
}

#[test]
fn typescript_and_tsx_use_their_specific_grammars() -> TestResult {
    let registry = LanguageRegistry::compile()?;
    let mut parser = Parser::new();

    let typescript_provider = registry
        .get("ts")
        .ok_or_else(|| anyhow::anyhow!("missing TypeScript provider"))?;
    parser.set_language(&typescript_provider.grammar)?;
    let typescript = parser
        .parse("const value: number = 1;", None)
        .ok_or_else(|| anyhow::anyhow!("TypeScript parser returned no tree"))?;
    assert!(!typescript.root_node().has_error());

    let tsx_provider = registry
        .get("tsx")
        .ok_or_else(|| anyhow::anyhow!("missing TSX provider"))?;
    parser.set_language(&tsx_provider.grammar)?;
    let tsx = parser
        .parse("const View = () => <section>{value}</section>;", None)
        .ok_or_else(|| anyhow::anyhow!("TSX parser returned no tree"))?;
    assert!(!tsx.root_node().has_error());
    Ok(())
}
