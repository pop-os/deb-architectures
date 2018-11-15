use std::{fs, io, str::FromStr};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Architecture {
    Any,
    All,
    Amd64,
    Armel,
    Armhf,
    Arm64,
    I386,
    Mips,
    Mipsel,
    Mips64,
    Mips64El,
    Ppc64El,
    S390X,
    LinuxAny,
    X32,
}

impl FromStr for Architecture {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let arch = match input {
            "any" => Architecture::Any,
            "all" => Architecture::All,
            "amd64" => Architecture::Amd64,
            "armel" => Architecture::Armel,
            "armhf" => Architecture::Armhf,
            "arm64" => Architecture::Arm64,
            "i386" => Architecture::I386,
            "mips" => Architecture::Mips,
            "mipsel" => Architecture::Mipsel,
            "mips64" => Architecture::Mips64,
            "mips64el" => Architecture::Mips64El,
            "ppc64el" => Architecture::Ppc64El,
            "s390x" => Architecture::S390X,
            "linux-any" => Architecture::LinuxAny,
            "x32" => Architecture::X32,
            _ => return Err("invalid architecture")
        };

        Ok(arch)
    }
}

impl From<Architecture> for &'static str {
    fn from(arch: Architecture) -> Self {
        match arch {
            Architecture::Any => "any",
            Architecture::All => "all",
            Architecture::Amd64 => "amd64",
            Architecture::Armel => "armel",
            Architecture::Armhf => "armhf",
            Architecture::Arm64 => "arm64",
            Architecture::I386 => "i386",
            Architecture::Mips => "mips",
            Architecture::Mipsel => "mipsel",
            Architecture::Mips64 => "mips64",
            Architecture::Mips64El => "mips64el",
            Architecture::Ppc64El => "ppc64el",
            Architecture::S390X => "s390x",
            Architecture::LinuxAny => "linux-any",
            Architecture::X32 => "x32",
        }
    }
}

pub fn supported_architectures() -> io::Result<Vec<Architecture>> {
    fs::read_to_string("/var/lib/dpkg/arch")?
        .lines()
        .map(|arch| {
            arch.parse::<Architecture>()
                .map_err(|why| io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("supported_architectures: {}: {}", why, arch)
                ))
        })
        .collect()
}
