use c3dio::prelude::*;

pub(crate) fn is_force_channel(c3d: &C3d, channel: u8) -> bool {
    let mut is_force = false;
    for force in c3d.forces.iter() {
        for force_channel in force.channels.iter() {
            if channel == *force_channel {
                is_force = true;
                break;
            }
        }
    }
    is_force
}
