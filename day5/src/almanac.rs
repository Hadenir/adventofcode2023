
pub trait AlmanacItem: Copy + Ord + Eq {
    fn new(num: u64) -> Self;

    fn value(&self) -> u64;
}

macro_rules! def_almanac_item {
    ($ident:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ident(pub u64);

        impl AlmanacItem for $ident {
            fn new(num: u64) -> Self {
                Self(num)
            }

            fn value(&self) -> u64 {
                self.0
            }
        }
    };
}

def_almanac_item!(Seed);
def_almanac_item!(Soil);
def_almanac_item!(Fertilizer);
def_almanac_item!(Water);
def_almanac_item!(Light);
def_almanac_item!(Temperature);
def_almanac_item!(Humidity);
def_almanac_item!(Location);
