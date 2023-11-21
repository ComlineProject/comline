// Standard Uses
use std::path::{Path, PathBuf};
use std::collections::HashMap;

// Crate Uses
use crate::client::registry;

// External Uses
use comline_core::project::build;
use comline_core::project::ir::context::ProjectContext;
use comline_core::project::ir::frozen as project_frozen;
use comline_core::project::ir::frozen::{PublishRegistry, RegistryKind};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use eyre::{bail, Result};


static COMLINE_CONFIGS_ENV_VAR: Lazy<String> = Lazy::new(||
    shellexpand::tilde(&shellexpand::env("$COMLINE_HOME").unwrap()).to_string()
);

static COMLINE_CONFIGS_PATH: Lazy<&Path> = Lazy::new(||
    Path::new(&*COMLINE_CONFIGS_ENV_VAR)
);

static CREDENTIALS_PATH: Lazy<PathBuf> = Lazy::new(||
    COMLINE_CONFIGS_PATH.join("credentials.toml")
);


#[derive(Serialize, Deserialize)]
pub struct Config {
    registries: Option<HashMap<String, PublishRegistry>>,
    credentials: Option<HashMap<String, Credential>>
}

#[derive(Serialize, Deserialize)]
pub struct Credential {
    token: String
}

#[allow(unused)]
/// Logs in to a registry using a credential token
/// If the login is successful, the credentials are added to the configuration
pub fn login_to_registry(
    package_registries: Option<&HashMap<String, PublishRegistry>>,
    registry: String, token: String
) -> Result<()> {
    let mut config = get_config_default(&CREDENTIALS_PATH)?;

    let credential = config.credentials.as_mut().unwrap()
        .entry(registry.clone())
        .or_insert(Credential { token: token.clone() });

    let save_credentials_fn = || {
        let result = toml::to_string(&config)?;
        std::fs::write(&*CREDENTIALS_PATH, result)?;
        Ok(())
    };

    let auth_fn = |reg: &PublishRegistry| {
        match reg.kind {
            RegistryKind::LocalStorage => {
                if Path::new(&reg.uri).exists() {
                    return save_credentials_fn()
                }
            }
            RegistryKind::RegistryServer => {
                registry::login(&registry, &token)?;
                return save_credentials_fn()
            }
        }

        Ok(())
    };

    if let Some(package_registries) = package_registries {
        if let Some(pkg_registry) = package_registries.get(&registry) {
            return auth_fn(pkg_registry)
        }
    }

    let Some(cfg_registry) = config.registries.as_ref().unwrap().get(&registry) else {
        bail!(
            "No registry '{}' found in package configuration or comline global configuration",
            &registry
        )
    };

    auth_fn(cfg_registry)
}

/// Gets configuration from given path, it also creates a configuration file if it doesn't exist
fn get_config_default(config_path: &Path) -> Result<Config> {
    let parent_path = config_path.parent().unwrap();

    if !parent_path.exists() {
        std::fs::create_dir_all(parent_path).unwrap();
    };

    let config = if !config_path.exists() {
        Config {
            registries: Some(HashMap::from([
                // TODO: Some of the data here could be referenced to stdlib's publish information
                ("default".to_owned(), PublishRegistry {
                    kind: RegistryKind::RegistryServer,
                    uri: "https://newwars.colean.cc/comline_public/".to_owned(),
                })
            ])),
            credentials: Some(Default::default())
        }
    } else {
        let contents = std::fs::read_to_string(config_path)?;
        toml::from_str::<Config>(&contents)?
    };

    Ok(config)
}

#[allow(unused)]
pub fn logout_from_registry(registry: String) -> Result<()> {
    todo!()
}

pub(crate) fn publish_to_registries(
    project_context: &ProjectContext, registries: Vec<String>
) -> Result<()> {
    for registry in &registries {
        publish_to_registry(project_context, registry.clone())?;
    }

    Ok(())
}

pub(crate) fn publish_to_registry(
    package_context: &ProjectContext, registry: String
) -> Result<()> {
    let config = get_config_default(&CREDENTIALS_PATH)?;

    let auth_fn = |reg: &PublishRegistry| {
        match reg.kind {
            RegistryKind::LocalStorage => {
                if Path::new(&reg.uri).exists() {
                    let package_name = project_frozen::namespace(
                        package_context.config_frozen.as_ref().unwrap()
                    ).unwrap();
                    let package_path = Path::new(&reg.uri).join(package_name);
                    if !package_path.exists() { std::fs::create_dir_all(&package_path)?; }

                    /*
                    let package_latest_version_path = package_path.join(
                        project_frozen::version(
                            package_context.config_frozen.as_ref().unwrap()
                        ).unwrap()
                    );
                    */

                    build::freeze_project_auto(
                        package_context, &package_path
                    )?;
                }
            }
            RegistryKind::RegistryServer => {
                todo!()
            }
        }

        Ok(())
    };

    let package_registries = project_frozen::publish_registries(
        &package_context.config_frozen.as_ref().unwrap()
    );
    for (reg_name, reg) in package_registries {
        if reg_name == &registry { return auth_fn(reg) }
    }

    let Some(cfg_registry) = config.registries.as_ref().unwrap().get(&registry)
        else
    {
        bail!(
            "No registry '{}' found in package configuration or comline global configuration",
            &registry
        )
    };

    auth_fn(cfg_registry)
}

