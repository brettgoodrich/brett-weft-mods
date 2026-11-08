use weft_sdk::*;

declare_mod!(deps: ["base"], init: init);

static GOLD_ARMOR: [Item; 4] = [
    Item::named("base:gold_helmet"),
    Item::named("base:gold_chestplate"),
    Item::named("base:gold_leggings"),
    Item::named("base:gold_boots"),
];

fn init(r: &mut Registry) {
    // Each gold piece reduces incoming damage by 5%.
    r.modify(&chain::DAMAGE, stage::MULTIPLY, |m| {
        let q = *m.query();
        let pieces = gold_pieces(&m.world(), q.entity);
        if pieces > 0 {
            m.value *= 1.0 - 0.05 * pieces as f64;
        }
    });

    // One, two, or three pieces give +0.3x each; the full set gives 3x total damage.
    r.modify(&chain::ATTACK_DAMAGE, stage::MULTIPLY, |m| {
        let q = *m.query();
        let pieces = gold_pieces(&m.world(), q.entity);
        let multiplier = match pieces {
            4 => 3.0,
            1..=3 => 1.0 + 0.3 * pieces as f64,
            _ => return,
        };
        m.value *= multiplier;
    });
}

fn gold_pieces(world: &World, entity: u64) -> usize {
    let Some(equipment) = world.inventory(entity).find_group(EQUIPMENT_GROUP) else {
        return 0;
    };

    (0..equipment.len())
        .filter(|&slot| {
            let item = equipment.get(slot).item;
            GOLD_ARMOR.iter().any(|piece| piece.id() == item)
        })
        .count()
}