// check build-in functions
use bevy::prelude::*;

pub struct TakiAppDemoPlugin;

fn log_demo() {
    trace!("very noisy");
    debug!("msg for debugging");
    info!("helpful information that is worth printing by default");
    warn!("some bad happended that isn't a failure, but thats worth calling out");
    error!("something failed and we need to know about it");
}

impl Plugin for TakiAppDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, log_demo);
    }
}

