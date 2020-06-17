//! Collector errors

use crate::download::{item::Item, errors as download_errors};

error_chain! {
    links {
        DownloadError(download_errors::Error, download_errors::ErrorKind);
    }
    foreign_links {
        CrossbeamSendError(crossbeam_channel::SendError<Item>);
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
        ReqwestError(reqwest::Error);
    }
}
