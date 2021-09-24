use mongodb::bson::{doc};
use mongodb::{error::Error, results::InsertOneResult, sync::{Client, Collection}};
use crate::models::Tree;

#[derive(Clone)]
pub struct DB {
  collection: Collection<Tree>,
}

impl DB {
    pub fn new(database_url: String) -> Result<DB, Error> {
        let client = Client::with_uri_str(database_url)?;
        let database = client.database("test");
        let collection = database.collection::<Tree>("trees");
        Ok(DB {collection: collection})
    }
  
    pub fn save(&self, tree: &Tree) -> Result<InsertOneResult, Error> {
      self.collection.insert_one(tree.clone(), None)
    }
}
