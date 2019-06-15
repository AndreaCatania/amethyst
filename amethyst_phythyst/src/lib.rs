
use amethyst_core::{
    bundle::SystemBundle,
    shred::DispatcherBuilder,
};
use amethyst_error::Error;

pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle{
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error>{
        
        println!("Physics bundle not yet registerd!");

        Ok(())
    }
}