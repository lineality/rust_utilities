mod three_part_tui;
use three_part_tui::ThreePartTui;

fn main() -> std::io::Result<()> {
    let mut tui = ThreePartTui::new()?;
    tui.run()
}
