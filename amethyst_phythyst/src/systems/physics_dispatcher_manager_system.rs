
use amethyst_core::ecs::{Resources, RunNow,};
use crate::{
    PhysicsDispatcherCreator,
    physics_dispatcher_creator::PhysicsDispatcher,
};

pub struct PhysicsDispatcherManagerSystem<G: PhysicsDispatcherCreator + Sync + Send>{
    graph_creator: G,
}

impl<G:  PhysicsDispatcherCreator + Sync + Send> PhysicsDispatcherManagerSystem<G>{
    pub fn new(graph_creator: G ) -> PhysicsDispatcherManagerSystem<G>{
        PhysicsDispatcherManagerSystem {
            graph_creator,
        }
    }
}

impl<'a, G: PhysicsDispatcherCreator + Send + Sync> RunNow<'a> for PhysicsDispatcherManagerSystem<G>{

    fn run_now(&mut self, res: &'a Resources){
        if self.graph_creator.rebuild(res){
            res.fetch_mut::<PhysicsDispatcher>().0 = self.graph_creator.build(res);
        }
    }

    fn setup(&mut self, res: &mut Resources) {

        res.insert(PhysicsDispatcher(None));
    }
}
