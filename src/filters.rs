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
                        // "gfycat.com" => {
                        //     let mut url = url;
                        //     url.set_host(Some("giant.gfycat.com"))
                        //         .expect("Somehow failed to set host to giant.gfycat.com");
                        //     url.set_path((url.path().to_owned() + ".mp4").as_str());
                        //     Some(url)
                        // }
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
