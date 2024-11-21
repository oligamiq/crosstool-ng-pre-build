use std::{
  fs::create_dir_all, io::Stdout, process::Command, sync::mpsc::Sender, thread::JoinHandle,
};

use pbr::ProgressBar;

use crate::{musl_root, targets::LinuxTargets};

const CONTENT_VERSION: &str = "v0.1.0";

pub trait Install {
  fn install(
    &self,
    error_sender: Sender<color_eyre::Report>,
    no_cache: bool,
  ) -> color_eyre::Result<JoinHandle<()>>;
}

impl Install for LinuxTargets {
  fn install(
    &self,
    error_sender: Sender<color_eyre::Report>,
    no_cache: bool,
  ) -> color_eyre::Result<JoinHandle<()>> {
    let sl = self.clone();

    let (sender, receiver) = std::sync::mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
      fn install_inner(
        sl: &LinuxTargets,
        sender: Sender<()>,
        no_cache: bool,
      ) -> color_eyre::Result<()> {
        #[cfg(target_os = "linux")]
        match sl {
          LinuxTargets::x86_64_unknown_linux_gnu => {
            log::warn!("x86_64_unknown_linux_gnu is the default target, skipping");
            sender.send(())?;
          }
          LinuxTargets::aarch64_unknown_fuchsia => {},
          LinuxTargets::aarch64_linux_android => {},
          LinuxTargets::aarch64_unknown_linux_ohos => {},
          LinuxTargets::aarch64_unknown_none_softfloat => {},
          LinuxTargets::aarch64_unknown_none => {},
          LinuxTargets::aarch64_unknown_uefi => {},
          LinuxTargets::arm_linux_androideabi => {},
          LinuxTargets::arm_unknown_linux_musleabi => {},
          LinuxTargets::arm_unknown_linux_musleabihf => {},
          LinuxTargets::armebv7r_none_eabi => {},
          LinuxTargets::armebv7r_none_eabihf => {},
          LinuxTargets::armv5te_unknown_linux_gnueabi => {},
          LinuxTargets::armv5te_unknown_linux_musleabi => {},
          LinuxTargets::armv7_linux_androideabi => {},
          LinuxTargets::armv7_unknown_linux_gnueabi => {},
          LinuxTargets::armv7_unknown_linux_musleabi => {},
          LinuxTargets::armv7_unknown_linux_musleabihf => {},
          LinuxTargets::armv7_unknown_linux_ohos => {},
          LinuxTargets::armv7a_none_eabi => {},
          LinuxTargets::armv7r_none_eabi => {},
          LinuxTargets::armv7r_none_eabihf => {},
          LinuxTargets::i586_unknown_linux_gnu => {},
          LinuxTargets::i586_unknown_linux_musl => {},
          LinuxTargets::i686_linux_android => {},
          LinuxTargets::i686_unknown_freebsd => {},
          LinuxTargets::i686_unknown_linux_musl => {},
          LinuxTargets::i686_unknown_uefi => {},
          LinuxTargets::loongarch64_unknown_none => {},
          LinuxTargets::loongarch64_unknown_none_softfloat => {},
          LinuxTargets::nvptx64_nvidia_cuda => {},
          LinuxTargets::riscv32imac_unknown_none_elf => {},
          LinuxTargets::riscv32i_unknown_none_elf => {},
          LinuxTargets::riscv32im_unknown_none_elf => {},
          LinuxTargets::riscv32imc_unknown_none_elf => {},
          LinuxTargets::riscv32imafc_unknown_none_elf => {},
          LinuxTargets::riscv64gc_unknown_none_elf => {},
          LinuxTargets::riscv64imac_unknown_none_elf => {},
          LinuxTargets::sparc64_unknown_linux_gnu => {},
          // LinuxTargets::sparcv9_sun_solaris => {},
          LinuxTargets::thumbv6m_none_eabi => {},
          LinuxTargets::thumbv7em_none_eabi => {},
          LinuxTargets::thumbv7em_none_eabihf => {},
          LinuxTargets::thumbv7m_none_eabi => {},
          LinuxTargets::thumbv7neon_linux_androideabi => {},
          LinuxTargets::thumbv7neon_unknown_linux_gnueabihf => {},
          LinuxTargets::thumbv8m_base_none_eabi => {},
          LinuxTargets::thumbv8m_main_none_eabi => {},
          LinuxTargets::thumbv8m_main_none_eabihf => {},
          LinuxTargets::wasm32_unknown_emscripten => {},
          LinuxTargets::wasm32_unknown_unknown => {},
          LinuxTargets::wasm32_wasip1 => {},
          LinuxTargets::wasm32_wasip2 => {},
          LinuxTargets::wasm32_wasip1_threads => {},
          LinuxTargets::wasm32v1_none => {},
          LinuxTargets::x86_64_fortanix_unknown_sgx => {},
          LinuxTargets::x86_64_unknown_fuchsia => {},
          LinuxTargets::x86_64_linux_android => {},
          LinuxTargets::x86_64_pc_solaris => {},
          LinuxTargets::x86_64_unknown_linux_gnux32 => {},
          LinuxTargets::x86_64_unknown_linux_ohos => {},
          LinuxTargets::x86_64_unknown_none => {},
          LinuxTargets::x86_64_unknown_redox => {},
          LinuxTargets::x86_64_unknown_uefi => {},
          _ => {
            let name = sl.to_name();
            let url = format!("https://github.com/oligamiq/toolchain-for-building-rustc/releases/download/{CONTENT_VERSION}-release/{name}.tar.gz");
            let body = download_from_url(&url)?;

            sender.send(())?;

            if no_cache {
              log::info!("Skipping cache saving");
            } else {
              save_cache(&body, &format!("{name}.tar.gz"))?;
            }
            normal::unpack_tarball_archive(body, &name)?;

            let crosstool_ng = || -> color_eyre::Result<()> {
              let tool_prefix = name.replace("-unknown-", "-");
              log::info!("Installing toolchain for {}", name);
              normal::install_toolchain(&format!("/x-tools/{name}/bin/"), &tool_prefix)?;
              log::info!("Checking if the toolchain[{name}] is installed correctly");
              normal::check_install_cmd(&tool_prefix)?;
              log::info!("Checking if the toolchain[{name}] can compile a simple C program");
              normal::check_compile_test_c(&tool_prefix)?;
              log::info!("Checking if the toolchain[{name}] can compile a simple C++ program");
              normal::check_compile_test_cpp(&tool_prefix)?;

              Ok(())
            };

            if name.find("musl").is_some() {
              if check_musl_libc(&format!("/x-tools/{name}/lib/"))? {
                if musl_root::musl_require_root().contains(&name.as_str()) {
                  log::warn!("We currently do not support compile checks using sysroot, so we will skip the compile check");
                  // let tool_prefix = name.replace("-unknown-", "-");
                  // let tool_prefix = tool_prefix.replace("musl", "gnu");

                  // todo!()
                } else {
                  unreachable!("musl libc detected, using musl-root");
                }
              } else {
                let prefix = name.replace("-unknown-", "-");
                if !(check_musl_libc(&format!("/x-tools/{name}/{prefix}/lib/"))?) {
                  Err(color_eyre::eyre::eyre!("Failed to find musl libc: {}", name))?;
                }
                crosstool_ng()?;
              }
            } else {
              crosstool_ng()?;
            }
          }
        }

        Ok(())
      }

      if let Err(e) = install_inner(&sl, sender, no_cache) {
        error_sender.send(e).unwrap();
      }
    });

    // If an error occurs here, this line will panic and swallow the error, so be careful.
    receiver.recv()?;

    Ok(thread)
  }
}

