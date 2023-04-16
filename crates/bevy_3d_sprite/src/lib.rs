use bevy::app::App;
use bevy::prelude::Plugin;

mod pbr_sprite_sheet;
mod sprite_animation;
mod sprite_sheet;

pub use pbr_sprite_sheet::PbrSpriteBundle;
pub use sprite_animation::SpriteAnimation;
pub use sprite_sheet::SpriteSheet;

pub struct Sprite3dPlugin;

impl Plugin for Sprite3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_sheet::detect_and_clip_texture);
        app.add_system(sprite_animation::sprite_animation);
        app.add_system(pbr_sprite_sheet::update_base_texture);
    }
}
