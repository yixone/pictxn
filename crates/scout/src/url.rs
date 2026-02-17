use std::fmt::Display;

pub(crate) struct Url {
    domain: String,

    endpoint: String,

    queries: Vec<(String, String)>,
}

impl Url {
    pub fn new(domain: &str, endpoint: &str, queries: Vec<(&str, &str)>) -> Self {
        Url {
            domain: domain.to_string(),
            endpoint: endpoint.to_string(),
            queries: queries
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let queries = self
            .queries
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let uri = [&self.domain, "/", &self.endpoint, "?", &queries].join("");
        f.write_str(&uri)
    }
}
