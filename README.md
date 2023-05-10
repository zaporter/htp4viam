<h1 >
<h1 align="center">
  <br>
  <img src="https://github.com/zaporter/htf4viam/blob/main/etc/logo.png?raw=true" alt="HTF Logo" width="200" style="border-radius:50%; ">
  <br>
  <p>Rigor Hardware Testing Platform</p>
  <br>
</h1>

<h1></h1>


<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#install">Install</a> •
  <a href="#credits">Credits</a> •
  <a href="#license">License</a>
</p>



## Key Features

## How to Use

### Setting up the test-orchestrator

Download the install ISO from: TODO
Run
`sudo /iso/rigor_htp/nixos/scripts/install/InstallISO.sh`
Reboot and remove the installation medium
Run `./rigor_htp/nixos/scripts/PostInstallSetup.sh`

(In the future it might be possible to combine these two steps but right now that would be difficult)


### Setting up a Raspberry Pi for rigor_htp

Setup the orchestrator
Then run:
curl -sSL http://orchestrator.local/setup.sh | bash


## License

<a href="https://www.fsf.org/bulletin/2021/fall/the-fundamentals-of-the-agplv3">AGPLv3</a>
