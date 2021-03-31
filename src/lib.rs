use serde_json::Value;
use std::error::Error;
use std::io::{Read, Write};
mod error;
pub use error::*;
pub trait DEFPlugin {
    fn export_def<W: Write>(&self, f: &mut W);
    fn import_def<R: Read>(&mut self, f: &mut R);
}

pub trait PdkPlugin {
    fn login(&mut self) -> CTSPluginRes<()>;
    fn get_sink_cap(&self, name: &str) -> CTSPluginRes<f32>;
    fn get_buffer(&self, name: &str) -> CTSPluginRes<Value>;
    fn list_all_clock_buffer(&self) -> CTSPluginRes<Vec<String>>;
    fn get_clock_routing_layer_resistance(&self) -> CTSPluginRes<f32>;
    fn get_clock_routing_layer_capacitance(&self) -> CTSPluginRes<f32>;
    fn get_layer_map(&self) -> CTSPluginRes<Vec<(i16, String)>>;
    fn get_via_map(&self) -> CTSPluginRes<Vec<(i16, String)>>;
}

pub trait DesignPlugin {
    fn get_clock_sinks(&self, clk: &str) -> CTSPluginRes<Vec<(String, (i32, i32), i8)>>;
    fn add_clocknet_seg(
        &mut self,
        name: &str,
        seg: Vec<((i32, i32), Vec<(i32, i32)>)>,
    ) -> CTSPluginRes<()>;
    fn add_clock_buffer(
        &mut self,
        name: &str,
        model: &str,
        location: (i32, i32),
        orient: i8,
    ) -> CTSPluginRes<()>;
    fn prepare_layer_map(&mut self, map: Vec<(String, i16)>) -> CTSPluginRes<()>;
    fn prepare_via_map(&mut self, map: Vec<(String, i16)>) -> CTSPluginRes<()>;
}

pub type CTSPluginRes<T> = Result<T, Box<dyn Error>>;
