# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure(2) do |config|
  # For a complete reference, please see the online documentation at
  # https://docs.vagrantup.com.

  # Every Vagrant development environment requires a box. You can search for
  # boxes at https://atlas.hashicorp.com/search.
  config.vm.box = "debian/jessie64"

  config.vm.provider :virtualbox do |v|
    v.name = "debian"
  end

  # Enable provisioning with a shell script. Additional provisioners such as
  # Puppet, Chef, Ansible, Salt, and Docker are also available. Please see the
  # documentation for more information about their specific syntax and use.
   config.vm.provision "shell", inline: <<-SHELL
      sudo apt-get update
      sudo apt-get install nasm -y
      sudo apt-get install xorriso -y
      sudo apt-get install git -y
      sudo apt-get install vim -y
      sudo apt-get install -y qemu
      sudo apt-get install -y make
      sudo apt-get install -y binutils
      curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh -s -- --yes
      multirust default nightly-2015-11-19
  SHELL

  config.ssh.forward_x11 = true
end
