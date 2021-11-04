mod vitality;
mod armor;
mod shield;

pub use vitality::Vitality;
pub use vitality::VitalityModifier;
pub use vitality::VitalityModifierTimed;
pub use armor::Armor;
pub use shield::Shield;


/*
enum MixedInts {
    SmallInt(i32),
    TwoSmallInts(i32, i32),
}

impl MixedInts {
    fn new<A>(args: A) -> MixedInts
        where A: Into<MixedInts>
    {
        args.into()
    }
}

impl From<i32> for MixedInts {
    fn from(a: i32) -> MixedInts {
        MixedInts::SmallInt(a)
    }
}

impl From<(i32, i32)> for MixedInts {
    fn from((a, b): (i32, i32)) -> MixedInts {
        MixedInts::TwoSmallInts(a, b)
    }
}

fn main() {
    let x = MixedInts::new(2i32);
    let y = MixedInts::new((2i32, 2i32));
}
 */