# Effitask

[![Build Status](https://gitlab.com/sanpi/effitask/badges/master/pipeline.svg)](https://gitlab.com/sanpi/effitask/commits/master)

Effitask is a graphical task manager, based on the [todo.txt
format](https://github.com/todotxt/todo.txt).

[<img title="Inbox view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/inbox.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/inbox.png)
[<img title="Add a task" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/add.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/add.png)
[<img title="Edit a task" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/edit.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/edit.png)
[<img title="Projects view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/projects.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/projects.png)
[<img title="Note" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/note.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/note.png)
[<img title="Contexts view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/contexts.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/contexts.png)
[<img title="Agenda view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/agenda.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/agenda.png)
[<img title="Flag view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/flag.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/flag.png)
[<img title="Done view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/done.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/done.png)
[<img title="Search view" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/flag.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/search.png)
[<img title="Light theme" src="https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/theme-light.png" width="200px" />](https://raw.githubusercontent.com/sanpii/effitask/master/screenshots/theme-light.png)

Supported toto.txt addons:

* [due](https://github.com/rebeccamorgan/due)
* [flag](https://github.com/sanpii/my-dotfiles/blob/master/todo.actions.d/flag)
* [future](https://github.com/FND/todo.txt-cli/blob/extensions/futureTasks)
* [note](https://github.com/mgarrido/todo.txt-cli/tree/note/todo.actions.d)
* [schedule](https://github.com/FND/todo.txt-cli/blob/extensions/schedule)

## Hidden features

I tried to develop a clear interface without surprises, but you can easily
miss some feature:

* Double click on a feature, *everywhere*, open the edit panel;
* You can create sub-projects (or sub-contexts) by adding a backslash. For
  example, the project `+work\admin\automation` create this arborescence:

```txt
work
└── admin
    └── automation
```

* Double click on a project/context select all there sub-projects/contexts,
  therefore show their tasks;
* The project/context tooltip (keep your mouse pointer on the name) display done
  tasks and total tasks number, including sub-projects/contexts (also showed as
  progress bar);
* Press enter in the "subject" input in the edit panel validate modification.

## Install

If you use Arch Linux, effitask is available in
[AUR](https://aur.archlinux.org/packages/effitask/).

### Manually

Compiling effitask requires rust. I recommend to use
[rustup](https://rustup.rs/).

You also need `gtk+ 3.0` on your system. Depending of your distribution, run:

```bash
apt install libgtk-3-dev # debian
pacman -S gtk3 # archlinux
```

```bash
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

```bash
mkdir ~/.todo.actions.d
ln -s /usr/bin/effitask ~/.todo.actions.d/et
todo.sh et
```

You can use it as standalone program by defining some environment variables:

```bash
export TODO_DIR="$HOME/.local/opt/share/todo"
export TODO_FILE="$TODO_DIR/todo.txt"
export DONE_FILE="$TODO_DIR/done.txt"

/usr/bin/effitask
```

## Configuration

As you can see above, effitask reuse todo.txt environment variables for
configuration.

* `TODO_DIR`: your todo.txt directory
* `TODO_FILE`: your todo.txt location
* `DONE_FILE`: your done.txt location
* `TODO_NOTES_DIR`: directory for notes, `$TODO_DIR/notes` by default
* `TODO_NOTE_EXT`: extension for note files, `.txt` by default
* `TODO_NOTE_TAG`: tag name to add to task description, `note` by default
