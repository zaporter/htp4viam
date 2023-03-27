<h1 >
<h1 align="center">
  <br>
  <img src="https://github.com/zaporter/htf4viam/blob/main/etc/logo.png?raw=true" alt="HTF Logo" width="200" style="border-radius:50%; ">
  <br>
  <p>Hardware Testing Platform <b style="color:0x44ffff">4</b> Viam</p>
  <br>
</h1>


<h4 align="center">A powerful and extensible tool to run tests with <a href="https://github.com/viamrobotics/">Viam</a> Robots.</h4>

<h4 align="center">⚠️ This project is not associated with Viam and it does not promise support to Viam or their customers ⚠️</h4>
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
`sudo /iso/htp4viam/nixos/scripts/install/InstallISO.sh`
Reboot and remove the installation medium
Run `./htp4viam/nixos/scripts/PostInstallSetup.sh`

(In the future it might be possible to combine these two steps but right now that would be difficult)


### Setting up a Raspberry Pi for htf4viam

Setup the orchestrator
Then run:
curl -sSL http://orchestrator.local/setup.sh | bash


## License

<a href="https://www.fsf.org/bulletin/2021/fall/the-fundamentals-of-the-agplv3">AGPLv3</a>
