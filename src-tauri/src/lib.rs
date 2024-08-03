extern crate hidapi;
use hidapi::{HidApi, HidDevice};
use maxtouch::{InformationBlock, ObjectTableElement, T6CommandProcessor};
use parking_lot::Mutex;
use std::{cmp, mem};
use std::collections::HashMap;
use std::default::Default;
use tauri::State;
use zerocopy::{FromBytes, FromZeroes, AsBytes};
use tauri::ipc::Response;
use image::{codecs::png::PngEncoder, Rgb, RgbImage, ImageEncoder};

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
    Bootloader,
    Read,
    Write,
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

            device.write(&data).expect("Failed to write to the device");
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

                device.write(&packet).expect("Failed to write to the device");
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

fn read_object(connection: &ConnectionState, id: u8) -> Result<Vec<u8>, String> {
    read_data(connection, connection.object_table[&id].address, connection.object_table[&id].size)
}

fn write_object(connection: &ConnectionState, id: u8, data: &[u8]) -> Result<(), String> {
    write_data(connection, connection.object_table[&id].address, data)
}

#[tauri::command]
fn get_debug_image(connection_state: State<Mutex<ConnectionState>>, mode: u8) -> Response {
    let connection = connection_state.lock();
    match &connection.device {
        Some(device) => {
            let mut img = RgbImage::new(connection.sensor_size[0] as u32, connection.sensor_size[1] as u32);
            let mut encoded_image = Vec::new();

            let mut t6: T6CommandProcessor = FromZeroes::new_zeroed();
            t6.diagnostic = mode;
            write_object(&connection, 6, t6.as_bytes());
            t6.diagnostic = 1; // Next page

            let sensor_nodes = (connection.sensor_size[0] as u16 * connection.sensor_size[1] as u16);
            let pages = ((sensor_nodes * 2) as f32 / 128.0).ceil() as u8;

            for page in 0..pages {
                let mut data = read_object(&connection, 37).expect("Failed to read debug object");
                if data[0] != 37 && data[1] != page {
                    // Retry if the page hasnt updated
                    data = read_object(&connection, 37).expect("Failed to read debug object");
                }
                if page != pages - 1 {
                    write_object(&connection, 6, t6.as_bytes());
                }
                for index in (0..128).step_by(2) {
                    let full_index = ((page as u32 * 128) + index) / 2;
                    if full_index < sensor_nodes as u32 {
                        let x = full_index / (connection.sensor_size[1] as u32);
                        let y = full_index % (connection.sensor_size[1] as u32);

                        let sample = i16::from_le_bytes(data[(index + 2) as usize .. (index + 4) as usize].try_into().unwrap());
                        if sample < 0 {
                            let value = 255 - (-sample / 3) as u8;
                            img.put_pixel(x, y, Rgb([255, value, value]));
                        }
                        else {
                            let value = 255 - ((sample / 3) & 0xFF) as u8;
                            img.put_pixel(x, y, Rgb([value, value, 255]));
                        }
                    }
                }
            }

            let encoder = PngEncoder::new(&mut encoded_image);
            encoder
                .write_image(&img, connection.sensor_size[0] as u32, connection.sensor_size[1] as u32, image::ExtendedColorType::Rgb8)
                .unwrap();
            Response::new(encoded_image)
        }
        _ => {
            // Not connected, send a single pixel image
            let mut img = RgbImage::new(1, 1);
            let mut encoded_image = Vec::new();
            img.put_pixel(0, 0, Rgb([255, 255, 255]));
            let encoder = PngEncoder::new(&mut encoded_image);
            encoder
                .write_image(&img, 1, 1, image::ExtendedColorType::Rgb8)
                .unwrap();
            Response::new(encoded_image)
        }
    }
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
                            // println!("{:?}", connection.object_table);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(ConnectionState::default()))
        .invoke_handler(tauri::generate_handler![connect, get_debug_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
