// MyCitadel: node, wallet library & command-line tool
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@mycitadel.io>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the AGPL License
// along with this software.
// If not, see <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

//! File storage driver

use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{fs, io};

use lnpbp::strict_encoding::{StrictDecode, StrictEncode};
use microservices::FileFormat;

use rgb::Genesis;
use rgb20::Asset;

use super::{Driver, Error};
use crate::data::{Data, WalletContract};
use crate::rpc::message::{IdentityInfo, SignerAccount};

#[derive(Debug)]
pub struct FileDriver {
    fd: fs::File,
    config: FileConfig,
    data: Data,
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Serialize,
    Deserialize,
    StrictEncode,
    StrictDecode,
)]
#[strict_encoding_crate(lnpbp::strict_encoding)]
#[serde(crate = "serde_crate")]
pub struct FileConfig {
    pub location: String,
    pub format: FileFormat,
}

impl FileDriver {
    pub fn with(config: FileConfig) -> Result<Self, Error> {
        info!(
            "Initializing file driver for data in {:?}",
            &config.location
        );
        let exists = Path::new(&config.location).exists();
        let fd = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(!exists)
            .open(&config.location)?;
        let mut me = Self {
            fd,
            config: config.clone(),
            data: Default::default(),
        };
        if !exists {
            warn!("Data file does not exist: initializing empty vault");
            me.store(&vec![])?;
        }
        Ok(me)
    }

    fn load(&mut self) -> Result<Vec<u8>, Error> {
        debug!("Loading data from {}", self.config.location);
        self.fd.seek(io::SeekFrom::Start(0))?;
        trace!("Parsing data (expected format {})", self.config.format);
        let accounts = match self.config.format {
            FileFormat::StrictEncode => Vec::<u8>::strict_decode(&mut self.fd)?,
            #[cfg(feature = "serde_yaml")]
            FileFormat::Yaml => serde_yaml::from_reader(&mut self.fd)?,
            #[cfg(feature = "toml")]
            FileFormat::Toml => {
                let mut data: Vec<u8> = vec![];
                self.fd.read_to_end(&mut data)?;
                toml::from_slice(&data)?
            }
            #[cfg(feature = "serde_json")]
            FileFormat::Json => serde_json::from_reader(&mut self.fd)?,
            _ => unimplemented!(),
        };
        trace!("Data loaded from storage: {:?}", accounts);
        Ok(accounts)
    }

    fn store(&mut self, data: &Vec<u8>) -> Result<(), Error> {
        debug!(
            "Storing data to the file {} in {} format",
            self.config.location, self.config.format
        );
        trace!("Current data to store: {:?}", data);
        self.fd.seek(io::SeekFrom::Start(0))?;
        self.fd.set_len(0)?;
        match self.config.format {
            FileFormat::StrictEncode => {
                data.strict_encode(&mut self.fd)?;
            }
            #[cfg(feature = "serde_yaml")]
            FileFormat::Yaml => {
                serde_yaml::to_writer(&mut self.fd, data)?;
            }
            #[cfg(feature = "toml")]
            FileFormat::Toml => {
                let data = toml::to_vec(data)?;
                self.fd.write_all(&data)?;
            }
            #[cfg(feature = "serde_json")]
            FileFormat::Json => {
                serde_json::to_writer(&mut self.fd, data)?;
            }
            _ => unimplemented!(),
        };
        trace!("Vault data stored");
        Ok(())
    }
}

impl Driver for FileDriver {
    fn wallets(&self) -> Result<Vec<WalletContract>, Error> {
        unimplemented!()
    }

    fn add_wallet(&mut self, contract: WalletContract) -> Result<(), Error> {
        unimplemented!()
    }

    fn signers(&self) -> Result<Vec<SignerAccount>, Error> {
        unimplemented!()
    }

    fn add_signer(&mut self, account: SignerAccount) -> Result<(), Error> {
        unimplemented!()
    }

    fn identities(&self) -> Result<Vec<IdentityInfo>, Error> {
        unimplemented!()
    }

    fn add_identity(&mut self, identity: IdentityInfo) -> Result<(), Error> {
        unimplemented!()
    }

    fn assets(&self) -> Result<Vec<Asset>, Error> {
        unimplemented!()
    }

    fn add_asset(&mut self, genesis: Genesis) -> Result<(), Error> {
        unimplemented!()
    }
}
