// Keymap Persistence - Coral Engine
// Save/Load keymaps to SQLite (simplified)

use super::context::InputContext;
use crate::core::database::Database;

pub struct KeymapManager {
    db: Database,
}

impl KeymapManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn save_keymap(
        &self,
        name: &str,
        context: InputContext,
        bindings: &[(&str, u8, &str)],
    ) -> Result<i64, rusqlite::Error> {
        let keymap_id = self.db.save_keymap(name, false)?;

        for (key_name, modifiers, action_name) in bindings {
            self.db.save_keymap_binding(
                keymap_id,
                context.name(),
                *key_name,
                *modifiers,
                action_name,
            )?;
        }
        Ok(keymap_id)
    }

    pub fn load_default_keymap(&self) -> Result<Option<i64>, rusqlite::Error> {
        self.db.load_keymap("blender_default")
    }

    pub fn load_keymap(
        &self,
        name: &str,
    ) -> Result<Vec<(String, String, u8, String)>, rusqlite::Error> {
        let keymap_id = match self.db.load_keymap(name)? {
            Some(id) => id,
            None => return Ok(Vec::new()),
        };
        self.db.load_keymap_bindings(keymap_id)
    }
}
