use std::io::Stdout;

use flate2::read::GzDecoder;
use pbr::ProgressBar;

use crate::targets::LinuxTargets;

const CONTENT_VERSION: &str = "v0.1.0";

pub trait Install {
    fn install(&self) -> anyhow::Result<()>;
}

impl Install for LinuxTargets {
    fn install(&self) -> anyhow::Result<()> {
        // #[cfg(target_os = "linux")]
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
                // let folder = format!("/x-tools/{name}");
                // let path = std::path::Path::new(&folder);
                // if !path.exists() {
                //     std::fs::create_dir_all(&path).with_context(|| format!("Failed to create folder: {}", folder))?;
                // } else {
                //     println!("Warning!: Remove existing folder: {}", folder);
                //     std::fs::remove_dir_all(&path)?;

                //     std::fs::create_dir_all(&path)?;
                // }
                let url = format!("https://github.com/oligamiq/toolchain-for-building-rustc/releases/download/{CONTENT_VERSION}-release/{name}.tar.gz");
                let body = download_from_url(&url)?;
                tarball_archive(body)?;
            }
        }

        Ok(())
    }
}

pub fn download_from_url(url: &str) -> anyhow::Result<Vec<u8>> {
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

pub fn tarball_archive(buffer: Vec<u8>) -> anyhow::Result<()> {
    let decompressed = GzDecoder::new(std::io::Cursor::new(buffer));
    let mut archive = tar::Archive::new(decompressed);
    archive.unpack("/x-tools")?;

    Ok(())
}
