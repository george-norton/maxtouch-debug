extern crate hidapi;
use hidapi::{HidApi, HidDevice};
use maxtouch::{InformationBlock, ObjectTableElement, T6CommandProcessor,
    T7PowerConfig, T8AcquisitionConfig, T25SelfTest, T42TouchSupression, T46CteConfig,
    T47ProciStylus, T56Shieldless, T65LensBending, T80RetransmissionCompensation,
    T100MultipleTouchTouchscreen};
use parking_lot::Mutex;
use std::{cmp, mem};
use std::collections::HashMap;
use std::default::Default;
use tauri::State;
use zerocopy::{FromBytes, FromZeroes, AsBytes};
use tauri::ipc::Response;
use image::{codecs::png::PngEncoder, Rgb, RgbImage, ImageEncoder};
use serde::{Deserialize, Serialize};

mod maxtouch;

const VENDOR_ID: u16 = 0xFEED;
const PRODUCT_ID: u16 = 0x0000;
const USAGE_PAGE: u16 = 0xFF60;
const USAGE: u16 = 0x61;
const REPORT_LENGTH: usize = 32 + 1;

#[repr(u8)]
enum MaxTouchStatus {
    OK = 0,
}

#[repr(u8)]
enum MaxTouchCommand {
    CheckVersion = 0,
    Command,
    Read,
    Write,
}

#[repr(u8)]
enum MaxTouchCommandType {
    RebootBootloader = 0,
    SetMouseMode,
    GetMouseMode
}

#[derive(Debug)]
pub struct ObjectDetails {
    address: u16,
    size: u8,
    instances: u8,
}

#[derive(Default)]
pub struct ConnectionState {
    device: Option<HidDevice>,
    sensor_size: [u8; 2],
    invert_x: bool,
    invert_y: bool,
    switch_xy: bool,
    object_table: HashMap<u8, ObjectDetails>,
}

fn check_version(connection: &ConnectionState) -> Result<(), String> {
    match &connection.device {
        Some(device) => {
            let mut data: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            data[0] = 0x0; // First byte of the first message is the Report ID
            data[1] = MaxTouchCommand::CheckVersion as u8; // Command
            data[2] = 0x9A; // Magic
            data[3] = 0x4D; // Magic
            data[4] = 0x00; // Version
            data[5] = 0x01; // Version

            match device.write(&data) {
                Ok(_) => {},
                Err(_) => { return Err(format!("Failed to write to the device.")); }
            }
            match device.read_timeout(&mut data, 1000) {
                Ok(size) => {
                    if size > 0 && data[0] == MaxTouchStatus::OK as u8 {
                        return Ok(());
                    }
                    return Err(format!("Version check failure. Error {}", data[0]));
                }
                Err(e) => {
                    return Err(format!("Read returned error {}", e));
                }
            }
        }
        _ => {
            return Err(format!("Not connected."));
        }
    }
}

fn read_data(connection: &ConnectionState, address: u16, length: u8) -> Result<Vec<u8>, String> {
    match &connection.device {
        Some(device) => {
            let mut response = Vec::new();
            let mut data: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            let mut remaining = length as usize;
            for offset in (0..length).step_by(REPORT_LENGTH - 5) {
                let read_length = cmp::min(remaining, REPORT_LENGTH - 5);
                data[0] = 0x0; // First byte of the first message is the Report ID
                data[1] = MaxTouchCommand::Read as u8;              // Command
                data[2] = ((address + offset as u16) & 0xff) as u8; // Address Low
                data[3] = ((address + offset as u16) >> 8) as u8;   // Address High
                data[4] = read_length as u8;                        // Length

                match device.write(&data) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(format!("Write returned error {}", e));
                    }
                }
                match device.read_timeout(&mut data, 1000) {
                    Ok(size) => {
                        if size > 0 && data[0] != MaxTouchStatus::OK as u8 {
                            return Err(format!("Device reported an error ({})", data[0]));
                        }

                        response.extend_from_slice(&data[4..(4 + read_length)]);
                        remaining -= read_length;
                    }
                    Err(e) => {
                        return Err(format!("Read returned error {}", e));
                    }
                }
            }
            return Ok(response);
        }
        _ => return Err(format!("Not connected.")),
    }
}

