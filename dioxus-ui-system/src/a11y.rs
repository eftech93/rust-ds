//! Accessibility helpers shared by components: stable element ids for ARIA
//! relationships, focus management and keyboard navigation.

use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// Returns an id that is unique for the lifetime of the program and stable for
/// the lifetime of the calling component, e.g. `"dialog-3"`.
///
/// Use it to connect elements through `aria-labelledby`, `aria-describedby` and
/// `aria-controls`. If the caller supplied its own `id` prop, prefer that.
pub fn use_unique_id(prefix: &str) -> String {
    let prefix = prefix.to_string();
    use_hook(move || unique_id(&prefix))
}

/// Non-hook version of [`use_unique_id`].
pub fn unique_id(prefix: &str) -> String {
    format!("{prefix}-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

/// Move keyboard focus to the element with `id` (no-op if it does not exist).
pub fn focus_element(id: &str) {
    let id = js_string(id);
    let _ = document::eval(&format!(
        "requestAnimationFrame(() => document.getElementById({id})?.focus());"
    ));
}

/// Trap Tab / Shift+Tab focus inside the element with `container_id`, focus its
/// first focusable element, and remember the previously focused element.
///
/// Call when an overlay opens; call [`release_focus_trap`] when it closes to
/// restore focus. Installing twice on the same container is a no-op.
pub fn install_focus_trap(container_id: &str) {
    let id = js_string(container_id);
    let _ = document::eval(&format!(
        r#"requestAnimationFrame(() => {{
  const el = document.getElementById({id});
  if (!el || el.__dxFocusTrap) return;
  const sel = 'a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex="-1"])';
  const items = () => Array.from(el.querySelectorAll(sel)).filter(n => n.offsetParent !== null || n === document.activeElement);
  const onKey = (e) => {{
    if (e.key !== 'Tab') return;
    const list = items();
    if (list.length === 0) {{ e.preventDefault(); el.focus(); return; }}
    const first = list[0], last = list[list.length - 1];
    if (e.shiftKey && (document.activeElement === first || document.activeElement === el)) {{ e.preventDefault(); last.focus(); }}
    else if (!e.shiftKey && document.activeElement === last) {{ e.preventDefault(); first.focus(); }}
  }};
  el.__dxFocusTrap = {{ onKey }};
  (window.__dxFocusRestore ||= {{}})[{id}] = document.activeElement;
  el.addEventListener('keydown', onKey);
  if (!el.hasAttribute('tabindex')) el.setAttribute('tabindex', '-1');
  (items()[0] || el).focus();
}});"#
    ));
}

/// Undo [`install_focus_trap`] and restore focus to the element that had it
/// before the trap was installed. Safe to call when no trap is installed, and
/// still restores focus if the container was already removed from the DOM.
pub fn release_focus_trap(container_id: &str) {
    let id = js_string(container_id);
    let _ = document::eval(&format!(
        r#"(() => {{
  const el = document.getElementById({id});
  const t = el && el.__dxFocusTrap;
  if (t) {{ el.removeEventListener('keydown', t.onKey); delete el.__dxFocusTrap; }}
  const store = window.__dxFocusRestore || {{}};
  const restore = store[{id}];
  delete store[{id}];
  if (restore && document.contains(restore)) restore.focus();
}})();"#
    ));
}

/// Layout direction used by [`next_index`] to pick which arrow keys move focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    /// Left/Right arrows move (tabs, menubars, carousels).
    Horizontal,
    /// Up/Down arrows move (menus, vertical tabs, trees, accordions).
    Vertical,
}

/// Compute the new active index for roving-focus widgets.
///
/// Handles the orientation's arrow keys (wrapping), `Home` and `End`. Returns
/// `None` for any other key or when `len == 0`, so callers can ignore the event.
pub fn next_index(
    key: &Key,
    current: usize,
    len: usize,
    orientation: Orientation,
) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let current = current.min(len - 1);
    let (prev, next) = match orientation {
        Orientation::Horizontal => (Key::ArrowLeft, Key::ArrowRight),
        Orientation::Vertical => (Key::ArrowUp, Key::ArrowDown),
    };
    match key {
        k if *k == next => Some((current + 1) % len),
        k if *k == prev => Some((current + len - 1) % len),
        Key::Home => Some(0),
        Key::End => Some(len - 1),
        _ => None,
    }
}

/// Like [`next_index`], but skips indexes whose `disabled` flag is set by
/// continuing in the direction of travel (`Home` searches forward from the
/// first item, `End` backward from the last). Returns `None` for unhandled keys
/// or when every item is disabled.
pub fn next_enabled_index(
    key: &Key,
    current: usize,
    disabled: &[bool],
    orientation: Orientation,
) -> Option<usize> {
    let len = disabled.len();
    let mut idx = next_index(key, current, len, orientation)?;
    let forward = match (key, orientation) {
        (Key::Home, _) => true,
        (Key::End, _) => false,
        (Key::ArrowRight, Orientation::Horizontal) | (Key::ArrowDown, Orientation::Vertical) => {
            true
        }
        _ => false,
    };
    for _ in 0..len {
        if !disabled[idx] {
            return Some(idx);
        }
        idx = if forward {
            (idx + 1) % len
        } else {
            (idx + len - 1) % len
        };
    }
    None
}

/// JSON-encode a string for safe interpolation into JavaScript.
fn js_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '<' => out.push_str("\\u003c"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids_differ() {
        assert_ne!(unique_id("x"), unique_id("x"));
        assert!(unique_id("dialog").starts_with("dialog-"));
    }

    #[test]
    fn next_index_wraps_and_jumps() {
        let h = Orientation::Horizontal;
        assert_eq!(next_index(&Key::ArrowRight, 2, 3, h), Some(0));
        assert_eq!(next_index(&Key::ArrowLeft, 0, 3, h), Some(2));
        assert_eq!(next_index(&Key::Home, 2, 3, h), Some(0));
        assert_eq!(next_index(&Key::End, 0, 3, h), Some(2));
        assert_eq!(next_index(&Key::ArrowDown, 0, 3, h), None);
        assert_eq!(
            next_index(&Key::ArrowDown, 0, 3, Orientation::Vertical),
            Some(1)
        );
        assert_eq!(next_index(&Key::ArrowRight, 0, 0, h), None);
    }

    #[test]
    fn next_enabled_index_skips_disabled() {
        let h = Orientation::Horizontal;
        let disabled = [false, true, false, true];
        assert_eq!(
            next_enabled_index(&Key::ArrowRight, 0, &disabled, h),
            Some(2)
        );
        assert_eq!(
            next_enabled_index(&Key::ArrowRight, 2, &disabled, h),
            Some(0)
        );
        assert_eq!(
            next_enabled_index(&Key::ArrowLeft, 0, &disabled, h),
            Some(2)
        );
        assert_eq!(next_enabled_index(&Key::End, 0, &disabled, h), Some(2));
        assert_eq!(
            next_enabled_index(&Key::Home, 2, &[true, false], h),
            Some(1)
        );
        assert_eq!(next_enabled_index(&Key::ArrowDown, 0, &disabled, h), None);
        assert_eq!(
            next_enabled_index(&Key::ArrowDown, 0, &disabled, Orientation::Vertical),
            Some(2)
        );
        assert_eq!(
            next_enabled_index(&Key::ArrowRight, 0, &[true, true], h),
            None
        );
        assert_eq!(next_enabled_index(&Key::Home, 0, &[], h), None);
    }

    #[test]
    fn js_string_escapes() {
        assert_eq!(js_string(r#"a"b\c</script>"#), r#""a\"b\\c\u003c/script>""#);
    }
}
