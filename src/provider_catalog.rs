pub use jcode_provider_metadata::*;
use std::collections::HashSet;

pub const OPENAI_COMPAT_LOCAL_ENABLED_ENV: &str = "JCODE_OPENAI_COMPAT_LOCAL_ENABLED";

pub const OPENAI_COMPAT_RUNTIME_API_BASE_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_API_BASE";
pub const OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_API_KEY_NAME";
pub const OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_ENV_FILE";
pub const OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE";
pub const OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_ID";
pub const OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_NAME";
pub const OPENAI_COMPAT_RUNTIME_MODEL_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_MODEL";
pub const OPENAI_COMPAT_RUNTIME_STATIC_MODELS_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_STATIC_MODELS";
pub const OPENAI_COMPAT_RUNTIME_MODEL_CATALOG_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_MODEL_CATALOG";
pub const OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES";
pub const OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH";
pub const OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_AUTH_HEADER";
pub const OPENAI_COMPAT_RUNTIME_AUTH_HEADER_NAME_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_AUTH_HEADER_NAME";
pub const OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER";
pub const OPENAI_COMPAT_RUNTIME_PROVIDER_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER";
pub const OPENAI_COMPAT_RUNTIME_NO_FALLBACK_ENV: &str = "JCODE_OPENAI_COMPAT_RUNTIME_NO_FALLBACK";
pub const OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER_ENV: &str =
    "JCODE_OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER";

pub const LEGACY_OPENROUTER_API_BASE_ENV: &str = "JCODE_OPENROUTER_API_BASE";
pub const LEGACY_OPENROUTER_API_KEY_NAME_ENV: &str = "JCODE_OPENROUTER_API_KEY_NAME";
pub const LEGACY_OPENROUTER_ENV_FILE_ENV: &str = "JCODE_OPENROUTER_ENV_FILE";
pub const LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV: &str = "JCODE_OPENROUTER_CACHE_NAMESPACE";
pub const LEGACY_OPENROUTER_MODEL_ENV: &str = "JCODE_OPENROUTER_MODEL";
pub const LEGACY_OPENROUTER_STATIC_MODELS_ENV: &str = "JCODE_OPENROUTER_STATIC_MODELS";
pub const LEGACY_OPENROUTER_MODEL_CATALOG_ENV: &str = "JCODE_OPENROUTER_MODEL_CATALOG";
pub const LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV: &str = "JCODE_OPENROUTER_PROVIDER_FEATURES";
pub const LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV: &str = "JCODE_OPENROUTER_ALLOW_NO_AUTH";
pub const LEGACY_OPENROUTER_AUTH_HEADER_ENV: &str = "JCODE_OPENROUTER_AUTH_HEADER";
pub const LEGACY_OPENROUTER_AUTH_HEADER_NAME_ENV: &str = "JCODE_OPENROUTER_AUTH_HEADER_NAME";
pub const LEGACY_OPENROUTER_DYNAMIC_BEARER_PROVIDER_ENV: &str =
    "JCODE_OPENROUTER_DYNAMIC_BEARER_PROVIDER";
pub const LEGACY_OPENROUTER_PROVIDER_ENV: &str = "JCODE_OPENROUTER_PROVIDER";
pub const LEGACY_OPENROUTER_NO_FALLBACK_ENV: &str = "JCODE_OPENROUTER_NO_FALLBACK";

pub const OPENAI_COMPAT_RUNTIME_ENV_PAIRS: &[(&str, &str)] = &[
    (
        OPENAI_COMPAT_RUNTIME_API_BASE_ENV,
        LEGACY_OPENROUTER_API_BASE_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
        LEGACY_OPENROUTER_API_KEY_NAME_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
        LEGACY_OPENROUTER_ENV_FILE_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
        LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV,
    ),
    (OPENAI_COMPAT_RUNTIME_MODEL_ENV, LEGACY_OPENROUTER_MODEL_ENV),
    (
        OPENAI_COMPAT_RUNTIME_STATIC_MODELS_ENV,
        LEGACY_OPENROUTER_STATIC_MODELS_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_MODEL_CATALOG_ENV,
        LEGACY_OPENROUTER_MODEL_CATALOG_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
        LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV,
        LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV,
        LEGACY_OPENROUTER_AUTH_HEADER_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_AUTH_HEADER_NAME_ENV,
        LEGACY_OPENROUTER_AUTH_HEADER_NAME_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER_ENV,
        LEGACY_OPENROUTER_DYNAMIC_BEARER_PROVIDER_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_PROVIDER_ENV,
        LEGACY_OPENROUTER_PROVIDER_ENV,
    ),
    (
        OPENAI_COMPAT_RUNTIME_NO_FALLBACK_ENV,
        LEGACY_OPENROUTER_NO_FALLBACK_ENV,
    ),
];

