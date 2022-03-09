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

pub fn connect_webhook(pair: &Vec<Row>, keep_running: &AtomicBool, account: &Account) {
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| match event {
        WebsocketEvent::AggrTrades(event) => {
            let symbol = event.symbol;
            let price = event.price.parse::<i32>().unwrap();
            let qty = event.qty.parse::<i32>().unwrap();
            if let order = find_order(
                symbol, 
                price, 
                qty
            ) {
                for o in order.iter() {
                    match account.limit_buy(o.pair, o.value, o.price) {
                        Ok(answer) => close_order(order.id),
                        // Err(e) => println!("Error: {:?}", e),
                        // Err(e) => return Err(e),
                    };
                }
                // match account.limit_buy(order.pair, order.value, order.price) {
                //     Ok(answer) => close_order(order.id),
                //     // Err(e) => println!("Error: {:?}", e),
                //     Err(e) => Err(e),
                // }
            }
        }
    });
}
