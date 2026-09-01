use crate::update::is_newer;

#[test]
fn release_versions_are_compared_semantically() {
    assert!(is_newer("1.0.0", "v1.0.1"));
    assert!(is_newer("1.9.9", "v2.0.0"));
    assert!(is_newer("1.0.0-rc.1", "v1.0.0"));
    assert!(!is_newer("1.0.0", "v1.0.0"));
    assert!(!is_newer("1.0.1", "v1.0.0"));
    assert!(!is_newer("invalid", "v2.0.0"));
    assert!(!is_newer("1.0.0", "invalid"));
}
