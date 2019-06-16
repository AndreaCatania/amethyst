
use log::warn;
use log::error;
use amethyst_core::ecs::prelude::Resources;

use crate::Physics;

/// This macro implements the `setup` function that checks if the current world
/// has all the required physics resources.
/// 
/// Before to panics, it carefully tell how to properly initialize the physics.
#[macro_export]
macro_rules! define_setup_with_physics_assertion{
    () => {
        fn setup(&mut self, res: &mut amethyst_core::ecs::Resources){
            use amethyst_core::ecs::prelude::SystemData;
            Self::SystemData::setup(res);
            crate::systems::assertions::assert_physics_resources(res);
        }
    }
}

/// This function `panics` if the physics resources are not availables.
/// It carefully tell how to properly initialize the physics.
/// 
/// This function is used by the Systems during its setup to assert the
/// correctness of physics server status.
pub fn assert_physics_resources(res: &mut Resources){

    warn!("TODO please improve the message of the function explain_physics_server_setup");

    if !res.has_value::<Physics>() {
        explain_physics_server_setup();
    }
}

/// Prints the explanation on how to setup the physics server on the logger, then panics.
fn explain_physics_server_setup(){

    error!("The physics server is not correctly initialized.");
    error!("Add the PhysicsBundle is not enough, is necessary to add the PhysicsServer resource.");
    error!("In the Application creation use the function `.with` to add the PhysicsServer resource.");
    error!("Example:");
    error!("```");
    error!("let mut game = Application::build(\"./\", GameState)?");
    error!("    .with_resource(Physics::default())");
    error!("    .build(game_data)?;");
    error!("```");
    panic!();
}