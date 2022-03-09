#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db::create_table;
use lib::binance_connector::pair_webhook;
mod endpoints;

fn main() {
    create_table().expect("Filed to create table");
    pair_webhook();
    rocket::ignite()
        .mount(
            "/",
            routes![
                endpoints::create_order,
                endpoints::get_order,
                endpoints::delete_order,
                endpoints::get_orders,
            ],
        )
        .launch();
}
