pub enum PluginError {
    FromDesign(DesignPluginError),
    FromPdk(PdkPluginError),
}

pub enum PdkPluginError {
    Login,
    LoadSinkCap,
    LoadBufferPower,
    LoadBufferTiming,
    LoadClockBuffers,
    LoadLayerMap,
    LoadViaMap,
    LoadRoutingRes,
    LoadRoutingCap,
}

pub enum DesignPluginError {
    LoadSinks,
    SetLayerMap,
    SetViaMap,
    SetClockNetSeg,
    SetClockBuffer,
}

//todo
// implement Error::source()
