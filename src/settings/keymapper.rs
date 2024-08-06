use crate::Keybind;

use super::LogitechSettings;

impl LogitechSettings {
    pub fn add_keybind(&self, keybind: &Keybind) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO keymapper (ptr, obj, exe) VALUES (?, ?, ?);",
            (
                u64::from(&keybind.input_pointer()),
                u64::from(&keybind.input_object()),
                keybind.executable.to_string(),
            ),
        )?;
        Ok(())
    }

    pub fn set_keybinds(&self, executable: String, keybinds: Vec<Keybind>) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM keymapper WHERE exe=?;", (executable,))?;
        for key in keybinds {
            if key.vkey_pointer != 0 && key.vkey_object != 0 {
                self.add_keybind(&key)?;
            }
        }
        Ok(())
    }

    pub fn get_keybinds(
        &self,
        executable: &String,
    ) -> rusqlite::Result<Vec<rusqlite::Result<Keybind>>> {
        let mut stmt = self
            .conn
            .prepare("SELECT idx, ptr, obj, exe FROM keymapper WHERE exe=?;")?;
        let keys = stmt.query_map([executable], |row| {
            Ok(Keybind::from_vkeys(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ))
        })?;
        Ok(Vec::from_iter(keys))
    }

    pub fn remove_keybind(&self, keybind: &Keybind) -> rusqlite::Result<()> {
        self.conn.execute(
            "DELETE FROM keymapper WHERE idx=? AND exe=?;",
            (keybind.index, keybind.executable.to_string()),
        )?;
        Ok(())
    }
}
