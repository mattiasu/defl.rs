use std::io::Read;

fn main() {
    let server = tiny_http::Server::http("0.0.0.0:8080").expect("failed to start server");
    println!("listening on http://0.0.0.0:8080");

    for mut request in server.incoming_requests() {
        let response = handle(&mut request);
        let _ = request.respond(response);
    }
}

fn handle(request: &mut tiny_http::Request) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let method = request.method().as_str().to_string();
    let remote = request
        .remote_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let target_url = match extract_target(request.headers()) {
        Some(url) => url,
        None => {
            println!("[{remote}] {method} -> missing X-Target-Url header -> 400");
            let body = b"missing X-Target-Url header";
            return tiny_http::Response::from_data(body.to_vec())
                .with_status_code(400);
        }
    };

    println!("[{remote}] {method} -> {target_url}");

    match forward(request, &target_url) {
        Ok((response, status)) => {
            println!("[{remote}] {method} -> {target_url} <- {status}");
            response
        }
        Err(err) => {
            println!("[{remote}] {method} -> {target_url} <- error: {err}");
            let body = format!("forward error: {err}");
            tiny_http::Response::from_data(body.into_bytes())
                .with_status_code(502)
        }
    }
}

fn extract_target(headers: &[tiny_http::Header]) -> Option<String> {
    headers.iter().find_map(|h| {
        let name: &str = h.field.as_str().into();
        if name.eq_ignore_ascii_case("x-target-url") {
            let value: &str = h.value.as_str().into();
            Some(value.to_string())
        } else {
            None
        }
    })
}

fn forward(
    request: &mut tiny_http::Request,
    target_url: &str,
) -> Result<
    (tiny_http::Response<std::io::Cursor<Vec<u8>>>, u16),
    Box<dyn std::error::Error>,
> {
    let mut body = Vec::new();
    request.as_reader().read_to_end(&mut body)?;

    let mut req = ureq::request(request.method().as_str(), target_url);

    for header in request.headers() {
        let name: &str = header.field.as_str().into();
        let value: &str = header.value.as_str().into();

        if name.eq_ignore_ascii_case("x-target-url") {
            continue;
        }

        if name.eq_ignore_ascii_case("defl-forward-auth-token") {
            req = req.set("Authorization", &format!("Bearer {value}"));
            continue;
        }

        if let Some(remainder) = name.get(5..).filter(|_| name[..5].eq_ignore_ascii_case("defl-")) {
            req = req.set(remainder, value);
            continue;
        }
    }

    let resp = match if body.is_empty() { req.call() } else { req.send_bytes(&body) } {
        Ok(r) => r,
        Err(ureq::Error::Status(_, r)) => r,
        Err(e) => return Err(e.into()),
    };

    let status = resp.status();
    let mut resp_body = Vec::new();
    resp.into_reader().read_to_end(&mut resp_body)?;

    Ok((tiny_http::Response::from_data(resp_body).with_status_code(status), status))
}
