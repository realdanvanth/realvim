# realvim
minimal text editor in c inspired by neovim

![image](https://github.com/user-attachments/assets/015dee65-f9ac-424a-b597-a8d10dcec538)

![image](https://github.com/user-attachments/assets/e04360ad-84ec-4e8e-a3c4-50dedf93a23f)



**Installation and Execution**

```install
git clone https://github.com/realdanvanth/realvim.git
```
```execute
cd ~/realvim/src && gcc -Wall "text.c" -o text -lncurses && ./text text.c
```
```
cd ~/realvim/src && gcc -Wall "text.c" -o text -lncurses && ./text sample.txt
```

# Text Editor Navigation Guide

| **Key**   | **Mode**      | **Action**                     |
|-----------|---------------|---------------------------------|
| `wasd`    | **Navigation**| Move cursor (up, left, down, right). |
| `q`       | **Command**   | Quit the editor.               |
| `i`       | **Insert**    | Switch to insert mode.         |
| `Escape`  | **Visual**    | Return to visual/command mode. |
| `:`       | **Command**   | Enter command mode             |

### Usage Notes:
- **Navigation Mode:** Use `wasd` to move the cursor around within the document.
- **Insert Mode:** Press `i` to start typing or editing the document content.
- **Quiting the Editor:** Press `q` to exit the editor. Ensure all changes are saved beforehand.
- **Returning to Visual Mode:** Press `Escape` to leave insert mode and switch back to visual mode.

