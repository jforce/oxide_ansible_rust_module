# Rust Ansible Binary Module POC for Oxide
This is just a proof of concept, and with the API not supporting updates yet it's not a true idempotent module. I previously POC'd a 'traditional' phython collection, see [oxide.computer python collection](https://github.com/jforce/oxide.computer), however, I'm not sure which way is most ideal to go with. I do like this approach, but it feels a bit more non-standard going the way of a python collection. The python collection I started with was/is very slapdash, but its shows the premise of where to go with that. I did enjoy the Rust approach, and I think it's a bit more fun to work with. I'm not sure if it's the 'right' way, would need to ask the Ansible folks.

## Dev notes
* Oxide docs:
  - [Oxide API Docs](https://docs.oxide.computer/api/current_user_ssh_key_create)
  - [Oxides Terraform Provider](https://github.com/oxidecomputer/terraform-provider-oxide) for reference
* Ansible docs:
  - [Developing binary modules](https://docs.ansible.com/ansible/latest/dev_guide/developing_program_flow_modules.html#non-native-want-json-modules)
  - [Sidecar documentation](https://docs.ansible.com/ansible/latest/dev_guide/sidecar.html)
  - [Documenting collections](https://docs.ansible.com/ansible/latest/dev_guide/developing_collections_documenting.html)
  - [Creating an info or a facts module](https://docs.ansible.com/ansible/latest/dev_guide/developing_modules_general.html#creating-an-info-or-a-facts-module)
* Misc:
  - [Example binary module](https://github.com/ansible/ansible/tree/devel/test/integration/targets/binary_modules)
  - [Nice slides on writing ansible modules](https://www.denog.de/media/DENOG11/day1_9_20191111-DENOG11-AnsibleModules-anim_HUBrLJX.pdf)

## TO DO
  * Introduce an update function when the API supports it.

## Thoughts
Maybe I need to revist the python collection and start somewhat afresh now I've gained some understanding of how to develop Ansible modules, however, I feel like I need some advise from the Ansible folks on the best way to go with this. Happy to work on this more, but I'm not sure what is the right direction.
