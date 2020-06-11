//! Collector errors

use crate::download;

error_chain! {
    errors {
        IgnoredUser(u: String) {
            description("User is being ignored")
            display("User [{}] is being ignored", u)
        }
        NoNewPosts(u: String, lu: String) {
            description("User has no new posts")
            display("{} has no new posts. Last updated ", u)
        }
    }
    links {
        DownloadError(download::Error, download::ErrorKind);
    }
    foreign_links {
        Io(std::io::Error);
        TOMLSerializeError(toml::ser::Error);
        TOMLDeserializeError(toml::de::Error);
        RawrError(rawr::errors::APIError);
    }
}
