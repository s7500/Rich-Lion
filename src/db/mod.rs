use crate::models::LimitOrder;

use postgres::error::Error;
use postgres::{Client, NoTls, Row};
use rand::prelude::random;
use std::env;

pub fn get_pool() -> Client {
    // return connection to database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&db_url, NoTls).expect("Filed to connect to database")
}

pub fn create_table() -> Result<(), Error> {
    let mut db = get_pool();

    db.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS limit_order (
            id     integer UNIQUE PRIMARY KEY,
            pair   char(10),
            price  integer,
            value  integer,
            closed bool DEFAULT FALSE,
            owner  varchar(100))
    ",
    )
}

pub fn read_order_list() -> Result<Vec<LimitOrder>, Error> {
    let mut db = get_pool();
    let limit_orders: Vec<LimitOrder> = db
        .query("SELECT * FROM limit_order", &[])?
        .iter()
        .map(|row| LimitOrder {
            id: row.get(0),
            pair: row.get(1),
            price: row.get(2),
            value: row.get(3),
            closed: row.get(4),
            owner: row.get(5),
        })
        .collect();
    Ok(limit_orders)
}

pub fn read_order(id: i32) -> Result<Option<LimitOrder>, Error> {
    let mut db = get_pool();

    let limit_order: Option<LimitOrder> = db
        .query("SELECT * FROM limit_order WHERE id = $1", &[&id])?
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
    let mut db = get_pool();

    let id: i32 = random();
    let new_order = db.execute(
        "INSERT INTO limit_order (id, pair, price, value, closed, owner) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &id,
            &order.pair,
            &order.price,
            &order.value,
            &true,
            &order.owner,
        ],
    )?;
    Ok(new_order)
}

pub fn delete_order(id: i32) -> Result<Vec<Row>, Error> {
    let mut db = get_pool();

    db.query("DELETE FROM limit_order WHERE id = $1", &[&id])
}

pub fn get_exist_pair() -> Result<Vec<Row>, Error> {
    let mut conn = get_pool();
    conn.query("SELECT DISTINCT pair FROM limit_order", &[])
}

pub fn close_order(id: i32) -> Result<Vec<Row>, Error> {
    let mut db = get_pool();

    let order = db
        .query(
            "UPDATE limit_order SET closed = TRUE WHERE id = {}", 
            &[&id]
        )?;
    Ok(order)
}

pub fn find_order(pair: String, price: i32, value: i32) -> Result<Vec<LimitOrder>, Error>{
    let mut db = get_pool();

    let order: Vec<LimitOrder> = db
        .query(
            "SELECT * FROM limit_order WHERE pair = $1, price = $2, value = $3",
            &[&pair, &price, &value]
        )?
        .iter()
        .map(|row| LimitOrder {
            id: row.get(0),
            pair: row.get(1),
            price: row.get(2),
            value: row.get(3),
            closed: row.get(4),
            owner: row.get(5),
        })
        .collect();
    Ok(order)
}