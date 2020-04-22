use rawr::structures::submission::Submission;
use reqwest::Url;

pub fn filter_domains(s: &Submission<'_>) -> bool {
    match s.link_url() {
        Some(url) => {
            if let Ok(url) = Url::parse(url.as_str()) {
                match url.domain() {
                    Some(domain) => {
                        debug!("domain: {}", domain);
                        match domain {
                            "i.redd.it" | "i.imgur.com" => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
