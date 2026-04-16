// Scene persistence - Save/Load scenes to database
// Integrates with Scene, WorldObject, and BlockRegistry

use crate::core::database::Database;
#[allow(unused_imports)]
use crate::core::scene::{ObjectId, Scene, WorldObject, WorldObjectType};
#[allow(unused_imports)]
use crate::ocean::{BlockRegistry, BlockTag};

pub struct SceneManager {
    pub db: Database,
    current_scene: Option<String>,
}

impl SceneManager {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            current_scene: None,
        }
    }

    pub fn save_scene(
        &mut self,
        name: &str,
        scene: &Scene,
        blocks: &BlockRegistry,
    ) -> Result<i64, rusqlite::Error> {
        let scene_id = self.db.save_scene(name)?;

        for obj in &scene.objects {
            self.db.save_object(
                scene_id,
                obj.id as i64,
                &obj.name,
                &format!("{:?}", obj.obj_type),
                obj.position,
                obj.rotation,
                obj.scale,
                obj.visible,
                obj.locked,
                obj.parent_id.map(|id| id as i64),
            )?;
        }

        for (pos, meta) in &blocks.metadata {
            let block_id = self.db.save_block(
                scene_id,
                pos.x,
                pos.z,
                Some(&meta.name),
                meta.hidden,
                meta.locked,
            )?;

            for tag in &meta.tags {
                self.db.save_block_tag(block_id, &tag.0)?;
            }
        }

        println!(
            "[SceneManager] Saved scene '{}' with {} objects, {} blocks",
            name,
            scene.objects.len(),
            blocks.count()
        );

        Ok(scene_id)
    }

    pub fn load_scene(
        &mut self,
        name: &str,
    ) -> Result<Option<(Scene, BlockRegistry)>, rusqlite::Error> {
        let scene_id = match self.db.load_scene(name)? {
            Some(id) => id,
            None => return Ok(None),
        };

        let scene = Scene::standard();
        let blocks = BlockRegistry::new();

        let loaded_blocks = self.db.load_blocks(scene_id)?;
        for (bx, bz, _name, hidden, locked) in loaded_blocks {
            println!(
                "[SceneManager] Loaded block at ({}, {}): hidden={}, locked={}",
                bx, bz, hidden, locked
            );
        }

        self.current_scene = Some(name.to_string());
        Ok(Some((scene, blocks)))
    }

    pub fn delete_scene(&self, name: &str) -> Result<bool, rusqlite::Error> {
        Ok(self.db.delete_scene(name)?)
    }

    pub fn list_scenes(&self) -> Result<Vec<(i64, String)>, rusqlite::Error> {
        self.db.list_scenes()
    }

    pub fn current_scene_name(&self) -> Option<&str> {
        self.current_scene.as_deref()
    }
}
