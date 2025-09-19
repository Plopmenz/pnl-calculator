use std::{error::Error, process};

#[derive(Debug, PartialEq, serde::Deserialize)]
enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, serde::Deserialize)]
struct Record {
    transaction_hash: String,
    trade_type: TradeType,
    USDC: f64,
    OPENX: f64,
    price: f64,
    wallet: String,
    eth: f64,
    weth: f64,
    price_eth: f64,
}

#[derive(Debug)]
struct RealizedTrade {
    openx: f64,
    sell_usd: f64,
    buy_usd: f64,
}

fn calculate() -> Result<(), Box<dyn Error>> {
    let mut buys: Vec<Record> = vec![];
    let mut sells: Vec<Record> = vec![];

    let mut rdr = csv::Reader::from_path("trades.csv")?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        if record.trade_type == TradeType::Buy {
            buys.push(record);
        } else {
            sells.push(record);
        }
    }

    println!("Expected assets");
    let start_openx = 10_700_000.0;
    let bought_openx = buys.iter().map(|buy| buy.OPENX).sum::<f64>();
    let sold_openx = sells.iter().map(|sell| sell.OPENX).sum::<f64>();
    let openx_correction = -8776300.0128; // OPENX withdraw
    let end_openx = start_openx + bought_openx - sold_openx + openx_correction;

    let start_eth = 5.98;
    let bought_eth = sells.iter().map(|sell| sell.eth).sum::<f64>();
    let sold_eth = buys.iter().map(|buy| buy.eth).sum::<f64>();
    let eth_correction = -2.205 - 7.42; // USDT trade + ETH withdraw
    let end_eth = start_eth + bought_eth - sold_eth + eth_correction;

    let start_weth = 0.0;
    let bought_weth = sells.iter().map(|sell| sell.weth).sum::<f64>();
    let sold_weth = buys.iter().map(|buy| buy.weth).sum::<f64>();
    let end_weth = start_weth + bought_weth - sold_weth;

    println!("OPENX: {}", end_openx);
    println!("ETH: {}", end_eth);
    println!("WETH: {}", end_weth);

    let mut realized_trades: Vec<RealizedTrade> = vec![];
    let mut sold_openx_usd = 0.0;
    for sell in &sells {
        if buys.is_empty() {
            sold_openx_usd += sell.USDC;
            break;
        }

        let mut openx = sell.OPENX;
        let sell_usd = sell.USDC;
        let mut buy_usd = 0.0;

        while openx > 0.0 && !buys.is_empty() {
            let buy = &mut buys[0];
            if buy.OPENX > openx {
                let spent_usd = (buy.USDC) * (openx / buy.OPENX);
                buy.OPENX -= openx;
                buy.USDC -= spent_usd;
                buy_usd += spent_usd;
                openx = 0.0;
            } else {
                openx -= buy.OPENX;
                buy_usd += buy.USDC;
                buys.pop();
            }
        }
        if buys.is_empty() {
            let realized_sold = sell_usd * ((sell.OPENX - openx) / sell.OPENX);
            realized_trades.push(RealizedTrade {
                openx: sell.OPENX - openx,
                sell_usd: realized_sold,
                buy_usd,
            });
            sold_openx_usd += sell_usd - realized_sold;
        } else {
            realized_trades.push(RealizedTrade {
                openx: sell.OPENX,
                sell_usd,
                buy_usd,
            });
        }
    }

    println!("Realized trades: {:?}", realized_trades);
    println!("OPENX sold total USD turnover: {}", sold_openx_usd);

    let realized_pnl = realized_trades
        .iter()
        .map(|trade| trade.sell_usd - trade.buy_usd)
        .sum::<f64>();
    println!("Realized PnL: {}", realized_pnl);

    Ok(())
}

fn main() {
    if let Err(err) = calculate() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
