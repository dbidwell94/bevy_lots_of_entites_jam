use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod rocks {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;
    #[derive(AssetCollection, Resource)]
    pub struct CappedRock {
        #[asset(path = "objects/rocks/capped/5.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/rocks/capped/4.png")]
        pub medium_large: Handle<Image>,
        #[asset(path = "objects/rocks/capped/3.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/rocks/capped/2.png")]
        pub medium_small: Handle<Image>,
        #[asset(path = "objects/rocks/capped/1.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct RedRock {
        #[asset(path = "objects/rocks/red/5.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/rocks/red/4.png")]
        pub medium_large: Handle<Image>,
        #[asset(path = "objects/rocks/red/3.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/rocks/red/2.png")]
        pub medium_small: Handle<Image>,
        #[asset(path = "objects/rocks/red/1.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct SaltRock {
        #[asset(path = "objects/rocks/salt/5.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/rocks/salt/4.png")]
        pub medium_large: Handle<Image>,
        #[asset(path = "objects/rocks/salt/3.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/rocks/salt/2.png")]
        pub medium_small: Handle<Image>,
        #[asset(path = "objects/rocks/salt/1.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct StoneRock {
        #[asset(path = "objects/rocks/stone/5.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/rocks/stone/4.png")]
        pub medium_large: Handle<Image>,
        #[asset(path = "objects/rocks/stone/3.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/rocks/stone/2.png")]
        pub medium_small: Handle<Image>,
        #[asset(path = "objects/rocks/stone/1.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct TanRock {
        #[asset(path = "objects/rocks/tan/5.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/rocks/tan/4.png")]
        pub medium_large: Handle<Image>,
        #[asset(path = "objects/rocks/tan/3.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/rocks/tan/2.png")]
        pub medium_small: Handle<Image>,
        #[asset(path = "objects/rocks/tan/1.png")]
        pub small: Handle<Image>,
    }

    pub trait RockAsset {
        fn get_large(&self) -> Handle<Image>;
        fn get_medium_large(&self) -> Handle<Image>;
        fn get_medium(&self) -> Handle<Image>;
        fn get_medium_small(&self) -> Handle<Image>;
        fn get_small(&self) -> Handle<Image>;
    }

    impl RockAsset for CappedRock {
        fn get_large(&self) -> Handle<Image> {
            self.large.clone()
        }
        fn get_medium_large(&self) -> Handle<Image> {
            self.medium_large.clone()
        }
        fn get_medium(&self) -> Handle<Image> {
            self.medium.clone()
        }
        fn get_medium_small(&self) -> Handle<Image> {
            self.medium_small.clone()
        }
        fn get_small(&self) -> Handle<Image> {
            self.small.clone()
        }
    }

    impl RockAsset for RedRock {
        fn get_large(&self) -> Handle<Image> {
            self.large.clone()
        }
        fn get_medium_large(&self) -> Handle<Image> {
            self.medium_large.clone()
        }
        fn get_medium(&self) -> Handle<Image> {
            self.medium.clone()
        }
        fn get_medium_small(&self) -> Handle<Image> {
            self.medium_small.clone()
        }
        fn get_small(&self) -> Handle<Image> {
            self.small.clone()
        }
    }

    impl RockAsset for SaltRock {
        fn get_large(&self) -> Handle<Image> {
            self.large.clone()
        }
        fn get_medium_large(&self) -> Handle<Image> {
            self.medium_large.clone()
        }
        fn get_medium(&self) -> Handle<Image> {
            self.medium.clone()
        }
        fn get_medium_small(&self) -> Handle<Image> {
            self.medium_small.clone()
        }
        fn get_small(&self) -> Handle<Image> {
            self.small.clone()
        }
    }

    impl RockAsset for StoneRock {
        fn get_large(&self) -> Handle<Image> {
            self.large.clone()
        }
        fn get_medium_large(&self) -> Handle<Image> {
            self.medium_large.clone()
        }
        fn get_medium(&self) -> Handle<Image> {
            self.medium.clone()
        }
        fn get_medium_small(&self) -> Handle<Image> {
            self.medium_small.clone()
        }
        fn get_small(&self) -> Handle<Image> {
            self.small.clone()
        }
    }

    impl RockAsset for TanRock {
        fn get_large(&self) -> Handle<Image> {
            self.large.clone()
        }
        fn get_medium_large(&self) -> Handle<Image> {
            self.medium_large.clone()
        }
        fn get_medium(&self) -> Handle<Image> {
            self.medium.clone()
        }
        fn get_medium_small(&self) -> Handle<Image> {
            self.medium_small.clone()
        }
        fn get_small(&self) -> Handle<Image> {
            self.small.clone()
        }
    }
}

pub mod trees {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;

    #[derive(AssetCollection, Resource)]
    pub struct FallTree {
        #[asset(path = "objects/trees/fallTree/large.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/trees/fallTree/medium.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/trees/fallTree/small.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct FruitTree {
        #[asset(path = "objects/trees/fruitTree/large.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/trees/fruitTree/medium.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/trees/fruitTree/small.png")]
        pub small: Handle<Image>,
    }

    #[derive(AssetCollection, Resource)]
    pub struct MossTree {
        #[asset(path = "objects/trees/mossTree/large.png")]
        pub large: Handle<Image>,
        #[asset(path = "objects/trees/mossTree/medium.png")]
        pub medium: Handle<Image>,
        #[asset(path = "objects/trees/mossTree/small.png")]
        pub small: Handle<Image>,
    }
}

#[derive(AssetCollection, Resource)]
pub struct GroundBase {
    #[asset(texture_atlas(tile_size_x = 200., tile_size_y = 200., columns = 3, rows = 3,))]
    #[asset(path = "dirtSpritesheet.png")]
    pub dirt: Handle<TextureAtlas>,
}

#[repr(usize)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(dead_code)]
pub enum DirtTile {
    TopLeft = 0,
    TopMiddle = 1,
    TopRight = 2,
    MiddleLeft = 3,
    #[default]
    MiddleMiddle = 4,
    MiddleRight = 5,
    BottomLeft = 6,
    BottomMiddle = 7,
    BottomRight = 8,
}

pub struct GameAssets;

impl Plugin for GameAssets {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, rocks::CappedRock>(GameState::Loading)
            .add_collection_to_loading_state::<_, rocks::RedRock>(GameState::Loading)
            .add_collection_to_loading_state::<_, rocks::SaltRock>(GameState::Loading)
            .add_collection_to_loading_state::<_, rocks::StoneRock>(GameState::Loading)
            .add_collection_to_loading_state::<_, rocks::TanRock>(GameState::Loading)
            .add_collection_to_loading_state::<_, trees::FallTree>(GameState::Loading)
            .add_collection_to_loading_state::<_, trees::FruitTree>(GameState::Loading)
            .add_collection_to_loading_state::<_, trees::MossTree>(GameState::Loading)
            .add_collection_to_loading_state::<_, GroundBase>(GameState::Loading);
    }
}
