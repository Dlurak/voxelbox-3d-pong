# Voxelbox 3D Pong

The Voxelbox is a 20³ led cube. Each vertex has a length of 2 Meters!
It can be found on [codeberg](https://codeberg.org/VoxelBox/voxelbox).

---

## Local Development

- Use a real voxelbox or use the [simulator](https://codeberg.org/VoxelBox/voxelbox).
    - In `main.rs` you need to adjust the ip and port to match your dev setup
- Connect a gaempad, I use a Dualshock 4 but all should work fine

### NixOS

There is a flake available so it should be enough to simply run this command:

```sh
nix develop
```

## Progress/Todo

- [x] Controller Input:
    - [x] No noticeable delay
    - [x] Natural speed
    - [x] Higher speed for bigger controller inputs
- [ ] Ball:
    - [x] Renders
    - [x] Moves
    - [x] Detects collisions
    - [ ] Changes direction

## Thanks <3

- Thanks derMicha for the voxelbox workshop on the 38c3
- Thanks for providing the voxelbox on the 38c3
- I had a small conversation with a random person - they thought of this idea and we discussed it

