use toml_edit::{DocumentMut, Item};
use tracing::instrument;

use crate::targets::LinuxTargets;

pub trait RewriteDoc {
  fn rewrite_doc(&self, doc: &mut DocumentMut) -> color_eyre::Result<()>;
}

impl RewriteDoc for LinuxTargets {
  fn rewrite_doc(&self, doc: &mut DocumentMut) -> color_eyre::Result<()> {
    match self {
      LinuxTargets::x86_64_unknown_linux_gnu => {
        log::warn!("x86_64_unknown_linux_gnu is the default target, skipping");
      },
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
        inner_table(doc, "target", &name);
        let target = &mut doc["target"][&name];
        let prefix = name.replace("-unknown-", "-");
        let place = format!("target.{name}");
        target.check_and_rewrite(&place, "cc", format!("{}-gcc", prefix).into())?;
        target.check_and_rewrite(&place, "cxx", format!("{}-g++", prefix).into())?;
        target.check_and_rewrite(&place, "ar", format!("{}-ar", prefix).into())?;
        target.check_and_rewrite(&place, "ranlib", format!("{}-ranlib", prefix).into())?;
        target.check_and_rewrite(&place, "linker", format!("{}-gcc", prefix).into())?;
      }
    }

    Ok(())
  }
}

pub fn inner_table(doc: &mut DocumentMut, key: &str, second_key: &str) {
  if let Some(table) = doc.get(key) {
    if table.get(second_key).is_none() {
      doc[key][second_key] = toml_edit::Item::Table(toml_edit::Table::new());
    }
  } else {
    let mut table = toml_edit::Table::new();
    table.set_implicit(true);
    table.insert(second_key, toml_edit::Item::Table(toml_edit::Table::new()));
    doc[key] = toml_edit::Item::Table(table);
  }
}

pub trait CheckAndRewrite {
  fn check_and_rewrite(
    &mut self,
    place: &str,
    key: &str,
    item: toml_edit::Item,
  ) -> color_eyre::Result<()>;
}

impl CheckAndRewrite for Item {
  fn check_and_rewrite(
    &mut self,
    place: &str,
    key: &str,
    item: toml_edit::Item,
  ) -> color_eyre::Result<()> {
    if self.get(key).is_none() {
      self[key] = item;
    } else {
      log::warn!("[{place}]: {key} already exists, but will be overwritten");
      self[key] = item;
    }

    Ok(())
  }
}
