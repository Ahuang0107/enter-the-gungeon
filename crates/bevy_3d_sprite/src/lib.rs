use bevy::app::App;
use bevy::prelude::{AddAsset, Plugin};

mod pbr_sprite;
mod sprite_animation;

pub use pbr_sprite::{PbrSpriteBundle, TextureAtlas, TextureAtlasSprite};
pub use sprite_animation::SpriteAnimation;

pub struct Sprite3dPlugin;

impl Plugin for Sprite3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TextureAtlas>();
        app.add_system(pbr_sprite::detect_and_clip_texture);
        app.add_system(pbr_sprite::update_base_texture);
        app.add_system(sprite_animation::sprite_animation);
    }
}
