file access debug
===

Produce a fapolicyd like log from fanotify events

### run

mount a tmpfs `mount -t tmpfs tmpfs /tmp/foo`

run the app `fad /tmp/foo`

### references

- https://github.com/linux-application-whitelisting/fapolicyd
- https://github.com/Serinalice/fanotify-rs
