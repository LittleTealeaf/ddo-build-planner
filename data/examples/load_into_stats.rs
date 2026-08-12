use data::load_feats;
use ddo_core::player_stats::PlayerStats;

fn main() -> anyhow::Result<()> {
    let feats = load_feats()?;

    let mut player_stats = PlayerStats::default();
    player_stats.load_feats(feats);

    println!("{player_stats:?}");

    Ok(())
}
