# OpenAI-Compatible Providers: First-Class Refactor Plan

## Goal

Make OpenAI-compatible providers feel like independent, first-class jcode providers instead of OpenRouter-shaped aliases, while preserving existing OpenRouter behavior and backward compatibility.

This includes built-in providers like Z.AI, Chutes, Cerebras, OpenCode Go, Ollama Cloud, Kimi, and user-defined custom providers.

## Success criteria

- Built-in OpenAI-compatible providers no longer need to be understood as OpenRouter at the UX/config layer.
- New neutral runtime env names are supported for OpenAI-compatible providers.
- Existing `JCODE_OPENROUTER_*` env vars continue to work as backward-compatible aliases.
- `/model` and model routes display the real provider identity, for example `Ollama Cloud`, `Z.AI`, `Chutes`, or a custom provider display name.
- Profile default models are applied consistently.
- models.dev metadata can be fetched and merged per provider, not just with provider-specific one-offs.
- Custom providers can opt into models.dev catalogs and appear with their configured identity.
- Tests cover backward compatibility, provider identity, default model propagation, and models.dev metadata enrichment.

## Non-goals for this pass

- Remove or break `JCODE_OPENROUTER_*` compatibility names.
- Rename every file/module mechanically in one massive patch.
- Change OpenRouter-specific routing semantics such as provider pinning or endpoint routing.
- Require network access in tests.

## Architecture approach

### 1. Neutral runtime configuration

Introduce neutral OpenAI-compatible runtime names:

```text
JCODE_OPENAI_COMPAT_RUNTIME_API_BASE
JCODE_OPENAI_COMPAT_RUNTIME_API_KEY_NAME
JCODE_OPENAI_COMPAT_RUNTIME_ENV_FILE
JCODE_OPENAI_COMPAT_RUNTIME_CACHE_NAMESPACE
JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_ID
JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_NAME
JCODE_OPENAI_COMPAT_RUNTIME_MODEL
JCODE_OPENAI_COMPAT_RUNTIME_STATIC_MODELS
JCODE_OPENAI_COMPAT_RUNTIME_MODEL_CATALOG
JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER_FEATURES
JCODE_OPENAI_COMPAT_RUNTIME_ALLOW_NO_AUTH
JCODE_OPENAI_COMPAT_RUNTIME_AUTH_HEADER
JCODE_OPENAI_COMPAT_RUNTIME_AUTH_HEADER_NAME
JCODE_OPENAI_COMPAT_RUNTIME_DYNAMIC_BEARER_PROVIDER
JCODE_OPENAI_COMPAT_RUNTIME_PROVIDER
JCODE_OPENAI_COMPAT_RUNTIME_NO_FALLBACK
JCODE_OPENAI_COMPAT_RUNTIME_MODELS_DEV_PROVIDER
```

Keep old `JCODE_OPENROUTER_*` names as aliases.

Precedence:

1. neutral runtime var
2. legacy OpenRouter var
3. autodetected profile/custom config
4. default value

### 2. Provider identity flows through routes

Track the active OpenAI-compatible profile identity in runtime config:

- provider id
- provider display name
- cache namespace
- optional models.dev provider id

Use this identity in model routes and picker display.

### 3. Generic models.dev support

Replace one-off Ollama Cloud enrichment with generic metadata fetching:

- fetch models.dev provider object by provider id
- parse model metadata into a small internal map
- merge context length and useful metadata into live `/models` response
- allow static catalog fallback when a provider supports models.dev but `/models` is unavailable or disabled

### 4. Custom providers feel integrated

Extend named provider config support with:

- `display_name`
- `models_dev_provider`
- provider identity in runtime env
- route provider label from config/profile identity

Custom provider model routes should show the configured display name instead of `OpenAI-compatible`.

### 5. Validation strategy

Use focused, deterministic tests:

- runtime env applies neutral vars and legacy aliases
- old OpenRouter vars still work
- built-in provider default model appears in routes by real provider name
- custom provider display name appears in routes
- models.dev parser/enrichment works with fixture JSON
- provider matrix still passes for configured built-in OpenAI-compatible providers

## Implementation checklist

- [x] Add neutral runtime constants/helpers.
- [x] Update profile env application to set neutral runtime vars and compatibility aliases.
- [x] Update OpenAI-compatible/OpenRouter provider runtime readers to prefer neutral vars.
- [x] Add provider identity accessors.
- [x] Add generic models.dev parser/fetch/enrichment helpers.
- [x] Wire models.dev provider id into built-in profiles and named custom providers.
- [x] Update model routes to display provider identity.
- [x] Update tests.
- [x] Run formatting/tests/selfdev build.
- [x] Commit changes.

## Rollout

Keep the implementation shippable at each step. The end of this pass should leave jcode behavior backward-compatible, but with a clearer first-class OpenAI-compatible provider path.
