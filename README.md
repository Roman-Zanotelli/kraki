# kraki

**kraki** is a Rust crate for interacting with the [Kraken API](https://docs.kraken.com/).  
This is a **personal-use** project, built primarily to experiment with and explore **kraken's websocket api** in Rust.  

---

## Goals

The guiding principles of **kraki** are:

- **WebSocket-first**  
  Prioritize the streaming API and real-time data feeds, with REST support only as-needed.

- **Channel-driven concurrency**  
  Leverage Rust’s channels to **fan-in** multiple data streams and **fan-out** messages across threads.

- **Thread-friendly architecture**  
  Make it straightforward to spawn workers that consume, transform, or route data without shared mutable state.

- **Practical, not polished**  
  This crate is **not intended for general public use**—it’s a personal project. APIs may be unstable, incomplete, or opinionated.

---

## Non-Goals

To make it clear what **kraki** is *not* aiming for:

- **Not a production SDK**  
  Error handling, performance guarantees, and backwards compatibility are not priorities.  

- **Not a REST-first library**  
  While REST support may exist, the focus is real-time data.  

- **Not feature-complete**  
  Only the Kraken API endpoints I personally need will be implemented.  

---

## Roadmap

Rough plan for development:

- [🔄] Core WebSocket connection
- [🕒] Subscribe/unsubscribe to streams
- [🔄] API Parsing
- [🕒] Typed Streams
- [_] Basic REST support (optional)

---

## Disclaimer

This crate is for **personal use** only.  
If you’re looking for a production-ready Kraken client, consider using an existing maintained project instead.  

# 🤖 AI Assistance Acknowledgment

Parts of this README were created with assistance from [ChatGPT by OpenAI](https://openai.com/chatgpt) to help organize ideas and improve clarity

> ⚖️ **Ethical Use Statement**: I believe in responsible and transparent use of AI tools. While AI helped shape the structure and language of the readme, all architectural decisions, implementation, and final content reflect my own understanding, judgment, and intent.