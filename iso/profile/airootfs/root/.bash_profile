#!/usr/bin/env bash
# Launch the installer on the first virtual console in the live image.
if [[ "$(tty 2>/dev/null || true)" == "/dev/tty1" && -x /usr/local/bin/wisp-installer ]]; then
  exec /usr/local/bin/wisp-installer
fi
