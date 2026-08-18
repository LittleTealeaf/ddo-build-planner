use core::iter::once;

use crate::{
    attribute::Attribute,
    bonus::{traits::ToValue, Bonus},
    items::feat::Feat,
    player_stats::PlayerStats,
    types::bonus_provider::BonusProvider,
    val,
};

impl PlayerStats {
    pub fn clear_provider(&mut self, provider: &BonusProvider) {
        if let Some(attributes) = self.bonuses.clear_provider(provider) {
            self.calculator().reset_attributes(attributes.into());
        }
    }

    pub fn insert_bonus(&mut self, bonus: Bonus, provider: BonusProvider, snapshot: Option<u32>) {
        self.insert_bonuses(once(bonus), provider, snapshot);
    }

    pub fn insert_bonuses<I>(&mut self, bonuses: I, provider: BonusProvider, snapshot: Option<u32>)
    where
        I: IntoIterator<Item = Bonus>,
    {
        log::debug!("Inserting Bonuses for [{provider}] using snapshot {snapshot:?}");
        // Insert bonuses
        let attributes = self
            .bonuses
            .insert_bonuses(bonuses, provider, snapshot)
            .collect();

        self.calculator().reset_attributes(attributes);
    }

    pub fn load_feats<I>(&mut self, feats: I)
    where
        I: IntoIterator<Item = Feat>,
    {
        log::debug!("Loading Feats...");
        self.insert_bonuses(
            feats.into_iter().flat_map(|feat| {
                log::debug!("Loading Bonuses: {}", feat.name());
                let name = feat.name().clone();
                feat.into_bonuses().map(move |bonus| {
                    bonus.with_show_condition_and(
                        Attribute::Feat(name.clone())
                            .to_value()
                            .greater_than(val!(0)),
                    )
                })
            }),
            BonusProvider::Feats,
            None,
        );
    }
}
