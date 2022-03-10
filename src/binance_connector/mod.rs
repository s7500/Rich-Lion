use binance::account::*;
use binance::api::*;
use binance::websockets::*;
use std::env;
use std::sync::atomic::AtomicBool;
use crate::db::{ find_order, close_order, get_exist_pair };
use postgres::Row;

pub async fn pair_webhook() {
    let api_key = Some(env::var("API_KEY").expect("1"));
    let secret_key = Some(env::var("SECRET_KEY").expect("2"));
    let account: Account = Binance::new(api_key, secret_key);
    let keep_running = AtomicBool::new(true);

    let pairs = get_exist_pair();

    for pair in pairs.iter() {
        connect_webhook(pair, &keep_running, &account);
    }
}

pub fn connect_webhook(
    pair: &Vec<Row>, 
    keep_running: &AtomicBool, 
    account: &Account
) {
    let agg_trade: String = format!("!ticker@arrTrades");
    let mut web_socket: WebSockets = WebSockets::new(
        |event: WebsocketEvent| {
            if let WebsocketEvent::AggrTrades(event) = event {
                let symbol = event.symbol;
                let price = event.price.parse::<i32>().unwrap();
                let qty = event.qty.parse::<i32>().unwrap();
                let orders = find_order(symbol, price, qty);
                if orders.is_ok() {
                   orders.unwrap().iter().map(
                        |order| match account.limit_buy(&order.pair, order.value, order.price.into()) {
                            Ok(answer) => close_order(order.id),
                            Err(e) => close_order(0),
                        }
                    );
                }
                // Ok(());
            }
            Ok(())
        }
    );
    // let mut web_socket: WebSockets = WebSockets::new(
    //     |event: WebsocketEvent| match event {
    //     WebsocketEvent::AggrTrades(event) => {
    //         let symbol = event.symbol;
    //         let price = event.price.parse::<i32>().unwrap();
    //         let qty = event.qty.parse::<i32>().unwrap();
    //         // if let order = find_order(
    //         //     symbol, 
    //         //     price, 
    //         //     qty
    //         // ) {
    //         //     order.unwrap().iter().map(|o|
    //         //         match account.limit_buy(o.pair, o.value, o.price.into()) {
    //         //             Ok(answer) => close_order(o.id),
    //         //             // Err(e) => println!("Error: {:?}", e),
    //         //             // Err(e) => return Err(e),
    //         //         });
    //         //     Ok(())
    //         // }
    //         // match find_order(symbol, price, qty) {
    //         //     Ok(order) => order.iter().map(|o|
    //         //         match account.limit_buy(o.pair, o.value, o.price.into()) {
    //         //             Ok(answer) => close_order(o.id),
    //         //         }
    //         //     ),
    //         // }
    //         // match find_order(symbol, price, qty) {
    //         //     Ok(orders) => Ok(for order in orders.iter() {
    //         //         match account.limit_buy(order.pair, order.value, order.price.into()) {
    //         //             Ok(answer) => close_order(order.id),
    //         //             Err(e) => PGError,
    //         //         };
    //         //     }),
    //         //     Err(e) => Error,
    //         // }
    //         // let orders = find_order(symbol, price, qty);
    //         // if orders.is_ok() {
    //         //     let result = orders.unwrap().iter().map(|order| 
    //         //         account.limit_buy(
    //         //             order.pair, 
    //         //             order.value, 
    //         //             order.price.into()
    //         //         )
    //         //     ).collect();
    //         //     Ok(result)
    //         // }
    //     }
    //     }
    // );

}
