
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlugInfo {
    pub system: Option<System>,
    pub emeter: Option<Emeter>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Emeter {
    pub get_realtime: Option<GetRealtime>,
    pub get_daystat: Option<GetDaystat>,
    pub get_vgain_igain: Option<GetVgainIgain>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRealtime {
    pub voltage_mv: i64,
    pub current_ma: i64,
    pub power_mw: i64,
    pub total_wh: i64,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDaystat {
    pub day_list: Vec<EmeterGetDaystatDayList>,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmeterGetDaystatDayList{
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub energy: f64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetVgainIgain {
    pub vgain: i64,
    pub igain: i64,
    pub err_code: i64,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct System {
    pub get_sysinfo: GetSysinfo,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSysinfo {
    pub active_mode: String,
    pub alias: String,
    pub dev_name: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub err_code: i64,
    pub feature: String,
    #[serde(rename = "fwId")]
    pub fw_id: String,
    #[serde(rename = "hwId")]
    pub hw_id: String,
    pub hw_ver: String,
    pub icon_hash: String,
    pub latitude_i: f64,
    pub led_off: i64,
    pub longitude_i: f64,
    pub mac: String,
    pub model: String,
    //pub next_action: String, // not sure what this is and what data it may contain
    #[serde(rename = "oemId")]
    pub oem_id: String,
    pub on_time: i64,
    pub relay_state: i64,
    pub rssi: i64,
    pub sw_ver: String,
    #[serde(rename = "type")]
    pub hw_type: String,
    pub updating: i64,
}