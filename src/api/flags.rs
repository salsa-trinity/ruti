#[derive(Clone)]
pub struct ApiFlags {
    // TODO: rewrite it to be an enum where each element is a sruct for each commands available flags
    pub pl: bool,
    pub len: f64,
    pub len_defined: bool,
    pub name: String,
    pub name_defined: i32,
    pub u_time: f64,
    pub u_time_defined: i32,
}

impl ApiFlags {
    pub fn new() -> ApiFlags {
        ApiFlags {
            pl: false,
            len: 0f64,
            len_defined: false,
            name: String::new(),
            name_defined: 0,
            u_time: 60f64,
            u_time_defined: 0,
        }
    }
}
