use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, FromZeroes, AsBytes};

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, Serialize, Deserialize, Debug, Clone)]
pub struct InformationBlock {
    pub family_id : u8,
    pub variant_id : u8,
    pub version : u8,
    pub build : u8,
    pub matrix_x_size : u8,
    pub matrix_y_size : u8,
    pub num_objects : u8
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, Serialize, Deserialize, Debug, Clone)]
pub struct ObjectTableElement {
    pub object_type : u8,
    pub position_ls_byte : u8,
    pub position_ms_byte : u8,
    pub size_minus_one : u8,
    pub instances_minus_one : u8,
    pub report_ids_per_instance : u8
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T6CommandProcessor {
    pub reset : u8,
    pub backupnv : u8,
    pub calibrate : u8,
    pub reportall : u8,
    pub debugctrl : u8,
    pub diagnostic : u8,
    pub debugctrl2 : u8
}
