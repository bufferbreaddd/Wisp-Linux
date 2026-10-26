# Installer requirements

The installer is planned as a guided network installer.

1. Welcome and clear data-loss warning.
2. Verify network access; guide the user through Wi-Fi if needed.
3. Detect hardware and report any known compatibility caveats.
4. Collect locale, keyboard, timezone, user, and desktop choices.
5. Present the selected disk and a readable partition/boot plan.
6. Require explicit confirmation before writing any partition table or filesystem.
7. Install the base system and selected desktop profile, with retryable package downloads.
8. Configure boot, user access, and selected optional apps.
9. Validate the installation, show logs, and offer a reboot only after success.

Every destructive step must be cancellable before execution. The installer must never silently guess a target disk or automatically resize an existing OS partition. Network failure should preserve diagnostics and explain which step needs retrying.

Implementation is not complete; this file records requirements, not an existing installer.
