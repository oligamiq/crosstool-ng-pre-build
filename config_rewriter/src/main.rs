use std::thread::JoinHandle;

use clean::Clean as _;
use cmd::CanonicalArgs;
use install::Install as _;
use rewrite::RewriteDoc as _;
use toml_edit::{Array, DocumentMut, Item, Value};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

pub mod clean;
pub mod cmd;
pub mod install;
pub mod rewrite;
pub mod targets;
pub mod utils;

// cargo run -- --only-llvm --tier all --os linux --file ./config.toml
// cargo run -- -f config.toml --target aarch64-unknown-linux-gnu --install
// cargo run -- --target aarch64-unknown-linux-gnu --clean
// cargo run -- -f config.toml --tier 1 --os linux --install -t aarch64-unknown-linux-musl
// cargo run -- -f config.toml --tier 1 --tier 2-host --os linux --exclude powerpc64le-unknown-linux-gnu --clean
// cargo run -- -f config.toml --tier 1 --tier 2-host --os linux -t sparcv9-sun-solaris --clean

use color_eyre::eyre::{ContextCompat, Result};

fn main() -> Result<()> {
  tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(
      EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy(),
    )
    .init();

  color_eyre::install()?;

  let CanonicalArgs {
    file,
    targets,
    llvm_targets,
    only_llvm,
    install,
    clean,
    no_cache,
  } = cmd::CanonicalArgs::parse_and_check()?;

  if install || clean {
    let install_or_clean = if install { "installing" } else { "cleaning" };
    #[allow(unused_variables)]
    let warn_msg =
      format!("elevating root permissions because {install_or_clean} toolchains requires root...");

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
      let check = elevate::check();
      match check {
        elevate::RunningAs::Root => {}
        elevate::RunningAs::Suid => {
          log::warn!("{warn_msg}");
          elevate::escalate_if_needed().expect("Failed to acquire root");
        }
        elevate::RunningAs::User => {
          log::warn!("{warn_msg}");
          elevate::escalate_if_needed().expect("Failed to acquire root");
        }
      }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    unimplemented!("install is not implemented yet");
  }

  if !clean && file.is_some() {
    let file = file.as_ref().unwrap();

    let file_data = std::fs::read_to_string(&file)?;
    let mut doc = file_data.parse::<DocumentMut>().expect("invalid doc");

    doc["llvm"]["targets"] = llvm_targets
      .iter()
      .filter(|x| !x.is_experimental())
      .map(|x| x.to_string())
      .fold("".to_string(), |acc, x| {
        if acc.is_empty() {
          x
        } else {
          format!("{};{}", acc, x)
        }
      })
      .into();

    doc["llvm"]["experimental-targets"] = llvm_targets
      .iter()
      .filter(|x| x.is_experimental())
      .map(|x| x.to_string())
      .fold("".to_string(), |acc, x| {
        if acc.is_empty() {
          x
        } else {
          format!("{};{}", acc, x)
        }
      })
      .into();

    if only_llvm {
      std::fs::write(&file, doc.to_string())?;
      return Ok(());
    }

    for target in targets.iter() {
      match target {
        cmd::Target::LinuxTargets(linux_targets) => linux_targets.rewrite_doc(&mut doc)?,
        cmd::Target::WindowsTargets(windows_targets) => todo!(),
        cmd::Target::MacTargets(mac_targets) => todo!(),
      }
    }

    //     match targets {
    //                     OS::Linux => {
    //                         doc["target"]["aarch64-unknown-linux-musl"]["musl-root"] =
    //                             "/musl-aarch64/aarch64-linux-musl".into();
    //                         doc["target"]["loongarch64-unknown-linux-musl"]["musl-root"] = "/x-tools/loongarch64-unknown-linux-musl/loongarch64-unknown-linux-musl/sysroot/usr".into();
    //                         doc["target"]["riscv64gc-unknown-linux-musl"]["musl-root"] =
    //                             "/musl-riscv64gc".into();
    //                         doc["target"]["x86_64-unknown-linux-musl"]["musl-root"] =
    //                             "/musl-x86_64/x86_64-linux-musl".into();
    //                         doc["target"]["arm-unknown-linux-musleabi"]["musl-root"] =
    //                             "/musl-arm/arm-linux-musleabi".into();
    //                         doc["target"]["arm-unknown-linux-musleabihf"]["musl-root"] =
    //                             "/musl-armhf/arm-linux-musleabihf".into();
    //                         doc["target"]["armv5te-unknown-linux-musleabi"]["musl-root"] =
    //                             "/musl-armv5te".into();
    //                         doc["target"]["armv7-unknown-linux-musleabi"]["musl-root"] =
    //                             "/musl-armv7".into();
    //                         doc["target"]["armv7-unknown-linux-musleabihf"]["musl-root"] =
    //                             "/musl-armv7hf".into();
    //                         doc["target"]["i586-unknown-linux-musl"]["musl-root"] = "/musl-i586".into();
    //                         doc["target"]["i686-unknown-linux-musl"]["musl-root"] =
    //                             "/musl-i686/i686-linux-musl".into();
    //                         doc["target"]["wasm32-wasip2"] = doc["target"]["wasm32-wasip1"].clone();

    //                         doc["target"]["riscv64gc-unknown-linux-musl"]["cc"] =
    //                             "riscv64-linux-gnu-gcc".into();
    //                         doc["target"]["riscv64gc-unknown-linux-musl"]["cxx"] =
    //                             "riscv64-linux-gnu-g++".into();
    //                         doc["target"]["riscv64gc-unknown-linux-musl"]["ar"] =
    //                             "riscv64-linux-gnu-ar".into();
    //                         doc["target"]["riscv64gc-unknown-linux-musl"]["linker"] =
    //                             "riscv64-linux-gnu-gcc".into();
    //                     }
    //                     OS::Windows => todo!(),
    //                     OS::Mac => todo!(),
    //                     OS::False => todo!(),
    //                 }
    //             }
    //         }
    //         Targets::WasmAndX64Linux => {
    //             doc["llvm"]["targets"] = "WebAssembly;X86".into();
    //             doc["llvm"]["experimental-targets"] = "".into();

    //             doc["build"]["target"] = vec![
    //                 "wasm32-unknown-unknown",
    //                 "wasm32-wasip1",
    //                 "wasm32-wasip2",
    //                 "wasm32-wasip1-threads",
    //                 "wasm32v1-none",
    //                 "x86_64-unknown-linux-gnu",
    //             ]
    //             .to_item();
    //         }
    //     }

    std::fs::write(file, doc.to_string())?;
  }

  if install {
    let (err_sender, err_receiver) = std::sync::mpsc::channel();

    let check_err = |threads: Vec<JoinHandle<()>>| -> color_eyre::Result<Vec<JoinHandle<()>>> {
      match err_receiver.try_recv() {
        Ok(e) => {
          log::info!("Error occurred, waiting for install threads to finish...");
          log::error!("{:?}", e);
          for thread in threads {
            thread
              .join()
              .map_err(|e| color_eyre::eyre::eyre!("{:?}", e))?;
          }
          Err(e)?
        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
          log::info!("Error occurred, waiting for install threads to finish...");
          log::error!("{:?}", std::sync::mpsc::TryRecvError::Disconnected);
          for thread in threads {
            thread
              .join()
              .map_err(|e| color_eyre::eyre::eyre!("{:?}", e))?;
          }
          Err(std::sync::mpsc::TryRecvError::Disconnected)?
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => Ok(threads),
      }
    };

    let mut threads = vec![];
    for target in &targets {
      threads.push(match target {
        cmd::Target::LinuxTargets(linux_targets) => linux_targets.install(err_sender.clone(), no_cache),
        cmd::Target::WindowsTargets(windows_targets) => todo!(),
        cmd::Target::MacTargets(mac_targets) => todo!(),
      }?);

      threads = check_err(threads)?;
    }

    log::info!("Waiting for install threads to finish...");
    for thread in threads {
      thread.join().expect("Failed to join thread");
    }

    check_err(vec![])?;
  }

  if clean {
    targets
      .iter()
      .map(|x| match x {
        cmd::Target::LinuxTargets(linux_targets) => linux_targets.clean(),
        cmd::Target::WindowsTargets(windows_targets) => todo!(),
        cmd::Target::MacTargets(mac_targets) => todo!(),
      })
      .collect::<color_eyre::Result<Vec<_>>>()?;

    // clean cache
    let cache_dir = "/x-tools/cache";
    if let Err(e) = std::fs::remove_dir_all(cache_dir) {
      match e.kind() {
        std::io::ErrorKind::NotFound => {
          log::warn!("{:?} doesn't exist, skipping", cache_dir);
        }
        _ => Err(e)?,
      }
    }
  }

  Ok(())
}

pub trait ToItem {
  fn to_item(&self) -> Item;
}

impl<T: AsRef<str>> ToItem for Vec<T> {
  fn to_item(&self) -> Item {
    let item: Value = Value::Array(
      self
        .iter()
        .map(|x| x.as_ref().to_string())
        .collect::<Array>(),
    );
    item.into()
  }
}