pub fn openai_compat_runtime_var(neutral: &str, legacy: &str) -> Option<String> {
    std::env::var(neutral)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .or_else(|| {
            std::env::var(legacy)
                .ok()
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
        })
}

pub fn openai_compat_runtime_var_is_set(neutral: &str, legacy: &str) -> bool {
    std::env::var_os(neutral).is_some() || std::env::var_os(legacy).is_some()
}

pub fn set_openai_compat_runtime_var(neutral: &str, legacy: &str, value: impl AsRef<str>) {
    let value = value.as_ref();
    crate::env::set_var(neutral, value);
    // Keep the legacy alias populated during the transition so older helpers and
    // user automation continue to work. New code should read the neutral name first.
    crate::env::set_var(legacy, value);
}

pub fn remove_openai_compat_runtime_var(neutral: &str, legacy: &str) {
    crate::env::remove_var(neutral);
    crate::env::remove_var(legacy);
}

pub fn openai_compatible_models_dev_provider_id(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "302ai" => Some("302ai"),
        "alibaba-coding-plan" => Some("alibaba-coding-plan"),
        "baseten" => Some("baseten"),
        "cerebras" => Some("cerebras"),
        "chutes" => Some("chutes"),
        "cortecs" => Some("cortecs"),
        "deepinfra" => Some("deepinfra"),
        "deepseek" => Some("deepseek"),
        "fireworks" => Some("fireworks-ai"),
        "firmware" => Some("firmware"),
        "groq" => Some("groq"),
        "huggingface" => Some("huggingface"),
        "kimi" => Some("kimi-for-coding"),
        "lmstudio" => Some("lmstudio"),
        "minimax" => Some("minimax"),
        "mistral" => Some("mistral"),
        "moonshotai" => Some("moonshotai"),
        "nebius" => Some("nebius"),
        "ollama-cloud" => Some("ollama-cloud"),
        "opencode" => Some("opencode"),
        "opencode-go" => Some("opencode-go"),
        "perplexity" => Some("perplexity-agent"),
        "scaleway" => Some("scaleway"),
        "stackit" => Some("stackit"),
        "togetherai" => Some("togetherai"),
        "xai" => Some("xai"),
        "zai" => Some("zai"),
        _ => None,
    }
}

fn api_base_uses_localhost(raw: &str) -> bool {
    let Ok(parsed) = url::Url::parse(raw) else {
        return false;
    };

    matches!(
        parsed
            .host_str()
            .map(|host| host.to_ascii_lowercase())
            .as_deref(),
        Some("localhost") | Some("127.0.0.1") | Some("::1")
    )
}

