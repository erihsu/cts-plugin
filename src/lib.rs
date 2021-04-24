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
    fn get_clock_sinks(&self, clk: &str) -> CTSPluginRes<Vec<(String, (i32, i32))>>;
    // given clock net name, clock net Route Definition, source component name, sink component name, change the internal design
    fn add_clock_net(&mut self, net: &[Path],from:(String,(i32,i32)),to:Vec<(String,(i32,i32))>) -> CTSPluginRes<String>;
    // given buffer model, buffer location and orientation, change the internal design
    fn add_intermediate_buffer(&mut self,model: &str,location: (i32, i32)) -> CTSPluginRes<String>;
    // add root buffer. this will automatically add the net between root buffer and clock source.
    fn add_root_buffer(&mut self,model:&str,location:(i32,i32)) -> CTSPluginRes<String>;
    // export standard def file
    fn export_def(&self, path: &str) -> CTSPluginRes<()>;
    // import standard def file to load design , make sure first prepare layer map and via map
    fn import_def(&mut self, path: &str) -> CTSPluginRes<()>;
    
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
