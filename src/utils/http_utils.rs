use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_svc::http::client::EspHttpConnection;

fn get_request_internal(client: &mut HttpClient<EspHttpConnection>, url: &str, headers: &[(&str, &str)], buf_size: usize) -> anyhow::Result<Vec<u8>> {
    let request = client.request(Method::Get, url, headers)?;
    log::info!("-> GET {}", url);
    let mut response = request.submit()?;

    let status = response.status();
    log::info!("<- {}", status);

    let mut buf = vec![0u8; buf_size];
    let bytes_read = io::try_read_full(&mut response, &mut buf)
        .map_err(|e| e.0)?;

    buf.truncate(bytes_read);
    Ok(buf)
}

pub fn get_request(client: &mut HttpClient<EspHttpConnection>, url: String, buf_size: usize) -> anyhow::Result<String> {
    let body = get_request_internal(client, &url, &[("accept", "text/plain")], buf_size)?;
    let body_str = std::str::from_utf8(&body)?;

    Ok(body_str.to_owned())
}

pub fn get_request_raw(client: &mut HttpClient<EspHttpConnection>, url: String, buf_size: usize) -> anyhow::Result<Vec<u8>> {
    get_request_internal(client, &url, &[], buf_size)
}
