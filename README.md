# Rust Ansible Binary Module POC for Oxide
Please note, this is a POC and GitHub co-pilot heavily helped me here with the rust! (Props to Zed with it's fancy pants co-pilot integration and vim mode!)

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
