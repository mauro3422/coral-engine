// Database module - SQLite persistence for scenes, blocks, and properties
// Coral Engine - Save/Load system

use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let conn = Connection::open(path.into())?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("Lock error: {}", e)))?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS scenes (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL,
                modified_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS scene_objects (
                id INTEGER PRIMARY KEY,
                scene_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                object_type TEXT NOT NULL,
                position_x REAL NOT NULL,
                position_y REAL NOT NULL,
                position_z REAL NOT NULL,
                rotation_x REAL NOT NULL,
                rotation_y REAL NOT NULL,
                rotation_z REAL NOT NULL,
                scale_x REAL NOT NULL,
                scale_y REAL NOT NULL,
                scale_z REAL NOT NULL,
                visible INTEGER NOT NULL,
                locked INTEGER NOT NULL,
                parent_id INTEGER,
                FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS scene_properties (
                id INTEGER PRIMARY KEY,
                object_id INTEGER NOT NULL,
                prop_type TEXT NOT NULL,
                prop_name TEXT NOT NULL,
                value_text TEXT,
                value_int INTEGER,
                value_float REAL,
                value_bool INTEGER,
                FOREIGN KEY (object_id) REFERENCES scene_objects(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS blocks (
                id INTEGER PRIMARY KEY,
                scene_id INTEGER NOT NULL,
                block_x INTEGER NOT NULL,
                block_z INTEGER NOT NULL,
                name TEXT,
                hidden INTEGER NOT NULL DEFAULT 0,
                locked INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS block_tags (
                block_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (block_id, tag),
                FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS block_voxel_data (
                block_id INTEGER NOT NULL,
                voxel_x INTEGER NOT NULL,
                voxel_y INTEGER NOT NULL,
                voxel_z INTEGER NOT NULL,
                block_type INTEGER NOT NULL,
                PRIMARY KEY (block_id, voxel_x, voxel_y, voxel_z),
                FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS configs (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS keymaps (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                is_default INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                modified_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS keymap_bindings (
                id INTEGER PRIMARY KEY,
                keymap_id INTEGER NOT NULL,
                context TEXT NOT NULL,
                key_code INTEGER NOT NULL,
                modifiers INTEGER NOT NULL DEFAULT 0,
                action TEXT NOT NULL,
                FOREIGN KEY (keymap_id) REFERENCES keymaps(id) ON DELETE CASCADE
            );
            ",
        )?;
        Ok(())
    }

    // Scene operations
    pub fn save_scene(&self, name: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = chrono_now();
        conn.execute(
            "INSERT OR REPLACE INTO scenes (name, created_at, modified_at) VALUES (?1, ?2, ?2)",
            params![name, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn load_scene(&self, name: &str) -> Result<Option<i64>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id FROM scenes WHERE name = ?1")?;
        let result: Result<i64> = stmt.query_row(params![name], |row| row.get(0));
        match result {
            Ok(id) => Ok(Some(id)),
            Err(_) => Ok(None),
        }
    }

    pub fn delete_scene(&self, name: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let deleted = conn.execute("DELETE FROM scenes WHERE name = ?1", params![name])?;
        Ok(deleted > 0)
    }

    pub fn list_scenes(&self) -> Result<Vec<(i64, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name FROM scenes ORDER BY modified_at DESC")?;
        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
        rows.collect()
    }

    // Scene object operations
    pub fn save_object(
        &self,
        scene_id: i64,
        id: i64,
        name: &str,
        obj_type: &str,
        position: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
        visible: bool,
        locked: bool,
        parent_id: Option<i64>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO scene_objects 
             (id, scene_id, name, object_type, position_x, position_y, position_z,
              rotation_x, rotation_y, rotation_z, scale_x, scale_y, scale_z,
              visible, locked, parent_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            params![
                id,
                scene_id,
                name,
                obj_type,
                position[0],
                position[1],
                position[2],
                rotation[0],
                rotation[1],
                rotation[2],
                scale[0],
                scale[1],
                scale[2],
                visible as i32,
                locked as i32,
                parent_id
            ],
        )?;
        Ok(())
    }

    // Block operations
    pub fn save_block(
        &self,
        scene_id: i64,
        block_x: i32,
        block_z: i32,
        name: Option<&str>,
        hidden: bool,
        locked: bool,
    ) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO blocks (scene_id, block_x, block_z, name, hidden, locked)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                scene_id,
                block_x,
                block_z,
                name,
                hidden as i32,
                locked as i32
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn load_blocks(&self, scene_id: i64) -> Result<Vec<(i32, i32, String, bool, bool)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT block_x, block_z, name, hidden, locked FROM blocks WHERE scene_id = ?1",
        )?;
        let rows = stmt.query_map(params![scene_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                row.get::<_, i32>(3)? != 0,
                row.get::<_, i32>(4)? != 0,
            ))
        })?;
        rows.collect()
    }

    pub fn save_block_tag(&self, block_id: i64, tag: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO block_tags (block_id, tag) VALUES (?1, ?2)",
            params![block_id, tag],
        )?;
        Ok(())
    }

    pub fn load_block_tags(&self, block_id: i64) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT tag FROM block_tags WHERE block_id = ?1")?;
        let rows = stmt.query_map(params![block_id], |row| row.get(0))?;
        rows.collect()
    }

    // Config operations
    pub fn save_config(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO configs (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn load_config(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM configs WHERE key = ?1")?;
        let result: Result<String> = stmt.query_row(params![key], |row| row.get(0));
        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }
}

impl Database {
    // Keymap operations
    pub fn save_keymap(&self, name: &str, is_default: bool) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let now = chrono_now();
        conn.execute(
            "INSERT INTO keymaps (name, is_default, created_at, modified_at) VALUES (?1, ?2, ?3, ?3)",
            params![name, is_default as i32, now],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn load_keymap(&self, name: &str) -> Result<Option<i64>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id FROM keymaps WHERE name = ?1")?;
        let result: Result<i64> = stmt.query_row(params![name], |row| row.get(0));
        match result {
            Ok(v) => Ok(Some(v)),
            Err(_) => Ok(None),
        }
    }

    pub fn save_keymap_binding(
        &self,
        keymap_id: i64,
        context: &str,
        key_code: &str,
        modifiers: u8,
        action: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO keymap_bindings (keymap_id, context, key_code, modifiers, action) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![keymap_id, context, key_code, modifiers, action],
        )?;
        Ok(())
    }

    pub fn load_keymap_bindings(
        &self,
        keymap_id: i64,
    ) -> Result<Vec<(String, String, u8, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT context, key_code, modifiers, action FROM keymap_bindings WHERE keymap_id = ?1",
        )?;
        let rows = stmt.query_map(params![keymap_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn delete_keymap(&self, name: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM keymaps WHERE name = ?1", params![name])?;
        Ok(affected > 0)
    }
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_secs())
}
