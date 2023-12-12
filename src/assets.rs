use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::prelude::*;

pub mod rocks {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;

    use crate::GameState;

    pub struct RockPlugin;

    impl Plugin for RockPlugin {
        fn build(&self, app: &mut App) {
            app.add_collection_to_loading_state::<_, CappedRock>(GameState::Loading)
                .add_collection_to_loading_state::<_, RedRock>(GameState::Loading)
                .add_collection_to_loading_state::<_, SaltRock>(GameState::Loading)
                .add_collection_to_loading_state::<_, StoneRock>(GameState::Loading)
                .add_collection_to_loading_state::<_, TanRock>(GameState::Loading)
                .add_systems(
                    Update,
                    init_rock_collection
                        .run_if(in_state(GameState::Loading))
                        .run_if(not(resource_exists::<RockCollection>())),
                );
        }
    }

    fn init_rock_collection(world: &mut World) {
        let Some(capped) = world.get_resource::<CappedRock>().map(|r| r.to_owned()) else {
            return;
        };
        let Some(red) = world.get_resource::<RedRock>().map(|r| r.to_owned()) else {
            return;
        };
        let Some(salt) = world.get_resource::<SaltRock>().map(|r| r.to_owned()) else {
            return;
        };
        let Some(stone) = world.get_resource::<StoneRock>().map(|r| r.to_owned()) else {
            return;
        };
        let Some(tan) = world.get_resource::<TanRock>().map(|r| r.to_owned()) else {
            return;
        };

        let rock_collection = RockCollection {
            capped_rock: capped,
            red_rock: red,
            salt_rock: salt,
            stone_rock: stone,
            tan_rock: tan,
        };

        world.insert_resource(rock_collection);
        info!("Rock collection initialized");
    }

    #[derive(Resource)]
    pub struct RockCollection {
        pub capped_rock: CappedRock,
        pub red_rock: RedRock,
        pub salt_rock: SaltRock,
        pub stone_rock: StoneRock,
        pub tan_rock: TanRock,
    }

    #[derive(AssetCollection, Resource, Clone)]
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

    #[derive(AssetCollection, Resource, Clone)]
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

    #[derive(AssetCollection, Resource, Clone)]
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

    #[derive(AssetCollection, Resource, Clone)]
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

    #[derive(AssetCollection, Resource, Clone)]
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
        fn get_stone_kind(&self) -> Handle<Image>;
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
        fn get_stone_kind(&self) -> Handle<Image> {
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
        fn get_stone_kind(&self) -> Handle<Image> {
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
        fn get_stone_kind(&self) -> Handle<Image> {
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
        fn get_stone_kind(&self) -> Handle<Image> {
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
        fn get_stone_kind(&self) -> Handle<Image> {
            self.small.clone()
        }
    }
}

pub mod trees {
    use bevy::prelude::*;
    use bevy_asset_loader::prelude::*;

    use crate::GameState;

    pub struct TreePlugin;

    impl Plugin for TreePlugin {
        fn build(&self, app: &mut App) {
            app.add_collection_to_loading_state::<_, FallTree>(GameState::Loading)
                .add_collection_to_loading_state::<_, FruitTree>(GameState::Loading)
                .add_collection_to_loading_state::<_, MossTree>(GameState::Loading);
        }
    }

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
    #[asset(texture_atlas(tile_size_x = 200., tile_size_y = 200., columns = 5, rows = 3,))]
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
    MiddleLeft = 5,
    #[default]
    MiddleMiddle = 6,
    MiddleRight = 7,
    BottomLeft = 10,
    BottomMiddle = 11,
    BottomRight = 12,
    OutsideTopLeft = 3,
    OutsideTopRight = 4,
    OutsideBottomLeft = 8,
    OutsideBottomRight = 9,
}

#[derive(AssetCollection, Resource)]
pub struct MalePawns {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_01.png")]
    pub male1: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_02.png")]
    pub male2: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_03.png")]
    pub male3: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_04.png")]
    pub male4: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_05.png")]
    pub male5: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_06.png")]
    pub male6: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_07.png")]
    pub male7: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_08.png")]
    pub male8: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_09.png")]
    pub male9: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_10.png")]
    pub male10: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_11.png")]
    pub male11: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 4))]
    #[asset(path = "objects/pawns/male/M_12.png")]
    pub male12: Handle<TextureAtlas>,
}

impl MalePawns {
    pub fn get_random(&self) -> Handle<TextureAtlas> {
        let random = rand::thread_rng().gen_range(1..12);
        match random {
            1 => self.male1.clone(),
            2 => self.male2.clone(),
            3 => self.male3.clone(),
            4 => self.male4.clone(),
            5 => self.male5.clone(),
            6 => self.male6.clone(),
            7 => self.male7.clone(),
            8 => self.male8.clone(),
            9 => self.male9.clone(),
            10 => self.male10.clone(),
            11 => self.male11.clone(),
            12 => self.male12.clone(),
            _ => self.male1.clone(),
        }
    }
}

#[repr(u8)]
#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CharacterFacing {
    Forward = 0,
    Right = 1,
    Backward = 2,
    Left = 3,
}

pub struct GameAssets;

impl Plugin for GameAssets {
    fn build(&self, app: &mut App) {
        app.add_plugins((rocks::RockPlugin, trees::TreePlugin))
            .add_collection_to_loading_state::<_, GroundBase>(GameState::Loading)
            .add_collection_to_loading_state::<_, MalePawns>(GameState::Loading);
    }
}
