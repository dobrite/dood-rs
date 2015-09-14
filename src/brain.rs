
use piston::input::GenericEvent;

use std::sync::mpsc;

use ai_behavior::{Success, Running, Failure, Action, If, Sequence, WaitForever, While, State};

use cascadecs::entity::Entity;
use cascadecs::event::Event;
use cascadecs::components::Components;
use cascadecs::food_component::FoodComponent;

use dir::Dir;
use loc::Loc;
use food::Food;
use action::Action;
use path::PathTarget;

#[derive(Clone, Copy, Debug)]
pub enum Brain {
    Dood,
}

impl Brain {
    pub fn new_state(brain: Brain) -> State<Action, ()> {
        match brain {
            Brain::Dood => {
                // abtract travelTo(X), eat(X)?
                let path_target = PathTarget::Component(FoodComponent::new(Food::Meat, 0.0));
                let find_food = Sequence(vec![Action(Action::PathTo { path_target: path_target }), Action(Action::EatFood)]);
                let dood_behavior = If(Box::new(Action(Action::Content)),
                                       Box::new(Action(Action::Idle)),
                                       Box::new(find_food));
                State::new(While(Box::new(WaitForever), vec![dood_behavior]))
            }
        }
    }

    pub fn update<E: GenericEvent>(&self,
                                   e: &E,
                                   entity: Entity,
                                   components: &Components,
                                   send: mpsc::Sender<Event>) {
        match *self {
            Brain::Dood => self.dood(e, entity, components, send),
        }
    }

    fn dood<E: GenericEvent>(&self,
                             e: &E,
                             entity: Entity,
                             components: &Components,
                             send: mpsc::Sender<Event>) {
        use rand;
        use rand::distributions::{IndependentSample, Range};

        if let Some(bc) = components.get_brain_component(entity) {
            let mut state = bc.state.clone(); // TODO dumb
            state.event(e, &mut |action_args| {
                if action_args.dt == 0.0 { return (Running, 0.0) };
                let (result, event) = match *action_args.action {
                    Action::Idle => {
                        let event = match Range::new(0, 11).ind_sample(&mut rand::thread_rng()) {
                            0 => Event::Movement { entity: entity, dir: Dir::Up },
                            1 => Event::Movement { entity: entity, dir: Dir::Down },
                            2 => Event::Movement { entity: entity, dir: Dir::Left },
                            3 => Event::Movement { entity: entity, dir: Dir::Right },
                            4...10 => Event::None,
                            _ => unreachable!()
                        };
                        ((Success, 0.0), event)
                    },
                    Action::Content => {
                        if let Some(hc) = components.get_hunger_component(entity) {
                            ((if hc.value > 50 { Success } else { Failure }, action_args.dt), Event::None)
                        } else {
                            unreachable!()
                        }
                    },
                    Action::PathTo { path_target } => {
                        // TODO this could use a rethink. also ignores noms and food type
                        if let Some(pc) = components.get_path_component(entity) {
                            match pc.path.last() {
                                Some(_) => ((Running, 0.0), Event::PopPath { entity: entity }),
                                None => ((Success, action_args.dt), Event::None)
                            }
                        } else {
                            let evnt = Event::PathTo { entity: entity, path_target: path_target };
                            ((Running, action_args.dt), evnt)
                        }
                    },
                    Action::EatFood => {
                        match components.get_brain_component(entity) {
                            Some(bc) => {
                                match bc.target {
                                    None => ((Success, 0.0), Event::None),
                                    Some(target) => match components.get_food_component(target) {
                                        None => ((Success, 0.0), Event::None),
                                        Some(_) => {
                                            let event = Event::EatFood {
                                                entity: entity,
                                                target: target
                                            };
                                            ((Running, 0.0), event)
                                        }
                                    }
                                }
                            },
                            None => unreachable!(),
                        }
                    },
                };
                send.send(event).unwrap();
                result
            });
            send.send(Event::UpdateBrainState { entity: entity, state: state.clone() }).unwrap();
        }
    }
}
