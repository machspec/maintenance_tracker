#![allow(unused)]

pub mod database {
    use std::collections::BTreeMap;

    use mysql::{prelude::*, *};

    use crate::core::structs::*;
    use crate::data::constants::{TABLE_NAME_CATEGORY, TABLE_NAME_ENTRY, TABLE_NAME_ITEM};

    pub fn collect_categories(conn: &mut PooledConn) -> Vec<Category> {
        /// Get all categories from the database.
        conn.query("SELECT * FROM category ORDER BY title").unwrap()
    }

    pub fn collect_items(conn: &mut PooledConn) -> BTreeMap<u32, Item> {
        /// Get all items from the database.
        /// Returns a BTreeMap to preserve order of insertion.
        let mut items: Vec<Item> = conn.query("SELECT * FROM item").unwrap();

        // get vector of most recent entries for each item
        let mut details_list: Vec<ItemDetails> = Vec::new();
        for item in items.iter() {
            let details: ItemDetails = match conn
                .exec_first(
                    r"
                    SELECT cost, note, status, visible, removed
                    FROM entry WHERE item_id = :item_id ORDER BY id DESC
                    ",
                    params! {
                        "item_id" => item.id,
                    },
                )
                .unwrap()
            {
                Some(details) => details,
                None => ItemDetails::new(),
            };

            details_list.push(details);
        }

        let mut result: BTreeMap<u32, Item> = BTreeMap::new();
        for (item, details) in items.iter_mut().zip(details_list.iter()) {
            item.details = Some(details.clone());
            result.insert(item.id.unwrap(), item.clone());
        }

        result
    }

    pub fn collect_item_entries(conn: &mut PooledConn, item_id: &str) -> Vec<Entry> {
        /// Get all entries from the database.
        /// Returns a Vector of Entry.
        let mut entries: Vec<Entry> = conn
            .query(&format!("SELECT * FROM entry WHERE item_id = {}", item_id))
            .unwrap();

        entries
    }

    pub fn connect(credentials: &DbCredentials) -> Result<mysql::PooledConn> {
        /// Get options from url and create a pooled connection
        let opts = Opts::from_url(&credentials.mysql_url())?;
        let pool = Pool::new(opts)?;

        match pool.get_conn() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete_category(conn: &mut PooledConn, id: u32) {
        /// Delete a category from the database.
        let result = conn
            .exec_drop(
                r"
                UPDATE category
                SET removed = 1
                WHERE id = :id;
                ",
                params! {
                    "id" => id,
                },
            )
            .unwrap();
    }

    pub fn delete_item(conn: &mut PooledConn, item_id: u32) -> Result<()> {
        /// Delete a category from the database.
        conn.exec_drop(
            r"
            UPDATE entry
            SET removed = 1
            WHERE item_id = :item_id;
            ",
            params! {
                "item_id" => item_id,
            },
        )
    }

    pub fn get_autoincremented_id(conn: &mut PooledConn, table_name: &str) -> u32 {
        /// Get the autoincremented id of the last inserted row.
        let new_id: u32 = conn
            .query(format!(
                "SELECT id FROM {} ORDER BY id DESC LIMIT 1",
                table_name
            ))
            .unwrap()[0];
        // update original item's id

        new_id
    }

    pub fn get_category(conn: &mut PooledConn, id: u32) -> Category {
        /// Get a category from the database.
        match conn
            .exec_first(
                "SELECT * FROM category WHERE id = :id",
                params! {
                    "id" => id,
                },
            )
            .unwrap()
        {
            Some(category) => category,
            None => panic!("No category with id {}", id),
        }
    }

    pub fn get_entry(conn: &mut PooledConn, id: u32) -> Entry {
        match conn
            .exec_first(
                r"
                SELECT cost, note, status, visible, removed
                FROM entry WHERE item_id = :item_id ORDER BY id DESC
                ",
                params! {
                    "item_id" => id,
                },
            )
            .unwrap()
        {
            Some(entry) => entry,
            None => panic!("No entries with item_id {}", id),
        }
    }

    pub fn get_item(conn: &mut PooledConn, id: u32) -> Item {
        /// Get an item from the database.
        let item: Option<Item> = conn
            .exec_first(
                "SELECT * FROM item WHERE id = :id",
                params! {
                    "id" => id,
                },
            )
            .unwrap();

        match item {
            Some(Item {
                category_id, title, ..
            }) => Item {
                id: Some(id),
                category_id: category_id,
                title: title,
                details: Some(ItemDetails::from_entry(&get_entry(conn, id))),
            },
            None => panic!("No item with id {}", id),
        }
    }

    pub fn insert_category(conn: &mut PooledConn, title: &str) -> mysql::Result<()> {
        /// Insert a category into the database.
        match conn.query_drop(format!(
            "INSERT INTO category (title, removed) VALUES ('{}', 0)",
            title
        )) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn insert_entry(conn: &mut PooledConn, item: &Item) -> mysql::Result<()> {
        /// Insert an entry into the database.
        let details = item.details.as_ref().unwrap();
        match conn.exec_drop(
            r"
            INSERT INTO entry (item_id, cost, note, status, visible, removed)
            VALUES (
                :item_id,
                :cost,
                :note,
                :status,
                :visible,
                :removed
            );
            ",
            params! {
                "item_id" => item.id,
                "cost" => details.cost,
                "note" => &details.note,
                "status" => details.status,
                "visible" => details.visible,
                "removed" => details.removed,
            },
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn insert_item(conn: &mut PooledConn, item: &mut Item) -> mysql::Result<()> {
        /// Insert an item into the database.
        let details = item.details.as_ref().unwrap();
        println!("{:?}", details);

        match conn.exec_drop(
            r"INSERT INTO item (title, category_id)
            VALUES (
                :title,
                :category_id
            );
            ",
            params! {
                "title" => &item.title,
                "category_id" => item.category_id,
            },
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        item.id = Some(get_autoincremented_id(conn, "item"));

        insert_entry(conn, item)?;

        Ok(())
    }

    pub fn title_taken(conn: &mut PooledConn, title: &str, table_name: &str) -> bool {
        /// Check if a title is taken by a category or item.
        if table_name == TABLE_NAME_CATEGORY {
            for category in collect_categories(conn) {
                if category.title == title && !category.removed {
                    return true;
                }
            }

            return false;
        } else if table_name == TABLE_NAME_ITEM {
            for (_, item) in collect_items(conn) {
                if item.title == title && !item.details.unwrap().removed {
                    return true;
                }
            }

            return false;
        } else {
            panic!("Invalid table name");
        }
    }

    pub fn test_auth(credentials: &DbCredentials) -> Result<()> {
        match connect(&credentials) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn update_item(conn: &mut PooledConn, item: &Item) -> mysql::Result<()> {
        /// Update an item in the database.
        match conn.exec_drop(
            r"
            UPDATE item
            SET title = :title,
            category_id = :category_id
            WHERE id = :id;
            ",
            params! {
                "id" => item.id,
                "title" => &item.title,
                "category_id" => item.category_id,
            },
        ) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        // create a new entry with updated information
        insert_entry(conn, item)
    }
}
