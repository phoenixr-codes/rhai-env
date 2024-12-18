use rhai::plugin::*;

#[export_module]
#[allow(clippy::ptr_arg)]
pub mod env_module {
    use std::env;

    // NOTE: `pub use` cannot be used as it won't be exported. See also: https://rhai.rs/book/plugins/module.html

    /// A string describing the architecture of the CPU that is currently in use. An example value
    /// may be: `"x86"`, `"arm"` or `"riscv64"`.
    pub const ARCH: &str = env::consts::ARCH;

    /// Specifies the file extension, if any, used for shared libraries on this platform that goes
    /// after the dot. An example value may be: `"so"`, `"elf"`, or `"dll"`.
    pub const DLL_EXTENSION: &str = env::consts::DLL_EXTENSION;

    /// Specifies the filename prefix, if any, used for shared libraries on this platform. This is
    /// either `"lib"` or an empty string (`""`).
    pub const DLL_PREFIX: &str = env::consts::DLL_PREFIX;

    /// Specifies the filename suffix, if any, used for shared libraries on this platform. An
    /// example value may be: `".so"`, `".elf"`, or `".dll"`.
    pub const DLL_SUFFIX: &str = env::consts::DLL_SUFFIX;

    /// Specifies the file extension, if any, used for executable binaries on this platform. An
    /// example value may be: `"exe"`, or an empty string (`""`).
    pub const EXE_EXTENSION: &str = env::consts::EXE_EXTENSION;

    /// Specifies the filename suffix, if any, used for executable binaries on this platform. An
    /// example value may be: `".exe"`, or `".efi"`.
    pub const EXE_SUFFIX: &str = env::consts::EXE_SUFFIX;

    /// A string describing the family of the operating system. An example value may be: `"unix"`,
    /// or `"windows"`.
    pub const FAMILY: &str = env::consts::FAMILY;

    /// A string describing the specific operating system in use. An example value may be:
    /// `"linux"`, or `"freebsd"`.
    pub const OS: &str = env::consts::OS;

    /// Fetches the environment variable `key` from the current process.
    #[rhai_fn(return_raw)]
    pub fn env(key: &str) -> Result<String, Box<EvalAltResult>> {
        env::var(key).map_err(|e| e.to_string().into())
    }

    /// Sets the environment variable `key` to the `value` value for the currently running process.
    pub fn set_env(key: &str, value: &str) {
        env::set_var(key, value);
    }

    /// Fetches all environment variables.
    pub fn envs() -> rhai::Map {
        let mut map = rhai::Map::new();
        for (key, value) in env::vars_os() {
            if !key.is_ascii() {
                continue;
            }
            map.insert(
                key.to_str().unwrap().into(),
                Dynamic::from(value.to_string_lossy().into_owned()),
            );
        }
        map
    }
}
