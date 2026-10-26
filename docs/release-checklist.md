# Wisp Preview Release Checklist

This checklist is deliberately conservative. A successful image build is not the same as a tested installer.

## Automated gates

- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo test --workspace --all-targets` passes.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] All shell scripts pass `bash -n`.
- [ ] Archiso produces a non-empty `.iso` file.
- [ ] The ISO checksum file is generated and uploaded with the image.

## Manual VM checks before calling a preview usable

- [ ] Boot the ISO in a disposable virtual machine using UEFI.
- [ ] Confirm the live environment reaches the installer without a blank screen or login loop.
- [ ] Confirm the installer clearly explains that it hands off to upstream Archinstall and does not yet apply Wisp desktop profiles.
- [ ] Test the no-network path, retry path, and cancel path.
- [ ] Verify no disk operation starts before the user reviews and confirms choices in Archinstall.
- [ ] Complete a test installation to a disposable virtual disk only.
- [ ] Confirm the VM can reboot into the installed system.
- [ ] Test the `wisp doctor`, `wisp store`, and `wisp mods` commands and ensure failures are understandable.
- [ ] On a separate disposable installed Arch VM, test each `wisp desktop list` / `wisp desktop install <profile>` path; confirm cancel makes no changes, package failure does not enable services, and successful installation reaches a graphical login after reboot.
- [ ] Confirm desktop profile installation refuses to run inside the temporary Archiso live environment.
- [ ] Verify each supported desktop session can log in and has working network, audio, and basic graphics in the VM.
- [ ] Check the install log and confirm the store requires explicit confirmation before package installation.

## Release decision

Do not label the preview stable or daily-use-ready until the automated gates and VM checks above pass. Do not test on a machine or disk containing important data. Real-hardware, GPU, Wi-Fi, suspend/resume, and dual-boot compatibility remain separate validation work.
