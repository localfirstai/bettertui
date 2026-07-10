use bettertui_engine::VERSION;

#[napi_derive::napi]
pub fn get_version() -> String {
    VERSION.to_string()
}
