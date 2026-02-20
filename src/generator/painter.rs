use crate::plate::Plate;

pub trait Painter {
    fn paint(&self, plate: Plate) -> String;
}
