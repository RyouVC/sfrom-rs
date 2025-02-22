use sfrom::Sfc;

fn main() {
    let data = std::fs::read("game.sfc").unwrap();

    let (_, sfc) = Sfc::parse(&data).unwrap();

    let switch_rom = sfc.convert_to_switch();

    std::fs::write("converted.sfrom", switch_rom).unwrap();
}
