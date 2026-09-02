use super::{CoreConfig, DuplicateConfig, MetricsConfig};

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            min_lines: 5,
            min_nodes: 30,
            exclude: [
                r"(^|/)tests(?:/|$)",
                r"(^|/)vendor(?:/|$)",
                r"(^|/)node_modules(?:/|$)",
                r"(^|/)target(?:/|$)",
            ]
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
