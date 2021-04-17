pub enum PluginError {
	FromDesign(DesignPlgError),
	FromPDK(PDKPlgError),
}

enum DesignPlgError {
	ClockNetNotFound,

}

enum PDKPlgError {
	LoginFailed,
	LostConnection,
	
}