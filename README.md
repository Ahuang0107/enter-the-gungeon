# readme

a re-implement of enter the gungeon with bevy engine

1. use matrix represent each level tiles, for example:
    ```rust
    fn main() {
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 0, 0, 0, 0, 0, 1, 0, 0,
            1, 0, 0, 0, 0, 0, 1, 0, 1,
            1, 0, 0, 0, 0, 1, 1, 0, 1,
            1, 0, 1, 0, 0, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0, 0, 0, 1,
            1, 0, 1, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
    }
    ```
   0 represent the floor and 1 represent the wall, of course the floor usually have multi-style.
2. each level should have multi gate that used to enter and exit. and combination with other level also need this
   information.