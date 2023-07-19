# Errata

This document aims to list some of the discrepancies, oddities, unexpected behaviours and undocumented features discovered while developing this library.

## Undocumented Features

### Boot Configuration
* Last boot configuration endpoints (`boot/{server-number}/<type>/last`) appear to exist for VNC, Windows, Plesk and CPanel, but are not documented.

* Active boot configuration options contain a `boot_time` field, which presumably indicates the time at which this boot configuration was booted into.
  I say presumably because the field is always `null` if the system has been enabled, but the server hasn't been rebooted. I haven't had a chance to test this yet.


## Typos & minor mistakes
* [IP Cancellation examples](https://robot.hetzner.com/doc/webservice/en.html#get-ip-ip-cancellation) indicate that the returned structure contains a field
  named `cancellation-date` (not the hyphen), but the documentation and every other field uses underscores.
