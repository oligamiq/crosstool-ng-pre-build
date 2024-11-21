use toml_edit::{DocumentMut, Item};

use crate::{install::check_musl_libc, musl_root, targets::LinuxTargets};

pub trait RewriteDoc {
  fn rewrite_doc(&self, doc: &mut DocumentMut) -> color_eyre::Result<()>;
}

impl RewriteDoc for LinuxTargets {
  fn rewrite_doc(&self, doc: &mut DocumentMut) -> color_eyre::Result<()> {
    match self {
      LinuxTargets::x86_64_unknown_linux_gnu => {
        log::warn!("x86_64_unknown_linux_gnu is the default target, skipping");
      }
      // LinuxTargets::x86_64_unknown_freebsd => {
      //   let name = self.to_name();
      //   inner_table(doc, "target", &name);
      //   let target = &mut doc["target"][&name];
      //   let place = format!("target.{name}");
      //   target.check_and_rewrite(&place, "ar", format!("x86_64-unknown-freebsd12-ar").into())?;
      //   target.check_and_rewrite(
      //     &place,
      //     "cc",
      //     format!("x86_64-unknown-freebsd12-clang").into(),
      //   )?;
      //   target.check_and_rewrite(
      //     &place,
      //     "cxx",
      //     format!("x86_64-unknown-freebsd12-clang++").into(),
      //   )?;
      // }
      LinuxTargets::aarch64_unknown_fuchsia => {}
      LinuxTargets::aarch64_linux_android => {}
      LinuxTargets::aarch64_unknown_linux_ohos => {}
      LinuxTargets::aarch64_unknown_none_softfloat => {}
      LinuxTargets::aarch64_unknown_none => {}
      LinuxTargets::aarch64_unknown_uefi => {}
      LinuxTargets::arm_linux_androideabi => {}
      LinuxTargets::arm_unknown_linux_musleabi => {}
      LinuxTargets::arm_unknown_linux_musleabihf => {}
      LinuxTargets::armebv7r_none_eabi => {}
      LinuxTargets::armebv7r_none_eabihf => {}
      LinuxTargets::armv5te_unknown_linux_gnueabi => {}
      LinuxTargets::armv5te_unknown_linux_musleabi => {}
      LinuxTargets::armv7_linux_androideabi => {}
      LinuxTargets::armv7_unknown_linux_gnueabi => {}
      LinuxTargets::armv7_unknown_linux_musleabi => {}
      LinuxTargets::armv7_unknown_linux_musleabihf => {}
      LinuxTargets::armv7_unknown_linux_ohos => {}
      LinuxTargets::armv7a_none_eabi => {}
      LinuxTargets::armv7r_none_eabi => {}
      LinuxTargets::armv7r_none_eabihf => {}
      LinuxTargets::i586_unknown_linux_gnu => {}
      LinuxTargets::i586_unknown_linux_musl => {}
      LinuxTargets::i686_linux_android => {}
      LinuxTargets::i686_unknown_freebsd => {}
      LinuxTargets::i686_unknown_linux_musl => {}
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
      LinuxTargets::wasm32_wasip1 => {}
      LinuxTargets::wasm32_wasip2 => {}
      LinuxTargets::wasm32_wasip1_threads => {}
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
        inner_table(doc, "target", &name);
        let target = &mut doc["target"][&name];
        let prefix = name.replace("-unknown-", "-");
        let place = format!("target.{name}");

        let mut crosstool_ng = || -> color_eyre::Result<()> {
          target.check_and_rewrite(&place, "cc", format!("{prefix}-gcc").into())?;
          target.check_and_rewrite(&place, "cxx", format!("{prefix}-g++").into())?;
          target.check_and_rewrite(&place, "ar", format!("{prefix}-ar").into())?;
          target.check_and_rewrite(&place, "ranlib", format!("{prefix}-ranlib").into())?;
          target.check_and_rewrite(&place, "linker", format!("{prefix}-gcc").into())?;
          Ok(())
        };

        if name.find("musl").is_some() {
          if musl_root::musl_require_root().contains(&name.as_str()) {
            log::warn!("musl libc detected, using musl-root");

            target.check_and_rewrite(&place, "musl-root", format!("/x-tools/{name}").into())?;
            let gnu_prefix = prefix.replace("musl", "gnu");
            target.check_and_rewrite(&place, "cc", format!("{gnu_prefix}-gcc").into())?;
            target.check_and_rewrite(&place, "cxx", format!("{gnu_prefix}-g++").into())?;
            target.check_and_rewrite(&place, "ar", format!("{gnu_prefix}-ar").into())?;
            target.check_and_rewrite(&place, "ranlib", format!("{gnu_prefix}-ranlib").into())?;
            target.check_and_rewrite(&place, "linker", format!("{gnu_prefix}-gcc").into())?;
          } else {
            crosstool_ng()?;
            target.check_and_rewrite(
              &place,
              "musl-root",
              format!("/x-tools/{name}/{prefix}").into(),
            )?;
          }
        } else {
          crosstool_ng()?;
        }
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
