#![warn(clippy::pedantic)]
use crate::prelude::*;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
#[write_component(Thoughness)]
#[write_component(Experience)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // entities that wish to initiate combat
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers // defines the intended type for victims to be collected by collect()
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim)) // map takes a closure as its parameter
        .collect(); // takes the query's results and put them in the vector

    // victims.iter().for_each(|(message, victim)| {
    //     if let Ok(toughness) = ecs
    //         .entry_mut(*victim)
    //         .unwrap()
    //         .get_component_mut::<Thoughness>()
    //     {
    //         println!("Monster toughness: {}", toughness.current);
    //     };
    // });

    // let mut attackers = <(Entity, &mut Experience)>::query();
    // attackers
    //     .iter_mut(ecs)
    //         .for_each(|(_entity, exp)|{
    //                     exp.current += toughness.current;
    //         });

    victims.iter().for_each(|(message, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*victim);
            }
            println!("Health after attack: {}", health.current);
        };

    // attackers.iter().for_each(|(attacker, _message, experience)| {
    //         if let Ok(mut experience) = ecs
    //             .entry_mut(*attacker)
    //             .unwrap()
    //             .get_component_mut::<Experience>()
    //         {
    //             println!("Experience before combat: {}", experience.current);
    //             experience.current += 10;
    //             println!("Experience after combat: {}", experience.current);
    //         };
    //     });
        commands.remove(*message);
    });
}
