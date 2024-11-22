use std::fs::symlink_metadata;

use color_eyre::eyre::{Context, Ok};

use crate::{
  targets::LinuxTargets,
  utils::{get_wasi_sdk_name, TOOLS},
};

pub trait Clean {
  fn clean(&self) -> color_eyre::Result<()>;
}

impl Clean for LinuxTargets {
  fn clean(&self) -> color_eyre::Result<()> {
    match self {
      LinuxTargets::x86_64_unknown_linux_gnu => {
        log::warn!("x86_64_unknown_linux_gnu is the default target, skipping");
      }
      LinuxTargets::aarch64_unknown_fuchsia => {}
      LinuxTargets::aarch64_linux_android => {}
      LinuxTargets::aarch64_unknown_linux_ohos => {}
      LinuxTargets::aarch64_unknown_none_softfloat => {}
      LinuxTargets::aarch64_unknown_none => {}
      LinuxTargets::aarch64_unknown_uefi => {}
      LinuxTargets::arm_linux_androideabi => {}
      LinuxTargets::armebv7r_none_eabi => {}
      LinuxTargets::armebv7r_none_eabihf => {}
      LinuxTargets::armv5te_unknown_linux_gnueabi => {}
      LinuxTargets::armv7_linux_androideabi => {}
      LinuxTargets::armv7_unknown_linux_gnueabi => {}
      LinuxTargets::armv7_unknown_linux_ohos => {}
      LinuxTargets::armv7a_none_eabi => {}
      LinuxTargets::armv7r_none_eabi => {}
      LinuxTargets::armv7r_none_eabihf => {}
      LinuxTargets::i686_linux_android => {}
      LinuxTargets::i686_unknown_freebsd => {}
      LinuxTargets::i686_unknown_uefi => {}
      LinuxTargets::loongarch64_unknown_none => {}
      LinuxTargets::loongarch64_unknown_none_softfloat => {}
      LinuxTargets::nvptx64_nvidia_cuda => {}
      LinuxTargets::riscv32imac_unknown_none_elf => {}
      LinuxTargets::riscv32i_unknown_none_elf => {}
      LinuxTargets::riscv32im_unknown_none_elf => {}
      LinuxTargets::riscv32imc_unknown_none_elf => {}
      LinuxTargets::riscv32imafc_unknown_none_elf => {}
      LinuxTargets::riscv64gc_unknown_none_elf => {}
      LinuxTargets::riscv64imac_unknown_none_elf => {}
      LinuxTargets::sparc64_unknown_linux_gnu => {}
      // LinuxTargets::sparcv9_sun_solaris => {},
      LinuxTargets::thumbv6m_none_eabi => {}
      LinuxTargets::thumbv7em_none_eabi => {}
      LinuxTargets::thumbv7em_none_eabihf => {}
      LinuxTargets::thumbv7m_none_eabi => {}
      LinuxTargets::thumbv7neon_linux_androideabi => {}
      LinuxTargets::thumbv7neon_unknown_linux_gnueabihf => {}
      LinuxTargets::thumbv8m_base_none_eabi => {}
      LinuxTargets::thumbv8m_main_none_eabi => {}
      LinuxTargets::thumbv8m_main_none_eabihf => {}
      LinuxTargets::wasm32_unknown_emscripten => {}
      LinuxTargets::wasm32_unknown_unknown => {}
      LinuxTargets::wasm32_wasip1 | LinuxTargets::wasm32_wasip2 | LinuxTargets::wasm32_wasip1_threads => {
        let sdk_name = get_wasi_sdk_name();
        let path = format!("/x-tools/{sdk_name}");
        let path = std::path::Path::new(&path);
        if !(std::fs::exists(&path)?) {
          log::warn!("{:?} doesn't exist, skipping", path);
        } else {
          std::fs::remove_dir_all(&path).with_context(|| format!("Failed to remove {:?}", path))?;
        }
      }
      LinuxTargets::wasm32v1_none => {}
      LinuxTargets::x86_64_fortanix_unknown_sgx => {}
      LinuxTargets::x86_64_unknown_fuchsia => {}
      LinuxTargets::x86_64_linux_android => {}
      LinuxTargets::x86_64_pc_solaris => {}
      LinuxTargets::x86_64_unknown_linux_gnux32 => {}
      LinuxTargets::x86_64_unknown_linux_ohos => {}
      LinuxTargets::x86_64_unknown_none => {}
      LinuxTargets::x86_64_unknown_redox => {}
      LinuxTargets::x86_64_unknown_uefi => {}
      _ => {
        let name = self.to_name();
        let tool_prefix = name.replace("-unknown-", "-");
        for tool in TOOLS {
          let path = format!("/usr/local/bin/{tool_prefix}-{tool}");
          let path = std::path::Path::new(&path);
          if !(std::fs::exists(&path)?) {
            log::warn!("{:?} doesn't exist, skipping", path);
            continue;
          }
          let metadata = symlink_metadata(path)
            .with_context(|| format!("Failed to get metadata for {:?}", path))?;
          if metadata.is_symlink() {
            std::fs::remove_file(&path)
              .with_context(|| format!("Failed to remove symlink {:?}", path))?;
          } else {
            log::warn!(
              "{:?} is not a symlink, it isn't installed by this script so it won't be removed",
              path
            );
          }
        }
        let toolchain_dir = format!("/x-tools/{}", name);
        if let Err(_) = std::fs::remove_dir_all(&toolchain_dir) {
          log::warn!("{:?} doesn't exist, skipping", toolchain_dir);
        }

        // check if the toolchain is installed
        check_install_cmd(&tool_prefix)?;
      }
    }

    Ok(())
  }
}

pub fn check_install_cmd(tool_prefix: &str) -> color_eyre::Result<()> {
  for tool in TOOLS.iter() {
    let cmd = format!("{tool_prefix}-{tool}");
    if let Result::Ok(path) = which::which(&cmd) {
      if path == std::path::PathBuf::from(format!("/usr/local/bin/{tool_prefix}-{tool}")) {
        panic!("Clean failed, toolchain is still installed");
      } else {
        log::warn!("{:?} isn't installed by this script, skipping", path);
      }
    }
  }

  Ok(())
}