fn install_fuchsia(arch: &str, sender: Sender<()>) -> color_eyre::Result<()> {
  #[allow(non_camel_case_types)]
  #[derive(strum::EnumString)]
  enum FuchsiaArch {
    aarch64,
    x86_64,
  }
  let fuchsia_arch = arch.parse::<FuchsiaArch>()?;

  let name = "xxxxxx-unknown-fuchsia";
  let url = format!("https://github.com/oligamiq/toolchain-for-building-rustc/releases/download/{CONTENT_VERSION}-release/{name}.tar.gz");
  let body = download_from_url(&url)?;

  sender.send(())?;

  save_cache(&body, &format!("{name}.tar.gz"))?;

  normal::unpack_tarball_archive(body, &name)?;

  todo!();

  Ok(())
}

pub fn check_musl_libc(folder: &str) -> color_eyre::Result<bool> {
  let path = format!("{folder}/libc.a");
  let path = std::path::Path::new(&path);

  Ok(std::fs::exists(path)?)
}

pub fn save_cache(file_data: &[u8], file_name: &str) -> color_eyre::Result<()> {
  let path = format!("/x-tools/cache/{}", file_name);
  let path = std::path::Path::new(&path);
  create_dir_all(path.parent().unwrap())?;
  std::fs::write(path, file_data)?;

  Ok(())
}

pub fn check_cache(file_name: &str) -> color_eyre::Result<Vec<u8>> {
  let path = format!("/x-tools/cache/{}", file_name);
  let path = std::path::Path::new(&path);
  let data = std::fs::read(path)?;

  Ok(data)
}

pub fn download_from_url(url: &str) -> color_eyre::Result<Vec<u8>> {
  let file_name = url.split('/').last().unwrap();
  if let Ok(data) = check_cache(file_name) {
    log::info!("Using cached file {}", file_name);

    return Ok(data);
  }

  let mut pb_and_len: Option<(ProgressBar<Stdout>, u64)> = None;
  let lazy = minreq::get(url).send_lazy()?;
  let len = lazy
    .headers
    .get("content-length")
    .unwrap()
    .parse::<u64>()
    .unwrap();

  let mut buffer = Vec::with_capacity(len as usize);

  for byte in lazy {
    let (byte, _) = byte?;

    if let Some((ref mut pb, _)) = pb_and_len {
      pb.inc();
    } else {
      let mut pb = ProgressBar::new(len);
      pb.set_max_refresh_rate(Some(std::time::Duration::from_millis(100)));
      pb.set_units(pbr::Units::Bytes);
      pb_and_len = Some((pb, len as u64));
      println!("Downloading from {}", url);
      std::io::Write::flush(&mut std::io::stdout())?;
    }

    buffer.push(byte);
  }
  if let Some((ref mut pb, _)) = pb_and_len {
    pb.finish();
  }

  Ok(buffer)
}

