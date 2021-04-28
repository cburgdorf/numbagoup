use crate::types::DbInfo;
use std::path::PathBuf;
use std::{fs, vec};

use rustbreak::{deser::Ron, FileDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{DbUserVaultHoldings, UserVaultHoldings};

pub type Db = FileDatabase<Data, Ron>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    // "0xdeadbeef_vaultname" -> entries
    pub group_entries: HashMap<String, Vec<DbUserVaultHoldings>>,
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

pub fn init_default_db() -> Result<Db, &'static str> {
    let app_dir = enforce_app_dir()?;
    let app_dir = app_dir.join("db.ron");
    init_db(app_dir)
}

pub fn init_db(path: PathBuf) -> Result<Db, &'static str> {
    FileDatabase::load_from_path_or(
        path,
        Data {
            group_entries: HashMap::new(),
            any: HashMap::new(),
        },
    )
    .map_err(|_| "Could not read database")
}

pub fn save_entry(
    db: &Db,
    group_id: &str,
    entry: &UserVaultHoldings,
) -> Result<(), rustbreak::RustbreakError> {
    db.write(|db| {
        let db_entry: DbUserVaultHoldings = entry.into();
        match db.group_entries.get_mut(group_id) {
            Some(entries) => {
                let last_entry = entries.last();
                if matches!(last_entry, Some(previous) if previous != &db_entry)
                    || last_entry.is_none()
                {
                    entries.push(db_entry);
                }
            }
            _ => {
                db.group_entries.insert(group_id.to_owned(), vec![db_entry]);
            }
        }
    })?;
    db.save()?;
    Ok(())
}

pub fn read_entries(db: &Db, group_id: &str) -> Vec<UserVaultHoldings> {
    match db.read(|db| db.group_entries.get(group_id).cloned()) {
        Ok(Some(val)) => val.iter().map(UserVaultHoldings::from).collect(),
        _ => vec![],
    }
}

pub fn db_info(db: &Db, group_id: &str) -> DbInfo {
    match db.read(|db| db.group_entries.get(group_id).cloned()) {
        Ok(Some(entries)) => DbInfo {
            entry_count: entries.len(),
            oldest_timestamp: entries.first().map(|val| val.timestamp).unwrap_or_default(),
            newest_timestamp: entries.last().map(|val| val.timestamp).unwrap_or_default(),
        },
        _ => DbInfo {
            entry_count: 0,
            oldest_timestamp: 0,
            newest_timestamp: 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::store::{
        get_app_dir, init_db, read_entries, save_entry, DbUserVaultHoldings, UserVaultHoldings,
    };
    use crate::utils::unix_time;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_get_app_dir() {
        assert_eq!(
            get_app_dir(),
            Ok(PathBuf::from("/home/cburgdorf/.local/share/numbagoup"))
        )
    }

    #[test]
    fn test_write_skips_duplicate_subsequent_entry() {
        let db_path = tempdir().unwrap();
        let db_path = db_path.path().join("test2.ron");
        let db = init_db(db_path).unwrap();

        let entry_1 = UserVaultHoldings::zero();
        let mut entry_2 = UserVaultHoldings::zero();
        // Different timestamps should be ignored
        entry_2.timestamp = entry_2.timestamp + 1;

        let group_id = "0xdeadbeef_some_vault";

        save_entry(&db, &group_id, &entry_1).unwrap();
        save_entry(&db, &group_id, &entry_2).unwrap();

        let entries = read_entries(&db, &group_id);
        assert_eq!(entries.len(), 1);
    }
}
