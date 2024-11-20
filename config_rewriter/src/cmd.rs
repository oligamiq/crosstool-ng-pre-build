use clap::Parser;
use color_eyre::eyre::eyre;

use crate::targets::{LinuxTargets, MacTargets, WindowsTargets};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(short, long, help = "Path to the config.toml file")]
  pub file: Option<String>,

  #[arg(long, help = "Tier to set, one of 1, 2, 2-host, 2-no-host, all")]
  pub tier: Vec<Tier>,

  #[arg(short, long, help = "set target toolchain")]
  pub target: Vec<Target>,

  #[arg(long, help = "exclude target toolchain")]
  pub exclude: Vec<Target>,

  #[arg(short, long, help = "target OS, one of linux, windows, mac")]
  pub os: Vec<OS>,

  #[arg(
    short,
    long,
    help = "LLVM target. If required, auto-detect will be used"
  )]
  pub llvm_targets: Vec<LLVMTargets>,

  #[arg(long, help = "If you build dist separately, you can change only llvm")]
  pub only_llvm: bool,

  #[arg(long, help = "install the toolchain")]
  pub install: bool,

  #[arg(long, help = "Clean installed toolchains")]
  pub clean: bool,

  #[arg(long, help = "No cache")]
  pub no_cache: bool,
}

pub struct CanonicalArgs {
  pub file: Option<String>,
  pub targets: Vec<Target>,
  pub llvm_targets: Vec<LLVMTargets>,
  pub only_llvm: bool,
  pub install: bool,
  pub clean: bool,
  pub no_cache: bool,
}

impl CanonicalArgs {
  pub fn parse_and_check() -> color_eyre::Result<CanonicalArgs> {
    let args = Args::parse();

    let Args {
      file,
      only_llvm,
      install,
      clean,
      no_cache,
      ..
    } = args;

    if install && clean {
      Err(eyre!("Cannot install and clean at the same time"))?;
    }

    let targets = args
      .target
      .into_iter()
      .map(|target| {
        if only_llvm {
          Ok(target)
        } else {
          target.get_os().check_os()?;
          Ok(target)
        }
      })
      .chain(
        args
          .os
          .into_iter()
          .flat_map(|os| {
            args
              .tier
              .iter()
              .flat_map(move |tier| os.get_targets_by_tier(tier.clone()))
          })
          .map(|target| {
            if only_llvm {
              Ok(target)
            } else {
              target.get_os().check_os()?;
              Ok(target)
            }
          }),
      )
      .collect::<color_eyre::Result<Vec<Target>>>()?;

    let targets = targets.into_iter().fold(Vec::new(), |mut acc, target| {
      if !acc.contains(&target) {
        acc.push(target);
      }
      acc
    });

    // exclude targets
    let targets = targets
      .into_iter()
      .filter(|target| !args.exclude.contains(target))
      .collect::<Vec<_>>();

    if targets.is_empty() {
      Err(eyre!("No targets specified"))?;
    }

    let llvm_targets = args
      .llvm_targets
      .into_iter()
      .map(|x| Ok(x))
      .chain(
        targets
          .iter()
          .map(|target| LLVMTargets::detect_target(target)),
      )
      .collect::<color_eyre::Result<Vec<LLVMTargets>>>()?;

    let llvm_targets = llvm_targets
      .into_iter()
      .fold(Vec::new(), |mut acc, target| {
        if !acc.contains(&target) {
          acc.push(target);
        }
        acc
      });

    Ok(CanonicalArgs {
      file,
      targets,
      llvm_targets,
      only_llvm,
      install,
      clean,
      no_cache,
    })
  }
}

#[derive(Clone, Debug, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum Tier {
  #[strum(serialize = "1", serialize = "1.0")]
  One,
  #[strum(serialize = "2", serialize = "2.0")]
  Two,
  #[strum(serialize = "2-host", serialize = "2.0-host")]
  TwoWithHost,
  #[strum(serialize = "2-no-host", serialize = "2.0-no-host")]
  TwoWithoutHost,
  #[strum(serialize = "all")]
  All,
}

#[derive(Clone, Debug, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum OS {
  #[strum(serialize = "linux")]
  Linux,
  #[strum(serialize = "windows")]
  Windows,
  #[strum(serialize = "mac")]
  Mac,
}

impl OS {
  pub fn check_os(&self) -> color_eyre::Result<()> {
    match self {
      OS::Linux => {
        #[cfg(not(target_os = "linux"))]
        {
          Err(eyre!("This is not a Linux system, Only Linux is supported",))?;
        }
      }
      OS::Windows => {
        #[cfg(not(target_os = "linux"))]
        {
          Err(eyre!("This is not a Linux system, Only Linux is supported",))?;
        }
      }
      OS::Mac => {
        #[cfg(not(target_os = "macos"))]
        {
          Err(eyre!("This is not a Mac system, Only Mac is supported",))?;
        }
      }
    }

    #[allow(unreachable_code)]
    Ok(())
  }

