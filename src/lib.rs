use std::{fs, io, str::FromStr};

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum Architecture {
    All,
    Amd64,
    Arm64,
    Armhf,
    I386,
    Ppc64el,
    S390x,
}

impl FromStr for Architecture {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let arch = match input {
            "all" => Architecture::All,
            "amd64" => Architecture::Amd64,
            "arm64" => Architecture::Arm64,
            "armhf" => Architecture::Armhf,
            "i386" => Architecture::I386,
            "ppc64el" => Architecture::Ppc64el,
            "s390x" => Architecture::S390x,
            _ => return Err("invalid architecture")
        };

        Ok(arch)
    }
}

impl From<Architecture> for &'static str {
    fn from(arch: Architecture) -> Self {
        match arch {
            Architecture::All => "all",
            Architecture::Amd64 => "amd64",
            Architecture::Arm64 => "arm64",
            Architecture::Armhf => "armhf",
            Architecture::I386 => "i386",
            Architecture::Ppc64el => "ppc64el",
            Architecture::S390x => "s390x",
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
