use super::*;
use crate::provider::models::{ensure_model_allowed_for_subscription, filtered_display_models};

fn with_clean_provider_test_env<T>(f: impl FnOnce() -> T) -> T {
    let _guard = crate::storage::lock_test_env();
    let temp = tempfile::tempdir().expect("tempdir");
    let prev_home = std::env::var_os("JCODE_HOME");
    let prev_subscription =
        std::env::var_os(crate::subscription_catalog::JCODE_SUBSCRIPTION_ACTIVE_ENV);
    crate::env::set_var("JCODE_HOME", temp.path());
    crate::subscription_catalog::clear_runtime_env();
    crate::auth::claude::set_active_account_override(None);
    crate::auth::codex::set_active_account_override(None);

    let result = f();

    crate::auth::claude::set_active_account_override(None);
    crate::auth::codex::set_active_account_override(None);
    if let Some(prev_home) = prev_home {
        crate::env::set_var("JCODE_HOME", prev_home);
    } else {
        crate::env::remove_var("JCODE_HOME");
    }
    if let Some(prev_subscription) = prev_subscription {
        crate::env::set_var(
            crate::subscription_catalog::JCODE_SUBSCRIPTION_ACTIVE_ENV,
            prev_subscription,
        );
    } else {
        crate::env::remove_var(crate::subscription_catalog::JCODE_SUBSCRIPTION_ACTIVE_ENV);
    }
    crate::subscription_catalog::clear_runtime_env();
    result
}

fn enter_test_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime")
}

fn with_env_var<T>(key: &str, value: &str, f: impl FnOnce() -> T) -> T {
    let prev = std::env::var_os(key);
    crate::env::set_var(key, value);
    let result = f();
    if let Some(prev) = prev {
        crate::env::set_var(key, prev);
    } else {
        crate::env::remove_var(key);
    }
    result
}

struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(key);
        crate::env::set_var(key, value);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            crate::env::set_var(self.key, previous);
        } else {
            crate::env::remove_var(self.key);
        }
    }
}

fn test_multi_provider_with_cursor() -> MultiProvider {
    MultiProvider {
        claude: RwLock::new(None),
        anthropic: RwLock::new(None),
        openai: RwLock::new(None),
        copilot_api: RwLock::new(None),
        antigravity: RwLock::new(None),
        gemini: RwLock::new(None),
        cursor: RwLock::new(Some(Arc::new(cursor::CursorCliProvider::new()))),
        openrouter: RwLock::new(None),
        active: RwLock::new(ActiveProvider::Cursor),
        use_claude_cli: false,
        startup_notices: RwLock::new(Vec::new()),
        forced_provider: None,
    }
}

#[test]
fn openai_compatible_routes_do_not_inject_openrouter_fallbacks() {
    with_clean_provider_test_env(|| {
        let _api_base = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_API_BASE_ENV,
            "https://ollama.com/v1",
        );
        let _key_name = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
            "OLLAMA_API_KEY",
        );
        let _api_key = EnvVarGuard::set("OLLAMA_API_KEY", "test-key");
        let _env_file = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
            "ollama-cloud.env",
        );
        let _cache = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
            "ollama-cloud",
        );
        let _provider_id = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV,
            "ollama-cloud",
        );
        let _provider_name = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV,
            "Ollama Cloud",
        );
        let _features = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
            "0",
        );
        let _static_models = EnvVarGuard::set(
            crate::provider_catalog::OPENAI_COMPAT_RUNTIME_STATIC_MODELS_ENV,
            "kimi-k2.6:cloud\nllama3.3:cloud",
        );

        let openrouter =
            Arc::new(openrouter::OpenRouterProvider::new().expect("openai-compatible provider"));
        let provider = MultiProvider {
            claude: RwLock::new(None),
            anthropic: RwLock::new(None),
            openai: RwLock::new(None),
            copilot_api: RwLock::new(None),
            antigravity: RwLock::new(None),
            gemini: RwLock::new(None),
            cursor: RwLock::new(None),
            openrouter: RwLock::new(Some(openrouter)),
            active: RwLock::new(ActiveProvider::OpenRouter),
            use_claude_cli: false,
            startup_notices: RwLock::new(Vec::new()),
            forced_provider: Some(ActiveProvider::OpenRouter),
        };

        let routes = provider.model_routes();
        assert!(
            routes.iter().any(|route| route.model == "kimi-k2.6:cloud"
                && route.provider == "Ollama Cloud"
                && route.api_method == "openai-compatible"),
            "routes should include Ollama Cloud static model: {routes:?}"
        );
        assert!(
            !routes.iter().any(|route| route.api_method == "openrouter"),
            "OpenRouter-only fallback routes should not be injected for Ollama Cloud: {routes:?}"
        );
    });
}

include!("tests/auth_refresh.rs");
include!("tests/model_resolution.rs");
include!("tests/fallback_failover.rs");
include!("tests/catalog_subscription.rs");