  pub fn get_targets_by_tier(&self, tier: Tier) -> Vec<Target> {
    match self {
      OS::Linux => match tier {
        Tier::One => LinuxTargets::get_tier1(),
        Tier::Two => LinuxTargets::get_tier2(),
        Tier::TwoWithHost => LinuxTargets::get_tier2_with_host_tools(),
        Tier::TwoWithoutHost => LinuxTargets::get_tier2_without_host_tools(),
        Tier::All => LinuxTargets::get_all(),
      }
      .into_iter()
      .map(|target| target.into())
      .collect(),
      OS::Windows => match tier {
        Tier::One => WindowsTargets::get_tier1(),
        Tier::Two => WindowsTargets::get_tier2(),
        Tier::TwoWithHost => WindowsTargets::get_tier2_with_host_tools(),
        Tier::TwoWithoutHost => WindowsTargets::get_tier2_without_host_tools(),
        Tier::All => WindowsTargets::get_all(),
      }
      .into_iter()
      .map(|target| target.into())
      .collect(),
      OS::Mac => match tier {
        Tier::One => MacTargets::get_tier1(),
        Tier::Two => MacTargets::get_tier2(),
        Tier::TwoWithHost => MacTargets::get_tier2_with_host_tools(),
        Tier::TwoWithoutHost => MacTargets::get_tier2_without_host_tools(),
        Tier::All => MacTargets::get_all(),
      }
      .into_iter()
      .map(|target| target.into())
      .collect(),
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Target {
  LinuxTargets(LinuxTargets),
  WindowsTargets(WindowsTargets),
  MacTargets(MacTargets),
}

impl Target {
  pub fn get_os(&self) -> OS {
    match self {
      Target::LinuxTargets(_) => OS::Linux,
      Target::WindowsTargets(_) => OS::Windows,
      Target::MacTargets(_) => OS::Mac,
    }
  }
}

impl std::str::FromStr for Target {
  type Err = color_eyre::Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Ok(target) = LinuxTargets::from_name(s) {
      Ok(Target::LinuxTargets(target))
    } else if let Ok(target) = WindowsTargets::from_name(s) {
      Ok(Target::WindowsTargets(target))
    } else if let Ok(target) = MacTargets::from_name(s) {
      Ok(Target::MacTargets(target))
    } else {
      Err(eyre!(format!("Unknown target: {}", s)))
    }
  }
}

impl ToString for Target {
  fn to_string(&self) -> String {
    match self {
      Target::LinuxTargets(target) => target.to_name(),
      Target::WindowsTargets(target) => target.to_name(),
      Target::MacTargets(target) => target.to_name(),
    }
  }
}

impl From<LinuxTargets> for Target {
  fn from(target: LinuxTargets) -> Self {
    Target::LinuxTargets(target)
  }
}

impl From<WindowsTargets> for Target {
  fn from(target: WindowsTargets) -> Self {
    Target::WindowsTargets(target)
  }
}

impl From<MacTargets> for Target {
  fn from(target: MacTargets) -> Self {
    Target::MacTargets(target)
  }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, strum::EnumString, strum::Display, strum::EnumIter)]
pub enum LLVMTargets {
  AArch64,
  ARM,
  BPF,
  Hexagon,
  LoongArch,
  MSP430,
  Mips,
  NVPTX,
  PowerPC,
  RISCV,
  Sparc,
  SystemZ,
  WebAssembly,
  X86,

  // Experimental
  AVR,
  M68k,
  CSKY,
}

impl LLVMTargets {
  pub fn detect_target(target: &Target) -> color_eyre::Result<Self> {
    let name = target.to_string();

    if name.contains("aarch64") {
      Ok(LLVMTargets::AArch64)
    } else if name.contains("arm") || name.contains("thumb") {
      Ok(LLVMTargets::ARM)
    } else if name.contains("bpf") {
      Ok(LLVMTargets::BPF)
    } else if name.contains("hexagon") {
      Ok(LLVMTargets::Hexagon)
    } else if name.contains("loongarch") {
      Ok(LLVMTargets::LoongArch)
    } else if name.contains("msp430") {
      Ok(LLVMTargets::MSP430)
    } else if name.contains("mips") {
      Ok(LLVMTargets::Mips)
    } else if name.contains("nvptx") {
      Ok(LLVMTargets::NVPTX)
    } else if name.contains("powerpc") {
      Ok(LLVMTargets::PowerPC)
    } else if name.contains("riscv") {
      Ok(LLVMTargets::RISCV)
    } else if name.contains("sparc") {
      Ok(LLVMTargets::Sparc)
    } else if name.contains("s390x") {
      Ok(LLVMTargets::SystemZ)
    } else if name.contains("wasm") {
      Ok(LLVMTargets::WebAssembly)
    } else if name.contains("x86") || name.contains("i686") || name.contains("i586") {
      Ok(LLVMTargets::X86)
    } else if name.contains("avr") {
      Ok(LLVMTargets::AVR)
    } else if name.contains("m68k") {
      Ok(LLVMTargets::M68k)
    } else if name.contains("csky") {
      Ok(LLVMTargets::CSKY)
    } else {
      Err(eyre!("Unknown target: {}", name))
    }
  }

  pub fn is_experimental(&self) -> bool {
    let n: usize = self.clone() as usize;
    n >= 14
  }
}
