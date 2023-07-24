# Errata

This document aims to list some of the discrepancies, oddities, unexpected behaviours and undocumented features discovered while developing this library.

## Consistency Issues

### vSwitch
* Most (almost all?) endpoints *except* the vSwitch endpoints return objects in the form of an outer object containing a single field indicating the type of object, and an
  inner object which actually constitutes the object itself.
  
  For instance, `GET https://robot-ws.your-server.de/server/321` will return something like:
  ```json
  {
    "server":{
      "server_ip":"123.123.123.123",
      "server_ipv6_net":"2a01:f48:111:4221::",
      "server_number":321,
      (and so on)
    }
  }
  ```
  Instead of just the plain object:
  ```json
  {
    "server_ip":"123.123.123.123",
    "server_ipv6_net":"2a01:f48:111:4221::",
    "server_number":321,
    (and so on)
  }
  ```
  This is a little strange, but if used consistenctly across the API might make sense, and could potentially resolve ambiguity for endpoints where the type of the 
  returned object is not obvious, though I can't think of any examples in the API.
  
  However, the vSwitch endpoints specifically *do not* follow this convention, and instead return the "naked" objects themselves.
  
* Update/modify endpoint does not return the updated object. Similar endpoints for servers, boot configs, etc. do.

* Delete endpoint does not return the cancellation date, and querying for cancellation date information is not available.

### Reverse DNS
 * Reverse DNS entry listing *may* sporadically return `null` for the `PTR` record field in entries, even if the reverse dns entry has been set.
   
   Setting a Reverse DNS entry to an empty string resets the value back to the original `static.<IP>.clients.your-server.de` which suggests that
   a `null` value should not even be possible.
   
### Storage Box
 * According to the API documentation, the Storage Box "snapshot plans" support a yearly schedule, and the API accepts this, but this format is 
   not representable by the Robot UI, which displays this, if set, as a monthly schedule.
   
 * According to the API documentation, when simply *disabling* snapshot plans for a storage box, you are not required to input the required
   hour and minute fields, yet when I tried this, by setting only the body `status=disabled`, I got back the following error message:
   
  ```json
  {
    "error":{
      "status":400,
      "code":"INVALID_INPUT",
      "message":"invalid input",
      "missing":["minute","hour"],
      "invalid":null
    }
  }
  ```

## Undocumented Features

### Boot Configuration
* Last boot configuration endpoints (`boot/{server-number}/<type>/last`) appear to exist for VNC, Windows, Plesk and CPanel, but are not documented.

* Active boot configuration options contain a `boot_time` field, which presumably indicates the time at which this boot configuration was booted into.
  I say presumably because the field is always `null` if the system has been enabled, but the server hasn't been rebooted. I haven't had a chance to test this yet.


## Typos & minor mistakes
* [IP Cancellation examples](https://robot.hetzner.com/doc/webservice/en.html#get-ip-ip-cancellation) indicate that the returned structure contains a field
  named `cancellation-date` (note the hyphen), but the documentation and every other field uses underscores.

* [Update Snapshot Plan example](https://robot.hetzner.com/doc/webservice/en.html#post-storagebox-storagebox-id-snapshotplan) points to `snapshot/{snapshot}/comment`
  as the target URL, when it should be `snapshot/{snapshot}/snapshotplan`