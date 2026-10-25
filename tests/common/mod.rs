use std::path::Path;

/// Check that all required paths exist. Returns `false` (and logs which path
/// is missing) if any path is absent, so the caller can skip the test.
#[allow(dead_code)]
pub fn require_paths(paths: &[&Path]) -> bool {
    for path in paths {
        if !path.exists() {
            eprintln!("Skipping test: {:?} not found", path);
            return false;
        }
    }
    true
}

/// The process's one ONNX Runtime environment, shared by every test in the binary (ort allows a
/// single environment per process and tests run on parallel threads).
#[cfg(any(feature = "onnx", feature = "vad-silero"))]
#[allow(dead_code)]
pub fn ort_env() -> &'static ort::environment::Environment {
    static ENV: std::sync::OnceLock<ort::environment::Environment> = std::sync::OnceLock::new();
    ENV.get_or_init(|| {
        ort::init()
            .build()
            .expect("create the ONNX Runtime environment")
    })
}
