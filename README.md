```
cargo install https://github.com/aquarhead/tmp-ssh-key.git
```

This small tool basically:

- Read private key from stdin
- Write to a temp file
- Change permission on it so `ssh` is okay to use the key (600)
- Output temp file name to stdout

This could be useful when key retriving can be automated/scripted, in a trusted environment. Example:

```
alias issh='ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null'
issh user@ip -i $(echo "some key" | tmp-ssh-key)
```

NOTE: this example explicitly chose to not verify server fingerprint, for example if these are virtual machines then their IPs can be rotated. It's not safe on public network.
