
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
    };
    ($x:ident) => {
        fn setup(&mut self, res: &mut amethyst_core::ecs::Resources){
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
pub fn assert_physics_resources(res: &mut Resources){

    if !res.has_value::<Physics>() {
        explain_physics_server_setup();
    }
}

/// Prints the explanation on how to setup the physics server on the logger, then panics.
fn explain_physics_server_setup(){

    error!("The physics server is not correctly initialized.");
    error!("Adding the PhysicsBundle is not enough, is necessary to add the PhysicsServer resource also.");
    error!("In the Application object creation use the function `.with_resource` to add the Physics");
    error!("backed object created by the function `create_physics`.");
    error!("**Example:**");
    error!("```");
    error!("let mut game = Application::build(\"./\", GameState)?");
    error!("    .with_resource(amethyst_nphysics::create_physics())");
    error!("    .build(game_data)?;");
    error!("```");
    panic!();
}