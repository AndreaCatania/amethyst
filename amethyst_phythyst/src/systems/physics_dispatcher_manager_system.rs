
use amethyst_core::ecs::{Resources, RunNow,};
use crate::{
    PhysicsDispatcherCreator,
    physics_dispatcher_creator::PhysicsDispatcher,
};

pub struct PhysicsDispatcherManagerSystem<G: PhysicsDispatcherCreator>{
    graph_creator: G,
}

impl<G: PhysicsDispatcherCreator> PhysicsDispatcherManagerSystem<G>{
    pub fn new(graph_creator: G ) -> PhysicsDispatcherManagerSystem<G>{
        PhysicsDispatcherManagerSystem {
            graph_creator,
        }
    }
}

impl<'a, G: PhysicsDispatcherCreator> RunNow<'a> for PhysicsDispatcherManagerSystem<G>{
    fn run_now(&mut self, res: &'a Resources){
        if self.graph_creator.rebuild(res){
            let dispatcher = self.graph_creator.build::<'a, 'a>(res);
            let disp_wrapper = res.fetch_mut::<PhysicsDispatcher>();
            (*disp_wrapper).0 = dispatcher;
        }
    }

    fn setup(&mut self, res: &mut Resources) {

        res.insert(PhysicsDispatcher(None));
    }
}
