# election-administrator

## Intro
A proof of concept smart contract for nigerian partisan election administration using ink!. A rust embedded  domain specific language (eDSL) for writing webassembly (wasm) smart contracts which can deployed to a substrate based blockchain/parachain.

 The election administrator smart contract aims to streamline the entire election administration process by providing the following core functionalities:

1. voter registration
2. Registration of political parties together with their candidate for elections
3. casting and recording of votes
4. voter accreditation
4. vote count and collation of election results

The election administration domain presents one of the numerous use cases where blockchain technology really shines not only in terms of security and integrity of election data  but also in the enablement of transparency and flexibility of the election process as well as drastically reducing the overall cost of administering nationwide partisan elections.

## How to build
Due to the heavy nature in terms of libraries, tooling and crates associated with developing wasam smart contracts, i recommend isolating smart contact development to a virtual machine (VM). To this end, Vagrantfile is included in the project root folder which is used to spin up a VM and provision a smart contract development environment(i.e tooling, libraries, .. etc). To build this smart contract please follow the steps below:

### Build requirements
1. Virtualbox (6.1.40)
2. Vagrant (2.2.9)
3. Ruby (2.7.2)

### Steps
1. $ git clone https://github.com/harryobas/election-administrator.git
2. $ cd election-administrator
3. $ vagrant up
4. $ vagrant ssh
5. $ cd /vagrant/
6. $ cargo +nightly test
7. $ cargo +nightly contract build





