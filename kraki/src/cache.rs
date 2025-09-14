use std::sync::Arc;

use dashmap::DashMap;
use tokio_stream::wrappers::WatchStream;

use crate::parse::market_data::{book::Book, candles::Candles, instruments::Instruments, orders::Orders, ticker::Ticker, trades::Trades};

/// `StateStream<T>`
///
/// A stream of states for a value of type `T`, where each state is wrapped in an `Arc`. <br>
/// Mainly used to stream kraken's `Book`, `Candles`, `Instruments`, `Orders`, `Ticker`, and `Trades` updates across consumer threads. <br>
///
/// - Uses `Arc<T>` internally to allow multiple consumers to share ownership of the data frame.
/// - Memory for `T` is automatically dropped when no `Arc` references remain.
/// - `WatchStream` ensures that the latest value is always available and strongly held until a new update arrives.
pub type StateStream<T> = WatchStream<Arc<T>>;

/// `StateStreamCache<T>`
///
/// A cache of `StateStream<T>` instances, keyed by `String`.
///
/// - Internally uses a `DashMap` to shard the streams, allowing concurrent reads and writes.
/// - New entries can be added without blocking reads of existing streams.
/// - Wrapped in an `Arc` to allow safe sharing across threads.
pub type StateStreamCache<T> = Arc<DashMap<String, StateStream<T>>>;

/// `BookCache`
///
/// A sharded, concurrent cache of `StateStream<Book>` keyed by symbol.
///
/// - Each `StateStream` holds the latest update for its symbol, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Adding new symbol streams to the cache does not block subscribing to existing ones.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type BookCache = StateStreamCache<Book>;

/// `CandlesCache`
///
/// A sharded, concurrent cache of `StateStream<Candles>` keyed by symbol.  
/// 
/// - Each `StateStream` holds the latest update for its symbol, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Adding new symbol streams to the cache does not block subscribing to existing ones.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type CandlesCache = StateStreamCache<Candles>;

/// `InstrumentsCache`
///
/// A single `StateStream<Instruments>` representing the current state of all instruments. <br> 
/// Unlike the other caches, there is only one stream, not keyed by symbol.
/// 
/// - `StateStream` holds the latest update, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type InstrumentsCache = StateStream<Instruments>;

/// `OrdersCache`
///
/// A sharded, concurrent cache of `StateStream<Orders>` keyed by symbol.  
/// 
/// - Each `StateStream` holds the latest update for its symbol, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Adding new symbol streams to the cache does not block subscribing to existing ones.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type OrdersCache = StateStreamCache<Orders>;

/// `TickerCache`
///
/// A sharded, concurrent cache of `StateStream<Ticker>` keyed by symbol.  
/// 
/// - Each `StateStream` holds the latest update for its symbol, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Adding new symbol streams to the cache does not block subscribing to existing ones.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type TickerCache = StateStreamCache<Ticker>;

/// `TradesCache`
///
/// A sharded, concurrent cache of `StateStream<Trades>` keyed by symbol.  
/// 
/// - Each `StateStream` holds the latest update for its symbol, allowing multiple threads to subscribe and consume data.
/// - Memory is bounded to the current frame; New streams consume from the current state and buffer proceeding updates.
/// - Adding new symbol streams to the cache does not block subscribing to existing ones.
/// - Memory is dropped when no stream is buffering it and it is no longer the current state of the symbol.
pub type TradesCache = StateStreamCache<Trades>;