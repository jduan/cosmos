Use Ansible to set up a Mac for software development.

Prerequisites:
* Manually install [homebrew](https://brew.sh/) first
* Install Ansible via homebrew: brew install ansible

Run the following command to set up your Mac:
$ ansible-playbook -i inventory site.yml -c local -vv
