# CoinLoan Test

**Essence**: to create a small service that will place orders on the exchange when certain conditions are reached (the price is lower or equal to or higher than or equal to the specified value). You can use binance testnet for testing.

**The task**:
- Create a json-rpc api interface that will:
    - accept order parameters and save them to the database
    - upon request, send order parameters with status
    - upon request, delete order parameters by id
- Monitor pairs that have order parameters
- When the condition is reached, place a limit order on the exchange, send a notification to the user in a telegram
- Monitor the placed order for closure (if not closed immediately when placed), send another notification to the user when fully executed and consider this rule worked out
- Cover code with tests

### Deploy

1. create .env file:
```bash
touch .env
```
2. Copy config from example to .env
3. Run docker-compose
```bash
docker-compose up -d --build
```
4. Enjoy :)

### Test

1. Open web container in docker-compose:
```bash
docker-compose exec web bash
```
2. Run test in container
```bash
cargo test
```