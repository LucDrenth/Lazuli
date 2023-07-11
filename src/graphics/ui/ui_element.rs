use crate::graphics::material::Material;

use super::Interface;

pub trait UiElement {
    fn material<'a>(&'a self, interface: &'a Interface) -> Option<&Material>;
    fn draw(&self, material: &Material);
}
