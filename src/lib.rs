pub mod db;
pub mod models;
pub mod binance_connector;

#[cfg(test)]
mod tests {
    use crate::db::*;
    use crate::models::LimitOrder;

    #[test]
    fn test_insert_order() {
        let new_limit_order = LimitOrder{
            id: 1,
            pair: String::from("BTCUSDT"),
            price: 100,
            value: 100,
            closed: false,
            owner: String::from("new_owner")
        };

        insert_order(&new_limit_order);

        let mut conn = get_pool();
        let lim_order = conn
            .query("SELECT * FROM limit_order WHERE owner='new_owner'", &[])
            .expect("Error: User not exist!");

        assert_eq!(lim_order.len(), 6);
    }

    #[test]
    fn test_read_order() {
        let mut conn = get_pool();
        let new_order = conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &(1 as i32),
                    &String::from("BTCUSDT"),
                    &(100 as i32),
                    &(100 as i32),
                    &false,
                    &String::from("new_owner")
                ],);
        let result = read_order(1).expect("order not found!");
        assert_eq!(result.unwrap().len(), 6);
    }

    #[test]
    fn test_delete_order() {
        let mut conn = get_pool();
        let new_order = conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &(1 as i32),
                    &String::from("BTCUSDT"),
                    &(100 as i32),
                    &(100 as i32),
                    &false,
                    &String::from("new_owner")
                ],);
        let result = delete_order(1).expect("order not found!");
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_get_exist_pair() {
        let mut conn = get_pool();
        let new_order = conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &1,
                    &String::from("BTCUSDT"),
                    &100,
                    &100,
                    &false,
                    &String::from("new_owner")
                ],);

        let result = get_exist_pair().expect("not found pair");
        assert_eq!(result.len(), 1);

        conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &2,
                    &String::from("BTCUSDT"),
                    &100,
                    &100,
                    &false,
                    &String::from("new_owner")
                ],);

       let result = get_exist_pair().expect("not found pair");
        assert_eq!(result.len(), 1);

        conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &3,
                    &String::from("SOLUSDT"),
                    &100,
                    &100,
                    &false,
                    &String::from("new_owner")
                ],);

       let result = get_exist_pair().expect("not found pair");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_close_order() {
        let mut conn = get_pool();

        let new_order = conn
            .query("INSERT INTO limit_order
                    VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &1,
                    &String::from("BTCUSDT"),
                    &100,
                    &100,
                    &false,
                    &String::from("new_owner")
                ],);
        let result = close_order(1).expect("order not found!");
        assert_eq!(result.len(), 6);
    }
}