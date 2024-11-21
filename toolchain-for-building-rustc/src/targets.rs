use strum::{EnumIter, EnumString, IntoEnumIterator};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, EnumString, strum::Display, EnumIter)]
pub enum LinuxTargets {
  // Tier 1
  aarch64_unknown_linux_gnu,
  i686_unknown_linux_gnu,
  x86_64_unknown_linux_gnu,

  // Tier 2
  aarch64_unknown_linux_musl,
  arm_unknown_linux_gnueabi,
  arm_unknown_linux_gnueabihf,
  armv7_unknown_linux_gnueabihf,
  loongarch64_unknown_linux_gnu,
  loongarch64_unknown_linux_musl,
  powerpc_unknown_linux_gnu,
  powerpc64_unknown_linux_gnu,
  powerpc64le_unknown_linux_gnu,
  riscv64gc_unknown_linux_gnu,
  riscv64gc_unknown_linux_musl,
  s390x_unknown_linux_gnu,
  x86_64_unknown_freebsd,
  x86_64_unknown_illumos,
  x86_64_unknown_linux_musl,
  x86_64_unknown_netbsd,

  // Tier 2 without host tools
  aarch64_unknown_fuchsia,
  aarch64_linux_android,
  aarch64_unknown_linux_ohos,
  aarch64_unknown_none_softfloat,
  aarch64_unknown_none,
  aarch64_unknown_uefi,
  arm_linux_androideabi,
  arm_unknown_linux_musleabi,
  arm_unknown_linux_musleabihf,
  armebv7r_none_eabi,
  armebv7r_none_eabihf,
  armv5te_unknown_linux_gnueabi,
  armv5te_unknown_linux_musleabi,
  armv7_linux_androideabi,
  armv7_unknown_linux_gnueabi,
  armv7_unknown_linux_musleabi,
  armv7_unknown_linux_musleabihf,
  armv7_unknown_linux_ohos,
  armv7a_none_eabi,
  armv7r_none_eabi,
  armv7r_none_eabihf,
  i586_unknown_linux_gnu,
  i586_unknown_linux_musl,
  i686_linux_android,
  i686_unknown_freebsd,
  i686_unknown_linux_musl,
  i686_unknown_uefi,
  loongarch64_unknown_none,
  loongarch64_unknown_none_softfloat,
  nvptx64_nvidia_cuda,
  riscv32imac_unknown_none_elf,
  riscv32i_unknown_none_elf,
  riscv32im_unknown_none_elf,
  riscv32imc_unknown_none_elf,
  riscv32imafc_unknown_none_elf,
  riscv64gc_unknown_none_elf,
  riscv64imac_unknown_none_elf,
  sparc64_unknown_linux_gnu,
  sparcv9_sun_solaris,
  thumbv6m_none_eabi,
  thumbv7em_none_eabi,
  thumbv7em_none_eabihf,
  thumbv7m_none_eabi,
  thumbv7neon_linux_androideabi,
  thumbv7neon_unknown_linux_gnueabihf,
  thumbv8m_base_none_eabi,
  thumbv8m_main_none_eabi,
  thumbv8m_main_none_eabihf,
  wasm32_unknown_emscripten,
  wasm32_unknown_unknown,
  wasm32_wasip1,
  wasm32_wasip2,
  wasm32_wasip1_threads,
  wasm32v1_none,
  x86_64_fortanix_unknown_sgx,
  x86_64_unknown_fuchsia,
  x86_64_linux_android,
  x86_64_pc_solaris,
  x86_64_unknown_linux_gnux32,
  x86_64_unknown_linux_ohos,
  x86_64_unknown_none,
  x86_64_unknown_redox,
  x86_64_unknown_uefi,
}

impl LinuxTargets {
  pub fn to_name(&self) -> String {
    let normal_name = self.to_string();
    let name = normal_name.replace("_", "-");
    let name = name.replace("x86-64", "x86_64");
    let name = name.replace("thumbv8m_", "thumbv8m.");
    name
  }

  pub fn from_name(name: &str) -> color_eyre::Result<Self> {
    let name = name.replace("-", "_");
    let name = name.replace(".", "_");
    Ok(name.parse()?)
  }

