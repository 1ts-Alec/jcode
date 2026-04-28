use anyhow::Result;

use crate::provider_catalog::{
    LEGACY_OPENROUTER_API_BASE_ENV, LEGACY_OPENROUTER_API_KEY_NAME_ENV,
    LEGACY_OPENROUTER_AUTH_HEADER_ENV, LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV,
    LEGACY_OPENROUTER_DYNAMIC_BEARER_PROVIDER_ENV, LEGACY_OPENROUTER_ENV_FILE_ENV,
    LEGACY_OPENROUTER_MODEL_CATALOG_ENV, LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV,
    OPENAI_COMPAT_RUNTIME_API_BASE_ENV, OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
    OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV, OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
    OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER_ENV, OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
    OPENAI_COMPAT_RUNTIME_MODEL_CATALOG_ENV, OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
    OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV, OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV,
    load_api_key_from_env_or_config, load_env_value_from_env_or_config, normalize_api_base,
    remove_openai_compat_runtime_var, set_openai_compat_runtime_var,
};

pub const ENV_FILE: &str = "azure-openai.env";
pub const ENDPOINT_ENV: &str = "AZURE_OPENAI_ENDPOINT";
pub const API_KEY_ENV: &str = "AZURE_OPENAI_API_KEY";
pub const MODEL_ENV: &str = "AZURE_OPENAI_MODEL";
pub const USE_ENTRA_ENV: &str = "AZURE_OPENAI_USE_ENTRA";
pub const COGNITIVE_SCOPE: &str = "https://cognitiveservices.azure.com/.default";

fn parse_bool(raw: &str) -> Option<bool> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

pub fn normalize_endpoint(raw: &str) -> Option<String> {
    let mut endpoint = normalize_api_base(raw)?;
    if endpoint.ends_with("/openai/v1") {
        return Some(endpoint);
    }
    endpoint.push_str("/openai/v1");
    Some(endpoint)
}

pub fn load_endpoint() -> Option<String> {
    let raw = load_env_value_from_env_or_config(ENDPOINT_ENV, ENV_FILE)?;
    normalize_endpoint(&raw)
}

pub fn load_model() -> Option<String> {
    load_env_value_from_env_or_config(MODEL_ENV, ENV_FILE)
}

pub fn has_api_key() -> bool {
    load_api_key_from_env_or_config(API_KEY_ENV, ENV_FILE).is_some()
}

pub fn uses_entra_id() -> bool {
    load_env_value_from_env_or_config(USE_ENTRA_ENV, ENV_FILE)
        .and_then(|value| parse_bool(&value))
        .unwrap_or(false)
}

pub fn has_configuration() -> bool {
    load_endpoint().is_some() && (has_api_key() || uses_entra_id())
}

pub fn method_detail() -> String {
    let mut parts = Vec::new();
    if has_api_key() {
        parts.push(format!("API key (`{API_KEY_ENV}`)"));
    }
    if uses_entra_id() {
        parts.push("Microsoft Entra ID (DefaultAzureCredential)".to_string());
    }
    if parts.is_empty() {
        "not configured".to_string()
    } else {
        parts.join(" + ")
    }
}

pub fn apply_runtime_env() -> Result<()> {
    let endpoint = load_endpoint().ok_or_else(|| {
        anyhow::anyhow!(
            "{} not found in environment or ~/.config/jcode/{}",
            ENDPOINT_ENV,
            ENV_FILE
        )
    })?;

    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_API_BASE_ENV,
        LEGACY_OPENROUTER_API_BASE_ENV,
        endpoint,
    );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
        LEGACY_OPENROUTER_API_KEY_NAME_ENV,
        API_KEY_ENV,
    );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
        LEGACY_OPENROUTER_ENV_FILE_ENV,
        ENV_FILE,
    );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
        LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV,
        "azure-openai",
    );
    crate::env::set_var(OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV, "azure-openai");
    crate::env::set_var(OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV, "Azure OpenAI");
    crate::env::remove_var(crate::provider_catalog::OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER_ENV);
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
        LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV,
        "0",
    );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_MODEL_CATALOG_ENV,
        LEGACY_OPENROUTER_MODEL_CATALOG_ENV,
        "0",
    );

    if uses_entra_id() {
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV,
            LEGACY_OPENROUTER_AUTH_HEADER_ENV,
            "authorization-bearer",
        );
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER_ENV,
            LEGACY_OPENROUTER_DYNAMIC_BEARER_PROVIDER_ENV,
            "azure",
        );
    } else {
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV,
            LEGACY_OPENROUTER_AUTH_HEADER_ENV,
            "api-key",
        );
        remove_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER_ENV,
            LEGACY_OPENROUTER_DYNAMIC_BEARER_PROVIDER_ENV,
        );
    }

    Ok(())
}

pub async fn get_bearer_token() -> Result<String> {
    jcode_azure_auth::get_bearer_token(COGNITIVE_SCOPE).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_endpoint_appends_openai_v1() {
        assert_eq!(
            normalize_endpoint("https://example.openai.azure.com"),
            Some("https://example.openai.azure.com/openai/v1".to_string())
        );
    }

    #[test]
    fn normalize_endpoint_preserves_existing_openai_v1() {
        assert_eq!(
            normalize_endpoint("https://example.openai.azure.com/openai/v1/"),
            Some("https://example.openai.azure.com/openai/v1".to_string())
        );
    }

    #[test]
    fn normalize_endpoint_rejects_insecure_remote_http() {
        assert_eq!(normalize_endpoint("http://example.com"), None);
    }
}
