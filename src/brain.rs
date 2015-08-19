
use piston::input::GenericEvent;

use std::sync::mpsc;

use ai_behavior;
use ai_behavior::{Action, If, Fail, Sequence, Wait, WaitForever, While};

use cascadecs::{entity, event, components};

use action::Action;
use dir;

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
                let find_food = Sequence(vec![Action(Action::TravelToFood), Action(Action::EatFood)]);
                let dood_behavior = If(Box::new(Action(Action::Content)), Box::new(Action(Action::Idle)), Box::new(find_food));
                ai_behavior::State::new(While(Box::new(WaitForever), vec![dood_behavior]))
            }
        }
    }

    pub fn update<E: GenericEvent>(&self, e: &E, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        match *self {
            Brain::Dood => self.dood(e, entity, components, send),
        }
    }

    fn none(&self, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        //send.send(event::Event::None);
    }


    fn dood<E: GenericEvent>(&self, e: &E, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        use rand;
        use rand::Rng;
        use rand::distributions::{IndependentSample, Range};

        if let Some(bc) = components.brain_components.get(&entity) {
            let mut state = bc.state.clone(); // TODO dumb
            state.event(e, &mut |action_args| {
                let mut result = (ai_behavior::Success, 0.0);
                let event = match *action_args.action {
                    Action::Idle => {
                        match Range::new(0, 11).ind_sample(&mut rand::thread_rng()) {
                            0 => event::Event::Movement { entity: entity, dir: dir::Dir::Up },
                            1 => event::Event::Movement { entity: entity, dir: dir::Dir::Down },
                            2 => event::Event::Movement { entity: entity, dir: dir::Dir::Left },
                            3 => event::Event::Movement { entity: entity, dir: dir::Dir::Right },
                            4...10 => event::Event::None,
                            _ => unreachable!()
                        }
                    },
                    Action::Content => {
                        if let Some(hc) = components.hunger_components.get(&entity) {
                            if hc.value < 80 {
                                result = (ai_behavior::Failure, 0.0);
                            }
                        }
                        event::Event::None
                    },
                    Action::TravelToFood => {
                        println!("traveling");
                        event::Event::None
                    },
                    Action::EatFood => {
                        println!("eating");
                        event::Event::None
                    },
                };
                send.send(event);
                result
            });
            send.send(event::Event::UpdateBrainState { entity: entity, state: state.clone() });
        }
    }
}

