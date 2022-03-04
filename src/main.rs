#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db::create_table;
mod endpoints;

fn main() {
    create_table();
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
