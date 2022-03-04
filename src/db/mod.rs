use postgres::error::Error;
use postgres::{Row, Client, NoTls};
use std::env;
use rand::prelude::random;
use crate::models::LimitOrder;

pub fn get_database_url() -> String {
    let db_host = env::var("POSTGRES_PORT").expect("DATABASE_HOST must be set");
    format!("db:{}", db_host)
}

pub fn get_pool() -> Result<Client, Error> {
//     //let manager = PostgresConnectionManager::new(get_database_url().parse().unwrap(), NoTls);
//     //let manager = PostgresConnectionManager::new("host=db, port=5432".parse().unwrap(), NoTls);
//     //let pool_size: u32 = env::var("PG_POOL_SIZE").expect("PG_POOL_SIZE must be set").parse::<u32>().unwrap();
//     //Pool::builder().max_size(pool_size).build(manager).unwrap()
//     //Pool::new(manager).unwrap()
    Client::connect("0.0.0.0:5432", NoTls)
}

pub fn create_table() -> Result<(), Error> {
    let mut db = get_pool()?;

    db.batch_execute("
        CREATE TABLE limit_order (
            id     PRIMARY KEY
            pair   CHAR 
            price  INT
            value  INT
            closed BOOL
            owner  CHAR)
    ")
}

pub fn read_order_list() -> Result<Vec<LimitOrder>, Error> {
    let mut db = get_pool()?;
    let limit_orders: Vec<LimitOrder> = db.query("SELECT * FROM limit_order", &[])?
        .iter()
        .map(|row| {
            LimitOrder {
                id: row.get(0),
                pair: row.get(1),
                price: row.get(2),
                value: row.get(3),
                closed: row.get(4),
                owner: row.get(5),
            }
        }).collect();
    Ok(limit_orders)
}

pub fn read_order(id: i32) -> Result<Option<LimitOrder>, Error> {
    let mut db = get_pool()?;

    let limit_order: Option<LimitOrder> = db.query("SELECT * FROM limit_order WHERE id = $1", &[&id])?
        .iter()
        .fold(None, |_acc, row| {
            Some(LimitOrder {
                id: row.get(0),
                pair: row.get(1),
                price: row.get(2),
                value: row.get(3),
                closed: row.get(4),
                owner: row.get(5),
            })
        });
    Ok(limit_order)
}

pub fn insert_order(order: &LimitOrder) -> Result<u64, Error> {
    let mut db = get_pool()?;

    let id: i32 = random();
    let new_order = db.execute(
        "INSERT INTO limit_order (id, pair, price, value, closed, owner) VALUES ($1, $2, $3, $4, $5)", 
        &[&id, &order.pair, &order.price, &order.value, &true, &order.owner]
    )?;
    Ok(new_order)
}

pub fn delete_order(id: i32) -> Result<Vec<Row>, Error> {
    let mut db = get_pool()?;

    db.query("DELETE FROM limit_order WHERE id = $1", &[&id])
}

// pub fn read_order_list(db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<LimitOrder>, Error> {
//     let statement =  db.prepare("SELECT * FROM limitOrders",)?;

//     let limit_orders: Vec<LimitOrder> = db.query(&statement, &[])?
//         .iter()
//         .map(|row| {
//             let id: i32 = row.get("id");
//             let pair: String = row.get("pair");
//             let price: u32 = row.get("price");
//             let value: u32 = row.get("value");
//             let closed: bool = row.get("closed");
//             let owner: String = row.get("owner");
//             LimitOrder {
//                 id,
//                 pair,
//                 price,
//                 value,
//                 closed,
//                 owner,
//             }
//         }).collect();
//     Ok(limit_orders)
// }

// pub fn read_order(id: i32, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Option<LimitOrder>, Error> {
//     let statement = db.prepare("SELECT * FROM movies WHERE id = $1 ",)?;

//     let limit_order: Option<LimitOrder> = db.query(&statement, &[&id])?
//         .iter()
//         .fold(None, |_acc, row| {
//             let id: i32 = row.get("id");
//             let pair: String = row.get("pair");
//             let price: u32 = row.get("price");
//             let value: u32 = row.get("value");
//             let closed: bool = row.get("closed");
//             let owner: String = row.get("owner");
//             Some(LimitOrder {
//                 id,
//                 pair,
//                 price,
//                 value,
//                 closed,
//                 owner,
//             })
//         });
//     Ok(limit_order)
// }

// pub fn insert_order(order: &LimitOrder, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
//     let statement = db.prepare("INSERT INTO localorders {id, pair, price, value, closed, owner} values ($1, $2, $3, $4, $4, $5)",)?;
//     db.query(&statement, &[&order.id, &order.pair, &order.price, &order.value, &true])
// }

// pub fn delete_order(id: i32, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
//     let statement = db.prepare("DELETE FROM localorders WHERE id = $1",)?;

//     db.query(&statement, &[&id])
// }
