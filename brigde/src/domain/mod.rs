use anyhow::Result;

#[async_trait::async_trait]
#[mockall::automock]
pub trait SolClimberOnChain {
    async fn initialize_player(&self) -> Result<String>;
    async fn summit_record(&self, player_address: &str) -> Result<u32>;
    async fn death_record(&self, player_address: &str) -> Result<u32>;
    async fn mint_nft_to_player(&self) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub symbol: &'static str,
    pub uri: &'static str,
}

pub const NFT_COLLECTIONS: &[Metadata; 4] = &[
    Metadata {
        name: "TrekkingPoles#1",
        symbol: "TP",
        uri: "https://raw.githubusercontent.com/Rayato159/sol-climber/refs/heads/main/nft-collections/TrekkingPoles%231.json",
    },
    Metadata {
        name: "RedBackpack",
        symbol: "BP",
        uri: "https://raw.githubusercontent.com/Rayato159/sol-climber/refs/heads/main/nft-collections/RedBackpack.json",
    },
    Metadata {
        name: "PinkBackpack",
        symbol: "BP",
        uri: "https://raw.githubusercontent.com/Rayato159/sol-climber/refs/heads/main/nft-collections/PinkBackpack.json",
    },
    Metadata {
        name: "RedHat",
        symbol: "H",
        uri: "https://raw.githubusercontent.com/Rayato159/sol-climber/refs/heads/main/nft-collections/RedHat.json",
    },
];

pub const PROGRAM_ID: &str = "C17Vg2mNNQ6tjYLFUuheUfQwQoGdopf6dW2goEbWLtM6";