pub fn resolve_openai_compatible_profile(
    profile: OpenAiCompatibleProfile,
) -> ResolvedOpenAiCompatibleProfile {
    let mut resolved = ResolvedOpenAiCompatibleProfile {
        id: profile.id.to_string(),
        display_name: profile.display_name.to_string(),
        api_base: profile.api_base.to_string(),
        api_key_env: profile.api_key_env.to_string(),
        env_file: profile.env_file.to_string(),
        setup_url: profile.setup_url.to_string(),
        default_model: profile.default_model.map(ToString::to_string),
        requires_api_key: profile.requires_api_key,
    };

    if profile.id != OPENAI_COMPAT_PROFILE.id {
        return resolved;
    }

    if let Some(base) = env_override("JCODE_OPENAI_COMPAT_API_BASE") {
        if let Some(normalized) = normalize_api_base(&base) {
            resolved.api_base = normalized;
        } else {
            eprintln!(
                "Warning: ignoring invalid JCODE_OPENAI_COMPAT_API_BASE '{}'. Use https://... (or http://localhost).",
                base
            );
        }
    }

    if let Some(key_name) = env_override("JCODE_OPENAI_COMPAT_API_KEY_NAME") {
        if is_safe_env_key_name(&key_name) {
            resolved.api_key_env = key_name;
        } else {
            eprintln!(
                "Warning: ignoring invalid JCODE_OPENAI_COMPAT_API_KEY_NAME '{}'.",
                key_name
            );
        }
    }

    if let Some(env_file) = env_override("JCODE_OPENAI_COMPAT_ENV_FILE") {
        if is_safe_env_file_name(&env_file) {
            resolved.env_file = env_file;
        } else {
            eprintln!(
                "Warning: ignoring invalid JCODE_OPENAI_COMPAT_ENV_FILE '{}'.",
                env_file
            );
        }
    }

    if let Some(setup_url) = env_override("JCODE_OPENAI_COMPAT_SETUP_URL") {
        resolved.setup_url = setup_url;
    }

    if let Some(model) = env_override("JCODE_OPENAI_COMPAT_DEFAULT_MODEL") {
        resolved.default_model = Some(model);
    }

    if api_base_uses_localhost(&resolved.api_base) {
        resolved.requires_api_key = false;
    }

    resolved
}

pub fn apply_openai_compatible_profile_env(profile: Option<OpenAiCompatibleProfile>) {
    if std::env::var_os("JCODE_PROVIDER_PROFILE_ACTIVE").is_some() {
        return;
    }

    for (neutral, legacy) in OPENAI_COMPAT_RUNTIME_ENV_PAIRS {
        remove_openai_compat_runtime_var(neutral, legacy);
    }

    let vars = [
        OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV,
        OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV,
        OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER_ENV,
        "JCODE_NAMED_PROVIDER_PROFILE",
        "JCODE_PROVIDER_PROFILE_ACTIVE",
        "JCODE_PROVIDER_PROFILE_NAME",
    ];

    for var in vars {
        crate::env::remove_var(var);
    }

    if let Some(profile) = profile {
        let resolved = resolve_openai_compatible_profile(profile);
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_API_BASE_ENV,
            LEGACY_OPENROUTER_API_BASE_ENV,
            &resolved.api_base,
        );
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
            LEGACY_OPENROUTER_API_KEY_NAME_ENV,
            &resolved.api_key_env,
        );
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
            LEGACY_OPENROUTER_ENV_FILE_ENV,
            &resolved.env_file,
        );
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
            LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV,
            &resolved.id,
        );
        crate::env::set_var(OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV, &resolved.id);
        crate::env::set_var(
            OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV,
            &resolved.display_name,
        );
        if let Some(models_dev_provider) = openai_compatible_models_dev_provider_id(&resolved.id) {
            crate::env::set_var(
                OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER_ENV,
                models_dev_provider,
            );
        }
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
            LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV,
            "0",
        );
        if let Some(model) = resolved.default_model.as_deref() {
            set_openai_compat_runtime_var(
                OPENAI_COMPAT_RUNTIME_MODEL_ENV,
                LEGACY_OPENROUTER_MODEL_ENV,
                model,
            );
        }
        if resolved.requires_api_key {
            remove_openai_compat_runtime_var(
                OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV,
                LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV,
            );
        } else {
            set_openai_compat_runtime_var(
                OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV,
                LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV,
                "1",
            );
        }
    }
}

fn inline_key_env_name(profile_name: &str) -> String {
    let suffix = profile_name
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    format!("JCODE_PROVIDER_{}_API_KEY", suffix)
}

pub fn apply_named_provider_profile_env(profile_name: &str) -> anyhow::Result<String> {
    let config = crate::config::config();
    apply_named_provider_profile_env_from_config(profile_name, config)
}

