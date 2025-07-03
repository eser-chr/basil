use geometry::Vec3d as Point;

struct AtomType{
    atomic_number:u8,
    symbol:Option([char;2]),
}

impl AtomType{
    fn new(atomic_number:u8, symbol:Option)->Self{
        if atomic_number>126{
            eprintln!("Warning: The atomic number is non existent. This will not affect the rest of the program though!");
            
        }
        Self{atomic_number, symbol}
    }
}


struct Atom{
    atomic_type:AtomType,
    point:Point<f64>,
}

