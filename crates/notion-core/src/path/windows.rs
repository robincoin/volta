use std::path::PathBuf;

use winfolder;

use notion_fail::{Fallible, FailExt};

use super::UnknownSystemFolderError;

// These are taken from: https://nodejs.org/dist/index.json and are used
// by `path::archive_root_dir` to determine the root directory of the
// contents of a Node installer archive.

pub const OS: &'static str = "win";

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub const ARCH: &'static str = "x86";
    } else if #[cfg(target_arch = "x86_64")] {
        pub const ARCH: &'static str = "x64";
    } else {
        compile_error!("Unsupported target_arch variant of Windows (expected 'x86' or 'x64').");
    }
}

// C:\
//     ProgramData\
//         Notion\
//             cache\                                  cache_dir
//                 node\                               node_cache_dir
//                     node-v4.8.4-win-x64.zip         archive_file("4.8.4")
//                     node-v6.11.3-win-x64.zip
//                     node-v8.6.0-win-x64.zip
//                     ...
//             versions\                               versions_dir
//                 node\                               node_versions_dir
//                     4.8.4\                          node_version_dir("4.8.4")
//                                                     node_version_bin_dir("4.8.4")
//                     6.11.3\
//                     8.6.0\
//                     ...
//             launchbin.exe                           launchbin_file
//             launchscript.exe                        launchscript_file

fn program_data_root() -> Fallible<PathBuf> {
    let pd = winfolder::known_path(&winfolder::id::PROGRAM_DATA)
        .ok_or_else(|| {
            UnknownSystemFolderError {
                name: "PROGRAM_DATA"
            }.unknown()
        })?;
    Ok(pd.join("Notion"))
}

pub fn cache_dir() -> Fallible<PathBuf> {
    Ok(program_data_root()?.join("cache"))
}

pub fn node_cache_dir() -> Fallible<PathBuf> {
    Ok(cache_dir()?.join("node"))
}

pub fn archive_extension() -> String {
    String::from("zip")
}

pub fn versions_dir() -> Fallible<PathBuf> {
    Ok(program_data_root()?.join("versions"))
}

pub fn node_versions_dir() -> Fallible<PathBuf> {
    Ok(versions_dir()?.join("node"))
}

pub fn node_version_dir(version: &str) -> Fallible<PathBuf> {
    Ok(node_versions_dir()?.join(version))
}

pub fn node_version_bin_dir(version: &str) -> Fallible<PathBuf> {
    node_version_dir(version)
}

pub fn launchbin_file() -> Fallible<PathBuf> {
    Ok(program_data_root()?.join("launchbin.exe"))
}

pub fn launchscript_file() -> Fallible<PathBuf> {
    Ok(program_data_root()?.join("launchscript.exe"))
}

// C:\
//     Program Files\
//         Notion\                                     bin_dir
//             notion.exe                              notion_file
//             shim\                                   shim_dir
//                 node.exe                            shim_file("node")
//                 npm.exe
//                 npx.exe
//                 ...

fn program_files_root() -> Fallible<PathBuf> {
    let pf = winfolder::known_path(&winfolder::id::PROGRAM_FILES_X64)
        .ok_or_else(|| {
            UnknownSystemFolderError {
                name: "PROGRAM_FILES_X64"
            }
        })?;
    Ok(pf.join("Notion"))
}

pub fn bin_dir() -> Fallible<PathBuf> {
    program_files_root()
}

pub fn notion_file() -> Fallible<PathBuf> {
    Ok(bin_dir()?.join("notion.exe"))
}

pub fn shim_dir() -> Fallible<PathBuf> {
    Ok(program_files_root()?.join("shim"))
}

pub fn shim_file(toolname: &str) -> Fallible<PathBuf> {
    Ok(shim_dir()?.join(&format!("{}.exe", toolname)))
}

// C:\
//     Users\
//         dherman\
//             AppData\
//                 Local\
//                     Notion\
//                         config.toml                 user_config_file
//                         catalog.toml                user_catalog_file

fn local_data_root() -> Fallible<PathBuf> {
    let adl = winfolder::known_path(&winfolder::id::LOCAL_APP_DATA)
        .ok_or_else(|| {
            UnknownSystemFolderError {
                name: "LOCAL_APP_DATA"
            }
        })?;
    Ok(adl.join("Notion"))
}

pub fn user_config_file() -> Fallible<PathBuf> {
    Ok(local_data_root()?.join("config.toml"))
}

pub fn user_catalog_file() -> Fallible<PathBuf> {
    Ok(local_data_root()?.join("catalog.toml"))
}
