use crate::attribute::Attribute;
use crate::simple_enum;
simple_enum!(ElementalType, "", (Acid "Acid", Chaos "Chaos", Cold "Cold", Electric "Electric", Evil "Evil", Fire "Fire", Force "Force", Good "Good", Lawful "Lawful", Light "Light", Negative "Negative", Poison "Poison", Sonic "Sonic"));


impl ElementalType {
    #[inline(always)]
    pub fn into_resistance_attribute(self) -> Attribute {
        Attribute::ElementalResistance(self)
    }
    
    #[inline(always)]
    pub fn into_absorption_attribute(self) -> Attribute {
        Attribute::ElementalAbsorption(self)
    }
    
}