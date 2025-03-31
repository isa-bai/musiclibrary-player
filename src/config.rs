use std::{env, fs::{self, File}, io::Write, path::PathBuf, sync::LazyLock};

use toml_edit::DocumentMut;

const DEFAULT_LIB_PATH: &str = "./";

const DEFAULT_DP_ENABLED: bool = true;
const DEFAULT_DP_CLIENTID: &str = "1354688246654697712";


const DEFAULT_WS_ENABLED: bool = true;
const DEFAULT_WS_PORT: u32 = 31466;


const DEFAULT_CFG: LazyLock<String> = LazyLock::new(|| {
    format!(r#"[library]
path="{DEFAULT_LIB_PATH}"

[websocket]
enabled={DEFAULT_WS_ENABLED}
port={DEFAULT_WS_PORT}

[discord]
enabled={DEFAULT_DP_ENABLED}
clientid="{DEFAULT_DP_CLIENTID}""#)
});

pub static PROGRAM_CFG: LazyLock<ProgramConfig> = LazyLock::new(|| {
    ProgramConfig::init()
});



#[derive(Debug)]
struct DiscordOptions {
    enabled: bool,
    client_id: String
}

impl Default for DiscordOptions {
    fn default() -> Self {
        Self { 
            enabled: DEFAULT_DP_ENABLED,
            client_id: String::from(DEFAULT_DP_CLIENTID)
        }
    }
}

#[derive(Debug)]
struct LibraryOptions {
    path: String,
}

impl Default for LibraryOptions {
    fn default() -> Self {
        Self {
            path: String::from(DEFAULT_LIB_PATH)
        }
    }
}

#[derive(Debug)]
struct WebsocketOptions {
    enabled: bool,
    port: u32
}

impl Default for WebsocketOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_WS_ENABLED,
            port: DEFAULT_WS_PORT
        }
    }
}

#[derive(Default, Debug)]
pub struct ProgramConfig {
    library_opts: LibraryOptions,
    discord_opts: DiscordOptions,
    websocket_opts: WebsocketOptions,    
}

impl ProgramConfig {

    fn init() -> ProgramConfig {
        let mut config = ProgramConfig::default();
        let mut fp: Option<PathBuf> = None;
        if let Ok(current_path) = env::current_exe() {
            if let Some(dir) = current_path.parent() {
                fp = Some(dir.join("./config.toml"));
            }
        }

        let Some(filepath) = fp else {
            //could not get path to executable, return default config
            return config;
        };

        if !filepath.exists() {
            if let Ok(mut file) = File::create(filepath) {
                _ = file.write_all(DEFAULT_CFG.as_bytes());
            }
            //config file does not exist, try create default config and return default config
            return config;
        }
        let Ok(file) = fs::read_to_string(filepath) else {
            //error reading file, return default config
            return config;
        };

        let doc;
        match file.parse::<DocumentMut>() {
            Ok(d) => doc = d,
            Err(_) => {
                //error parsing config file, return default config
                return config;
            }
        }

        // library options
        if let Some(library_opts) = doc.get("library").and_then(|item| item.as_table()) {
            for (k, v) in library_opts.iter() {
                match k {
                    "path" => {
                        if v.is_str() {
                            if let Some(path) = v.as_str() {config.library_opts.path = String::from(path)};
                        }
                    },
                    _ => {}
                }
            }
        }

        // discord rich presence options
        if let Some(discord_opts) = doc.get("discord").and_then(|item| item.as_table()) {
            for (k, v) in discord_opts.iter() {
                match k {
                    "enabled" => {
                        if v.is_bool() {
                            if let Some(enabled) = v.as_bool() {config.discord_opts.enabled = enabled};
                        }
                    },
                    "clientid" => {
                        if v.is_str() {
                            if let Some(id) = v.as_str() {config.discord_opts.client_id = String::from(id)};
                        }
                    },
                    _ => {}
                }
            }
        }

        // websocket options
        if let Some(websocket_opts) = doc.get("websocket").and_then(|item| item.as_table()) {
            for (k, v) in websocket_opts.iter() {
                match k {
                    "enabled" => {
                        if v.is_bool() {
                            if let Some(enabled) = v.as_bool() {config.websocket_opts.enabled = enabled};
                        }
                    },
                    "port" => {
                        if v.is_integer() {
                            match v.as_integer() {
                                Some(val) => {
                                    if val <= u32::MAX.into() && val >= u32::MIN.into() {config.websocket_opts.port = val as u32};
                                },
                                None => {}
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        config

    }

}


