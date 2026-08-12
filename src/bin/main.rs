// [[file:../../org/cargo.org::*Initialization and setup][Initialization and setup:1]]
use core::ffi::c_void;
use cta_tracker::{
    cta::{CtaPredictions, CtaStop, TransitMode},
    display::{
        color::Rgb565,
        psram::PsramBuffer,
        screen::{Lcd, FRAMEBUFFER_BYTES, LCD_PINS},
    },
};
use embedded_svc::{
    http::client::Client,
    wifi::{AuthMethod, ClientConfiguration, Configuration as WifiConfiguration},
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, peripherals::Peripherals},
    http::client::{Configuration as HttpConfig, EspHttpConnection},
    nvs::EspDefaultNvsPartition,
    sys,
    wifi::{BlockingWifi, EspWifi},
};
use std::time::Duration;

use log::{debug, error, info, warn};
// Initialization and setup:1 ends here
