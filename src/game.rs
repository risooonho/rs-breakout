use crate::{
    blueprints::blu_common::Blueprint,
    components::{
        Components,
        com_transform::Transform,
        com_render::{RenderKind, Render},
        com_render_basic::RenderBasic,
    }
};


const MAX_ENTITIES: usize = 10000;

pub struct Game {
    pub world: Vec<i32>,

    // Components here
    pub transform: Vec<Option<Transform>>,
    // pub render: [RenderKind; MAX_ENTITIES],
}

impl Game {
    pub fn new() -> Game {
        Game {
            world: vec![0; MAX_ENTITIES],
            transform: vec![None; MAX_ENTITIES],
            // render: Vec::with_capacity(MAX_ENTITIES),
        }
    }

    fn create_entity(&mut self, mask: i32) -> usize {
        for i in 0..MAX_ENTITIES {
            if self.world[i] == 0 {
                self.world[i] = mask;
                return i;
            }
        }

        panic!("No more entities available!");
    }

   pub fn add(&mut self, blueprint: &mut Blueprint) -> usize {
        let entity = self.create_entity(Components::Transform as i32);
        let transform_mixin = Transform::new(blueprint.translation, blueprint.rotation, blueprint.scale);
        transform_mixin(self, entity);

        for mixin in blueprint.using.iter_mut() {
            mixin(self, entity);
        }

        entity
   }
}


#[test]
fn game_add_test() {
    use crate::{
        math::{
            vec3::Vec3,
            quat::Quat,
        },
        materials::mat_common::{
            Material,
            Shape,
        }
    };

    let mut game = Game::new();
    let translation = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Quat::new(0.0, 1.0, 0.0, 0.0);
    let scale = Vec3::new(2.0, 2.0, 2.0);

    let shape = Shape {
        key: String::from("shape"),
        indices: vec![],
        vertices: vec![],
        normals: vec![],
    };

    let material = Material {
        mode: 0,
        program: 0,
        uniforms: vec![],
    };

    let mut blueprint_without_mixins = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![],
    };

    let mut blueprint_with_mixins = Blueprint {
        translation: Some(translation),
        rotation: Some(rotation),
        scale: Some(scale),
        using: vec![
            Box::new(RenderBasic::new(&material, &shape, [1.0, 0.0, 1.0, 0.0]))
        ],
    };

    let entity_1 = game.add(&mut blueprint_without_mixins);
    let entity_2 = game.add(&mut blueprint_with_mixins);

    let mask = 1 << Components::Transform as i32;
    let mask_with_mixins = 1 << Components::Transform as i32 | 1 << Components::Render as i32;

    assert_eq!(entity_1, 0, "proper entity index created");
    assert_eq!(entity_2, 1, "proper entity index created");

    for i in vec![entity_1, entity_2] {
        assert_eq!(game.world[i] & mask, mask, "proper entity component mask created");

        assert_eq!(game.transform[i].unwrap().translation.x, translation.x, "translation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().translation.y, translation.y, "translation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().translation.z, translation.z, "translation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].unwrap().rotation.x, rotation.x, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().rotation.y, rotation.y, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().rotation.z, rotation.w, "rotation on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().rotation.w, rotation.w, "rotation on entity fits the one in blueprint");

        assert_eq!(game.transform[i].unwrap().scale.x, scale.x, "scale on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().scale.y, scale.y, "scale on entity fits the one in blueprint");
        assert_eq!(game.transform[i].unwrap().scale.z, scale.z, "scale on entity fits the one in blueprint");
    }

    assert_eq!(game.world[1] & mask_with_mixins, mask_with_mixins, "proper entity component mask created for entity with mixins");

}
