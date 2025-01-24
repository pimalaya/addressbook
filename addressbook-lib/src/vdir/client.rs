use std::path::PathBuf;

use crate::{Addressbook, PartialAddressbook};

use super::{Config, CreateAddressbook, DeleteAddressbook, ListAddressbooks, UpdateAddressbook};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Client {
    pub config: Config,
}

impl Client {
    pub fn new(home_dir: impl Into<PathBuf>) -> Self {
        Self {
            config: Config {
                home_dir: home_dir.into(),
            },
        }
    }

    #[cfg(debug_assertions)]
    pub fn new_from_envs() -> Self {
        let home_dir = match std::env::var("HOME_DIR") {
            Ok(var) => var,
            Err(_) => std::env::temp_dir()
                .join(uuid::Uuid::new_v4().to_string())
                .to_string_lossy()
                .to_string(),
        };

        std::fs::create_dir_all(&home_dir).expect("should create home dir");

        Self::new(home_dir)
    }

    pub fn create_addressbook(&self, addressbook: Addressbook) -> CreateAddressbook {
        CreateAddressbook::new(&self.config, addressbook)
    }

    pub fn list_addressbooks(&self) -> ListAddressbooks {
        ListAddressbooks::new(&self.config)
    }

    pub fn update_addressbook(&self, addressbook: PartialAddressbook) -> UpdateAddressbook {
        UpdateAddressbook::new(&self.config, addressbook)
    }

    pub fn delete_addressbook(&self, addressbook_id: impl AsRef<str>) -> DeleteAddressbook {
        DeleteAddressbook::new(&self.config, addressbook_id)
    }
}
