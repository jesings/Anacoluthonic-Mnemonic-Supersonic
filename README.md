Epic Sequel to https://github.com/anotherLostKitten/minecraft-hunger-games-sans-3d
this time featuring gamer gamer Mo(can't actually eat ham because of his religion)med Uddin in addition to returning veterans Theodore "YBHM" Peters, Jonathicc Singer

run server with cargo run, need to port forward for internet connections
run client with cargo run .... with ip adress are arg

Unbuntu Packages
```bash
sudo apt-get install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-gfx-dev libsdl2-mixer-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh
sudo apt-get install cargo
```

Arch Packages

```bash
sudo pacman -S sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf
sudo pacman -S rustup
```

Macintosh

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)" #to install homebrew if you haven't
brew install rustup sdl2 sdl2_mixer sdl2_ttf sdl2_gfx sdl2_image
echo export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib" >> ~/.bash_profile
export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"
rustup-init
```
