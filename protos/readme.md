


NOTES for G1P2P build & deploy




# some pre-reqs

1. To be able to compile russt binary linked with older versions of glibc 

from https://stackoverflow.com/questions/63724484/build-and-bind-against-older-libc-version

$ cargo install cross
$ cross build --target x86_64-unknown-linux-gnu  --release

Note: 
    ~Try cross.

    Install it globally:

    cargo install cross

    Then build your project with it:

    cross build --target x86_64-unknown-linux-gnu --release

    cross take the same arguments as cargo but you have to specify a target explicitly. Also, the build directory is always target/{TARGET}/(debug|release), not target/(debug|release)

    cross uses docker images prebuilt for different target architectures but nothing stops you from "cross-compiling" against the host architecture. The glibc version in these docker images should be conservative enough. If it isn't, you can always configure cross to use a custom image.



# compliation

1. using cargo

2. Auction App 

3. Auction Services


# deploy

1. provisioning of machines using vagrant

- at the end: copy all SSH private keys to Ansible-controller-SSD (to be used by ansible)

        C> copia_chaves_ssh.bat


2. deploy using ansible from Ansible-controller-SSD machine


- at the beginnig, add ssh keys to trustable:

        $ ssh-agent bash
        $ adiciona_chaves_porivadas.sh

- deploy of binaries

        $ ansible-playbook -i SSD_hosts   SSDDeploy.yml

(SSD_hosts is the inventory file with machines list)




