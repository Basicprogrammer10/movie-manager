use std::path::PathBuf;

pub mod show;
pub mod new_show;

pub trait ShowInterface {
    fn name(&self) -> String;
    fn path(&self) -> PathBuf;
}