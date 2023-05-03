

pub enum ItemType {
    Weapon(WeaponType)
}


pub enum EquipmentType {

}


#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum WeaponType {
    Simple(SimpleWeapon),
    Martial(MartialWeapon),
    Exotic(ExoticWeapon)
}


#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum SimpleWeapon {
    Club,
    Dagger,
    Dart,
    Quarterstaff,
    Sickle,
    Unarmed
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum MartialWeapon {
    Battleaxe,
    Falchion,
    GreatAxe,
    GreatClub,
    GreatSword,
    Handaxe,
    HeavyPick,
    Kukri,
    LightHammer,
    LightPick,
    Longbow,
    Longsword,
    Maul,
    Rapier,
    Scimitar,
    ShortSword,
    Shortbow,
    ThrowingAxe,
    ThrowingHammer,
    Warhammer
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum ExoticWeapon {
    BastardSword,
    DwarvenWaraxe,
    GreatCrossbow,
    Handwraps,
    Kama,
    Khopesh,
    RepeatingHeavyCrossbow,
    RepeatingLightCrossbow,
    Shuriken
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum ArmorType {
    Cloth,
    Light,
    Medium,
    Heavy
}
