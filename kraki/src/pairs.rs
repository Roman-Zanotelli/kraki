use crate::symbols::Symbol;

pub struct DefaultPair(Symbol, Symbol);

impl DefaultPair{
    const BTC_ETH: DefaultPair = DefaultPair(Symbol::BTC, Symbol::ETH);
    const BTC_USD: DefaultPair = DefaultPair(Symbol::BTC, Symbol::USD);
    const ETH_USD: DefaultPair = DefaultPair(Symbol::ETH, Symbol::USD);
    //Continue writing other default pairs later
}