use std::fs;

use tempfile::TempDir;

use crate::analysis::analyze;
use crate::detection::{CloneKind, detect};
use crate::ingest::discover;
use crate::language::LanguageRegistry;

use super::configuration::load_config;
type TestResult<T = ()> = anyhow::Result<T>;
type GroupSummary = (CloneKind, Vec<(String, usize, usize)>);

const RUST: &str = r#"fn first(a: i32, b: i32) -> Vec<i32> {
    println!("{a}");
    let total = a + b;
    let mut items = Vec::new();
    for item in 0..total {
        if item > 0 {
            items.push(item);
        }
    }
    items
}

fn second(x: i32) -> Vec<i32> {
    let y = x * 2;
    let total = x + y;
    let mut items = Vec::new();
    for item in 0..total {
        if item > 0 {
            items.push(item);
        }
    }
    items
}
"#;

const PYTHON: &str = "def first(a, b):
    print(a)
    total = a + b
    items = []
    for item in range(total):
        if item:
            items.append(item)
    return items


def second(x):
    y = x * 2
    total = x + y
    items = []
    for item in range(total):
        if item:
            items.append(item)
    return items
";

const TYPESCRIPT: &str = "export function first(a: number, b: number) {
  console.log(a);
  const total = a + b;
  const items = [];
  for (let item = 0; item < total; item++) {
    if (item > 0) {
      items.push(item);
    }
  }
  return items;
}

export function second(x: number) {
  const y = x * 2;
  const total = x + y;
  const items = [];
  for (let item = 0; item < total; item++) {
    if (item > 0) {
      items.push(item);
    }
  }
  return items;
}
";

const TSX: &str = "export function First({ a, b }: Props) {
  console.log(a);
  const total = a + b;
  const items = [];
  for (let item = 0; item < total; item++) {
    if (item > 0) {
      items.push(<li key={item}>{item}</li>);
    }
  }
  return <ul>{items}</ul>;
}

export function Second({ x }: Props) {
  const y = x * 2;
  const total = x + y;
  const items = [];
  for (let item = 0; item < total; item++) {
    if (item > 0) {
      items.push(<li key={item}>{item}</li>);
    }
  }
  return <ul>{items}</ul>;
}
";

const GO: &str = "package sample

func first(a int, b int) []int {
	fmt.Println(a)
	total := a + b
	items := []int{}
	for item := 0; item < total; item++ {
		if item > 0 {
			items = append(items, item)
		}
	}
	return items
}

func second(x int) []int {
	y := x * 2
	total := x + y
	items := []int{}
	for item := 0; item < total; item++ {
		if item > 0 {
			items = append(items, item)
		}
	}
	return items
}
";

#[test]
fn every_provider_reports_the_outermost_copied_statement_run() -> TestResult {
    let cases = [
        ("copy.rs", RUST, (3, 10), (15, 22)),
        ("copy.py", PYTHON, (3, 8), (13, 18)),
        ("copy.ts", TYPESCRIPT, (3, 10), (15, 22)),
        ("copy.tsx", TSX, (3, 10), (15, 22)),
        ("copy.go", GO, (5, 12), (17, 24)),
    ];
    for (path, source, first, second) in cases {
        let fixture = TempDir::new()?;
        fs::write(fixture.path().join(path), source)?;

        let groups = detect_fixture(
            &fixture,
            "[core]\nmin_lines = 3\nmin_nodes = 10\nexclude = []\n[duplicates_detection]\ntype_3 = false",
        )?;

        let expected = vec![(
            CloneKind::Type2,
            vec![
                (path.to_owned(), first.0, first.1),
                (path.to_owned(), second.0, second.1),
            ],
        )];
        assert_eq!(groups, expected, "{path}");
    }
    Ok(())
}

#[test]
fn disabled_languages_and_repeated_statement_units_report_no_runs() -> TestResult {
    let fixture = TempDir::new()?;
    fs::write(fixture.path().join("copy.py"), PYTHON)?;
    fs::write(fixture.path().join("copy.rs"), RUST)?;
    fs::write(
        fixture.path().join("assign.py"),
        "def first(target, source):\n    target.alpha = source.alpha\n    target.beta = source.beta\n    target.gamma = source.gamma\n    target.delta = source.delta\n\n\ndef second(other):\n    print(other)\n    other.one = other.two\n    other.three = other.four\n    other.five = other.six\n    other.seven = other.eight\n",
    )?;

    let groups = detect_fixture(
        &fixture,
        "[core]\nmin_lines = 3\nmin_nodes = 10\nexclude = []\n[duplicates_detection]\ntype_3 = false\n[languages.rust]\nstatement_runs = false",
    )?;

    let expected = vec![(
        CloneKind::Type2,
        vec![("copy.py".to_owned(), 3, 8), ("copy.py".to_owned(), 13, 18)],
    )];
    assert_eq!(groups, expected);
    Ok(())
}

#[test]
fn runs_skip_block_duplicates_and_never_overlap() -> TestResult {
    let fixture = TempDir::new()?;
    let copied =
        "def copied(path):\n    handle = open(path)\n    handle.write(path)\n    handle.close()\n";
    fs::write(fixture.path().join("copy_a.py"), copied)?;
    fs::write(fixture.path().join("copy_b.py"), copied)?;
    fs::write(
        fixture.path().join("repeat.py"),
        "def repeat(path):\n    handle = open(path)\n    handle.write(1)\n    handle = open(path)\n    handle.write(2)\n    handle = open(path)\n    handle.write(3)\n",
    )?;
    fs::write(
        fixture.path().join("wrap.py"),
        "def first(items):\n    items.sort(\n        key=lambda item: (\n            item.size,\n            item.name,\n        )\n    )\n\n\ndef second(values):\n    print(values)\n    values.sort(\n        key=lambda item: (\n            item.size,\n            item.name,\n        )\n    )\n",
    )?;

    let groups = detect_fixture(
        &fixture,
        "[core]\nmin_lines = 2\nmin_nodes = 10\nexclude = []\n[duplicates_detection]\ntype_3 = false",
    )?;

    let expected = vec![
        (
            CloneKind::Type1,
            vec![
                ("copy_a.py".to_owned(), 1, 4),
                ("copy_b.py".to_owned(), 1, 4),
            ],
        ),
        (
            CloneKind::Type2,
            vec![
                ("repeat.py".to_owned(), 2, 3),
                ("repeat.py".to_owned(), 4, 5),
                ("repeat.py".to_owned(), 6, 7),
            ],
        ),
        (
            CloneKind::Type1,
            vec![("wrap.py".to_owned(), 3, 6), ("wrap.py".to_owned(), 13, 16)],
        ),
    ];
    assert_eq!(groups, expected);
    Ok(())
}

fn detect_fixture(fixture: &TempDir, config: &str) -> TestResult<Vec<GroupSummary>> {
    let config = load_config(config)?;
    let registry = LanguageRegistry::compile()?;
    let discovery = discover(fixture.path(), &config, &registry)?;
    let files = analyze(discovery.files, &registry)?;
    Ok(detect(&files, &config)
        .into_iter()
        .map(|group| {
            let instances = group
                .instances
                .into_iter()
                .map(|location| {
                    (
                        location.path.to_string_lossy().into_owned(),
                        location.start_line,
                        location.end_line,
                    )
                })
                .collect();
            (group.kind, instances)
        })
        .collect())
}