pub fn apply_named_provider_profile_env_from_config(
    profile_name: &str,
    config: &crate::config::Config,
) -> anyhow::Result<String> {
    let Some(profile) = config.providers.get(profile_name) else {
        anyhow::bail!(
            "Unknown provider profile '{}'. Add [providers.{}] to config.toml.",
            profile_name,
            profile_name
        );
    };

    let api_base = normalize_api_base(&profile.base_url).ok_or_else(|| {
        anyhow::anyhow!(
            "Provider profile '{}' has invalid base_url '{}'. Use https://... or http://localhost.",
            profile_name,
            profile.base_url
        )
    })?;

    crate::env::remove_var("JCODE_PROVIDER_PROFILE_ACTIVE");
    crate::env::remove_var("JCODE_PROVIDER_PROFILE_NAME");
    crate::env::remove_var("JCODE_NAMED_PROVIDER_PROFILE");
    apply_openai_compatible_profile_env(None);
    let display_name = profile
        .display_name
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .unwrap_or(profile_name);

    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_API_BASE_ENV,
        LEGACY_OPENROUTER_API_BASE_ENV,
        &api_base,
    );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE_ENV,
        LEGACY_OPENROUTER_CACHE_NAMESPACE_ENV,
        profile_name,
    );
    crate::env::set_var(OPENAI_COMPAT_RUNTIME_PROVIDER_ID_ENV, profile_name);
    crate::env::set_var(OPENAI_COMPAT_RUNTIME_PROVIDER_NAME_ENV, display_name);
    if let Some(models_dev_provider) = profile
        .models_dev_provider
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        crate::env::set_var(
            OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER_ENV,
            models_dev_provider,
        );
    }
    crate::env::set_var("JCODE_NAMED_PROVIDER_PROFILE", profile_name);

    let provider_features = matches!(
        profile.provider_type,
        crate::config::NamedProviderType::OpenRouter
    ) || profile.provider_routing
        || profile.allow_provider_pinning;
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES_ENV,
        LEGACY_OPENROUTER_PROVIDER_FEATURES_ENV,
        if provider_features { "1" } else { "0" },
    );
    let model_catalog = profile.model_catalog
        || profile.models_dev_provider.is_some()
        || matches!(
            profile.provider_type,
            crate::config::NamedProviderType::OpenRouter
        );
    set_openai_compat_runtime_var(
        OPENAI_COMPAT_RUNTIME_MODEL_CATALOG_ENV,
        LEGACY_OPENROUTER_MODEL_CATALOG_ENV,
        if model_catalog { "1" } else { "0" },
    );

    if let Some(model) = profile
        .default_model
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_MODEL_ENV,
            LEGACY_OPENROUTER_MODEL_ENV,
            model,
        );
    }

    let static_models = profile
        .models
        .iter()
        .map(|model| model.id.trim())
        .filter(|id| !id.is_empty())
        .collect::<Vec<_>>();
    if !static_models.is_empty() {
        set_openai_compat_runtime_var(
            OPENAI_COMPAT_RUNTIME_STATIC_MODELS_ENV,
            LEGACY_OPENROUTER_STATIC_MODELS_ENV,
            static_models.join("\n"),
        );
    }

    match profile.auth {
        crate::config::NamedProviderAuth::None => {
            set_openai_compat_runtime_var(
                OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV,
                LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV,
                "1",
            );
        }
        crate::config::NamedProviderAuth::Bearer | crate::config::NamedProviderAuth::Header => {
            let key_env = profile
                .api_key_env
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(ToString::to_string)
                .or_else(|| {
                    profile.api_key.as_deref().map(str::trim).filter(|v| !v.is_empty()).map(|key| {
                        let env_name = inline_key_env_name(profile_name);
                        crate::env::set_var(&env_name, key);
                        crate::logging::warn(&format!(
                            "Provider profile '{}' stores an inline API key in config.toml. Prefer api_key_env to avoid accidental leaks.",
                            profile_name
                        ));
                        env_name
                    })
                });

            if let Some(key_env) = key_env {
                if !is_safe_env_key_name(&key_env) {
                    anyhow::bail!(
                        "Provider profile '{}' has invalid api_key_env '{}'.",
                        profile_name,
                        key_env
                    );
                }
                set_openai_compat_runtime_var(
                    OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
                    LEGACY_OPENROUTER_API_KEY_NAME_ENV,
                    &key_env,
                );
            }

            if let Some(env_file) = profile
                .env_file
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
            {
                if !is_safe_env_file_name(env_file) {
                    anyhow::bail!(
                        "Provider profile '{}' has invalid env_file '{}'.",
                        profile_name,
                        env_file
                    );
                }
                set_openai_compat_runtime_var(
                    OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
                    LEGACY_OPENROUTER_ENV_FILE_ENV,
                    env_file,
                );
            }

            let requires_key = profile
                .requires_api_key
                .unwrap_or(!api_base_uses_localhost(&api_base));
            if !requires_key {
                set_openai_compat_runtime_var(
                    OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH_ENV,
                    LEGACY_OPENROUTER_ALLOW_NO_AUTH_ENV,
                    "1",
                );
            }

            match profile.auth {
                crate::config::NamedProviderAuth::Bearer => {
                    set_openai_compat_runtime_var(
                        OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV,
                        LEGACY_OPENROUTER_AUTH_HEADER_ENV,
                        "bearer",
                    );
                }
                crate::config::NamedProviderAuth::Header => {
                    set_openai_compat_runtime_var(
                        OPENAI_COMPAT_RUNTIME_AUTH_HEADER_ENV,
                        LEGACY_OPENROUTER_AUTH_HEADER_ENV,
                        "api-key",
                    );
                    if let Some(header) = profile
                        .auth_header
                        .as_deref()
                        .map(str::trim)
                        .filter(|v| !v.is_empty())
                    {
                        set_openai_compat_runtime_var(
                            OPENAI_COMPAT_RUNTIME_AUTH_HEADER_NAME_ENV,
                            LEGACY_OPENROUTER_AUTH_HEADER_NAME_ENV,
                            header,
                        );
                    }
                }
                crate::config::NamedProviderAuth::None => {}
            }
        }
    }

    Ok(profile_name.to_string())
}

