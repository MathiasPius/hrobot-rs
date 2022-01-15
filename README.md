# hrobot-rs
Hetzner Robot API Client library for Rust.

Uses the blocking Reqwest client and rustls under the hood.

# API Endpoint Implementation Progress

* ❌ [Failover](https://robot.your-server.de/doc/webservice/en.html#failover): Not implemented.
* ❌ [Traffic](https://robot.your-server.de/doc/webservice/en.html#traffic): Not implemented.
* ❌ [Server Ordering](https://robot.your-server.de/doc/webservice/en.html#server-ordering): Not implemented.
* ❌ [Storage Box](https://robot.your-server.de/doc/webservice/en.html#storage-box): Not implemented.
* ⚠️ [Server](https://robot.your-server.de/doc/webservice/en.html#server)
    * ✅ list, get, and rename servers.
    * ✅ view cancellation status and options.
    * ❌ revoke server order.
* ⚠️ [Firewall](https://robot.your-server.de/doc/webservice/en.html#firewall)
    * ✅ get & update firewall rules & configuration directly.
    * ❌ create, modify or delete firewall templates.
    * ❌ apply firewall templates.
* ⚠️ [IP](https://robot.your-server.de/doc/webservice/en.html#ip)
    * ✅ get ip address information.
    * ✅ get mac address information.
    * ❌ updating traffic warnings for IP.
    * ❌ modifying or deleting additional MAC-addresses.
* ⚠️ [Subnet](https://robot.your-server.de/doc/webservice/en.html#subnet)
    * ✅ list & get subnets.
    * ❌ updating traffic warnings for subnet.
    * ❌ get subnet MAC.
    * ❌ update subnet MAC.
    * ❌ delete subnet MAC.
* ❌ [vSwitch](https://robot.your-server.de/doc/webservice/en.html#vswitch): Not implemented.
* ✅ [Reset](https://robot.your-server.de/doc/webservice/en.html#reset): All functionality implemented.
* ✅ [Wake On Lan](https://robot.your-server.de/doc/webservice/en.html#wake-on-lan): All functionality implemented.
* ✅ [Boot Configuration](https://robot.your-server.de/doc/webservice/en.html#boot-configuration): All functionality implemented (but untested due to costs of add-ons).
* ✅ [Reverse DNS](https://robot.your-server.de/doc/webservice/en.html#reverse-dns): All functionality implemented.
* ✅ [SSH Keys](https://robot.your-server.de/doc/webservice/en.html#ssh-keys): All functionality implemented.
