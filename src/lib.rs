// use std::collections::HashMap;
use serde_json::Value;
use std::error::Error;

pub trait CTSPlugin {
    /// Design related api
    // login
    fn login(&mut self, username: &str, password: &str) -> CTSPluginRes<()>;
    // logout
    fn logout(&mut self) -> CTSPluginRes<()>;
    // given clock network name, return a set of sinks. The information include sink name, sink location
    fn get_clock_sinks(&self, clk:&str) -> CTSPluginRes<Vec<(String, (i32, i32))>>;
    // given clock net name, clock net Route Definition, change the internal design
    fn update_clock_net(&mut self,net_name:&str, net_data: &[Path]) -> CTSPluginRes<()>;
    // given buffer model, buffer location, net name being hold up, change the internal design, return new clock net name
    fn insert_clock_buffer(&mut self,buffer_name:&str, model:&str, location:(i32, i32),net_name:&str) -> CTSPluginRes<String>;
    // export standard def file
    fn export_def(&self, path: &str) -> CTSPluginRes<()>;
    // export verilog netlist
    fn export_verilog(&self,path: &str) -> CTSPluginRes<()>;
    // import standard def file to load design , make sure first prepare layer map and via map
    fn import_def(&mut self, path: &str) -> CTSPluginRes<()>;
    // import verilog netlist
    fn import_verilog(&mut self,path:&str) -> CTSPluginRes<()>;
    
    /// Pdk related api
    // given model name, get buffer detailed information which includes power and timing
    fn get_buffer(&self, name: &str) -> CTSPluginRes<Value>;
    // list all avaliale buffer for clock tree synthesis
    fn list_all_clock_buffer(&self) -> CTSPluginRes<Vec<String>>;
    // get metal layer unit resistance
    fn get_unit_resistance(&self) -> CTSPluginRes<f32>; // fF
    // get metal layer unit capacitance
    fn get_unit_capacitance(&self) -> CTSPluginRes<f32>; // kohm
}

pub type CTSPluginRes<T> = Result<T, Box<dyn Error>>;

pub struct Path {
    pub from: (i32, i32),
    pub turn: Option<(i32,i32)>,
    pub to: (i32, i32),
}
