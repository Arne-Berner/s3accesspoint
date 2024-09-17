use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::Modem, prelude::Peripherals},
    nvs::EspDefaultNvsPartition,
    sys::EspError,
    wifi::{AccessPointConfiguration, AuthMethod, BlockingWifi, Configuration, EspWifi},
};
use std::{thread::sleep, time::Duration};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    log::info!("before wifi");
    let _wifi = create_wifi(&sys_loop, &nvs, peripherals.modem)?;

    log::info!("Hello, world!");
    loop {
        //delay::Ets::delay_ms(1000);
        sleep(Duration::from_millis(100));
    }
}

fn create_wifi(
    sys_loop: &EspSystemEventLoop,
    nvs: &EspDefaultNvsPartition,
    modem: Modem,
) -> Result<BlockingWifi<EspWifi<'static>>, EspError> {
    let esp_wifi = EspWifi::new(modem, sys_loop.clone(), Some(nvs.clone()))?;
    let mut wifi = BlockingWifi::wrap(esp_wifi, sys_loop.clone())?;

    let config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "EspWifi".try_into().unwrap(),
        auth_method: AuthMethod::WPA3Personal,
        password: "supertopsecret".try_into().unwrap(),
        ..Default::default()
    });

    wifi.set_configuration(&config)?;

    wifi.start()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    while !wifi.is_up().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting to set up {:?}", config);
    }

    Ok(wifi)
}
