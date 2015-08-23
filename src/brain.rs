
use piston::input::GenericEvent;

use std::sync::mpsc;

use ai_behavior;
use ai_behavior::{Action, If, Fail, Sequence, Wait, WaitForever, While};

use cascadecs::entity::Entity;
use cascadecs::event::Event;
use cascadecs::components::Components;

use action::Action;
use dir;
use path::PathTarget;

#[derive(Clone, Copy, Debug)]
pub enum Brain {
    Dood,
}

//let circling = Action(Action::Circling);
//let give_up_or_attack = WhenAny(vec![
//    Action(Action::PlayerFarAwayFromTarget(100.0)),
//    Sequence(vec![
//        Action(Action::PlayerWithinDistance(10.0)),
//        Action(Action::AttackPlayer(0.1)),
//    ])
//]);
//let attack_attempt =
//    While(Box::new(give_up_or_attack), vec![
//        Action(Action::FlyTowardPlayer)
//    ]);
//let behavior = While(Box::new(WaitForever), vec![Sequence(vec![
//        While(Box::new(Wait(5.0)), vec![circling.clone()]),
//        While(Box::new(Action(Action::PlayerWithinDistance(50.0))), vec![circling.clone()]),
//    ]),
//    While(Box::new(give_up_or_attack), vec![
//        Action(Action::FlyTowardPlayer)
//    ]);
//]);

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

    fn none(&self, entity: Entity, components: &Components, send: mpsc::Sender<Event>) {
        //send.send(event::Event::None);
    }

    fn dood<E: GenericEvent>(&self, e: &E, entity: Entity, components: &Components, send: mpsc::Sender<Event>) {
        use rand;
        use rand::Rng;
        use rand::distributions::{IndependentSample, Range};

        if let Some(bc) = components.brain_components.get(&entity) {
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
                        if let Some(hc) = components.hunger_components.get(&entity) {
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
                        if let Some(pc) = components.path_components.get(&entity) {
                            match pc.path.last() {
                                Some(loc) => ((ai_behavior::Running, 0.0), Event::PopPath { entity: entity }),
                                None => ((ai_behavior::Success, action_args.dt), Event::None)
                            }
                        } else {
                            ((ai_behavior::Running, action_args.dt), Event::PathToFood { entity: entity })
                        }
                    },
                    Action::EatFood => {
                        match components.brain_components.get(&entity) {
                            Some(bc) => {
                                match bc.target {
                                    None => ((ai_behavior::Success, 0.0), Event::None),
                                    Some(target) => match components.food_components.get(&target) {
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
                send.send(event);
                result
            });
            send.send(Event::UpdateBrainState { entity: entity, state: state.clone() });
        }
    }
}

