# NoWayOS starter

An application on [NoWayOS](https://www.nowayos.com), ready to run and to change. Two screens written in React, sixty lines of Rust that join them, and everything needed to see it in a browser window as you work and then boot it on seL4.

Clone it, run three commands, and start changing it.

```sh
git clone https://github.com/NoWayOS/starter.git my-app
cd my-app
make setup     # once: fetches NoWayOS and Screen Compiler, builds the dev container
make dev       # your application in a browser window: http://localhost:8017/
```

Now open `app/starter-app/tsx/welcome.tsx`, change some text and save. The window shows it about a second later, on the screen you were on.

NoWayOS is in development and so is this starter: it is for learning and building, not yet for production.

## What you need

- **git** and **GNU Make** (on a Mac: `xcode-select --install`).
- **Podman** ([podman.io](https://podman.io)). Docker works too: add `ENGINE=docker` to each command.
- Nothing else. No Rust, no seL4 tools, no QEMU: they are all inside the dev container, pinned to exact versions.

On macOS, Podman runs containers in a virtual machine. Create and start it once:

```sh
podman machine init --cpus 4 --memory 8192
podman machine start
```

`make setup` checks all of this and says what is missing. It builds the dev container the first time (a few minutes and about 5 GB), fetches NoWayOS and Screen Compiler into this folder at the commits `nowayos.rev` and `screen-compiler.rev` name, and changes nothing else on your machine. It is safe to run again.

NoWayOS is developed and tested on Apple silicon Macs. Linux on x86-64 has not yet been tested end to end.

## Your first five minutes

**1. Run it.** `make dev`, then open <http://localhost:8017/>. Type a name, press Continue. This is the same Rust that runs on the device, built for your machine. Tab, Enter, the mouse and the keyboard all work.

**2. Change a screen.** Edit `app/starter-app/tsx/welcome.tsx`:

```tsx
<Text role="title">Welcome</Text>
<Text>What should we call you?</Text>
```

Save. The screen is checked, converted to Rust, rebuilt and shown again. If you write something Screen Compiler does not convert, it says so above the screen, with the line and what to use instead, and the last good version stays up.

**3. Change what a button does.** Edit `app/starter-app/src/lib.rs`. The window follows that too.

**4. Boot it on seL4.**

```sh
make qemu VNC=1
```

This builds a real seL4 system and boots it in QEMU. It prints a password; open `vnc://localhost:5900` (on a Mac: Finder, Go, Connect to Server) and use the application with your keyboard and mouse. Quit with Ctrl-A, then x.

## How it fits together

A screen is a React component built from the kit's parts: `Screen`, `Column`, `Row`, `Text`, `Button`, `TextInput` and about twenty more.

```tsx
export function WelcomeScreen(props: WelcomeProps) {
  return (
    <Screen>
      <Column gap={GAP} padding={EDGE}>
        <Text role="title">Welcome</Text>
        <TextInput id="name" placeholder="Your name" maxLength={NAME_LENGTH} />
        {props.nameError && <Text>{props.nameError}</Text>}
        <Button id="next" onPress={props.onContinue}>Continue</Button>
      </Column>
    </Screen>
  );
}
```

**Screen Compiler** turns it into Rust at build time. No JavaScript, browser or interpreter goes on the device. It converts a declared subset of React: layout, text, form controls, conditions, lists. State, hooks and effects stay out, because the logic lives in Rust. Anything outside the subset is refused with its line and a suggestion, never approximated.

Four things in the TSX become four things your Rust uses:

| In the TSX | In Rust | What you do with it |
| --- | --- | --- |
| a callback prop, `onContinue` | an **action**, `welcome::actions::CONTINUE` | match on it when a button is pressed |
| a control with an `id` that holds a value, `id="name"` | a **field**, `welcome::fields::NAME` | read it: `tree.value(...)` |
| a string prop shown as text, `nameError` | a **slot**, `welcome::slots::NAME_ERROR` | fill it: `tree.set_slot(...)` |
| content inside `{props.x && ...}` | a **group**, `welcome::groups::NAME_ERROR` | show or hide it: `tree.set_visible(...)` |

And the logic, in `app/starter-app/src/lib.rs`, is one function:

```rust
pub fn handle(&mut self, event: InputEvent) -> Result<Option<&'static str>, UiError> {
    let Some(UiEvent::Action(action)) = self.tree.handle(event) else {
        return Ok(None);
    };
    if self.showing == "welcome" && action == welcome::actions::CONTINUE {
        self.greet()?;
        Ok(Some("Continue"))
    } else if self.showing == "hello" && action == hello::actions::START_AGAIN {
        self.show("welcome")?;
        Ok(Some("Start again"))
    } else {
        Ok(None)
    }
}
```

That same `App` runs in three places: in the browser window while you work (`make dev`), in the tests (`make test`), and in the UI protection domain on the device (`app/pds/ui`).

## Add a screen

```sh
make new-screen NAME=thanks
```

This writes `app/starter-app/tsx/thanks.tsx` from a template, converts it, and prints the three lines that wire it into `lib.rs`. You can look at it straight away, before wiring anything:

```sh
make preview FILE=starter-app/tsx/thanks.tsx     # a picture, in app/build/preview/
make dev SCREEN=thanks                           # the application, opened on that screen
```

`make preview` draws one screen as a PNG with NoWayOS's own renderer; add `WATCH=1` and it redraws on every save. Keep the PNG open beside the code.

Your editor helps too. Open this folder in VS Code and the kit's components complete as you type, with wrong props underlined, using VS Code's built-in TypeScript. Nothing to install.

## What runs on the device

`make qemu` and `make qemu-test` build the system in `app/systems/`:

- `starter.system` describes four protection domains: NoWayOS's input driver, input broker and display, and your UI. Each can reach only what the file gives it.
- `starter.grants` lists every one of those grants again, one fact a line. The build fails if the two disagree, or if the kernel objects that come out differ. A protection domain can never quietly gain a capability: widening a grant is a visible change to both files.
- `starter.expect` and `starter.qmp` are the test: what `make qemu-test` types, and what the system must print. It never prints anything typed.

Your UI protection domain holds its message rings and the framebuffer. No device, no interrupt, no network.

The starter runs in QEMU. Putting a system on a Raspberry Pi 5 is covered in [NoWayOS's guide to the Pi 5](https://www.nowayos.com/docs/raspberry-pi-5).

## Make it yours

```sh
make rename NAME=my-product
```

Renames the crates, the programs, the system and what it prints, in every file. Do it once, early. Then make the repository your own: change this README, and set the licence you want.

## Every command

| Command | What it does |
| --- | --- |
| `make setup` | fetch NoWayOS and Screen Compiler, build the dev container; safe to repeat |
| `make dev` | the application in a browser window, following your edits (`SCREEN=NAME` opens it on a screen) |
| `make preview FILE=starter-app/tsx/welcome.tsx` | one screen as a picture (`WATCH=1` redraws on save) |
| `make new-screen NAME=thanks` | a new screen from a template |
| `make rename NAME=my-product` | make the starter yours |
| `make test` | the application's tests, on your machine: keys in, screens out |
| `make screens` | check every screen converts as checked in (`BLESS=1` rewrites `src/generated`) |
| `make lint` / `make fmt` | rustfmt and clippy / format |
| `make qemu VNC=1` | boot it on seL4 in QEMU, the screen over VNC |
| `make qemu-test` | boot it headless and check what it prints and shows |
| `make check` | lint, test, screens and qemu-test: run it before you share a change |
| `make ci` | every check NoWayOS holds its own repositories to, the security checks among them |
| `make shell` | a shell in the dev container |

Run them from this folder or from `app/`.

## What is in the folder

```text
my-app/
├── Makefile, setup            the commands above
├── nowayos.rev                the NoWayOS commit you build on
├── screen-compiler.rev        the Screen Compiler commit that converts your screens
├── nowayos/, screen-compiler/ fetched by make setup; not part of your repository
├── secrets-allow.toml         reviewed false positives of the secrets scan (none)
└── app/
    ├── deny.toml              the licences and sources your dependencies may have
    ├── starter-app/           your application
    │   ├── tsx/               its screens, in React
    │   ├── src/lib.rs         its logic
    │   ├── src/generated/     the screens as Rust: written by Screen Compiler, never by hand
    │   └── tests/app.rs       its tests
    ├── starter-dev/           the program make dev runs (your machine only)
    ├── pds/                   the protection domains: ui is yours, the rest are NoWayOS's
    ├── systems/               the seL4 system: description, grants, test
    └── tools/                 new-screen and rename
```

`src/generated/` is checked in on purpose: a reviewer sees exactly what Rust a change to a screen produces.

## Moving to a newer NoWayOS

Change the commit in `nowayos.rev` (or `screen-compiler.rev`), run `make setup`, then `make check`. If Screen Compiler changed, `make screens BLESS=1` rewrites the generated Rust; read the diff before you commit it.

## The rules the code keeps

The workspace's lints are NoWayOS's own: no `unsafe`, no `unwrap` or `panic!`, no unchecked indexing or arithmetic, in anything that goes on a device. They are why `make lint` is strict. Code on the device has no heap and never panics on what it is given, and the lints keep yours that way from the first line.

Treat everything typed into a screen as sensitive: never log it, and wipe it when it is done with. Showing the next screen clears the tree and everything typed into it.

## The security checks

Your application starts with the security checks NoWayOS runs on itself. `make ci` runs them all, with the tests, in a minute or so once the first build is done; each also runs on its own.

| Check | What it holds |
| --- | --- |
| `make pins` | NoWayOS and Screen Compiler are exactly the commits `nowayos.rev` and `screen-compiler.rev` name, unchanged |
| `make secrets` | no key or credential in the tree, or in any commit |
| `make deny`, `make pds-deny` | every dependency, and everything it brings with it: a permissive licence, from crates.io, with no known advisory. No browser engine, window system or C GUI runtime can arrive as a dependency of something else |
| `make unsafe-audit` | no `unsafe` code anywhere in the application |
| `make nostd`, `make std-crates-check` | what the device runs builds with no standard library, and holds only the standard-library crates NoWayOS has reviewed |
| `make qemu-test-all` | every grant is in the grants file; the system boots, behaves as its test says, and prints nothing typed |
| `make sbom` | a CycloneDX bill of materials for what the device is built from, in `app/build/sbom/` |

The rules are in files you own: `app/deny.toml` and `app/pds/deny.toml` for dependencies, `secrets-allow.toml` for a reviewed false positive, `app/systems/starter.grants` for what each part may reach. Loosen one and the change shows in a diff.

A change to the starter itself passes the gates in [Defence Agent](https://github.com/NoWayOS/defence-agent), as every NoWayOS repository's does.

## Where next

- [Screens from React](https://www.nowayos.com/docs/screens-from-react): every component and style property that converts.
- [Porting from React](https://www.nowayos.com/docs/porting-from-react): what to change in an existing interface, and each diagnostic explained.
- [How a system fits together](https://www.nowayos.com/docs/how-a-system-fits-together): protection domains, rings and grants.
- [Getting help](https://www.nowayos.com/docs/getting-help).

## Licence

MIT OR Apache-2.0, at your option (`LICENSE-MIT`, `LICENSE-APACHE`), as NoWayOS is. Make it whatever you need for your own product.
