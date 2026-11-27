//! Sunrise Zoom replaces the vanilla spyglass magnification and animates the FOV transition.
use mod_api::{
    self, ClientHost, ComputeFovEvent, ComputeFovModifierEvent, Hooks, RegistrationContext,
    Registrar, SettingSpec,
};
use mod_api::client::camera::UseItem;
use mod_api::KeyRegistration;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

mod animation;

const MAGNIFICATION: &str = "magnification";
const TRANSITION_SECONDS: &str = "transition_seconds";
const MODE: &str = "mode";
const ZOOM_KEY: &str = "zoom";
const MODE_SPYGLASS: u32 = 0;
const MODE_KEY: u32 = 1;
const DEFAULT_MAGNIFICATION: f64 = 4.0;
const DEFAULT_TRANSITION_SECONDS: f64 = 0.18;

/// Vanilla narrows FOV to one tenth while scoping. `compute_fov` runs after that step.
const VANILLA_SPYGLASS_FOV_FACTOR: f32 = 0.1;

#[derive(Default)]
struct ZoomState {
    transition: animation::Transition,
    using_spyglass: bool,
}

static ZOOM_STATE: OnceLock<Mutex<ZoomState>> = OnceLock::new();

fn zoom_state() -> &'static Mutex<ZoomState> {
    ZOOM_STATE.get_or_init(|| Mutex::new(ZoomState::default()))
}

#[derive(Default)]
struct SunriseZoom;

#[mod_api::sunrise_mod]
impl Hooks for SunriseZoom {
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_key_mapping(
            KeyRegistration::new(ZOOM_KEY, "key.keyboard.c")
                .label("sunrise_zoom.key.zoom")
                .category("sunrise_zoom.key.category")
                .context(mod_api::KeyConflictContext::InGame),
        );
        let _ = registrar.register_setting(SettingSpec::choice(
            MODE,
            "sunrise_zoom.settings.mode",
            [
                "sunrise_zoom.settings.mode.spyglass",
                "sunrise_zoom.settings.mode.key",
            ],
            MODE_SPYGLASS as usize,
        ));
        let _ = registrar.register_setting(
            SettingSpec::range(
                MAGNIFICATION,
                "sunrise_zoom.settings.magnification",
                2.0,
                20.0,
                0.25,
                DEFAULT_MAGNIFICATION,
            )
            .tooltip("sunrise_zoom.settings.magnification.tooltip"),
        );
        let _ = registrar.register_setting(
            SettingSpec::range(
                TRANSITION_SECONDS,
                "sunrise_zoom.settings.transition_seconds",
                0.0,
                0.6,
                0.02,
                DEFAULT_TRANSITION_SECONDS,
            )
            .tooltip("sunrise_zoom.settings.transition_seconds.tooltip"),
        );
    }

    /// Track spyglass use once per client tick. The render-time hook samples the transition on
    /// every FOV calculation, so animation remains smooth above the game tick rate.
    fn compute_fov_modifier(&self, host: &dyn ClientHost, event: &mut ComputeFovModifierEvent<'_>) {
        let using_spyglass = matches!(event.player.using, Some((UseItem::Spyglass, _)));
        let mode = host.setting_choice(MODE).unwrap_or(MODE_SPYGLASS);
        let zoom_key_down = host.is_down("sunrise_zoom", ZOOM_KEY);
        let magnification = host
            .setting_f64(MAGNIFICATION)
            .unwrap_or(DEFAULT_MAGNIFICATION)
            .clamp(2.0, 20.0) as f32;
        let seconds = host
            .setting_f64(TRANSITION_SECONDS)
            .unwrap_or(DEFAULT_TRANSITION_SECONDS)
            .clamp(0.0, 0.6);
        let target_fov_factor = match (mode, using_spyglass, zoom_key_down) {
            (MODE_SPYGLASS, true, _) => 1.0 / magnification,
            // In key mode, retain vanilla spyglass zoom and avoid stacking both zooms.
            (MODE_KEY, true, _) => VANILLA_SPYGLASS_FOV_FACTOR,
            (MODE_KEY, false, true) => 1.0 / magnification,
            _ => 1.0,
        };

        if let Ok(mut state) = zoom_state().lock() {
            state.using_spyglass = using_spyglass;
            state.transition.retarget(
                target_fov_factor,
                Duration::from_secs_f64(seconds),
                Instant::now(),
            );
        }
    }

    /// Normalize the vanilla spyglass FOV before applying the shared animated factor. This keeps
    /// the effective zoom continuous when the incoming FOV switches to/from the scoped value.
    fn compute_fov(&self, _host: &dyn ClientHost, event: &mut ComputeFovEvent) {
        let Ok(state) = zoom_state().lock() else {
            return;
        };
        let normal_fov = if state.using_spyglass {
            event.degrees / VANILLA_SPYGLASS_FOV_FACTOR
        } else {
            event.degrees
        };
        let degrees = normal_fov * state.transition.sample(Instant::now());
        if degrees.is_finite() {
            // Sunrise ignores out-of-range values. Clamp rather than falling back abruptly to
            // vanilla when a high magnification would cross the 1-degree floor.
            event.degrees = degrees.clamp(1.0, 179.0);
        }
    }
}
