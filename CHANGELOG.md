# Changelog

## 2.0.0

* Replace Decimal export with rust_decimal re-export.
* Replace ByteSize export with bytesize re-export.
* Fix doc and tes references to the above exports.
* Remove explicit crate links where unnecessary.

## 1.1.0

* Correctly handle Hetzner API returning `null` server ips as an empty vec [#5](https://github.com/MathiasPius/hrobot-rs/issues/5)

## 1.0.0

* Stable release of rewritten crate with 100% Hetzner API coverage.