# Quick Start

Run an example playbook (expects an ansible vault and respective password file):
```bash
ANSIBLE_LIBRARY=./oxide.computer ansible-playbook ./playbook.yml --vault-pass-file .vault_pass.txt
```

Render ansible docs for the module:
```bash
ansible-doc -M oxide.computer/plugins/modules oxide_ssh_key
```

Check ssh key is as expected:
```bash
oxide current-user ssh-key view --ssh-key rusty-ansible-key
```

Example:
```bash
$ ANSIBLE_LIBRARY=./oxide.computer ansible-playbook ./playbook.yml --vault-pass-file .vault_pass.txt

PLAY [Test the oxide_ssh_key module] *********************************************************************************************************************************************************

TASK [Gathering Facts] ***********************************************************************************************************************************************************************
ok: [somemachine]

TASK [Create SSH key] ************************************************************************************************************************************************************************
changed: [somemachine]

TASK [Idempodency check] *********************************************************************************************************************************************************************
ok: [somemachine]

TASK [Pause] *********************************************************************************************************************************************************************************
Pausing for 15 seconds
(ctrl+C then 'C' = continue early, ctrl+C then 'A' = abort)
Press 'C' to continue the play or 'A' to abort
ok: [somemachine]

TASK [Delete SSH key] ************************************************************************************************************************************************************************
changed: [somemachine]

TASK [Delete SSH key check] ******************************************************************************************************************************************************************
ok: [somemachine]

TASK [Create SSH key] ************************************************************************************************************************************************************************
changed: [somemachine]

TASK [Failure check - this is expected to fail!] *********************************************************************************************************************************************
fatal: [somemachine]: FAILED! => {"changed": false, "msg": "SSH key name 'rusty-ansible-key' already exists", "ssh_key": null}
...ignoring

PLAY RECAP ***********************************************************************************************************************************************************************************
somemachine : ok=8    changed=2    unreachable=0    failed=0    skipped=0    rescued=0    ignored=1
```