fn write_data(connection: &ConnectionState, address: u16, data: &[u8]) -> Result<(), String> {
    match &connection.device {
        Some(device) => {
            let mut packet: [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            let mut remaining = data.len();
            for offset in (0..data.len()).step_by(REPORT_LENGTH - 5) {
                let write_length = cmp::min(remaining, REPORT_LENGTH - 5);
                packet[0] = 0x0; // First byte of the first message is the Report ID
                packet[1] = MaxTouchCommand::Write as u8;               // Command
                packet[2] = ((address + offset as u16) & 0xff) as u8;   // Address Low
                packet[3] = ((address + offset as u16) >> 8) as u8;     // Address High
                packet[4] = write_length as u8;                         // Length
                packet[5..(5 + write_length)].clone_from_slice(&data[offset..(offset + write_length)]);

                match device.write(&packet) {
                    Ok(_) => {},
                    Err(_) => { return Err(format!("Failed to write to the device.")); }
                }
                match device.read_timeout(&mut packet, 1000) {
                    Ok(size) => {
                        if size > 0 && packet[0] != MaxTouchStatus::OK as u8 {
                            return Err(format!("Device reported an error ({})", packet[0]));
                        }
                        remaining -= write_length;
                    }
                    Err(e) => {
                        return Err(format!("Read returned error {}", e));
                    }
                }
            }
            return Ok(());
        }
        _ => return Err(format!("Not connected.")),
    }
}

fn read_object_impl(connection: &ConnectionState, id: u8) -> Result<Vec<u8>, String> {
    if connection.object_table.contains_key(&id) {
        return read_data(connection, connection.object_table[&id].address, connection.object_table[&id].size);
    }
    Err(format!("Object {} not found", id))
}

fn write_object_impl(connection: &ConnectionState, id: u8, data: &[u8]) -> Result<(), String> {
    if connection.object_table.contains_key(&id) {
        return write_data(connection, connection.object_table[&id].address, data);
    }
    Err(format!("Object {} not found", id))
}

#[tauri::command]
fn read_object(connection_state: State<Mutex<ConnectionState>>, id: u8) -> Result<String, String> {
    let mut connection = connection_state.lock();
    let data = read_object_impl(&connection, id)?;
    match id {
        7 => {
            let t7 = T7PowerConfig::ref_from_prefix(&data).expect("Could not create T7PowerConfig");
            let json_str = serde_json::to_string(&t7).expect("Could not serialize T7PowerConfig");
            return Ok(json_str);
        }
        8 => {
            let t8 = T8AcquisitionConfig::ref_from_prefix(&data).expect("Could not create T8AcquisitionConfig");
            let json_str = serde_json::to_string(&t8).expect("Could not serialize T8AcquisitionConfig");
            return Ok(json_str);
        }
        25 => {
            let t25 = T25SelfTest::ref_from_prefix(&data).expect("Could not create T25SelfTest");
            let json_str = serde_json::to_string(&t25).expect("Could not serialize T25SelfTest");
            return Ok(json_str);
        }
        42 => {
            let t42 = T42TouchSupression::ref_from_prefix(&data).expect("Could not create T42TouchSupression");
            let json_str = serde_json::to_string(&t42).expect("Could not serialize T42TouchSupression");
            return Ok(json_str);
        }
        46 => {
            let t46 = T46CteConfig::ref_from_prefix(&data).expect("Could not create T46CteConfig");
            let json_str = serde_json::to_string(&t46).expect("Could not serialize T46CteConfig");
            return Ok(json_str);
        }
        47 => {
            let t47 = T47ProciStylus::ref_from_prefix(&data).expect("Could not create T47ProciStylus");
            let json_str = serde_json::to_string(&t47).expect("Could not serialize T47ProciStylus");
            return Ok(json_str);
        }
        56 => {
            // This object is variable length, for now pad it up to the object size for the 1066 IC.
            let mut data_padded = Vec::from(data);
            data_padded.resize(mem::size_of::<T56Shieldless>(), 0);
            let t56 = T56Shieldless::ref_from_prefix(&data_padded).expect("Could not create T56Shieldless");
            let json_str = serde_json::to_string(&t56).expect("Could not serialize T56Shieldless");
            return Ok(json_str);
        }
        65 => {
            let t65 = T65LensBending::ref_from_prefix(&data).expect("Could not create T65LensBending");
            let json_str = serde_json::to_string(&t65).expect("Could not serialize T65LensBending");
            return Ok(json_str);
        }
        80 => {
            let t80 = T80RetransmissionCompensation::ref_from_prefix(&data).expect("Could not create T80RetransmissionCompensation");
            let json_str = serde_json::to_string(&t80).expect("Could not serialize T80RetransmissionCompensation");
            return Ok(json_str);
        }
        100 => {
            let t100 = T100MultipleTouchTouchscreen::ref_from_prefix(&data).expect("Could not create T100MultipleTouchTouchscreen");
            let json_str = serde_json::to_string(&t100).expect("Could not serialize T100MultipleTouchTouchscreen");
            connection.invert_x = (t100.cfg1 & 0x80) != 0;
            connection.invert_y = (t100.cfg1 & 0x40) != 0;
            connection.switch_xy = (t100.cfg1 & 0x20) != 0;
            println!("Rotation information: Invert X {}, Invert Y {}, Switch XY {}, Sensor {}x{}", connection.invert_x, connection.invert_y, connection.switch_xy, connection.sensor_size[0], connection.sensor_size[1]);
            return Ok(json_str);
        }
        _ => {
            return Err(format!("Object type {} is not serializable", id))
        }
    }
}

#[tauri::command]
fn write_register(connection_state: State<Mutex<ConnectionState>>, id: u8, offset: u8, data: Vec<u8>) -> Result<(), String> {
    let connection = connection_state.lock();
    if offset + data.len() as u8 >= connection.object_table[&id].size {
        return Err(format!("Attempt to write off the end of object {}.", id));
    }
    write_data(&connection, connection.object_table[&id].address + offset as u16, &data)
}

#[tauri::command]
fn get_debug_image(connection_state: State<Mutex<ConnectionState>>, mode: u8, low: i16, high: i16) -> Result<Response, String> {
    let connection = connection_state.lock();
    let width;
    let height;
    if connection.switch_xy {
        width = connection.sensor_size[1] as u32;
        height = connection.sensor_size[0] as u32;
    }
    else {
        width = connection.sensor_size[0] as u32;
        height = connection.sensor_size[1] as u32;
    }
    let mut img = RgbImage::new(width, height);
    let mut encoded_image = Vec::new();

    let mut t6: T6CommandProcessor = FromZeroes::new_zeroed();
    t6.diagnostic = mode;
    write_object_impl(&connection, 6, t6.as_bytes())?;
    t6.diagnostic = 1; // Next page

    let sensor_nodes = connection.sensor_size[0] as u16 * connection.sensor_size[1] as u16;

    let pages = ((sensor_nodes * 2) as f32 / 128.0).ceil() as u8;
    let mut min_sample = i16::MAX;
    let mut max_sample = i16::MIN;
    for page in 0..pages {
        let mut data = read_object_impl(&connection, 37)?;
        if data[0] != 37 && data[1] != page {
            // Retry if the page hasnt updated
            data = read_object_impl(&connection, 37)?;
        }
        if page != pages - 1 {
            write_object_impl(&connection, 6, t6.as_bytes())?;
        }
        for index in (0..128).step_by(2) {
            let full_index = ((page as u32 * 128) + index) / 2;
            if full_index < sensor_nodes as u32 {
                let mut x = full_index / (connection.sensor_size[1] as u32);
                let mut y = full_index % (connection.sensor_size[1] as u32);
                if connection.invert_x {
                    x = width - x - 1;
                }
                if connection.invert_y {
                    y = height - y - 1;
                }
                if connection.switch_xy {
                    let tmp = x;
                    x = y;
                    y = tmp;
                }

                let sample = i16::from_le_bytes(data[(index + 2) as usize .. (index + 4) as usize].try_into().unwrap());
                min_sample = cmp::min(min_sample, sample);
                max_sample = cmp::max(max_sample, sample);

                let normalized_sample : f32;
                if low < 0 {
                    // If the low-high range goes negative, generate a normalized
                    // value in the range -1..1.
                    let range = cmp::max(-low, high);
                    normalized_sample = (sample as f32 / range as f32).clamp(-1.0, 1.0);
                }
                else {
                    normalized_sample = ((sample - low) as f32 / (high - low) as f32).clamp(0.0, 1.0);
                }

                if normalized_sample < 0.0 {
                    let value = 255 - (-255.0 * normalized_sample) as u8;
                    img.put_pixel(x, y, Rgb([255, value, value]));
                }
                else {
                    let value = 255 - (255.0 * normalized_sample) as u8;
                    img.put_pixel(x, y, Rgb([value, value, 255]));
                }
            }
        }
    }
    /*
    // TODO: I would like to display this in the UI, but how do we return
    // an image and some json?
    if min_sample < low {
        println!("Sample was out of range {} < {}", min_sample, low); 
    }
    if max_sample > high {
        println!("Sample was out of range {} > {}", max_sample, high); 
    }
    println!("Sample range {} .. {}", min_sample, max_sample);*/

    let encoder = PngEncoder::new(&mut encoded_image);
    encoder
        .write_image(&img, width, height, image::ExtendedColorType::Rgb8)
        .unwrap();
    Ok(Response::new(encoded_image))
}

#[tauri::command]
fn connect(connection_state: State<Mutex<ConnectionState>>) -> Result<InformationBlock, String> {
    let mut connection = connection_state.lock();
    connection.device = None;

    match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == VENDOR_ID
                    && device.product_id() == PRODUCT_ID
                    && device.usage() == USAGE
                    && device.usage_page() == USAGE_PAGE
                {
                    println!(
                        "Found device: {} {}",
                        device.manufacturer_string().unwrap(),
                        device.product_string().unwrap()
                    );
                    match device.open_device(&api) {
                        Ok(device) => {
                            connection.device = Some(device);
                            check_version(&connection).expect("Version Check Failed");
                            let data =
                                read_data(&connection, 0, mem::size_of::<InformationBlock>() as u8)
                                    .expect("Failed to read info block.");
                            let info = InformationBlock::ref_from_prefix(&data)
                                .expect("Could not parse info block");
                            for index in 0..info.num_objects {
                                let object_data = read_data(
                                    &connection,
                                    mem::size_of::<InformationBlock>() as u16
                                        + mem::size_of::<ObjectTableElement>() as u16
                                            * index as u16,
                                    mem::size_of::<ObjectTableElement>() as u8,
                                )
                                .expect("Failed to read info block.");

                                connection.sensor_size[0] = info.matrix_x_size;
                                connection.sensor_size[1] = info.matrix_y_size;

                                let object = ObjectTableElement::ref_from_prefix(&object_data)
                                    .expect("Could not object table element.");
                                connection.object_table.insert(
                                    object.object_type,
                                    ObjectDetails {
                                        address: ((object.position_ms_byte as u16) << 8)
                                            | object.position_ls_byte as u16,
                                        size: object.size_minus_one + 1,
                                        instances: object.instances_minus_one + 1,
                                    },
                                );
                            }
                            println!("{:?}", connection.object_table);
                            return Ok(info.clone());
                        }
                        Err(e) => {
                            return Err(format!("Error opening the device: {}", e));
                        }
                    }
                }
            }
            Err(format!("No device found"))
        }
        Err(e) => {
            Err(format!(
                "Error reading from the configuration interface: {}",
                e
            ))
        }
    }
}

