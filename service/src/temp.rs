use anyhow::Result;
use std::process::{
    Output,
    Command
};

/// Measuring temperature.
/// 
/// Due to the architecture of the SoCs used on the Raspberry Pi range, 
/// and the use of the upstream temperature monitoring code in the 
/// Raspberry Pi OS distribution, Linux-based temperature measurements 
/// can be inaccurate. There is a command that can provide an accurate 
/// and instantaneous reading of the current SoC temperature, as it 
/// communicates with the GPU directly:
///
/// ```bash
/// vcgencmd measure_temp
/// ```
#[rustfmt::skip]
pub fn get_temp() -> Result<f32> {
    let Output { 
        stdout, 
        stderr: _, 
        status: _ 
    } = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()?;
    let rate = String::from_utf8_lossy(&stdout)
        .split('=')
        .next_back()
        .unwrap_or("0'c")
        .split('\'')
        .next()
        .unwrap_or("0")
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);
    Ok(rate)
}

/// Compute duty-cycle.
/// 
/// All Raspberry Pi models perform a degree of thermal management 
/// to avoid overheating under heavy load. The SoCs have an internal 
/// temperature sensor, which software on the GPU polls to ensure that 
/// temperatures do not exceed a predefined limit; this is 85°C on 
/// all models. It is possible to set this to a lower value, but not 
/// to a higher one. As the device approaches the limit, various 
/// frequencies and sometimes voltages used on the chip (ARM, GPU) are 
/// reduced. This reduces the amount of heat generated, keeping 
/// the temperature under control.
/// 
/// When the core temperature is between 80°C and 85°C, a warning icon 
/// showing a red half-filled thermometer will be displayed, and the 
/// ARM cores will be progressively throttled back. If the temperature 
/// reaches 85°C, an icon showing a fully filled thermometer will be 
/// displayed, and both the ARM cores and the GPU will be throttled back. 
/// See the page on warning icons for images of the icons.
/// 
/// For Raspberry Pi 3 Model B+, the PCB technology has been changed to 
/// provide better heat dissipation and increased thermal mass. In addition, 
/// a soft temperature limit has been introduced, with the goal of 
/// maximising the time for which a device can "sprint" before reaching 
/// the hard limit at 85°C. When the soft limit is reached, the clock 
/// speed is reduced from 1.4GHz to 1.2GHz, and the operating voltage is 
/// reduced slightly. This reduces the rate of temperature increase: 
/// we trade a short period at 1.4GHz for a longer period at 1.2GHz. 
/// By default, the soft limit is 60°C.
/// 
/// The Raspberry Pi 4 Model B continues with the same PCB technology 
/// as the Raspberry Pi 3B+ to help dissipate excess heat. 
/// There is currently no soft limit defined.
///
/// #Example
///
/// ```
/// let temp = get_temp().unwrap();
/// let dutycycle = get_pwm(temp);
/// ```
#[rustfmt::skip]
pub fn get_pwm(temp: f32) -> u8 {
    if temp <= 40.0 { return 0 }
    if temp >= 60.0 { return 255 }
    ((temp - 40.0) * 12.75).ceil() as u8
}
