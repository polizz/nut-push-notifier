# NUT notifications to Gotify

## Why?

This is a small utility for pushing [NUT] UPS status changes to Gotify. This was originally created because [TrueNAS] has support for running the NUT daemon but does not allow a custom notification URL to be set.

With the NUT service option in TrueNAS turned on, this utility talks to NUT daemon and pushes appropriate notifications when encountered.

[NUT]: https://networkupstools.org/
[Gotify]: https://gotify.net/
[TrueNAS]: https://www.truenas.com/

## Commands
**Typical invocation:**
```sh
nut-push-notifier -h <NUT_HOST> -j <NUT_USER> -x <NUT_PASS> SUB_COMMAND
```

**Help:**
```sh
# Top help
nut-push-notifier help

# Subcommand help
nut-push-notifier help SUB_COMMAND
```

**List all UPS' connected to NUT daemon and list their variables:**
```sh
nut-push-notifier -h <NUT_HOST> -j <NUT_USER> -x <NUT_PASS> listvars
```

**Monitor specific UPS status:**
```sh
nut-push-notifier -h <NUT_HOST> -j <NUT_USER> -x <NUT_PASS> watch -u <GOFITY_URL> -p <GOFITY_TOKEN>
```

**Override watch with specific UPS name, status variable, and state text values to match (from list above)**
```sh
nut-push-notifier -h <NUT_HOST> -j <NUT_USER> -x <NUT_PASS> watch -u <GOFITY_URL> -p <GOFITY_TOKEN> -b UPS2 -w ups2.status -o "STATUS_ONLINE" 
```

**Verbose online notifications**

If you want to be notified when your ups transitions from ***online*** to ***online and charging*** and vice versa, use the **`-v`** option.