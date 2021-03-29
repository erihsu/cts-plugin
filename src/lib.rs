pub trait DEFPlugin {
    fn export_def(&self) -> String;
    fn import_def<F>(file: F) -> Self;
}

pub trait PDBPlugin {}
