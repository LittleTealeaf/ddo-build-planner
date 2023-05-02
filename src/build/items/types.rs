

pub enum ItemType {
    Weapon(WeaponType)
}


pub enum EquipmentType {

}


#[derive(Clone, Copy)]
pub enum WeaponType {
    Simple(SimpleWeapon),
    Martial(MartialWeapon),
    Exotic(ExoticWeapon)
}


#[derive(Clone, Copy)]
pub enum SimpleWeapon {
    Club,
    Dagger,
    Dart,
    Quarterstaff,
    Sickle,
    Unarmed
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum ArmorType {
    Cloth,
    Light,
    Medium,
    Heavy
}
