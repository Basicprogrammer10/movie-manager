pub mod show;
pub mod new_show;

pub trait ShowInterface {
    fn name(&self) -> String;
}