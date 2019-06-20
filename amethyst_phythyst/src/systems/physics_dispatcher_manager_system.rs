
use amethyst_core::ecs::{Resources, RunNow,};
use crate::{
    PhysicsDispatcherCreator,
    physics_dispatcher_creator::PhysicsDispatcher,
};

pub struct PhysicsDispatcherManagerSystem<G: PhysicsDispatcherCreator + Sync + Send>{
    graph_creator: Box<G>,
}

impl<G:  PhysicsDispatcherCreator + Sync + Send> PhysicsDispatcherManagerSystem<G>{
    pub fn new(graph_creator: Box<G> ) -> PhysicsDispatcherManagerSystem<G>{
        PhysicsDispatcherManagerSystem {
            graph_creator,
        }
    }
}

impl<'a, G: PhysicsDispatcherCreator + Send + Sync> RunNow<'a> for PhysicsDispatcherManagerSystem<G>{

    fn setup(&mut self, res: &mut Resources) {

        res.insert(PhysicsDispatcher(None));
    }

    fn run_now(&mut self, res: &'a Resources){
        if self.graph_creator.rebuild(res){
            let dispatcher = self.graph_creator.build::<'a, 'a>(res);
            let disp_wrapper = res.fetch_mut::<PhysicsDispatcher>();
            (*disp_wrapper).0 = dispatcher;
        }
    }
}
