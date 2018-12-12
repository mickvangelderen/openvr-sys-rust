A crate already exists but I don't have enough experience right now to
contribute. I would if I did.

OpenVR clutters stdout/stderr with a whole bunch of errors, warnings and shader
compilation messages. Not too happy about that. Also some of the function are
inlined in the headers which bindgen cann't translate to rust automatically.

OpenVR requires SteamVR, the application that actually provides the driver
implementations for common HMDs. SteamVR should be installed on the system
through Steam.

It doesn't seem like the OpenVR is very "open". Basically its just a bunch of
virtual function table definitions and constants. The real meat is hidden in
SteamVR, as far as I understand at least.
