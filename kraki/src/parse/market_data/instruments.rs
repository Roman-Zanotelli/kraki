pub struct Instruments{
    assets: Vec<Asset>,
    pairs: Vec<Pair>
}

pub struct Asset{
    borrowable: bool,
    collateral_value: f64,
    id: String,
    margin_rate: f64,
    precision: i64,
    precision_display: i64,
    multiplier: f64,
    status: String,
}

pub struct Pair{
    base: String,
    quote: String,
    cost_min: String,
    cost_precision: i64,
    has_index: bool,
    margin_initial: Option<f64>,
    marginable: bool,
    position_limit_long: Option<i64>,
    position_limit_short: Option<i64>,
    price_increment: f64,
    price_precision: i64,
    qty_increment: f64,
    qty_min: f64,
    qty_precision: i64,
    status: String,
    symbol: String,
    tick_size: Option<f64> //deprecated
}