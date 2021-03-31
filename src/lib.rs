use std::io::{Read, Write};

pub trait DEFPlugin {
    fn export_def<W: Write>(&self, f: &mut W);
    fn import_def<R: Read>(&mut self, f: &mut R);
}

pub trait PdkPlugin {
    fn login(&mut self);
    fn get_sink_cap(&self, name: &str) -> f32;
    fn get_buffer_power_model<T>(&self, name: &str) -> T;
    fn get_buffer_timing_model<T>(&self, name: &str) -> T;
    fn list_all_clock_buffer(&self) -> Vec<String>;
    fn get_clock_routing_layer_resistance(&self) -> f32;
    fn get_clock_routing_layer_capacitance(&self) -> f32;
    fn get_layer_map(&self) -> Vec<(i16, String)>;
    fn get_via_map(&self) -> Vec<(i16, String)>;
}

pub trait DesignPlugin {
    fn get_clock_sinks(&self, clk: &str) -> Vec<(String, (i32, i32), i8)>;
    fn add_clocknet_seg(&mut self, name: &str, seg: Vec<((i32, i32), Vec<(i32, i32)>)>);
    fn add_clock_buffer(&mut self, name: &str, model: &str, location: (i32, i32), orient: i8);
    fn prepare_layer_map(&mut self, map: Vec<(String, i16)>);
    fn prepare_via_map(&mut self, map: Vec<(String, i16)>);
}
