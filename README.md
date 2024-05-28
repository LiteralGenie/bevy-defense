# setup (devcontainer)

Clone with HTTP url (SSH url causes git permission errors when used with vscode + devcontainers)
```
git clone https://github.com/LiteralGenie/bevy-defense.git
```

Build wasm module that contains most of the game logic / rendering code
```
bash build.sh
```

GUI and webserver should already be running (via devcontainer's post-start command) so app will be accessible at http://localhost:5173