pub fn soft_link_file(src: &str, dest: &str) -> color_eyre::Result<()> {
  #[cfg(unix)]
  std::os::unix::fs::symlink(src, dest)?;

  #[cfg(windows)]
  std::os::windows::fs::symlink_file(src, dest)?;

  Ok(())
}

pub mod normal {
  use crate::utils::TOOLS;

  use super::*;
  use color_eyre::eyre::Context as _;
  use flate2::read::GzDecoder;

  pub fn unpack_tarball_archive(buffer: Vec<u8>, name: &str) -> color_eyre::Result<()> {
    let decompressed = GzDecoder::new(std::io::Cursor::new(buffer));
    let mut archive = tar::Archive::new(decompressed);
    let folder = format!("/x-tools/{}", name);
    if std::fs::exists(&folder)? {
      log::warn!("{:?} already exists, but will be overwritten", folder);
      std::fs::remove_dir_all(&folder)?;
    }
    std::fs::create_dir_all(&folder)?;
    archive.unpack("/x-tools")?;

    Ok(())
  }

  pub fn install_toolchain(prefix: &str, tool_prefix: &str) -> color_eyre::Result<()> {
    for tool in TOOLS.iter() {
      let cmd = format!("{tool_prefix}-{tool}");
      let from_cmd = format!("{prefix}/{cmd}");
      let canonicalized = std::fs::canonicalize(&from_cmd)
        .with_context(|| format!("Failed to get canonicalized path for {:?}", &from_cmd))?;
      let dest = format!("/usr/local/bin/{}", cmd);
      if std::fs::metadata(&dest).is_ok() {
        log::warn!("{dest} already exists, but will be overwritten");
        std::fs::remove_file(&dest)?;
      }
      soft_link_file(canonicalized.to_str().unwrap(), &dest)?;
    }

    Ok(())
  }

  pub fn check_install_cmd(tool_prefix: &str) -> color_eyre::Result<()> {
    for tool in TOOLS.iter() {
      let cmd = format!("{tool_prefix}-{tool}");
      let which = which::which(&cmd)?;
      if which.to_str().unwrap() != format!("/usr/local/bin/{}", cmd) {
        panic!("{} is not installed correctly", cmd);
      }
    }

    Ok(())
  }

  #[tracing::instrument]
  pub fn check_compile_test_c(tool_prefix: &str) -> color_eyre::Result<()> {
    let code = r#"
      #include <stdio.h>
      int main() {
        printf("Hello, World!\n");
        return 0;
      }
    "#;

    let prefix = format!("/tmp/{tool_prefix}");
    let c_file_path = format!("{prefix}.c");
    let path = std::path::Path::new(&c_file_path);
    std::fs::write(path, code)?;

    let cmd = format!("{tool_prefix}-gcc");
    let compiled_file = format!("{prefix}");
    let output = std::process::Command::new(cmd)
      .arg(&c_file_path)
      .arg("-o")
      .arg(&compiled_file)
      .output()?;

    if !output.status.success() {
      log::error!("Failed to compile test file with c compiler");
      log::error!("stdout: {}", String::from_utf8(output.stdout).unwrap());
      log::error!("stderr: {}", String::from_utf8(output.stderr).unwrap());
      Err(color_eyre::eyre::eyre!(
        "Failed to compile test file with c compiler"
      ))?;
    }

    std::fs::remove_file(&c_file_path)?;
    std::fs::remove_file(&compiled_file)?;

    Ok(())
  }

  #[tracing::instrument]
  pub fn check_compile_test_cpp(tool_prefix: &str) -> color_eyre::Result<()> {
    let code = r#"
      #include <iostream>
      int main() {
        std::cout << "Hello, World!" << std::endl;
        return 0;
      }
    "#;

    let prefix = format!("/tmp/{tool_prefix}");
    let cpp_file_path = format!("{prefix}.cpp");
    let path = std::path::Path::new(&cpp_file_path);
    std::fs::write(path, code)?;

    let cmd = format!("{tool_prefix}-g++");
    let compiled_file = format!("{prefix}");
    let output = std::process::Command::new(cmd)
      .arg(&cpp_file_path)
      .arg("-o")
      .arg(&compiled_file)
      .output()?;

    if !output.status.success() {
      log::error!("Failed to compile test file with c++ compiler");
      log::error!("stdout: {}", String::from_utf8(output.stdout).unwrap());
      log::error!("stderr: {}", String::from_utf8(output.stderr).unwrap());
      panic!("Failed to compile test file with c++ compiler");
    }

    Ok(())
  }
}
