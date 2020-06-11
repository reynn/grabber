use rawr::structures::submission::Submission;
use reqwest::Url;

pub fn filter_domains(s: &Submission<'_>) -> Option<Url> {
    match s.link_url() {
        Some(link_url) => {
            debug!("filter check link_url: {}", &link_url);
            if let Ok(url) = Url::parse(link_url.as_str()) {
                match url.domain() {
                    Some(domain) => match domain {
                        "i.redd.it" | "i.imgur.com" => Some(url),
                        "gfycat.com" => {
                            // Request fails silently when the path is all lowercase letters
                            // this is trying to filter those out so we dont have
                            // 1kb files littering everything
                            if url.path().chars().find(|x| x.is_uppercase()).is_some() {
                                let mut url = url;
                                url.set_host(Some("giant.gfycat.com"))
                                    .expect("Somehow failed to set host to giant.gfycat.com");
                                url.set_path((url.path().to_owned() + ".mp4").as_str());
                                Some(url)
                            } else {
                                None
                            }
                        }
                        _ => {
                            debug!("skipped url {}", url);
                            None
                        }
                    },
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
