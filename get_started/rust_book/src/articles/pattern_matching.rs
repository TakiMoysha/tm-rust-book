use std::collections::HashMap;

struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

#[cfg(test)]
mod pattern_matching_demo {
    use super::*;
    use rstest::*;

    use crate::articles::pattern_matching::HttpResponse;

    #[rstest]
    #[case(HttpResponse { status: 200, headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]), body: Some(Vec::new()) })]
    fn demo_destructuring(#[case] http_response: HttpResponse) {
        match http_response {
            // ================================================= SUCCESS
            HttpResponse {
                status: 200..=299,
                headers,
                body: Some(ref data),
                ..
            } if headers
                .get("Content-Type")
                .is_some_and(|ct| ct.starts_with("application/json")) =>
            {
                todo!()
            }

            // ================================================= REDIRECT
            ref resp @ HttpResponse {
                status: 300 | 301 | 302 | 303 | 307 | 308,
                ..
            } => {
                todo!()
            }

            // ================================================= NOT FOUND
            HttpResponse {
                status: 404 | 410, ..
            } => {
                todo!()
            }

            _ => {
                todo!()
            }
        }
    }
}
