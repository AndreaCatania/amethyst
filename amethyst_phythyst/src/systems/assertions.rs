use amethyst_core::ecs::prelude::World;
use log::error;
use log::warn;

use crate::{
    objects::*,
    servers::{RBodyPhysicsServer, WorldPhysicsServer},
    PhysicsTime,
};

/// This macro implements the `setup` function that checks if the current world
/// has all the required physics resources.
///
/// Before to panics, it carefully tell how to properly initialize the physics.
// TODO these are no more needed!
#[macro_export]
macro_rules! define_setup_with_physics_assertion{
    () => {
        fn setup(&mut self, res: &mut amethyst_core::ecs::World){
            use amethyst_core::ecs::prelude::SystemData;
            Self::SystemData::setup(res);
            crate::systems::assertions::assert_physics_resources(res);
        }
    };
    ($x:ident) => {
        fn setup(&mut self, res: &mut amethyst_core::ecs::World){
            use amethyst_core::ecs::prelude::SystemData;
            Self::SystemData::setup(res);
            crate::systems::assertions::assert_physics_resources(res);
            self.$x(res);
        }
    }
}

/// This function `panics` if the physics resources are not availables.
/// It carefully tell how to properly initialize the physics.
///
/// This function is used by the Systems during its setup to assert the
/// correctness of physics server status.
pub fn assert_physics_resources(res: &mut World) {
    if !res.has_value::<WorldPhysicsServer<f32>>() {
        error!("The resource WorldPhysicsServer not found");
        explain_physics_server_setup();
    } else if !res.has_value::<RBodyPhysicsServer<f32>>() {
        error!("The resource RBodyPhysicsServer not found");
        explain_physics_server_setup();
    } else if !res.has_value::<PhysicsTime>() {
        error!("The resource PhysicsTime not found");
        explain_physics_server_setup();
    } else if !res.has_value::<PhysicsHandle<PhysicsWorldTag>>() {
        error!("The resource PhysicsWorld not found");
        explain_physics_server_setup();
    }
}

/// Prints the explanation on how to setup the physics server on the logger, then panics.
fn explain_physics_server_setup() {
    error!(" |");
    error!(" | Note: The physics server is not correctly initialized.");
    error!(" | Adding the PhysicsBundle is not enough, is necessary to add all physics resources.");
    error!(" | In the Application object creation use the function `.with_physics` to add them.");
    error!(" | ");
    error!(" | **Example:**");
    error!(" | ```");
    error!(" | let mut game = Application::build(\"./\", GameState)?");
    error!(" |     .with_physics(amethyst_nphysics::create_physics())");
    error!(" |     .build(game_data)?;");
    error!(" | ```");
    error!(" |__________________________");
    panic!();
}
