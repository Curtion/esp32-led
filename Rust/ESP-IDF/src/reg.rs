use esp_idf_sys::*; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::delay::FreeRtos;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let config = gpio_config_t {
        pin_bit_mask: 2, // "10" GPIO1
        mode: gpio_mode_t_GPIO_MODE_OUTPUT,
        pull_up_en: gpio_pullup_t_GPIO_PULLUP_ENABLE,
        pull_down_en: gpio_pulldown_t_GPIO_PULLDOWN_DISABLE,
        intr_type: gpio_int_type_t_GPIO_INTR_DISABLE,
    };

    unsafe {
        gpio_config(&config);
    }

    loop {
        unsafe {
            gpio_set_level(1, 1);
        }
        FreeRtos::delay_ms(1000);
        unsafe {
            gpio_set_level(1, 0);
        }
        FreeRtos::delay_ms(1000);
    }

}
