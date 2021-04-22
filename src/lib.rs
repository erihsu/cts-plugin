use std::collections::HashMap;
use serde_json::Value;
use std::error::Error;

pub trait PdkPlugin {
    // given model name , get clk pin input capacitance
    fn login(&mut self, username: &str, password: &str) -> CTSPluginRes<()>;
    fn logout(&mut self) -> CTSPluginRes<()>;
    fn get_sink_cap(&self, name: &str) -> CTSPluginRes<f32>;
    // given model name and placement, get its clk pin offset. The offset is mapping between orient code and coords
    // refer orient code to vulcan/src/parser/encoder.rs
    fn get_sink_clk_pin_offset(&self, name: &str) -> CTSPluginRes<HashMap<i8,(f32, f32)>>;
    // given model name, get buffer detailed information which includes power and timing
    fn get_buffer(&self, name: &str) -> CTSPluginRes<Value>;
    // list all avaliale buffer for clock tree synthesis
    fn list_all_clock_buffer(&self) -> CTSPluginRes<Vec<String>>;
    // get metal layer unit resistance
    fn get_unit_resistance(&self) -> CTSPluginRes<f32>; // fF
    // get metal layer unit capacitance
    fn get_unit_capacitance(&self) -> CTSPluginRes<f32>; // kohm
}

pub trait DesignPlugin {
    // login
    fn login(&mut self, username: &str, password: &str) -> CTSPluginRes<()>;
    fn logout(&mut self) -> CTSPluginRes<()>;
    // given clock network name, return a set of sinks. The information include model name, sink location and sink placement orient
    fn get_clock_sinks(&self, clk: &str) -> CTSPluginRes<Vec<(String, (i32, i32), i8)>>;
    // get clock input source, aka. CLK PIN
    fn get_clock_source(&self, clk: &str) -> CTSPluginRes<(i32, i32)>;
    // given clock net name, clock net Route Definition, change the internal design
    fn add_clock_net(&mut self, net_name: &str, net: &Vec<Route>) -> CTSPluginRes<()>;
    // given buffer model, buffer location and orientation, change the internal design
    fn add_clock_buffer(
        &mut self,
        model: &str,
        location: (i32, i32),
        orient: i8,
    ) -> CTSPluginRes<()>;
    // export standard def file
    fn export_def(&self, path: &str) -> CTSPluginRes<()>;
    // import standard def file to load design , make sure first prepare layer map and via map
    fn import_def(&mut self, path: &str) -> CTSPluginRes<()>;
}

pub type CTSPluginRes<T> = Result<T, Box<dyn Error>>;

pub struct Route<'a> {
    pub layer: &'a str,
    pub element: Vec<Element<'a>>,
}

pub enum Element<'a> {
    Path(Path),
    Via(Via<'a>),
}

pub struct Path {
    pub from: (i32, i32),
    pub to: (i32, i32),
}

pub struct Via<'a> {
    pub coord: (i32, i32),
    pub vname: &'a str,
}
