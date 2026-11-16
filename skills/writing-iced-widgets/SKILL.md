---
name: writing-iced-widgets
description: Write custom widgets for iced 0.14 (Widget trait, tree state, event routing, theming without iced_style). Use when creating or debugging a custom widget, an overlay on a Stack, hit-testing/cursor interaction, or when iced widget code fails to compile against trait signatures.
---

# Writing custom iced widgets (0.14)

Field-tested notes from implementing the `Cursors` overlay widget in this repo.
The complete working reference is [`src/widgets/cursor.rs`](../../src/widgets/cursor.rs);
a second, simpler example is [`src/widgets/axis.rs`](../../src/widgets/axis.rs).

## The `Widget` trait skeleton

Implement `iced::advanced::widget::Widget<Message, Theme, Renderer>` and provide:

| Method | Purpose |
| --- | --- |
| `size()` | Return `Size { width, height }` of the `Length`s for the layout algorithm. |
| `layout()` | Produce a `layout::Node`. Leaf widgets: `layout::atomic(limits, self.width, self.height)`. |
| `update()` | React to `Event`s, publish messages with `shell.publish(msg)`. |
| `draw()` | Issue quads/text/primitives via the `Renderer`. |
| `mouse_interaction()` | Hover cursor shape (`mouse::Interaction::Pointer`, `None`, …). |
| `tag()` / `state()` | Only when the widget needs persistent internal state (see below). |

Then make it usable in a view tree with:

```rust
impl<'a, Theme, Renderer> From<MyWidget<'a, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(w: MyWidget<'a, Theme>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(w)
    }
}
```

## API gotchas (compile errors the docs won't warn you about)

These exact mistakes were made and fixed while writing `cursor.rs`:

1. **`fn state(&self) -> tree::State`, not `-> Tree`.**
   Construct it with `iced::advanced::widget::tree::State::new(value)`.
2. **`tree::Tag::of::<T>()`** — `Tag` is a newtype over `TypeId`, *not* a
   `&'static str` (that was pre-0.10 API).
3. **The trait method is `mouse_interaction`, not `mouse_cursor`.**
4. **Never use `Tree::stateless()` / `tree::State::None` if you call
   `tree.state.downcast_mut::<T>()`** — downcast on `State::None` *panics*.
   If you implement `tag()`, you must also implement `state()` returning
   `State::new(YourState::default())`.
5. **Keep the widget struct's generics minimal.** Only parametrize what appears
   in fields or builder signatures. `cursor.rs` started with
   `Cursors<'a, Theme, Renderer>` and had to drop `Renderer` because it was
   unused in the struct; it is still supplied by the `Widget`/`From` impls.
6. Edition-2024 clippy wants **let-chains** instead of nested `if let`:
   `if let Some(t) = a && let Some(x) = b { … }`.

## Persistent widget state (`Tree`)

View functions rebuild the whole `Element` tree every frame, so any state the
widget itself owns (drag in progress, focus, …) must live in the widget `Tree`:

```rust
#[derive(Debug, Default, Clone, Copy)]
struct DragState { dragging: Option<CursorId> }

fn tag(&self) -> tree::Tag { tree::Tag::of::<DragState>() }
fn state(&self) -> tree::State { tree::State::new(DragState::default()) }

// in update():
let state = tree.state.downcast_mut::<DragState>();
```

The `Tree` is keyed by widget identity, so state survives view rebuilds.

## Event routing (what actually calls `update`)

Verified against `iced_runtime-0.14.0/src/user_interface.rs` (~line 320) and
`iced_widget-0.14.2/src/stack.rs`:

- The runtime calls `root.as_widget_mut().update(...)` directly. **Custom
  widgets receive events without implementing `operate`.** (Implementing
  `operate` to route events is pre-0.10 folklore.)
- `Stack::update` iterates children **in reverse draw order** — the *last*
  pushed child (drawn topmost) receives the event **first**.
