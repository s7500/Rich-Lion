use lib::models::LimitOrder;
use lib::db;
use rocket_contrib::json::Json;

#[post("/create", data="<order>")]
pub fn create_order(order: Json<LimitOrder>) -> Json<Option<LimitOrder>> {
    let inserted_order: Option<LimitOrder> = db::insert_order(&order)
        .ok()
        .map(|_| order.0);
    Json(inserted_order)
}

#[get("/get")]
pub fn get_orders() -> Json<Option<Vec<LimitOrder>>> {
    Json(db::read_order_list().ok())
}

#[get("/get/<id>")]
pub fn get_order(id: i32) -> Json<Option<LimitOrder>> {
    Json(db::read_order(id).ok().flatten())
}

#[delete("/delete/<id>")]
pub fn delete_order(id: i32) -> Json<bool> {
    let result: bool = db::delete_order(id).is_ok();
    Json(result)
}