pub fn openrouter_like_api_key_sources() -> Vec<(String, String)> {
    let mut sources = Vec::with_capacity(10);
    sources.push((
        "OPENROUTER_API_KEY".to_string(),
        "openrouter.env".to_string(),
    ));

    for profile in openai_compatible_profiles() {
        if profile.requires_api_key {
            sources.push((
                profile.api_key_env.to_string(),
                profile.env_file.to_string(),
            ));
        }
    }

    if let Some(source) = configured_api_key_source(
        OPENAI_COMPAT_RUNTIME_API_KEY_NAME_ENV,
        OPENAI_COMPAT_RUNTIME_ENV_FILE_ENV,
        "OPENROUTER_API_KEY",
        "openrouter.env",
    ) {
        sources.push(source);
    }

    if let Some(source) = configured_api_key_source(
        "JCODE_OPENROUTER_API_KEY_NAME",
        "JCODE_OPENROUTER_ENV_FILE",
        "OPENROUTER_API_KEY",
        "openrouter.env",
    ) {
        sources.push(source);
    }

    if let Some(source) = configured_api_key_source(
        "JCODE_OPENAI_COMPAT_API_KEY_NAME",
        "JCODE_OPENAI_COMPAT_ENV_FILE",
        OPENAI_COMPAT_PROFILE.api_key_env,
        OPENAI_COMPAT_PROFILE.env_file,
    ) {
        sources.push(source);
    }

    dedup_sources(sources)
}

fn parse_bool_like(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub fn openai_compatible_profile_is_configured(profile: OpenAiCompatibleProfile) -> bool {
    let resolved = resolve_openai_compatible_profile(profile);
    if resolved.requires_api_key {
        return load_api_key_from_env_or_config(&resolved.api_key_env, &resolved.env_file)
            .is_some();
    }

    if profile.id == OPENAI_COMPAT_PROFILE.id && api_base_uses_localhost(&resolved.api_base) {
        return true;
    }

    load_env_value_from_env_or_config(OPENAI_COMPAT_LOCAL_ENABLED_ENV, &resolved.env_file)
        .map(|value| parse_bool_like(&value))
        .unwrap_or(false)
}

pub fn configured_api_key_source(
    key_var: &str,
    file_var: &str,
    default_key: &str,
    default_file: &str,
) -> Option<(String, String)> {
    if std::env::var_os(key_var).is_none() && std::env::var_os(file_var).is_none() {
        return None;
    }

    let env_key = std::env::var(key_var)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| default_key.to_string());
    let file_name = std::env::var(file_var)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| default_file.to_string());

    if !is_safe_env_key_name(&env_key) {
        crate::logging::warn(&format!(
            "Ignoring invalid {}='{}' while probing auth status",
            key_var, env_key
        ));
        return None;
    }
    if !is_safe_env_file_name(&file_name) {
        crate::logging::warn(&format!(
            "Ignoring invalid {}='{}' while probing auth status",
            file_var, file_name
        ));
        return None;
    }

    Some((env_key, file_name))
}

