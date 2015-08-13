
use piston::input;

use std::sync::mpsc;

use ai_behavior;
use ai_behavior::{Action, Fail, Sequence, Wait, WaitForever, While};

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
                let behavior = While(Box::new(WaitForever), vec![Sequence(vec![
                    Action(Action::GoDown), Action(Action::ComeUp)
                ])]);
                ai_behavior::State::new(behavior)
            }
        }
    }

    pub fn update(&self, e: &input::GenericEvent, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        match *self {
            Brain::Dood => self.dood(e, entity, components, send),
        }
    }

    fn none(&self, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        //send.send(event::Event::None);
    }

    fn dood(&self, e: &input::GenericEvent, entity: entity::Entity, components: &components::Components, send: mpsc::Sender<event::Event>) {
        if let Some(bc) = components.brain_components.get(&entity) {
            let mut state = bc.state.clone(); // TODO dumb
            //let e = Input::Focus(true); // TODO dumb
            //let e = input::Event::Update(*dt);
            //let upd = e.update(|args| Some(args.dt)).unwrap_or(None);
            //println!("{:?}", upd);
            state.event(e, &mut |action_args| {
                send.send(event::Event::Movement {
                    entity: entity,
                    dir: match *action_args.action {
                        Action::GoDown => dir::Dir::Down,
                        Action::ComeUp => dir::Dir::Up,
                    }
                });
                (ai_behavior::Success, 1.0)
            });
            send.send(event::Event::UpdateBrainState { entity: entity, state: state.clone() });
        }
    }
}