- Call `shell.capture_event()` to stop the event from reaching widgets below.
  This is how an overlay intercepts clicks over a plot.
- Get the mouse position inside your widget with
  `cursor.position_in(layout.bounds())` (`mouse::Cursor` is `Unavailable`
  outside the window — treat `None` as "no event for me").
- Buttons arrive as `Event::Mouse(mouse::Event::ButtonPressed(Button::Left))`
  etc. A press+release pair is *not* synthesized into a click — handle press,
  motion and release yourself (that is what makes drag-to-move work).

## Drawing

- Simplest primitive: `renderer.fill_quad(renderer::Quad { bounds, snap: false, ..Default::default() }, color)`.
- A crisp 1-logical-px vertical line at pixel `x`: draw width `1.0` at
  `x - 0.5` with `snap: false` (otherwise the compositor snaps it to the
  device grid and the line lands on the wrong pixel / gets blurry).
- `draw()` must be side-effect free; it is called every frame. Read your data
  from the `&Window`-style borrowed state passed at construction.

## Theming without `iced_style`

iced 0.14 in this project has **no separate `iced_style` crate**. The pattern
used by every custom widget here:

1. In the widget module, declare the local styling types:

   ```rust
   #[derive(Debug, Clone, Copy, PartialEq)]
   pub struct Style { pub primary: Color, pub secondary: Color }

   pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

   pub trait Catalog {
       type Class<'a>;
       fn default<'a>() -> Self::Class<'a>;
       fn style(&self, class: &Self::Class<'_>) -> Style;
   }
   ```

2. Store `class: Theme::Class<'a>` in the widget, default it with
   `Theme::default()`, and expose a `style()` builder guarded by
   `where Theme::Class<'a>: From<StyleFn<'a, Theme>>`.
3. Implement `Catalog for MakoTheme` in a sibling theme module
   ([`src/theme/cursor.rs`](../../src/theme/cursor.rs)) — mirroring how
   `iced_widget` keeps its theme impls out of the widget file.

## App wiring pattern

- A widget that mutates app state should **only publish messages** — it never
  touches state directly (`&Window` is borrowed, and messages keep the
  unidirectional flow).
- Nested message pattern used here: `Message::Window(index, window::Message)`
  with `Place { cursor, position }` / `Dismiss(id)` variants;
  `TimescapeViewer::update` matches and applies to `self.windows[index]`.
- Widgets that need "who am I?" info: keep the index **on the state object**
  (`Window::index()`), maintained by `push`/`close` in the app, rather than
  threading indices through every view function.
- Guard index-based messages: `if let Some(window) = self.windows.get_mut(index)`
  so stale messages from a rebuilt view are a no-op.

## Testing

- `iced_test::Simulator` drives a widget headlessly:
  `Simulator::with_settings(settings, app.view())`, then
  `simulator.point_at(pos)`, `simulator.simulate(events)`,
  `simulator.into_messages()` to assert emitted messages,
  `simulator.snapshot(&theme)` + `matches_image(path)` for pixel snapshots.
  `instruction::button_at(mouse::Button::Left, at)` builds raw button events.
- **When a snapshot/rendering test fails, reproduce on the clean tree first**
  (`git stash push -u` → run test → `git stash pop`) before assuming your
  change caused it. `test_screenshot_example_sines_line_chart` in this repo
  fails on the untouched tree (environment/font dependent).

## Pre-flight checklist

- [ ] `layout::atomic` for leaves; `size()` returns the same `Length`s.
- [ ] `tag()` + `state()` both implemented together, `State::new(...)`.
- [ ] `mouse_interaction` (not `mouse_cursor`); `position_in(bounds)` guards.
- [ ] `shell.capture_event()` where the widget consumes an event.
- [ ] Local `Style`/`StyleFn`/`Catalog`; `Catalog` impl for the app theme.
- [ ] `From<Widget> for Element` so `.push(MyWidget::new(..))` just works.
- [ ] `cargo clippy --all-targets` clean (watch let-chain lints on edition 2024).
