use super::{CoreConfig, DuplicateConfig, FileLengthConfig, MetricsConfig};

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            min_lines: 5,
            min_nodes: 30,
            exclude: ["tests/", "vendor/", "node_modules/", "target/"]
                .map(str::to_owned)
                .to_vec(),
            use_cache: true,
        }
    }
}

impl Default for DuplicateConfig {
    fn default() -> Self {
        Self {
            type_1: true,
            type_2: true,
            type_3: true,
            type_3_threshold: 0.85,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            calculate_complexity: true,
            complexity_threshold: 15,
        }
    }
}

impl Default for FileLengthConfig {
    fn default() -> Self {
        Self {
            max_lines: 300,
            exclude: Vec::new(),
        }
    }
}
