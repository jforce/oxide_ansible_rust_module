# Quick Start

Run an example playbook (expects an ansible vault and respective password file):
```bash
ANSIBLE_LIBRARY=./oxide.computer ansible-playbook ./playbook.yml --vault-pass-file .vault_pass.txt
```

Render ansible docs for the module:
```bash
ansible-doc -M oxide.computer/plugins/modules oxide_ssh_key
ansible-doc -M oxide.computer/plugins/modules oxide_ssh_key_info
```

Check ssh key is as expected:
```bash
oxide current-user ssh-key view --ssh-key rusty-ansible-key
```
