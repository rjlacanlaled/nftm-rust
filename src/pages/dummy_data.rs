use super::{ collection::Collection, asset::Asset };

// pub struct Asset {
//     pub id: u32,
//     pub img_url: String,
//     pub name: String,
//     pub price: f64,
// }

pub fn get_dummy_assets() -> Vec<Asset> {
    let mut assets = vec![];

    assets.push(Asset {
        id: 1,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 2,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 2,
    });
    assets.push(Asset {
        id: 3,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 3,
    });
    assets.push(Asset {
        id: 4,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 4,
    });
    assets.push(Asset {
        id: 5,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 6,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 2,
    });
    assets.push(Asset {
        id: 7,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 8,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 2,
    });
    assets.push(Asset {
        id: 9,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 3,
    });
    assets.push(Asset {
        id: 10,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 4,
    });
    assets.push(Asset {
        id: 11,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 12,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.1,
        collection_id: 2,
    });
    assets.push(Asset {
        id: 13,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 43.2,
        collection_id: 3,
    });
    assets.push(Asset {
        id: 14,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 44.1,
        collection_id: 4,
    });
    assets.push(Asset {
        id: 15,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 45.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 16,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 46.1,
        collection_id: 2,
    });
    assets.push(Asset {
        id: 17,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 47.1,
        collection_id: 3,
    });
    assets.push(Asset {
        id: 18,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 48.1,
        collection_id: 4,
    });
    assets.push(Asset {
        id: 19,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 49.1,
        collection_id: 1,
    });
    assets.push(Asset {
        id: 20,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Asset Name"),
        price: 50.1,
        collection_id: 2,
    });

    assets
}

pub fn get_dummy_collections() -> Vec<Collection> {
    let mut collections = vec![];
    collections.push(Collection {
        id: 1,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 2,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 3,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 4,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 5,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 6,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 7,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 8,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections.push(Collection {
        id: 9,
        img_url: String::from("../static/images/nft-collection-placeholder.webp"),
        name: String::from("Collection Name"),
        volume: 43.1,
    });
    collections
}
