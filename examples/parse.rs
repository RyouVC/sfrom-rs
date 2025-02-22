use sfrom::Sfrom;

fn main() {
    let data = std::fs::read("game.sfrom").unwrap();

    let (_, sfrom) = Sfrom::parse(&data).unwrap();

    println!("Header: {:?}", sfrom.header_type);
    
    let dump_data = sfrom.rom_data;
    
    
    std::fs::write("dump.sfc", dump_data).unwrap();
    
    
    // println!("ROM Data: {:?}", sfrom);
}