pub fn load_api_key_from_env_or_config(env_key: &str, file_name: &str) -> Option<String> {
    if !is_safe_env_key_name(env_key) {
        crate::logging::warn(&format!(
            "Ignoring invalid API key variable name '{}' while loading credentials",
            env_key
        ));
        return None;
    }
    if !is_safe_env_file_name(file_name) {
        crate::logging::warn(&format!(
            "Ignoring invalid env file name '{}' while loading credentials",
            file_name
        ));
        return None;
    }

    if let Ok(key) = std::env::var(env_key) {
        let key = key.trim();
        if !key.is_empty() {
            return Some(key.to_string());
        }
    }

    let config_path = crate::storage::app_config_dir().ok()?.join(file_name);
    crate::storage::harden_secret_file_permissions(&config_path);
    let content = std::fs::read_to_string(config_path).ok()?;
    let prefix = format!("{}=", env_key);

    for line in content.lines() {
        if let Some(key) = line.strip_prefix(&prefix) {
            let key = key.trim().trim_matches('"').trim_matches('\'');
            if !key.is_empty() {
                return Some(key.to_string());
            }
        }
    }

    if env_key == "ZHIPU_API_KEY" {
        if let Ok(key) = std::env::var("ZAI_API_KEY") {
            let key = key.trim();
            if !key.is_empty() {
                return Some(key.to_string());
            }
        }

        let legacy_prefix = "ZAI_API_KEY=";
        for line in content.lines() {
            if let Some(key) = line.strip_prefix(legacy_prefix) {
                let key = key.trim().trim_matches('"').trim_matches('\'');
                if !key.is_empty() {
                    return Some(key.to_string());
                }
            }
        }
    }

    if let Some(key) = crate::auth::external::load_api_key_for_env(env_key) {
        return Some(key);
    }

    None
}

pub fn load_env_value_from_env_or_config(env_key: &str, file_name: &str) -> Option<String> {
    if !is_safe_env_key_name(env_key) {
        crate::logging::warn(&format!(
            "Ignoring invalid variable name '{}' while loading config value",
            env_key
        ));
        return None;
    }
    if !is_safe_env_file_name(file_name) {
        crate::logging::warn(&format!(
            "Ignoring invalid env file name '{}' while loading config value",
            file_name
        ));
        return None;
    }

    if let Ok(value) = std::env::var(env_key) {
        let value = value.trim();
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }

    let config_path = crate::storage::app_config_dir().ok()?.join(file_name);
    crate::storage::harden_secret_file_permissions(&config_path);
    let content = std::fs::read_to_string(config_path).ok()?;
    let prefix = format!("{}=", env_key);

    for line in content.lines() {
        if let Some(value) = line.strip_prefix(&prefix) {
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

pub fn save_env_value_to_env_file(
    env_key: &str,
    file_name: &str,
    value: Option<&str>,
) -> anyhow::Result<()> {
    if !is_safe_env_key_name(env_key) {
        anyhow::bail!("Invalid variable name: {}", env_key);
    }
    if !is_safe_env_file_name(file_name) {
        anyhow::bail!("Invalid env file name: {}", file_name);
    }

    let config_dir = crate::storage::app_config_dir()?;
    let file_path = config_dir.join(file_name);
    crate::storage::upsert_env_file_value(&file_path, env_key, value)?;

    if let Some(value) = value {
        crate::env::set_var(env_key, value);
    } else {
        crate::env::remove_var(env_key);
    }

    Ok(())
}

fn env_override(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .or_else(|| load_env_value_from_env_or_config(name, OPENAI_COMPAT_PROFILE.env_file))
}

fn dedup_sources(sources: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::with_capacity(sources.len());
    for (env_key, env_file) in sources {
        if seen.insert((env_key.clone(), env_file.clone())) {
            deduped.push((env_key, env_file));
        }
    }
    deduped
}

#[cfg(test)]
#[path = "provider_catalog_tests.rs"]
mod provider_catalog_tests;
