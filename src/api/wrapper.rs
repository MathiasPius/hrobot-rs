use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Deserializer};

/// Deserialize an array of objects where each object is nested
/// under a key indicating its type.
///
/// Hetzner's Robot API does not return naked objects, but
/// instead encapsulates each in a single key-value pair
/// indicating the type of the object.
///
/// For example, listing all `servers` with the Robot API does
/// not produce  an array of servers, but instead an array
/// of objects with a single key-value pair of:
///
/// `[ {"server": <ServerObject>}, ... ]`.
///
/// In order to transform this result, it is necessary to first
/// deserialize the outer map, and then extract the contained
/// server objects.
fn deserialize_inner_vec<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<T>, D::Error> {
    let tmp = Vec::<HashMap<&str, T>>::deserialize(deserializer)?;

    tmp.into_iter()
        .map(|map| {
            map.into_values()
                .next()
                .ok_or(serde::de::Error::custom("empty map"))
        })
        .try_fold(Vec::new(), |mut acc, result| {
            acc.push(result?);
            Ok(acc)
        })
}

/// Deserialize a Map of `{"object_name": <Object> }` into [`Vec<Object>`].
///
/// Hetzner's Robot API does not return naked objects, but
/// instead encapsulates each in a single key-value pair
/// indicating the type of the object.
///
/// For example, retrieving a `server` with the Robot API does
/// not produce a server objects, but instead an objects with
/// a single key-value pair of:
///
/// `{"server": <ServerObject>}`.
///
/// In order to transform this result, it is necessary to first
/// deserialize the outer map, and then extract the contained
/// server objects.
fn deserialize_inner<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    HashMap::<&str, T>::deserialize(deserializer)?
        .into_values()
        .next()
        .ok_or(serde::de::Error::custom("empty map"))
}

/// Deserialize a list of [`T`], where each T is wrapped.
#[derive(Debug, Deserialize)]
pub struct List<T: DeserializeOwned>(
    #[serde(deserialize_with = "deserialize_inner_vec")] pub Vec<T>,
);

/// Deserialize a single wrapped [`T`].
#[derive(Debug, Deserialize)]
pub struct Single<T: DeserializeOwned>(#[serde(deserialize_with = "deserialize_inner")] pub T);

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::models::Server;

    #[test]
    fn deserialize_wrapped() {
        let json = r#"{
            "server": {
                "server_ip":"1.1.1.1",
                "server_ipv6_net":"2a01:4f8:1::",
                "server_number":2000001,
                "server_name":"n1",
                "product":"Server Auction",
                "dc":"FSN1-DC1",
                "traffic":"unlimited",
                "status":"ready",
                "cancelled":false,
                "paid_until":"2070-01-01","ip":["1.1.1.1"],
                "subnet":[
                    {"ip":"2a01:4f8:1::","mask":"64"}
                ],
                "linked_storagebox":null
            }
        }"#;

        #[derive(Debug, Deserialize)]
        struct UnwrappingSingleServer {
            #[serde(deserialize_with = "crate::api::wrapper::deserialize_inner", flatten)]
            server: Server,
        }

        let UnwrappingSingleServer { server } = serde_json::from_str(json).unwrap();
        println!("{server:#?}");
    }

    #[test]
    fn deserialize_wrapped_vec() {
        let json = r#"[
        {
            "server":{"server_ip":"1.1.1.1",
                "server_ipv6_net":"2a01:4f8:1::",
                "server_number":2000001,
                "server_name":"n1",
                "product":"Server Auction",
                "dc":"FSN1-DC1",
                "traffic":"unlimited",
                "status":"ready",
                "cancelled":false,
                "paid_until":"2070-01-01",
                "ip":["1.1.1.1"],
                "subnet":[
                    {
                        "ip":"2a01:4f8:1::",
                        "mask":"64"
                    }
                ],
                "linked_storagebox":null
            }
        },
        {
            "server":{
                "server_ip":"2.2.2.2",
                "server_ipv6_net":"2a01:4f8:2::",
                "server_number":2000002,
                "server_name":"n2",
                "product":"Server Auction",
                "dc":"FSN1-DC2",
                "traffic":"unlimited",
                "status":"ready",
                "cancelled":false,
                "paid_until":"2070-01-01",
                "ip":["2.2.2.2"],
                "subnet":[
                    {
                        "ip":"2a01:4f8:2::",
                        "mask":"64"
                    }
                ],
                "linked_storagebox":null
            }
        },
            {
                "server":{
                    "server_ip":"3.3.3.3",
                    "server_ipv6_net":"2a01:4f8:3::",
                    "server_number":2000003,
                    "server_name":"n3",
                    "product":"Server Auction",
                    "dc":"FSN1-DC3",
                    "traffic":"unlimited",
                    "status":"ready",
                    "cancelled":false,
                    "paid_until":"2070-01-01",
                    "ip":["3.3.3.3"],
                    "subnet": [
                        {
                            "ip":"2a01:4f8:3::",
                            "mask":"64"
                        }
                    ],
                    "linked_storagebox":null
                }
            }
        ]"#;

        #[derive(Debug, Deserialize)]
        struct UnwrappingManyServers(
            #[serde(deserialize_with = "crate::api::wrapper::deserialize_inner_vec")] Vec<Server>,
        );

        let UnwrappingManyServers(servers) = serde_json::from_str(json).unwrap();
        println!("{servers:#?}");
    }
}
