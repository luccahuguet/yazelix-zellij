use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::{consts::ZELLIJ_PLUGIN_PERMISSIONS_CACHE, data::PermissionType};

pub type GrantedPermission = HashMap<String, Vec<PermissionType>>;

#[derive(Default, Debug)]
pub struct PermissionCache {
    path: PathBuf,
    granted: GrantedPermission,
}

impl PermissionCache {
    pub fn cache(&mut self, plugin_name: String, permissions: Vec<PermissionType>) {
        self.granted.insert(plugin_name, permissions);
    }

    pub fn get_permissions(&self, plugin_name: String) -> Option<&Vec<PermissionType>> {
        self.granted.get(&plugin_name)
    }

    pub fn check_permissions(
        &self,
        plugin_name: String,
        permissions_to_check: &Vec<PermissionType>,
    ) -> bool {
        if let Some(target) = self.granted.get(&plugin_name) {
            let mut all_granted = true;
            for permission in permissions_to_check {
                if !target.contains(permission) {
                    all_granted = false;
                }
            }
            return all_granted;
        }

        false
    }

    pub fn from_path_or_default(cache_path: Option<PathBuf>) -> Self {
        let cache_path = cache_path
            .or_else(|| env::var_os("ZELLIJ_PLUGIN_PERMISSIONS_CACHE").map(PathBuf::from))
            .unwrap_or(ZELLIJ_PLUGIN_PERMISSIONS_CACHE.to_path_buf());

        let granted = match fs::read_to_string(cache_path.clone()) {
            Ok(raw_string) => PermissionCache::from_string(raw_string).unwrap_or_default(),
            Err(e) => {
                log::error!("Failed to read permission cache file: {}", e);
                GrantedPermission::default()
            },
        };

        PermissionCache {
            path: cache_path,
            granted,
        }
    }

    pub fn write_to_file(&self) -> std::io::Result<()> {
        let mut f = File::create(&self.path)?;
        write!(f, "{}", PermissionCache::to_string(&self.granted))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_cache_path_precedes_default_but_not_explicit_path() {
        let environment_cache = tempfile::NamedTempFile::new().unwrap();
        let explicit_cache = tempfile::NamedTempFile::new().unwrap();

        let previous = std::env::var_os("ZELLIJ_PLUGIN_PERMISSIONS_CACHE");
        std::env::set_var("ZELLIJ_PLUGIN_PERMISSIONS_CACHE", environment_cache.path());
        let from_environment = PermissionCache::from_path_or_default(None);
        let from_explicit =
            PermissionCache::from_path_or_default(Some(explicit_cache.path().into()));
        match previous {
            Some(value) => std::env::set_var("ZELLIJ_PLUGIN_PERMISSIONS_CACHE", value),
            None => std::env::remove_var("ZELLIJ_PLUGIN_PERMISSIONS_CACHE"),
        }

        assert_eq!(from_environment.path, environment_cache.path());
        assert_eq!(from_explicit.path, explicit_cache.path());
    }
}
