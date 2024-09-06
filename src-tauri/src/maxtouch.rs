use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, FromZeroes, AsBytes};
use serde_big_array::BigArray;

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

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T7PowerConfig {
    pub idleacqint : u8,
    pub actacqint : u8,
    pub actv2idelto : u8,
    pub cfg : u8,
    pub cfg2 : u8,
    pub idleacqintfine : u8,
    pub actvaqintfine : u8
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T8AcquisitionConfig {
    pub chrgtime : u8,
    pub reserved : u8,
    pub tchdrift : u8,
    pub driftst : u8,
    pub tchautocal : u8,
    pub sync : u8,
    pub atchcalst : u8,
    pub atchcalsthr : u8,
    pub atchfrccalthr : u8,
    pub atchfrccalratio : u8,
    pub measallow : u8,
    pub reserved2 : [u8; 3],
    pub cfg : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T42TouchSupression {
    pub ctrl : u8,
    pub reserved : u8,
    pub maxapprarea : u8,
    pub maxtcharea : u8,
    pub supstrength : u8,
    pub supextto : u8,
    pub maxnumtchs : u8,
    pub shapestrength : u8,
    pub supdist : u8,
    pub disthyst : u8,
    pub maxscrnarea : u8,
    pub cfg : u8,
    pub reserved2 : u8,
    pub edgesupstrength : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T46CteConfig {
    pub reserved : [u8; 2],
    pub idlesyncsperx : u8,
    pub activesyncsperx : u8,
    pub adcspersync : u8,
    pub pulsesperadc : u8,
    pub xslew : u8,
    pub syncdelay : i16,
    pub xvoltage : u8,
    pub reserved2 : u8,
    pub inrushcfg : u8,
    pub reserved3 : [u8; 6],
    pub cfg : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T47ProciStylus {
    pub ctrl : u8,
    pub reserved : u8,
    pub contmax : u8,
    pub stability : u8,
    pub maxtcharea : u8,
    pub amplthr : u8,
    pub styshape : u8,
    pub hoversup : u8,
    pub confthr : u8,
    pub syncsperx : u8,
    pub xposadj : u8,
    pub yposadj : u8,
    pub cfg : u8,
    pub reserved2 : [u8; 7],
    pub supstyto : u8,
    pub maxnumsty : u8,
    pub xedgectrl : u8,
    pub yedgectrl : u8,
    pub supto : u8,
    pub supclassmode : u8,
    pub dxxedgectrl : u8,
    pub dxxedgedist : u8,
    pub xedgectrlhi : u8,
    pub xedgedisthi : u8,
    pub dxxedgectrlhi : u8,
    pub dxxedgedisthi : u8,
    pub yedgectrlhi : u8,
    pub yedgedisthi : u8,
    pub cfg2 : u8,
    pub movfilter : u8,
    pub movsmooth : u8,
    pub movpred : u8,
    pub satbxlo : u8,
    pub satbxhi : u8,
    pub satbylo : u8,
    pub satbyhi : u8,
    pub satbdxxlo : u8,
    pub satbdxxhi : u8,
    pub movhistcfg : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T56Shieldless {
	pub ctrl : u8,
	pub reserved : u8,
	pub optint : u8,
	pub inttime : u8,
    #[serde(with = "BigArray")]
	pub intdelay : [u8; 41],
    // TODO: Variable sized array - depends on the number of transmit pins..
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T65LensBending {
    pub ctrl : u8,
    pub gradthr : u8,
    pub ylonoisemul : u16,
    pub ylonoisediv : u16,
    pub yhinoisemul : u16,
    pub yhinoisediv : u16,
    pub lpfiltcoef : u8,
    pub forcescale : u16,
    pub forcethr : u8,
    pub forcethrhyst : u8,
    pub forcedi : u8,
    pub forcehyst : u8,
    pub atchratio : u8,
    pub reserved : [u8; 2],
    pub exfrcthr : u8,
    pub exfrcthrhyst : u8,
    pub exfrcto : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T80RetransmissionCompensation {
    pub ctrl : u8,
    pub compgain : u8,
    pub targetdelta : u8,
    pub compthr : u8,
    pub atchthr : u8,
    pub moistcfg : u8,
    pub reserved : u8,
    pub moistthr : u8,
    pub moistinvtchthr : u8,
    pub moistcfg2 : u8,
    pub compstrthr : u8,
    pub compcfg : u8,
    pub moistvldthrsf : u8,
    pub moistcfg3 : u8,
    pub moistdegthr : u8,
}

#[repr(C, packed)]
#[derive(FromBytes, FromZeroes, AsBytes, Serialize, Deserialize, Debug, Clone)]
pub struct T100MultipleTouchTouchscreen {
    pub ctrl : u8,
    pub cfg1 : u8,
    pub scraux : u8,
    pub tchaux : u8,
    pub tcheventcfg : u8,
    pub akscfg : u8,
    pub numtch : u8,
    pub xycfg : u8,
    pub xorigin : u8,
    pub xsize : u8,
    pub xpitch : u8,
    pub xlocip : u8,
    pub xhiclip : u8,
    pub xrange : u16,
    pub xedgecfg : u8,
    pub xedgedist : u8,
    pub dxxedgecfg : u8,
    pub dxxedgedist : u8,
    pub yorigin : u8,
    pub ysize : u8,
    pub ypitch : u8,
    pub ylocip : u8,
    pub yhiclip : u8,
    pub yrange : u16,
    pub yedgecfg : u8,
    pub yedgedist : u8,
    pub gain : u8,
    pub dxgain : u8,
    pub tchthr : u8,
    pub tchhyst : u8,
    pub intthr : u8,
    pub noisesf : u8,
    pub cutoffthr : u8,
    pub mrgthr : u8,
    pub mrgthradjstr : u8,
    pub mrghyst : u8,
    pub dxthrsf : u8,
    pub tchdidown : u8,
    pub tchdiup : u8,
    pub nexttchdi : u8,
    pub calcfg : u8,
    pub jumplimit : u8,
    pub movfilter : u8,
    pub movsmooth : u8,
    pub movpred : u8,
    pub movhysti : u16,
    pub movhystn : u16,
    pub amplhyst : u8,
    pub scrareahyst : u8,
    pub intthryst : u8,
    pub xedgecfghi : u8,
    pub xedgedisthi : u8,
    pub dxxedgecfghi : u8,
    pub dxxedgedisthi : u8,
    pub yedgecfghi : u8,
    pub yedgedisthi : u8,
    pub cfg2 : u8,
    pub movhystcfg : u8,
    pub amplcoeff : u8,
    pub amploffset : u8,
    pub jumplimitmov : u8,
    pub jlmmovthr : u16,
    pub jlmmovintthr : u8
}