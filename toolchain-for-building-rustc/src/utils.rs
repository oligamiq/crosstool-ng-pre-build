use crate::cmd::OS;

pub const TOOLS: [&'static str; 4] = ["gcc", "g++", "ar", "ranlib"];

pub fn get_wasi_sdk_name() -> String {
  #[allow(non_camel_case_types)]
  #[derive(Debug, strum::AsRefStr)]
  enum Arch {
    arm64,
    x86_64,
  }

  let arch = if cfg!(target_arch = "aarch64") {
    Arch::arm64
  } else {
    Arch::x86_64
  };

  let os: OS = if cfg!(target_os = "linux") {
    OS::Linux
  } else if cfg!(target_os = "macos") {
    OS::Mac
  } else if cfg!(target_os = "windows") {
    OS::Windows
  } else {
    panic!("Unsupported OS");
  };

  let prefix = "wasi-sdk-24.0";
  let os = match os {
    OS::Linux => "linux",
    OS::Mac => "macos",
    OS::Windows => "windows",
  };
  format!("{}-{}-{}", prefix, arch.as_ref(), os)
}
