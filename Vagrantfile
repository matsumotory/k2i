Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-16.04"
  config.vm.provision "shell", :path => "misc/provision.sh", :privileged => false
  config.vm.provider "virtualbox" do |vb|
    vb.cpus = 2
  end
end
