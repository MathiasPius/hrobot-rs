# hrobot-rs [![Latest Version]][crates.io] [![Docs]][docs.rs] [![codecov]][codecov.io]

[Latest Version]: https://img.shields.io/crates/v/hrobot
[crates.io]: https://crates.io/crates/hrobot
[Docs]: https://docs.rs/hrobot/badge.svg
[docs.rs]: https://docs.rs/hrobot
[codecov]: https://codecov.io/gh/MathiasPius/hrobot-rs/graph/badge.svg?token=AZZRACXKN1
[codecov.io]: https://app.codecov.io/gh/MathiasPius/hrobot-rs

<!-- cargo-rdme start -->

`hrobot` is an unofficial asynchronous Rust client for interacting with the [Hetzner Robot API](https://robot.your-server.de/doc/webservice/en.html)

See the `AsyncRobot` struct for a complete list of supported API Endpoints.

**Disclaimer:** the authors are not associated with Hetzner (except as customers), and the crate is in no way endorsed or supported by Hetzner Online GmbH.

## Requirements for usage
A Hetzner WebService/app user is required to make use of this library.

If you already have a Hetzner account, you can create one through the [Hetzner Robot](https://robot.your-server.de) web interface under [Settings/Preferences](https://robot.your-server.de/preferences/index).

## Example
Here's a quick example showing how to instantiate the `AsyncRobot` client object
and fetching a list of all dedicated servers owned by the account identified by `username`
```rust
use hrobot::*;

#[tokio::main]
async fn main() {
    // Robot is instantiated using the environment
    // variables HROBOT_USERNAME an HROBOT_PASSWORD.
    let robot = AsyncRobot::default();

    for server in robot.list_servers().await.unwrap() {
        println!("{name}: {product} in {location}",
            name = server.name,
            product = server.product,
            location = server.dc
        );
    }
}
```

Running the above example should yield something similar to the output below:
```text
foo: AX51-NVMe in FSN1-DC18
bar: Server Auction in FSN1-DC5
```

<!-- cargo-rdme end -->

## API Endpoint Implementation Progress

The entire Hetzner Robot API is covered at this point!

Some endpoints have not been tested due to the cost and disruption to running servers, so
please consult the section below for a detailed overview of the endpoints.
<details><summary>
Detailed API overview.
</summary>

**Warning!**

[^1] not tested, use at your own risk.

[^2] not officially documented by Hetzner, use at own risk.

- [x] **Server.**
    - [x] List servers.
    - [x] Get server.
    - [x] Rename server.
    - [x] **Cancellation.**
        - [x] Get cancellation status.
        - [x] Cancel server.[^1]
        - [x] Withdraw cancellation.[^1]
    - [x] ~~Withdraw server order.~~[^1] - Deprecated by Hetzner.
- [x] **IP.**
    - [x] List IPs.
    - [x] Get IP.
    - [x] Update traffic warnings.
    - [x] **Separate MAC.**
        - [x] Get separate MAC.
        - [x] Generate separate MAC.[^1]
        - [x] Disable separate MAC.[^1]
    - [x] **Cancellation.**
        - [x] Get cancellation status.
        - [x] Cancel IP address[^1]
        - [x] Withdraw cancellation[^1]
- [x] **Subnet.**
    - [x] List subnets.
    - [x] Get subnet.
    - [x] **Separate MAC.**
        - [x] Get separate MAC.
        - [x] Generate separate MAC.[^1]
        - [x] Disable separate MAC.[^1]
    - [x] **Cancellation.**[^1]
        - [x] Get cancellation status.[^1]
        - [x] Cancel subnet.[^1]
        - [x] Withdraw cancellation.[^1]
- [x] **Reset**
    - [x] List reset options for all servers.
    - [x] Get reset options for single server
    - [x] Trigger reset.[^1]
- [x] **Failover**
    - [x] Get failover IP.[^1]
    - [x] Switch routing of failover traffic.[^1]
    - [x] Disable failover routing.[^1]
- [x] **Wake on LAN**
    - [x] Check availability of Wake-on-LAN.
    - [x] Send Wake-on-LAN packet to server.
- [x] **Boot Configuration**
    - [x] Get status of all boot configurations.
    - [x] **Rescue.**
        - [x] Get rescue config.
        - [x] Get last rescue config.
        - [x] Enable rescue config.
        - [x] Disable rescue config.
    - [x] **Linux.**
        - [x] Get linux config.
        - [x] Get last linux config.
        - [x] Enable linux config.
        - [x] Disable linux config.
    - [x] **VNC.**
        - [x] Get VNC config.
        - [x] Get last VNC config.[^2]
        - [x] Enable VNC config.
        - [x] Disable VNC config.
    - [x] **Windows.**[^1]
        - [x] Get Windows config.[^1]
        - [x] Get last Windows config.[^1] [^2]
        - [x] Enable Windows config.[^1]
        - [x] Disable Windows config.[^1]
    - [x] **Plesk.**[^1]
        - [x] Get Plesk config.[^1]
        - [x] Get last Plesk config.[^1] [^2]
        - [x] Enable Plesk config.[^1]
        - [x] Disable Plesk config.[^1]
    - [x] **CPanel.**[^1]
        - [x] Get CPanel config.[^1]
        - [x] Get last CPanel config.[^1] [^2]
        - [x] Enable CPanel config.[^1]
        - [x] Disable CPanel config.[^1]
- [x] **Reverse DNS.**
    - [x] List reverse DNS entries.
    - [x] Get reverse DNS entry
    - [x] Create reverse DNS entry.
    - [x] Update/create reverse DNS entry.
- [x] **Traffic.**
    - [x] Query traffic data.
- [x] **SSH Keys.**
    - [x] List SSH keys
    - [x] Upload new SSH key
    - [x] Get SSH key
    - [x] Rename SSH key
    - [x] Delete SSH key
- [x] **Server Ordering.**
    - [x] **Products.**
        - [x] List products.
        - [x] Get product information.
        - [x] List recent product transactions.[^1]
        - [x] Get specific transaction information.[^1]
        - [x] Order new product.
    - [x] **Market (auction).**
        - [x] List market products.
        - [x] Get market product information.
        - [x] List recent market transactions.[^1]
        - [x] Get specific market transaction information.[^1]
        - [x] Order new server from market.
    - [x] **Addons.**
        - [x] List available addons for server.
        - [x] List recent addon transactions.[^1]
        - [x] Get specific addon transaction information.[^1]
        - [x] Order addon for server.
- [x] **Storage Box.**
    - [x] List storageboxes.
    - [x] Get specific storage box.
    - [x] Change storage box password.
    - [x] Toggle storage box services.
        - [x] Enable/disable Samba
        - [x] Enable/disable WebDAV
        - [x] Enable/disable SSH
        - [x] Enable/disable External reachability
        - [x] Enable/disable snapshot directory visibility.
    - [x] **Snapshots.**
        - [x] List storagebox snapshots.
        - [x] Create storagebox snapshot.
        - [x] Delete storagebox snapshot.
        - [x] Revert storagebox to snapshot.
        - [x] Change comment for snapshot.
        - [x] Get storagebox snapshot plan.
        - [x] Edit storagebox snapshot plan.
    - [x] **Subaccounts.**
        - [x] List subaccounts.
        - [x] Create subaccount.
        - [x] Update subaccount configuration.
        - [x] Delete subaccount.
        - [x] Change subaccount password.
- [x] **Firewall.**
    - [x] Get firewall configuration for server.
    - [x] Apply firewall configuration to server.
        - [x] Override rules.
        - [x] Apply template.
    - [x] Clear firewall configuration for server.
    - [x] **Template.**
        - [x] List firewall templates.
        - [x] Create firewall template.
        - [x] Get firewall template.
        - [x] Update firewall template.
        - [x] Delete firewall template.
- [x] **vSwitch.**
    - [x] List vSwitches.
    - [x] Create new vSwitch.
    - [x] Get vSwitch.
    - [x] Update vSwitch.
    - [x] Cancel vSwitch.
    - [x] Add servers to vSwitch.
    - [x] Remove servers from vSwitch.

[^1]: Not tested, but *should* work. Use at own risk.

[^2]: Not officially documented by Hetzner, use at own risk.
</details>

## Testing
Tests are divided into two categories:
* **Unit tests.**
s
    These do not touch the Hetzner API at all and generally test assumptions made in some of the constructs of the library
    such as serialization/deserialization from known API output. These are always safe to run and do not require Hetzner credentials.

* **Integration tests. ⚠️**

    These tests *do* interact with the Hetzner API, and therefore require the following two environment variables to be set:

    * `HROBOT_USERNAME`
    * `HROBOT_PASSWORD`

    These tests will ⚠️ **PURCHASE AN AUCTION SERVER** ⚠️ and (ideally) *cancel* the server once tests have run.

    They interact ⚠️ **DESTRUCTIVELY** ⚠️ with the resources provided through the following environment variables:

    * `HETZNER_INTEGRATION_TEST_STORAGEBOX_ID` specifies a pre-allocated storagebox resource which Storagebox-related tests are run against.
