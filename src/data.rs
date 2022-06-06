#![allow(unused)]

pub mod constants {
    pub const APP_TITLE: &str = "Maintenance Tracker";
    pub const APP_VERSION: &str = "1.0.0";
    pub const CONFIG_FILE: &str = "config.json";
    pub const CREDENTIALS_FILE: &str = "credentials.json";
    pub const CREDENTIALS_INVALID_MSG: &str = "ERROR: Invalid login credentials, please try again.";
    pub const MAX_CATEGORY_TITLE_LEN: u8 = 30;
    pub const MAX_ITEM_TITLE_LEN: u8 = 30;
    pub const MAX_ENTRY_NOTE_LEN: u32 = 65_535;
    pub const REFERENCE_ID_CATEGORY: Option<&'static str> = None;
    pub const REFERENCE_ID_ENTRY: Option<&'static str> = Some("item_id");
    pub const REFERENCE_ID_ITEM: Option<&'static str> = Some("category_id");
    pub const SAVED_CREDENTIALS_INVALID_MSG: &str = "ERROR: Saved login credentials are invalid, please run with the -s flag and enter the correct information.";
    pub const TABLE_NAME_CATEGORY: &str = "category";
    pub const TABLE_NAME_ENTRY: &str = "entry";
    pub const TABLE_NAME_ITEM: &str = "item";
}