  pub fn get_tier1() -> Vec<Self> {
    LinuxTargets::iter().collect::<Vec<_>>()[0..=2].to_vec()
  }

  pub fn get_tier2_with_host_tools() -> Vec<Self> {
    LinuxTargets::iter().collect::<Vec<_>>()[3..=18].to_vec()
  }

  pub fn get_tier2_without_host_tools() -> Vec<Self> {
    LinuxTargets::iter().collect::<Vec<_>>()[19..=81].to_vec()
  }

  pub fn get_tier2() -> Vec<Self> {
    Self::get_tier2_with_host_tools()
      .into_iter()
      .chain(Self::get_tier2_without_host_tools().into_iter())
      .collect::<Vec<_>>()
  }

  pub fn get_all() -> Vec<Self> {
    LinuxTargets::iter().collect::<Vec<_>>()
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, EnumString, strum::Display, EnumIter)]
pub enum WindowsTargets {
  // Tier 1
  i686_pc_windows_gnu,
  i686_pc_windows_msvc,
  x86_64_pc_windows_gnu,
  x86_64_pc_windows_msvc,

  // Tier 2
  aarch64_pc_windows_msvc,

  // Tier 2 without host tools
  aarch64_pc_windows_gnullvm,
  arm64ec_pc_windows_msvc,
  i586_pc_windows_msvc,
  i686_pc_windows_gnullvm,
  x86_64_pc_windows_gnullvm,
}

impl WindowsTargets {
  pub fn to_name(&self) -> String {
    let normal_name = self.to_string();
    let name = normal_name.replace("_", "-");
    let name = name.replace("x86-64", "x86_64");
    name
  }

  pub fn from_name(name: &str) -> color_eyre::Result<Self> {
    let name = name.replace("-", "_");
    Ok(name.parse()?)
  }

  pub fn get_tier1() -> Vec<Self> {
    WindowsTargets::iter().collect::<Vec<_>>()[0..=3].to_vec()
  }

  pub fn get_tier2_with_host_tools() -> Vec<Self> {
    WindowsTargets::iter().collect::<Vec<_>>()[4..=4].to_vec()
  }

  pub fn get_tier2_without_host_tools() -> Vec<Self> {
    WindowsTargets::iter().collect::<Vec<_>>()[5..=9].to_vec()
  }

  pub fn get_tier2() -> Vec<Self> {
    Self::get_tier2_with_host_tools()
      .into_iter()
      .chain(Self::get_tier2_without_host_tools().into_iter())
      .collect::<Vec<_>>()
  }

  pub fn get_all() -> Vec<Self> {
    WindowsTargets::iter().collect::<Vec<_>>()
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, EnumString, strum::Display, EnumIter)]
pub enum MacTargets {
  // Tier 1
  aarch64_apple_darwin,
  x86_64_apple_darwin,

  // Tier 2 without host tools
  aarch64_apple_ios,
  aarch64_apple_ios_macabi,
  aarch64_apple_ios_sim,
  x86_64_apple_ios,
  x86_64_apple_ios_macabi,
}

impl MacTargets {
  pub fn to_name(&self) -> String {
    let normal_name = self.to_string();
    let name = normal_name.replace("_", "-");
    let name = name.replace("x86-64", "x86_64");
    name
  }

  pub fn from_name(name: &str) -> color_eyre::Result<Self> {
    let name = name.replace("-", "_");
    Ok(name.parse()?)
  }

  pub fn get_tier1() -> Vec<Self> {
    MacTargets::iter().collect::<Vec<_>>()[0..=1].to_vec()
  }

  pub fn get_tier2_with_host_tools() -> Vec<Self> {
    vec![]
  }

  pub fn get_tier2_without_host_tools() -> Vec<Self> {
    MacTargets::iter().collect::<Vec<_>>()[2..=6].to_vec()
  }

  pub fn get_tier2() -> Vec<Self> {
    Self::get_tier2_with_host_tools()
      .into_iter()
      .chain(Self::get_tier2_without_host_tools().into_iter())
      .collect::<Vec<_>>()
  }

  pub fn get_all() -> Vec<Self> {
    MacTargets::iter().collect::<Vec<_>>()
  }
}
