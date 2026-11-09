use weft_sdk::*;
use base_api::{Armor, ARMOR};

declare_mod!(deps: ["base"], init: init);

fn init(r: &mut Registry) {

    static GOLD_HELMET: Item = Item::named("base:gold_helmet");
    static GOLD_CHESTPLATE: Item = Item::named("base:gold_chestplate");
    static GOLD_LEGGINGS: Item = Item::named("base:gold_leggings");
    static GOLD_BOOTS: Item = Item::named("base:gold_boots");

    //// Remove armor value for gold ////
    r.edit(&GOLD_HELMET)
        .set(&ARMOR, Armor { points: 0, toughness: 0.0 });
    r.edit(&GOLD_CHESTPLATE)
        .set(&ARMOR, Armor { points: 3, toughness: 0.0 });
    r.edit(&GOLD_LEGGINGS)
        .set(&ARMOR, Armor { points: 0, toughness: 0.0 });
    r.edit(&GOLD_BOOTS)
        .set(&ARMOR, Armor { points: 0, toughness: 0.0 });

    //// Get number of gold armor pieces on
    fn gold_piece_count(w: &World, entity: u64) -> usize {
        let Some(worn) = w.inventory(entity).find_group(EQUIPMENT_GROUP) else {
            return 0;
        };
        (0..worn.len())
            .filter(|&slot| {
                let item = worn.get(slot).item;
                item == GOLD_HELMET.id()
                    || item == GOLD_CHESTPLATE.id()
                    || item == GOLD_LEGGINGS.id()
                    || item == GOLD_BOOTS.id()
            })
            .count()
    }

    //// Increase damage done with gold worn ////
    r.modify(&chain::ATTACK_DAMAGE, stage::MULTIPLY, |m| {
        let q = *m.query();
        let w = m.world();
        let gold_pieces = gold_piece_count(&w, q.entity);
        const GOLD_DAMAGE: [f64; 5] = [1.0, 1.1, 1.4, 1.9, 3.0];
        m.value *= GOLD_DAMAGE[gold_pieces.min(GOLD_DAMAGE.len() - 1)];

    });

    //// Increase fall damage taken with gold worn ////
    r.modify(&chain::DAMAGE, stage::MULTIPLY, |m| {
        let q = *m.query();
        if q.cause != damage_cause::FALL { return; } // Only scale fall damage.
        let w = m.world();
        let gold_pieces = gold_piece_count(&w, q.entity);

        const GOLD_FALL_DAMAGE: [f64; 5] = [1.0, 1.1, 1.4, 1.9, 3.0];
        m.value *= GOLD_FALL_DAMAGE[gold_pieces.min(GOLD_FALL_DAMAGE.len() - 1)];
    });

    //// Call mob alert on doing damage ////
    r.on_event(&actions::HURT, |b| {
        let w = b.world();
        for h in b.iter().filter(|h| h.cause == damage_cause::ATTACK) {
            let Some(victim) = w.entity(h.entity).filter(|s| s.health > 0.0) else {
                continue;
            };
            alert_mob_when_gilded(&w, h.source, victim.pos, h.entity);
        }
    });

    //// Call mob alert on kill ////
    r.on_event(&actions::DIE, |b| {
        let w = b.world();
        for d in b.iter().filter(|d| d.cause == damage_cause::ATTACK) {
            alert_mob_when_gilded(&w, d.source, d.pos, d.entity);
        }
    });

    //// Mob alert: When doing damage with gold on, alert nearby mobs as well
    fn alert_mob_when_gilded(w: &World, source: u64, center: Vec3, excluded: u64) {
        if source == 0 || !w.players_iter().any(|p| p == source) {
            return;
        }
        let gold_pieces = gold_piece_count(w, source);
        let radius = match gold_pieces {
            1 => 2.0,
            2 => 8.0,
            3 => 18.0,
            4 => 30.0,
            _ => return,
        };
        for id in w.targets_near(center, radius, None) {
            if id == excluded || w.players_iter().any(|p| p == id) {
                continue;
            }
            // Some chance a mob isn't angered, just to make cheesing a bit less simple.
            if w.random_below(100) >= 90 {
                continue;
            }
            w.emit(
                &actions::HURT,
                Hurt {
                    entity: id,
                    source,
                    amount: 0.0,
                    cause: damage_cause::GENERIC,
                },
            );
        }
    }
}