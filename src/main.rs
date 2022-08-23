use altrios_core;

fn main() {
    let mut loco_sim = altrios_core::consist::locomotive::loco_sim::LocomotiveSimulation::default();
    loco_sim.walk();
    let energy_fuel = if let altrios_core::consist::locomotive::LocoType::Conventional(loco_type) =
        loco_sim.loco_unit.loco_type
    {
        loco_type
            .fc
            .state
            .energy_fuel
            .get::<altrios_core::si::joule>()
    } else {
        0.0
    };
    println!("{}", energy_fuel);
}
