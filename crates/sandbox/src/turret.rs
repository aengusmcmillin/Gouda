use cgmath::{Vector2, Vector3};
use gouda::ecs::{Entity, Mutation, ECS};
use gouda::rendering::sprites::SpriteComponent;
use gouda::transform::TransformComponent;

use crate::building::Turret;
use crate::supplies::Supplies;
use crate::tilemap::Tile;

pub struct CreateTurretMutation {
    pub tile_e: Entity,
}

impl Mutation for CreateTurretMutation {
    fn apply(&self, ecs: &mut ECS) {
        if ecs.write_res::<Supplies>().spend_supplies(0, 5, 0) {
            Turret::create(ecs, self.tile_e);

            ecs.write::<Tile>(&self.tile_e).unwrap().occupied = true;
        } else {
        }
    }
}

pub struct TurretSelectMutation {
    pub turret_e: Entity,
}

impl Mutation for TurretSelectMutation {
    fn apply(&self, ecs: &mut ECS) {
        let mut loc = *ecs.read::<TransformComponent>(&self.turret_e).unwrap();
        loc.scale = Vector3::new(3.0, 3.0, 1.);
        let range_sprite =
            SpriteComponent::new(ecs, "./assets/bitmap/range_indicator.png".to_string());
        let range_indicator = Some(ecs.build_entity().add_component(range_sprite).add_component(loc).entity());
        let turret = ecs.write::<Turret>(&self.turret_e).unwrap();
        turret.selected = true;
        turret.range_indicator = range_indicator;
    }
}

pub struct TurretDeselectMutation {}

impl Mutation for TurretDeselectMutation {
    fn apply(&self, ecs: &mut ECS) {
        let turrets = ecs.get1::<Turret>();
        for turret in &turrets {
            let turret = ecs.write::<Turret>(&turret).unwrap();
            turret.selected = false;
            let indicator = turret.range_indicator;
            if let Some(e) = indicator {
                println!("Deletintg indicator");
                turret.range_indicator = None;
                ecs.delete_entity(&e);
            }
        }
    }
}
