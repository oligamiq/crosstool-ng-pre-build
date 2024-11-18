use std::fs::{canonicalize, symlink_metadata};

use color_eyre::eyre::Context;

use crate::{targets::LinuxTargets, utils::TOOLS};

pub trait Clean {
  fn clean(&self) -> color_eyre::Result<()>;
}

impl Clean for LinuxTargets {
  fn clean(&self) -> color_eyre::Result<()> {
    match self {
      LinuxTargets::i686_unknown_linux_gnu => todo!(),
      LinuxTargets::x86_64_unknown_linux_gnu => todo!(),
      LinuxTargets::aarch64_unknown_linux_musl => todo!(),
      LinuxTargets::arm_unknown_linux_gnueabi => todo!(),
      LinuxTargets::arm_unknown_linux_gnueabihf => todo!(),
      LinuxTargets::armv7_unknown_linux_gnueabihf => todo!(),
      LinuxTargets::loongarch64_unknown_linux_gnu => todo!(),
      LinuxTargets::loongarch64_unknown_linux_musl => todo!(),
      LinuxTargets::powerpc_unknown_linux_gnu => todo!(),
      LinuxTargets::powerpc64_unknown_linux_gnu => todo!(),
      LinuxTargets::powerpc64le_unknown_linux_gnu => todo!(),
      LinuxTargets::riscv64gc_unknown_linux_gnu => todo!(),
      LinuxTargets::riscv64gc_unknown_linux_musl => todo!(),
      LinuxTargets::s390x_unknown_linux_gnu => todo!(),
      LinuxTargets::x86_64_unknown_freebsd => todo!(),
      LinuxTargets::x86_64_unknown_illumos => todo!(),
      LinuxTargets::x86_64_unknown_linux_musl => todo!(),
      LinuxTargets::x86_64_unknown_netbsd => todo!(),
      LinuxTargets::aarch64_unknown_fuchsia => todo!(),
      LinuxTargets::aarch64_linux_android => todo!(),
      LinuxTargets::aarch64_unknown_linux_ohos => todo!(),
      LinuxTargets::aarch64_unknown_none_softfloat => todo!(),
      LinuxTargets::aarch64_unknown_none => todo!(),
      LinuxTargets::aarch64_unknown_uefi => todo!(),
      LinuxTargets::arm_linux_androideabi => todo!(),
      LinuxTargets::arm_unknown_linux_musleabi => todo!(),
      LinuxTargets::arm_unknown_linux_musleabihf => todo!(),
      LinuxTargets::armebv7r_none_eabi => todo!(),
      LinuxTargets::armebv7r_none_eabihf => todo!(),
      LinuxTargets::armv5te_unknown_linux_gnueabi => todo!(),
      LinuxTargets::armv5te_unknown_linux_musleabi => todo!(),
      LinuxTargets::armv7_linux_androideabi => todo!(),
      LinuxTargets::armv7_unknown_linux_gnueabi => todo!(),
      LinuxTargets::armv7_unknown_linux_musleabi => todo!(),
      LinuxTargets::armv7_unknown_linux_musleabihf => todo!(),
      LinuxTargets::armv7_unknown_linux_ohos => todo!(),
      LinuxTargets::armv7a_none_eabi => todo!(),
      LinuxTargets::armv7r_none_eabi => todo!(),
      LinuxTargets::armv7r_none_eabihf => todo!(),
      LinuxTargets::i586_unknown_linux_gnu => todo!(),
      LinuxTargets::i586_unknown_linux_musl => todo!(),
      LinuxTargets::i686_linux_android => todo!(),
      LinuxTargets::i686_unknown_freebsd => todo!(),
      LinuxTargets::i686_unknown_linux_musl => todo!(),
      LinuxTargets::i686_unknown_uefi => todo!(),
      LinuxTargets::loongarch64_unknown_none => todo!(),
      LinuxTargets::loongarch64_unknown_none_softfloat => todo!(),
      LinuxTargets::nvptx64_nvidia_cuda => todo!(),
      LinuxTargets::riscv32imac_unknown_none_elf => todo!(),
      LinuxTargets::riscv32i_unknown_none_elf => todo!(),
      LinuxTargets::riscv32im_unknown_none_elf => todo!(),
      LinuxTargets::riscv32imc_unknown_none_elf => todo!(),
      LinuxTargets::riscv32imafc_unknown_none_elf => todo!(),
      LinuxTargets::riscv64gc_unknown_none_elf => todo!(),
      LinuxTargets::riscv64imac_unknown_none_elf => todo!(),
      LinuxTargets::sparc64_unknown_linux_gnu => todo!(),
      LinuxTargets::sparcv9_sun_solaris => todo!(),
      LinuxTargets::thumbv6m_none_eabi => todo!(),
      LinuxTargets::thumbv7em_none_eabi => todo!(),
      LinuxTargets::thumbv7em_none_eabihf => todo!(),
      LinuxTargets::thumbv7m_none_eabi => todo!(),
      LinuxTargets::thumbv7neon_linux_androideabi => todo!(),
      LinuxTargets::thumbv7neon_unknown_linux_gnueabihf => todo!(),
      LinuxTargets::thumbv8m_base_none_eabi => todo!(),
      LinuxTargets::thumbv8m_main_none_eabi => todo!(),
      LinuxTargets::thumbv8m_main_none_eabihf => todo!(),
      LinuxTargets::wasm32_unknown_emscripten => todo!(),
      LinuxTargets::wasm32_unknown_unknown => todo!(),
      LinuxTargets::wasm32_wasip1 => todo!(),
      LinuxTargets::wasm32_wasip2 => todo!(),
      LinuxTargets::wasm32_wasip1_threads => todo!(),
      LinuxTargets::wasm32v1_none => todo!(),
      LinuxTargets::x86_64_fortanix_unknown_sgx => todo!(),
      LinuxTargets::x86_64_unknown_fuchsia => todo!(),
      LinuxTargets::x86_64_linux_android => todo!(),
      LinuxTargets::x86_64_pc_solaris => todo!(),
      LinuxTargets::x86_64_unknown_linux_gnux32 => todo!(),
      LinuxTargets::x86_64_unknown_linux_ohos => todo!(),
      LinuxTargets::x86_64_unknown_none => todo!(),
      LinuxTargets::x86_64_unknown_redox => todo!(),
      LinuxTargets::x86_64_unknown_uefi => todo!(),
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
      }
    }

    Ok(())
  }
}
