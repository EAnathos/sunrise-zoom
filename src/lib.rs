//! Sunrise Zoom's native client entry point.
//!
//! The first implementation step is to verify the pinned Sunrise client input and FOV hooks;
//! see `docs/architecture.md` before wiring runtime behavior here.
use mod_api::{self, Hooks};

#[derive(Default)]
struct SunriseZoom;

#[mod_api::sunrise_mod]
impl Hooks for SunriseZoom {}
