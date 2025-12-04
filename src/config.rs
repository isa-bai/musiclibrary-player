use std::{env, fs::{self, File}, io::Write, path::PathBuf, sync::LazyLock};

use toml_edit::DocumentMut;

const DEFAULT_LIB_PATH: &str = "./";
const DEFAULT_IMG_SIZE: u32 = 120;
const DEFAULT_STRIP_REMASTERED: bool = false;

const DEFAULT_DP_ENABLED: bool = false;
pub const DEFAULT_DP_CLIENTID: &str = "1354688246654697712";


const DEFAULT_WS_ENABLED: bool = false;
pub const DEFAULT_WS_PORT: u16 = 31466;


const DEFAULT_CFG: LazyLock<String> = LazyLock::new(|| {
    format!(r#"[library]
path="{DEFAULT_LIB_PATH}"
image_size="{DEFAULT_IMG_SIZE}"
strip_remastered={DEFAULT_STRIP_REMASTERED}

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
struct LibraryOptions {
    path: String,
    imgsize: u32,
    strip_remastered: bool
}

impl Default for LibraryOptions {
    fn default() -> Self {
        Self {
            path: String::from(DEFAULT_LIB_PATH),
            imgsize: DEFAULT_IMG_SIZE,
            strip_remastered: DEFAULT_STRIP_REMASTERED
        }
    }
}

#[derive(Debug)]
struct WebsocketOptions {
    enabled: bool,
    port: u16
}

impl Default for WebsocketOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_WS_ENABLED,
            port: DEFAULT_WS_PORT
        }
    }
}

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
                    "imgsize" => {
                        if v.is_integer() {
                            let mut v = v.as_integer().unwrap();
                            if v < 20 {
                                v = 20;
                            } else {
                                if v > 300 {
                                    v = 300;
                                }
                            }
                            config.library_opts.imgsize = v as u32;
                        }
                    },
                    "strip_remastered" => {
                        if v.is_bool() {
                            if let Some(strip) = v.as_bool() {config.library_opts.strip_remastered = strip};
                        }
                    }
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
                                    if val <= u16::MAX.into() && val >= u16::MIN.into() {config.websocket_opts.port = val as u16};
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

    pub fn library_path(&self) -> &str {
        &self.library_opts.path
    }

    pub fn img_size(&self) -> u32 {
        self.library_opts.imgsize
    }

    pub fn strip_remastered(&self) -> bool {
        self.library_opts.strip_remastered
    }

    pub fn discord_rp_enabled(&self) -> bool {
        self.discord_opts.enabled
    }

    pub fn discord_client_id(&self) -> &str {
        &self.discord_opts.client_id
    }

    pub fn ws_enabled(&self) -> bool {
        self.websocket_opts.enabled
    }

    pub fn ws_port(&self) -> &u16 {
        &self.websocket_opts.port
    }

}


