use std::fs;
use std::path::PathBuf;

use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{DbUserVaultHoldings, UserVaultHoldings};

pub type DB = FileDatabase<Data, Ron>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub entries: Vec<DbUserVaultHoldings>,
    pub any: HashMap<String, String>,
}

pub fn get_app_dir() -> Result<PathBuf, &'static str> {
    dirs::data_local_dir()
        .ok_or("Can't obtain system application directory")
        .map(|path| path.join("numbagoup"))
}

pub fn enforce_app_dir() -> Result<PathBuf, &'static str> {
    let app_dir = get_app_dir()?;
    if app_dir.exists() {
        Ok(app_dir)
    } else {
        fs::create_dir(&app_dir)
            .map_err(|_| "Can't create application directory")
            .map(|_| app_dir)
    }
}

pub fn init_default_db() -> Result<DB, &'static str> {
    let app_dir = enforce_app_dir()?;
    let app_dir = app_dir.join("db.ron");
    init_db(app_dir)
}

pub fn init_db(path: PathBuf) -> Result<DB, &'static str> {
    FileDatabase::load_from_path_or(
        path,
        Data {
            entries: vec![],
            any: HashMap::new(),
        },
    )
    .map_err(|_| "Could not read database")
}

pub fn save_entry(db: &DB, entry: &UserVaultHoldings) -> Result<(), rustbreak::RustbreakError> {
    // TODO: We should check if the previous entry is identical and skip if that's the case
    db.write(|db| {
        db.entries.push(entry.into());
    })?;
    db.save()?;
    Ok(())
}

pub fn read_entries(db: &DB) -> Vec<UserVaultHoldings> {
    db.read(|db| db.entries.clone())
        .unwrap()
        .iter()
        .map(UserVaultHoldings::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::store::{get_app_dir, init_db, init_default_db, DbUserVaultHoldings};
    use bigdecimal::BigDecimal;
    use std::fs;
    use std::path::PathBuf;
    use std::{env, ops::RangeBounds};
    use tempfile::tempdir;
    #[test]
    fn test_get_app_dir() {
        assert_eq!(
            get_app_dir(),
            Ok(PathBuf::from("/home/cburgdorf/.local/share/numbagoup"))
        )
    }

    #[test]
    fn test_db() {
        let db_path = tempdir().unwrap();
        let db_path = db_path.path().join("test2.ron");
        let db = init_db(db_path).unwrap();
        let _ = db.write(|db| {
            db.any.insert("foo".to_string(), "bar".to_string());
            db.entries.push(DbUserVaultHoldings {
                price_per_share: "0".to_string(),
                dai: "0".to_string(),
                usdc: "0".to_string(),
                both: "0".to_string(),
                cdai: "0".to_string(),
                cusdc: "0".to_string(),
                cboth: "0".to_string(),
            });
            db.entries.push(DbUserVaultHoldings {
                price_per_share: "1".to_string(),
                dai: "1".to_string(),
                usdc: "1".to_string(),
                both: "1".to_string(),
                cdai: "1".to_string(),
                cusdc: "1".to_string(),
                cboth: "1".to_string(),
            });
        });
        let foo = db.read(|db| db.any.get("foo").cloned()).unwrap();
        let entries = db.read(|db| db.entries.clone()).unwrap();
        assert_eq!(foo, Some("bar".to_string()));
        assert_eq!(entries.len(), 2);
    }
}
