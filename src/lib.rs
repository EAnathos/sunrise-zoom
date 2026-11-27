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

/// Vanilla's spyglass narrows FOV by 0.1 before the `compute_fov` hook.
/// Multiplying by 10 / desired magnification replaces that zoom factor.
const VANILLA_SPYGLASS_MAGNIFICATION: f32 = 10.0;

static TRANSITION: OnceLock<Mutex<animation::Transition>> = OnceLock::new();

fn transition() -> &'static Mutex<animation::Transition> {
    TRANSITION.get_or_init(|| Mutex::new(animation::Transition::default()))
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
        let target_ratio = match (mode, using_spyglass, zoom_key_down) {
            (MODE_SPYGLASS, true, _) => VANILLA_SPYGLASS_MAGNIFICATION / magnification,
            (MODE_KEY, false, true) => 1.0 / magnification,
            // In key mode, keep the game's spyglass behavior intact and avoid stacking both zooms.
            _ => 1.0,
        };

        if let Ok(mut transition) = transition().lock() {
            transition.retarget(target_ratio, Duration::from_secs_f64(seconds), Instant::now());
        }
    }

    /// Apply the current render-time ratio: compensate for vanilla's spyglass zoom in spyglass
    /// mode, or narrow the ordinary FOV in separate-key mode.
    fn compute_fov(&self, _host: &dyn ClientHost, event: &mut ComputeFovEvent) {
        let Ok(transition) = transition().lock() else {
            return;
        };
        let ratio = transition.sample(Instant::now());
        let degrees = event.degrees * ratio;
        if degrees.is_finite() && (1.0..=179.0).contains(&degrees) {
            event.degrees = degrees;
        }
    }
}
