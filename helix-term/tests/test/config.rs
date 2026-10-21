use super::helpers::*;
use helix_term::config::ConfigRaw;
use helix_view::editor::ConfigEvent;

fn build_app(config_toml: toml::Table) -> anyhow::Result<helix_term::application::Application> {
    let config_raw: ConfigRaw = config_toml.try_into()?;

    let mut editor: helix_view::editor::Config = config_raw.editor.unwrap().try_into()?;

    // Fork-specific: `Application::new` runs the steel init script
    // (helix-term/src/application.rs -> ScriptingEngine::run_initialization_script), which is
    // gated on `editor.enable_steel` and costs ~25s per test in debug. Upstream has no such
    // field, so the upstream tests are not drop-in here.
    #[cfg(feature = "steel")]
    {
        editor.enable_steel = false;
    }

    let config = helix_term::config::Config {
        theme: config_raw.theme,
        keys: config_raw.keys.unwrap_or_default(),
        editor,
    };

    let app = AppBuilder::new().with_config(config).build()?;

    Ok(app)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_set_config_single() -> anyhow::Result<()> {
    let mut app = build_app(toml::toml! {
        editor.trim-final-newlines = false
        editor.trim-trailing-whitespace = false
        keys.normal.space.t = ":set trim-trailing-whitespace true"
    })?;

    test_key_sequence(
        &mut app,
        Some("<space>t"),
        Some(&|app| {
            let config = app.editor.config.load();
            assert!(!config.trim_final_newlines);
            assert!(config.trim_trailing_whitespace);
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_set_config_multi() -> anyhow::Result<()> {
    let mut app = build_app(toml::toml! {
        editor.trim-final-newlines = false
        editor.trim-trailing-whitespace = false
        keys.normal.space.t = [
            ":set trim-final-newlines true",
            ":set trim-trailing-whitespace true",
        ]
    })?;

    test_key_sequence(
        &mut app,
        Some("<space>t"),
        Some(&|app| {
            let config = app.editor.config.load();
            assert!(config.trim_final_newlines);
            assert!(config.trim_trailing_whitespace);
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_toggle_config_single() -> anyhow::Result<()> {
    let mut app = build_app(toml::toml! {
        editor.trim-final-newlines = false
        editor.trim-trailing-whitespace = false
        keys.normal.space.t = ":toggle trim-trailing-whitespace"
    })?;

    test_key_sequence(
        &mut app,
        Some("<space>t"),
        Some(&|app| {
            let config = app.editor.config.load();
            assert!(!config.trim_final_newlines);
            assert!(config.trim_trailing_whitespace);
        }),
        false,
    )
    .await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_toggle_config_multi() -> anyhow::Result<()> {
    let mut app = build_app(toml::toml! {
        editor.trim-final-newlines = false
        editor.trim-trailing-whitespace = false
        keys.normal.space.t = [
            ":toggle trim-final-newlines",
            ":toggle trim-trailing-whitespace",
        ]
    })?;

    test_key_sequence(
        &mut app,
        Some("<space>t"),
        Some(&|app| {
            let config = app.editor.config.load();
            assert!(config.trim_final_newlines);
            assert!(config.trim_trailing_whitespace);
        }),
        false,
    )
    .await?;

    Ok(())
}

/// The field-scoped config path introduced by PR#15860
/// (`helix-term/src/application.rs`, `ConfigEvent::UpdateField` arm) serializes the *whole*
/// editor config to JSON, patches a single field and deserializes it back into
/// `helix_view::editor::Config`. That config is `deny_unknown_fields` and carries fork-only
/// fields (`enable_steel`), so the round trip has to be lossless — otherwise every
/// `:set-option` would reset unrelated settings.
#[tokio::test(flavor = "multi_thread")]
async fn test_editor_config_roundtrip_preserves_fork_fields() -> anyhow::Result<()> {
    let config = helix_view::editor::Config::default();

    let mut json = serde_json::to_value(&config)?;
    *json.pointer_mut("/trim-final-newlines")
        .expect("trim-final-newlines is a known field") = serde_json::json!(true);

    let roundtripped: helix_view::editor::Config = serde_json::from_value(json)?;

    let mut expected = config.clone();
    expected.trim_final_newlines = true;
    assert_eq!(
        roundtripped, expected,
        "the editor config must survive the JSON round trip unchanged"
    );
    #[cfg(feature = "steel")]
    assert_eq!(roundtripped.enable_steel, config.enable_steel);

    Ok(())
}

/// The steel engine still writes the whole config (`ConfigEvent::Update`, see
/// `helix-term/src/commands/engine/steel/mod.rs`), while the editor's `:set-option` now writes
/// a single field (`ConfigEvent::UpdateField`). Both have to be usable in the same session.
///
/// Observed semantics (whole config = last writer wins, field update = applied on top of the
/// config stored at the time the event is handled):
///  * a whole-config update carrying a snapshot taken *before* a field update clobbers that
///    field update,
///  * a field update applied after a whole-config update is applied on top of it.
#[tokio::test(flavor = "multi_thread")]
async fn test_field_update_and_full_config_update_coexist() -> anyhow::Result<()> {
    let mut app = build_app(toml::toml! {
        editor.trim-final-newlines = false
        editor.trim-trailing-whitespace = false
    })?;

    // snapshot of the config, as the steel engine holds it
    let snapshot = (*app.editor.config.load()).clone();

    // field-scoped update: the new `:set-option` path
    app.handle_config_events(ConfigEvent::UpdateField(
        "trim-final-newlines".into(),
        serde_json::json!(true),
    ));
    assert!(app.editor.config.load().trim_final_newlines);

    // whole-config update: the steel `set-option!` path, with the stale snapshot
    app.handle_config_events(ConfigEvent::Update(Box::new(snapshot.clone())));
    assert!(!app.editor.config.load().trim_final_newlines);
    assert!(!app.editor.config.load().trim_trailing_whitespace);

    // reverse order: the field update reads the config stored at handling time
    app.handle_config_events(ConfigEvent::Update(Box::new(snapshot)));
    app.handle_config_events(ConfigEvent::UpdateField(
        "trim-final-newlines".into(),
        serde_json::json!(true),
    ));
    assert!(app.editor.config.load().trim_final_newlines);

    Ok(())
}
