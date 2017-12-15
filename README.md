# Effitask

Effitask is a graphical task manager, based on the [todo.txt
format](https://github.com/todotxt/todo.txt).

[<img title="Inbox view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/inbox.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/inbox.png)
[<img title="Add a task" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/add.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/add.png)
[<img title="Edit a task" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/edit.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/edit.png)
[<img title="Projects view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/projects.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/projects.png)
[<img title="Contexts view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/contexts.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/contexts.png)
[<img title="Agenda view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/agenda.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/agenda.png)
[<img title="Done view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/done.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/done.png)
[<img title="Light theme" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/theme-light.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/theme-light.png)

Supported toto.txt addons:

* [due](https://github.com/rebeccamorgan/due)
* [note](https://github.com/mgarrido/todo.txt-cli/tree/note/todo.actions.d)

## Install

If you use archlinux, effitask is available in
[AUR](https://aur.archlinux.org/packages/effitask/).

### Manually

```
git clone https://github.com/sanpii/effitask
cd effitask
make
sudo make install
```

## Launch

This program is designed to be used as
[todo.sh](https://github.com/todotxt/todo.txt-cli) add-on. Install it as others
add-ons:
<https://github.com/todotxt/todo.txt-cli/wiki/Creating-and-Installing-Add-ons>.

```
ln -s "$(pwd)/target/release/effitask" ~/.todo.actions.d/et
todo.sh et
```

You can use it as standalone program by defining some environment variables:

```
export TODO_DIR="$HOME/.local/opt/share/todo"
export TODO_FILE="$TODO_DIR/todo.txt"
export DONE_FILE="$TODO_DIR/done.txt"

./target/release/effitask
```
