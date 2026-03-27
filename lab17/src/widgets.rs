pub mod button;
pub mod label;
pub mod window;

pub trait Widget {
    fn width(&self) -> usize;

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;

    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer).unwrap();
        println!("{buffer}");
    }
}