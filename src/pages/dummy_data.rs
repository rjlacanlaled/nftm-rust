use super::collection::Collection;

// pub struct Collection {
//     id: u32,
//     img_url: String,
//     name: String,
//     volume: f64,
// }

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
