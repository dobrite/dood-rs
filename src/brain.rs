
use piston::input::GenericEvent;

use std::sync::mpsc;

use ai_behavior;
use ai_behavior::{Action, If, Sequence, WaitForever, While};

use cascadecs::entity::Entity;
use cascadecs::event::Event;
use cascadecs::components::Components;

use action::Action;
use dir;

#[derive(Clone, Copy, Debug)]
pub enum Brain {
    Dood,
}

impl Brain {
    pub fn new_state(brain: Brain) -> ai_behavior::State<Action, ()> {
        match brain {
            Brain::Dood => {
                // abtract travelTo(X), eat(X)?
                let find_food = Sequence(vec![Action(Action::PathToFood), Action(Action::EatFood)]);
                let dood_behavior = If(Box::new(Action(Action::Content)), Box::new(Action(Action::Idle)), Box::new(find_food));
                ai_behavior::State::new(While(Box::new(WaitForever), vec![dood_behavior]))
            }
        }
    }

    pub fn update<E: GenericEvent>(&self, e: &E, entity: Entity, components: &Components, send: mpsc::Sender<Event>) {
        match *self {
            Brain::Dood => self.dood(e, entity, components, send),
        }
    }

    fn dood<E: GenericEvent>(&self, e: &E, entity: Entity, components: &Components, send: mpsc::Sender<Event>) {
        use rand;
        use rand::distributions::{IndependentSample, Range};

        if let Some(bc) = components.get_brain_component(entity) {
            let mut state = bc.state.clone(); // TODO dumb
            state.event(e, &mut |action_args| {
                if action_args.dt == 0.0 { return (ai_behavior::Running, 0.0) };
                let (result, event) = match *action_args.action {
                    Action::Idle => {
                        let event = match Range::new(0, 11).ind_sample(&mut rand::thread_rng()) {
                            0 => Event::Movement { entity: entity, dir: dir::Dir::Up },
                            1 => Event::Movement { entity: entity, dir: dir::Dir::Down },
                            2 => Event::Movement { entity: entity, dir: dir::Dir::Left },
                            3 => Event::Movement { entity: entity, dir: dir::Dir::Right },
                            4...10 => Event::None,
                            _ => unreachable!()
                        };
                        ((ai_behavior::Success, 0.0), event)
                    },
                    Action::Content => {
                        // TODO switch these direct gets with get_X_components;
                        if let Some(hc) = components.get_hunger_component(entity) {
                            if hc.value > 50 {
                                ((ai_behavior::Success, action_args.dt), Event::None)
                            } else {
                                ((ai_behavior::Failure, action_args.dt), Event::PathToFood { entity: entity })
                            }
                        } else {
                            unreachable!()
                        }
                    },
                    Action::PathToFood => {
                        if let Some(pc) = components.get_path_component(entity) {
                            match pc.path.last() {
                                Some(_) => ((ai_behavior::Running, 0.0), Event::PopPath { entity: entity }),
                                None => ((ai_behavior::Success, action_args.dt), Event::None)
                            }
                        } else {
                            ((ai_behavior::Running, action_args.dt), Event::PathToFood { entity: entity })
                        }
                    },
                    Action::EatFood => {
                        match components.get_brain_component(entity) {
                            Some(bc) => {
                                match bc.target {
                                    None => ((ai_behavior::Success, 0.0), Event::None),
                                    Some(target) => match components.get_food_component(target) {
                                        None => ((ai_behavior::Success, 0.0), Event::None),
                                        Some(_) => {
                                            let event = Event::EatFood { entity: entity, target: target };
                                            ((ai_behavior::Running, 0.0), event)
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

