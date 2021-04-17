pub enum PluginError {
    FromDesign(DesignPlgError),
    FromPDK(PDKPlgError),
}

pub enum DesignPlgError {
    ClockNetNotFound,
}

pub enum PDKPlgError {
    LoginFailed,
    LostConnection,
}