#[tauri::command]
fn reboot_bootloader(connection_state: State<Mutex<ConnectionState>>) -> Result<(), String> {
    let connection = connection_state.lock();
    match &connection.device {
        Some(device) => {
            let mut packet : [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            packet[1] = MaxTouchCommand::Command as u8;
            packet[2] = MaxTouchCommandType::RebootBootloader as u8;
            match device.write(&packet) {
                Ok(_) => return Ok(()),
                Err(_) => { return Err(format!("Failed to write to the device.")); }
            }
            // Dont expect a response, the device has rebooted
        }
        _ => {
            return Err(format!("Not connected."));
        }
    }
}

#[tauri::command]
fn set_mouse_mode(connection_state: State<Mutex<ConnectionState>>, enable: bool) -> Result<(), String> {
    let connection = connection_state.lock();
    match &connection.device {
        Some(device) => {
            let mut packet : [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            packet[1] = MaxTouchCommand::Command as u8;
            packet[2] = MaxTouchCommandType::SetMouseMode as u8;
            packet[3] = enable as u8;
            match device.write(&packet) {
                Ok(_) => {},
                Err(_) => return Err(format!("Failed to write to the device."))
            }
            match device.read_timeout(&mut packet, 1000) {
                Ok(size) => {
                    if size > 0 && packet[0] == MaxTouchStatus::OK as u8 {
                        return Ok(());
                    }
                    return Err(format!("Failed to set mouse mode. Error {}", packet[0]));
                }
                Err(e) => {
                    return Err(format!("Read returned error {}", e));
                }
            }
        }
        _ => {
            return Err(format!("Not connected."));
        }
    }
}

#[tauri::command]
fn get_mouse_mode(connection_state: State<Mutex<ConnectionState>>) -> Result<(bool), String> {
    let connection = connection_state.lock();
    match &connection.device {
        Some(device) => {
            let mut packet : [u8; REPORT_LENGTH] = [0; REPORT_LENGTH];
            packet[1] = MaxTouchCommand::Command as u8;
            packet[2] = MaxTouchCommandType::GetMouseMode as u8;
            match device.write(&packet) {
                Ok(_) => {},
                Err(_) => return Err(format!("Failed to write to the device."))
            }
            match device.read_timeout(&mut packet, 1000) {
                Ok(size) => {
                    if size > 0 && packet[0] == MaxTouchStatus::OK as u8 {
                        return Ok(packet[1] != 0);
                    }
                    return Err(format!("Failed to get mouse mode. Error {}", packet[0]));
                }
                Err(e) => {
                    return Err(format!("Read returned error {}", e));
                }
            }
        }
        _ => {
            return Err(format!("Not connected."));
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(ConnectionState::default()))
        .invoke_handler(tauri::generate_handler![connect, get_debug_image, write_register, read_object, reboot_bootloader, set_mouse_mode, get_mouse_mode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
