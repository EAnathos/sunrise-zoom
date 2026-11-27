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

#[derive(Default)]
struct ZoomState {
    transition: animation::Transition,
    using_spyglass: bool,
    replacing_spyglass: bool,
    last_unscoped_fov: Option<f32>,
    spyglass_baseline_fov: Option<f32>,
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
            (MODE_KEY, true, _) => 1.0,
            (MODE_KEY, false, true) => 1.0 / magnification,
            _ => 1.0,
        };

        if let Ok(mut state) = zoom_state().lock() {
            let replacing_spyglass = mode == MODE_SPYGLASS && using_spyglass;
            if replacing_spyglass && !state.replacing_spyglass {
                // Sunrise's camera FOV eases toward the vanilla spyglass modifier over several
                // ticks. Keep the ordinary rendered FOV as an anchor instead of dividing the
                // first scoped frames by 0.1 before that easing has reached its target.
                state.spyglass_baseline_fov = state.last_unscoped_fov;
            }
            state.replacing_spyglass = replacing_spyglass;
            state.using_spyglass = using_spyglass;
            state.transition.retarget(
                target_fov_factor,
                Duration::from_secs_f64(seconds),
                Instant::now(),
            );
        }
    }

    /// Apply spyglass replacement from the last ordinary rendered FOV. Sunrise eases its own
    /// vanilla spyglass modifier over several ticks, so the incoming degrees cannot safely be
    /// divided by 0.1 on the first scoped frame.
    fn compute_fov(&self, _host: &dyn ClientHost, event: &mut ComputeFovEvent) {
        let Ok(mut state) = zoom_state().lock() else {
            return;
        };
        let incoming_degrees = event.degrees;
        let factor = state.transition.sample(Instant::now());
        // Key mode leaves the native spyglass path entirely to Sunrise. Applying its key
        // transition while scoped would stack a second zoom on the vanilla scope.
        if state.using_spyglass && !state.replacing_spyglass {
            return;
        }
        if state.replacing_spyglass && state.spyglass_baseline_fov.is_none() {
            // This only occurs if the player starts using a spyglass before the first ordinary
            // world FOV callback (for example, immediately after loading into a world).
            state.spyglass_baseline_fov = Some(event.degrees);
        }
        let vanilla_fov_restored = state
            .spyglass_baseline_fov
            .is_some_and(|baseline| (incoming_degrees - baseline).abs() <= 0.5);
        let replacing_or_releasing = state.spyglass_baseline_fov.is_some()
            && (state.replacing_spyglass
                || (factor - 1.0).abs() > f32::EPSILON
                || !vanilla_fov_restored);
        let degrees = if replacing_or_releasing {
            state.spyglass_baseline_fov.unwrap_or(event.degrees) * factor
        } else {
            event.degrees * factor
        };
        if degrees.is_finite() {
            // Sunrise ignores out-of-range values. Clamp rather than falling back abruptly to
            // vanilla when a high magnification would cross the 1-degree floor.
            event.degrees = degrees.clamp(1.0, 179.0);
        }

        if replacing_or_releasing {
            if !state.replacing_spyglass
                && (factor - 1.0).abs() <= f32::EPSILON
                && vanilla_fov_restored
            {
                state.spyglass_baseline_fov = None;
                state.last_unscoped_fov = Some(event.degrees);
            }
        } else {
            state.last_unscoped_fov = Some(incoming_degrees);
        }
    }
}
