# Changelog

⚠️ indicates breaking changes.

## 6.1.1

* Updated `thiserror` to 2.0
* Updated `rand` to 0.9

## 6.1.0

* Added `AsyncRobot::new_with_default_client` for constructing a client using only username and password, and the default client.

## 6.0.0

* ⚠️ `revoke_ip_cancellation` renamed to `withdraw_ip_cancellation` for consistency.
* ⚠️ `revoke_subnet_cancellation` renamed to `withdraw_subnet_cancellation` for consistency.
* ⚠️ `withdraw_server_cancellation` now returns `()` not `Cancellation`, as defined in the Robot API.
* ⚠️ `cancel_server` now takes an `Option<Date>`, defaulting to *immediate* cancellation.
* ⚠️ `withdraw_server_order` removed. The API endpoint appears to have been silently removed, meaning calls to it will fail.

## 5.0.1

* Fix improper deserialization of response from calling `trigger_reset`.

## 5.0.0

* ⚠️ vSwitch cancellation's `date` argument is now optional. Omitting the parameter causes immediate cancellation.
* Implement PartialEq & Eq for vSwitch/server `ConnectionStatus`.
* Implement PartialEq & Eq for server `Status`, `SubnetReference` and `ServerFlags`.

## 4.0.0

* ⚠️ Update server pricing models to include the new [hourly pricing](https://docs.hetzner.com/general/others/new-billing-model/).

## 3.0.0

* Update `hyper` dependency to `1.0`. This is considered a breaking change because we expose the ability to
    construct your own `AsyncRobot` using a custom `hyper::Client`, which was [removed in hyper v1.0](https://hyper.rs/guides/1/upgrading/).
    The *Client* functionality is still available in the spin-off crate [hyper-util](https://github.com/hyperium/hyper-util),
    which is also what `hrobot-rs` uses now.
* Switch to using the rustls built-in webpki roots by default, instead of native roots. This is potentially a breaking change,
    but in all likelihood, this won't impact you unless you're behind an intercepting firewall man-in-the-middling your traffic.
    If you need to override this behaviour, see [AsyncRobot::new](https://docs.rs/hrobot/3.0.0/hrobot/struct.AsyncRobot.html#method.new)
    for information about providing your own customized hyper client.
* Update `serial_test` dependency to v3.0.0

## 2.0.0

* Replace Decimal export with rust_decimal re-export.
* Replace ByteSize export with bytesize re-export.
* Fix doc and test references to the above exports.
* Remove explicit crate links where unnecessary.

## 1.1.0

* Correctly handle Hetzner API returning `null` server ips as an empty vec [#5](https://github.com/MathiasPius/hrobot-rs/issues/5)

## 1.0.0

* Stable release of rewritten crate with 100% Hetzner API coverage.
