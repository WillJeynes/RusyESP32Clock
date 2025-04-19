use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use crate::{PASSWORD, SSID};
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_svc::http::client::EspHttpConnection;

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    Ok(())
}

pub fn get_request(client: &mut HttpClient<EspHttpConnection>, url: String) -> anyhow::Result<String> {
    // Prepare headers and URL
    //Only HTTP for now, since SSL is a pain
    let headers = [("accept", "text/plain")];

    let request = client.request(Method::Get, &*url, &headers)?;
    log::info!("-> GET {}", url);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    log::info!("<- {}", status);
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    log::info!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Ok(body_string) => {
            return Ok(body_string.to_owned());
        },
        Err(e) => log::error!("Error decoding response body: {}", e),
    };

    return Err(anyhow::anyhow!("Error decoding response body"));
}

pub fn get_request_raw(client: &mut HttpClient<EspHttpConnection>, url: String) -> anyhow::Result<Vec<u8>> {
    //TODO: add retry logic, fail gracefully
    //Prepare headers and URL
    //Only HTTP for now, since SSL is a pain
    let headers = [];

    let request = client.request(Method::Get, &*url, &headers)?;
    log::info!("-> GET {}", url);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    log::info!("<- {}", status);
    let mut buf = vec![0u8; 100000];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    log::info!("Read {} bytes", bytes_read);
    buf.truncate(bytes_read);
    Ok(buf)
}