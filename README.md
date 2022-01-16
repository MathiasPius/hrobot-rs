# hrobot-rs [![Latest Version]][crates.io] [![Docs]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/hrobot
[crates.io]: https://crates.io/crates/hrobot
[Docs]: https://docs.rs/hrobot/badge.svg
[docs.rs]: https://docs.rs/hrobot

Hetzner Robot API Client library for Rust.

Uses the blocking Reqwest client and rustls under the hood.

**Disclaimer:** the authors are not associated with Hetzner (except as customers), and the crate is in no way endorsed or supported by Hetzner Online GmbH.

# API Endpoint Implementation Progress

* ❌ [Failover](https://robot.your-server.de/doc/webservice/en.html#failover): Not implemented.
* ❌ [Traffic](https://robot.your-server.de/doc/webservice/en.html#traffic): Not implemented.
* ❌ [Server Ordering](https://robot.your-server.de/doc/webservice/en.html#server-ordering): Not implemented. I'm not made of money!
* ❌ [Storage Box](https://robot.your-server.de/doc/webservice/en.html#storage-box): Not implemented.
* ⚠️ [Firewall](https://robot.your-server.de/doc/webservice/en.html#firewall)
    * ✅ get & update firewall rules & configuration directly.
    * ❌ create, modify or delete firewall templates.
    * ❌ apply firewall templates.
* ⚠️ [IP](https://robot.your-server.de/doc/webservice/en.html#ip)
    * ✅ get ip address information.
    * ✅ get mac address information.
    * ❌ update traffic warnings for IP.
    * ❌ modify or delete additional MAC-addresses.
* ⚠️ [Subnet](https://robot.your-server.de/doc/webservice/en.html#subnet)
    * ✅ list & get subnets.
    * ❌ update traffic warnings for subnet.
    * ❌ get subnet MAC.
    * ❌ update subnet MAC.
    * ❌ delete subnet MAC.
* ✅ [vSwitch](https://robot.your-server.de/doc/webservice/en.html#vswitch): All functionality implemented.
* ✅ [Server](https://robot.your-server.de/doc/webservice/en.html#server): All functionality implemented.
* ✅ [Reset](https://robot.your-server.de/doc/webservice/en.html#reset): All functionality implemented.
* ✅ [Wake On Lan](https://robot.your-server.de/doc/webservice/en.html#wake-on-lan): All functionality implemented.
* ✅ [Boot Configuration](https://robot.your-server.de/doc/webservice/en.html#boot-configuration): All functionality implemented (but some untested due to costs of add-ons).
* ✅ [Reverse DNS](https://robot.your-server.de/doc/webservice/en.html#reverse-dns): All functionality implemented.
* ✅ [SSH Keys](https://robot.your-server.de/doc/webservice/en.html#ssh-keys): All functionality implemented.

# Testing
Testing relies on `$HROBOT_USERNAME` and `$HROBOT_PASSWORD` being defined in the environment, corresponding to a Hetzner WebService/app login.

Some of the tests which interact with the Hetzner API can be disruptive, and therefore any test which interacts with Hetzner is marked as `#[ignore]` just in case `cargo test` is accidentally run while the `HROBOT_USERNAME` and `HROBOT_PASSWORD` environment variables are available. To explicitly run these potentially disruptive tests, either use `cargo test -- --ignored` to run all of them, or run the test explicitly using `cargo test server::tests::list_servers -- --ignored`