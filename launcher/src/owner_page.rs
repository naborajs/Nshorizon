//! The owner gate in the window (`horizon_launcher::account::launch`): the screen that asks for a
//! Microsoft account that owns Minecraft: Java Edition. It covers the launcher once for players
//! without one (those who played offline before the gate), and again whenever Play is refused.
//! The sign-in runs on it, code and all.
use super::*;

impl Launcher {
    /// Whether the screen covers the window: no owner, and either Play was refused or the player
    /// hasn't read it yet. The data question comes first.
    pub(crate) fn asking_owner(&self) -> bool {
        (!horizon_launcher::developer_offline::selected() || sign_in_preview())
            && (self.owner == Some(false) || sign_in_preview())
            && (self.owner_screen_open || !self.settings.owner_notice_seen || sign_in_preview())
            && !self.asking_consent()
            && self.opening.is_none()
    }

    /// Opens the screen and starts the sign-in on it; a sign-in already under way (its code on
    /// screen, or Microsoft still answering) is shown as it is, not started over.
    pub(crate) fn sign_in_to_play(&mut self) {
        self.owner_screen_open = true;
    }

    /// Closes it; it comes back with the next refused Play.
    fn close_owner_screen(&mut self) {
        self.cancel_sign_in();
        self.owner_screen_open = false;
        if !self.settings.owner_notice_seen {
            self.settings.owner_notice_seen = true;
            self.save_settings();
        }
    }

    pub(crate) fn owner_screen(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let t = self.theme;
        let n = t.night;
        let configured = account::client_id().is_some();
        // The screenshot preview shows a code without asking Microsoft for one.
        let state = match &self.sign_in_state {
            _ if sign_in_preview() => Shown::Code("https://www.microsoft.com/link", "K7Q-4MXR"),
            Some(SignInState::Starting) if self.nsgo.active => {
                Shown::Starting("Opening NSGO sign-in in browser…")
            }
            Some(SignInState::Browser(url)) => Shown::Browser(url),
            Some(SignInState::Starting) => Shown::Starting("Asking Microsoft…"),
            Some(SignInState::Working(phase)) => Shown::Starting(phase.label()),
            Some(SignInState::Code(code)) => Shown::Code(&code.verification_uri, &code.user_code),
            Some(SignInState::Done(Err(e))) => Shown::Error(e),
            Some(SignInState::Done(Ok(_))) | None => Shown::Idle,
        };
        if crate::glass_ui::on() {
            return self.glass_owner_screen(state, configured, cx);
        }
        let line = |s: &str, color: u32| {
            shell::text(theme::night::UI, shell::c(color))
                .relative()
                .child(tr(s).to_string())
        };
        let mut body = div()
            .relative()
            .flex()
            .flex_col()
            .gap(px(10.))
            .child(
                shell::text(theme::night::CAPTION, shell::c(0xfb923c))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(tr("NSGO account").to_string()),
            )
            .child(
                shell::text(theme::night::TITLE, shell::c(n.ink))
                    .mb(px(4.))
                    .child(tr("Sign in to play").to_string()),
            )
            .child(line(
                "Sign in with your NSGO account to play online with custom skin, cape, and server access.",
                n.ink_soft,
            ))
            .child(line(
                "No Minecraft install needed: Horizon downloads it from Mojang for your account.",
                n.ink_soft,
            ));
        let raised = || {
            div()
                .relative()
                .mt(px(10.))
                .p(px(18.))
                .rounded(px(theme::night::RADIUS_CONTROL))
                .bg(plate::alpha(n.card_hi, n.card_hi_alpha))
                .border_1()
                .border_color(plate::alpha(n.stroke, n.stroke_alpha))
        };
        body = match state {
            Shown::Code(uri, user_code) => {
                let copy = user_code.to_string();
                let (private_uri, link_uri, other_uri) =
                    (uri.to_string(), uri.to_string(), uri.to_string());
                body.child(
                    raised()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(
                                    shell::text(theme::night::CAPTION, shell::c(n.ink_soft)).child(
                                        trf(
                                            "Open {link} in your browser and enter this code:",
                                            // The page as the player would type it.
                                            &[("link", short_link(uri))],
                                        ),
                                    ),
                                )
                                .child(
                                    div()
                                        .font_family(theme::MONO)
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(30.))
                                        .line_height(px(38.))
                                        .text_color(shell::c(n.ink))
                                        .child(user_code.to_string()),
                                ),
                        )
                        .child(
                            shell::action("owner-copy", shell::Action::Raised, &t)
                                .child(shell::label(tr("Copy code").to_string()))
                                .on_click(move |_, _, cx| {
                                    copy_to_clipboard(
                                        cx,
                                        gpui::ClipboardItem::new_string(copy.clone()),
                                    )
                                }),
                        ),
                )
                .child(
                    div()
                        .mt(px(8.))
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(4.))
                        .when(!microsoft::has_private_window(), |col| {
                            col.child(shell::text(theme::night::CAPTION, shell::c(n.muted)).child(
                                tr("Open this link in a private window to pick another account."),
                            ))
                        })
                        .child(
                            div()
                                .flex()
                                .gap(px(8.))
                                .child(if microsoft::has_private_window() {
                                    shell::action("owner-private", shell::Action::Quiet, &t)
                                        .child(shell::label(tr("Open in private window")))
                                        .on_click(move |_, _, cx| {
                                            microsoft::open_private(&private_uri, cx)
                                        })
                                } else {
                                    shell::action("owner-copy-link", shell::Action::Quiet, &t)
                                        .child(shell::label(tr("Copy link")))
                                        .on_click(move |_, _, cx| {
                                            copy_to_clipboard(
                                                cx,
                                                gpui::ClipboardItem::new_string(link_uri.clone()),
                                            )
                                        })
                                })
                                .child(
                                    shell::action("owner-other", shell::Action::Quiet, &t)
                                        .child(shell::label(tr(
                                            "Use a different Microsoft account",
                                        )))
                                        .on_click(move |_, _, cx| {
                                            microsoft::use_different_account(&other_uri, cx)
                                        }),
                                ),
                        ),
                )
            }
            Shown::Starting(what) => body.child(raised().child(
                shell::text(theme::night::UI, shell::c(n.ink_soft)).child(tr(what).to_string()),
            )),
            Shown::Browser(uri) => {
                let link_uri = uri.to_string();
                body.child(
                    raised()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            shell::text(theme::night::UI, shell::c(n.ink_soft))
                                .child(tr("Complete sign-in in your browser…").to_string()),
                        )
                        .child(
                            shell::action("owner-browser-open", shell::Action::Main, &t)
                                .child(shell::label(tr("Open link").to_string()))
                                .on_click(move |_, _, cx| cx.open_url(&link_uri)),
                        ),
                )
            }
            Shown::Error(e) => body.child(
                raised()
                    .child(shell::text(theme::night::UI, shell::c(n.danger)).child(e.to_string())),
            ),
            Shown::Idle => body,
        };
        let main = match state {
            Shown::Code(uri, _) => {
                let uri = uri.to_string();
                Some(
                    shell::action("owner-open", shell::Action::Main, &t)
                        .child(shell::label(tr("Open link").to_string()))
                        .on_click(move |_, _, cx| cx.open_url(&uri)),
                )
            }
            Shown::Browser(_) => Some(
                shell::action("owner-cancel", shell::Action::Quiet, &t)
                    .child(shell::label(tr("Cancel").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.cancel_sign_in();
                        this.nsgo.active = false;
                        cx.notify();
                    })),
            ),
            Shown::Starting(_) => None,
            _ => Some(
                nsgo_brand::sign_in_button("owner-nsgo-sign-in").on_click(cx.listener(
                    |this, _: &ClickEvent, _, cx| {
                        this.submit_nsgo();
                        cx.notify();
                    },
                )),
            ),
        };
        let waiting = matches!(state, Shown::Code(..)).then(|| {
            shell::text(theme::night::CAPTION, shell::c(n.muted))
                .relative()
                .child(tr("Waiting for Microsoft… the window goes on by itself.").to_string())
        });
        let answers = div()
            .relative()
            .flex()
            .items_center()
            .gap(px(10.))
            .mt(px(14.))
            .children(main)
            .child(if configured {
                microsoft::sign_in_button("owner-sign-in", &t).on_click(cx.listener(
                    |this, _: &ClickEvent, _, cx| {
                        this.start_sign_in();
                        cx.notify();
                    },
                ))
            } else {
                shell::action("owner-sign-in", shell::Action::Quiet, &t)
                    .child(shell::label(tr("Microsoft").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.sign_in_state = Some(SignInState::Done(Err(tr(
                            "Microsoft sign-in requires HORIZON_MSA_CLIENT_ID. Please sign in with NSGO.",
                        )
                        .to_string())));
                        cx.notify();
                    }))
            })
            .child(
                shell::action("owner-later", shell::Action::Quiet, &t)
                    .child(shell::label(tr("Not now").to_string()))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.close_owner_screen();
                        cx.notify();
                    })),
            );
        // Opaque: it sits over the page's content, not over the bare sky.
        let card = plate::cut(
            theme::night::PIXEL_CORNER,
            2,
            Some(plate::alpha(n.card, 0.98)),
            Some((plate::alpha(n.stroke, 2. * n.stroke_alpha), 1.)),
        )
        .flex()
        .flex_col()
        .gap(px(8.))
        // A set width: text measured at it wraps where it is drawn.
        .w(px(520.))
        .max_w_full()
        .p(px(32.))
        .child(body)
        .child(answers)
        .children(waiting);
        // The sky stays behind, dimmed: the sheet is over the launcher, not instead of it.
        div()
            .id("owner-gate")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .px(px(32.))
            .py(px(24.))
            .bg(plate::alpha(n.page, 0.8))
            .on_scroll(|_, _, cx| cx.stop_propagation())
            .occlude()
            .child(card)
    }

    /// The screen in the glass look: a dialog plate over the dimmed, blurred launcher (the
    /// mockup's modals), its Sign in the Account card's white Microsoft button, its code in the
    /// card's inner panel.
    fn glass_owner_screen(
        &self,
        state: Shown,
        _configured: bool,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        use crate::glass_ui::{self as ui, white};
        use horizon_theme::glass::{fill, ink, radius};
        let t = self.theme;
        let panel = || {
            div()
                .mt(px(20.))
                .p(px(20.))
                .rounded(px(radius::POPOVER))
                .bg(white(0.045))
        };
        let mut card = ui::dialog_plate(radius::MODAL)
            .id("owner-card")
            .occlude()
            .flex()
            .flex_col()
            // A set width: text measured at it wraps where it is drawn.
            .w(px(520.))
            .max_w_full()
            .p(px(32.))
            .child(
                ui::text(13., FontWeight::MEDIUM, ink::FULL)
                    .text_color(gpui::rgb(0xfb923c))
                    .child(tr("NSGO account")),
            )
            .child(
                div()
                    .mt(px(8.))
                    .font_family(horizon_theme::glass::DISPLAY_FONT)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(px(28.))
                    .line_height(px(35.))
                    .text_color(white(ink::FULL))
                    .child(tr("Sign in to play")),
            )
            .child(
                div()
                    .mt(px(12.))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(ui::text(14., FontWeight::NORMAL, ink::BODY).child(tr(
                        "Sign in with your NSGO account to play online with custom skin, cape, and server access.",
                    )))
                    .child(ui::text(14., FontWeight::NORMAL, ink::BODY).child(tr(
                        "No Minecraft install needed: Horizon downloads it from Mojang for your account.",
                    ))),
            );
        card = match state {
            Shown::Code(uri, user_code) => {
                let copy = user_code.to_string();
                let (private_uri, link_uri, other_uri) =
                    (uri.to_string(), uri.to_string(), uri.to_string());
                card.child(
                    panel()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(ui::text(13., FontWeight::NORMAL, ink::NOTE).child(trf(
                                    "Open {link} in your browser and enter this code:",
                                    &[("link", short_link(uri))],
                                )))
                                .child(
                                    div()
                                        .font_family(theme::MONO)
                                        .font_weight(FontWeight::BOLD)
                                        .text_size(px(30.))
                                        .line_height(px(38.))
                                        .text_color(white(ink::FULL))
                                        .child(user_code.to_string()),
                                ),
                        )
                        .child(ui::quiet("owner-copy", tr("Copy code"), 36., &t).on_click(
                            move |_, _, cx| {
                                copy_to_clipboard(cx, gpui::ClipboardItem::new_string(copy.clone()))
                            },
                        )),
                )
                .child(
                    div()
                        .mt(px(8.))
                        .flex()
                        .flex_col()
                        .items_start()
                        .gap(px(4.))
                        .when(!microsoft::has_private_window(), |col| {
                            col.child(ui::text(13., FontWeight::NORMAL, ink::QUIET).child(tr(
                                "Open this link in a private window to pick another account.",
                            )))
                        })
                        .child(
                            div()
                                .flex()
                                .gap(px(8.))
                                .child(if microsoft::has_private_window() {
                                    ui::quiet(
                                        "owner-private",
                                        tr("Open in private window"),
                                        32.,
                                        &t,
                                    )
                                    .on_click(
                                        move |_, _, cx| microsoft::open_private(&private_uri, cx),
                                    )
                                } else {
                                    ui::quiet("owner-copy-link", tr("Copy link"), 32., &t).on_click(
                                        move |_, _, cx| {
                                            copy_to_clipboard(
                                                cx,
                                                gpui::ClipboardItem::new_string(link_uri.clone()),
                                            )
                                        },
                                    )
                                })
                                .child(
                                    ui::quiet(
                                        "owner-other",
                                        tr("Use a different Microsoft account"),
                                        32.,
                                        &t,
                                    )
                                    .on_click(
                                        move |_, _, cx| {
                                            microsoft::use_different_account(&other_uri, cx)
                                        },
                                    ),
                                ),
                        ),
                )
            }
            Shown::Browser(uri) => {
                let link_uri = uri.to_string();
                card.child(
                    panel()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(
                                    ui::text(14., FontWeight::SEMIBOLD, ink::FULL)
                                        .child(tr("Complete sign-in in your browser…")),
                                )
                                .child(
                                    ui::text(12., FontWeight::NORMAL, ink::NOTE).child(tr(
                                        "If your browser didn't open automatically, click Open Link.",
                                    )),
                                ),
                        )
                        .child(
                            ui::quiet("owner-browser-open", tr("Open Link"), 36., &t)
                                .on_click(move |_, _, cx| cx.open_url(&link_uri)),
                        ),
                )
            }
            Shown::Starting(what) => card
                .child(panel().child(ui::text(14., FontWeight::NORMAL, ink::BODY).child(tr(what)))),
            Shown::Error(e) => card.child(
                panel().child(
                    ui::text(14., FontWeight::NORMAL, ink::FULL)
                        .text_color(ui::color(horizon_theme::glass::DANGER))
                        .child(e.to_string()),
                ),
            ),
            Shown::Idle => card,
        };
        let main: Option<AnyElement> = match state {
            Shown::Code(uri, _) => {
                let uri = uri.to_string();
                Some(
                    ui::button(
                        "owner-open".into(),
                        tr("Open link").into(),
                        Kind::Primary,
                        &t,
                    )
                    .h(px(44.))
                    .px(px(24.))
                    .on_click(move |_, _, cx| cx.open_url(&uri))
                    .into_any_element(),
                )
            }
            Shown::Browser(_) => Some(
                ui::button(
                    "owner-cancel".into(),
                    tr("Cancel").into(),
                    Kind::Ghost,
                    &t,
                )
                .h(px(44.))
                .px(px(20.))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.cancel_sign_in();
                    this.nsgo.active = false;
                    cx.notify();
                }))
                .into_any_element(),
            ),
            Shown::Starting(_) => None,
            _ => Some(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .w_full()
                    .child(
                        nsgo_brand::sign_in_button("glass-owner-nsgo-sign-in")
                            .w_full()
                            .h(px(46.))
                            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                this.submit_nsgo();
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(12.))
                            .child(
                                keyboard(
                                    div()
                                        .id("owner-sign-in")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(px(8.))
                                        .h(px(40.))
                                        .px(px(16.))
                                        .rounded(px(10.))
                                        .bg(white(fill::BUTTON))
                                        .border_1()
                                        .border_color(white(0.12))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(white(fill::BUTTON_HOVER)))
                                        .font_family(horizon_theme::glass::FONT)
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(white(ink::FULL))
                                        .child(microsoft::logo(16.))
                                        .child(tr("Sign in with Microsoft")),
                                    &t,
                                )
                                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                    if account::client_id().is_some() {
                                        this.start_sign_in();
                                    } else {
                                        this.sign_in_state = Some(SignInState::Done(Err(tr(
                                            "Microsoft sign-in requires HORIZON_MSA_CLIENT_ID. Please sign in with NSGO to play.",
                                        )
                                        .to_string())));
                                    }
                                    cx.notify();
                                })),
                            )
                            .child(
                                ui::button("owner-later".into(), tr("Not now").into(), Kind::Ghost, &t)
                                    .h(px(40.))
                                    .px(px(16.))
                                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                                        this.close_owner_screen();
                                        cx.notify();
                                    })),
                            ),
                    )
                    .into_any_element(),
            ),
        };
        let later = if matches!(state, Shown::Code(..)) {
            Some(
                ui::button("owner-later".into(), tr("Not now").into(), Kind::Ghost, &t)
                    .h(px(44.))
                    .px(px(20.))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.close_owner_screen();
                        cx.notify();
                    })),
            )
        } else {
            None
        };
        card = card.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .mt(px(24.))
                .children(main)
                .children(later),
        );
        if matches!(state, Shown::Code(..)) {
            card = card.child(
                ui::text(13., FontWeight::NORMAL, ink::QUIET)
                    .mt(px(12.))
                    .child(tr("Waiting for Microsoft… the window goes on by itself.")),
            );
        }
        div()
            .id("owner-gate")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .p(px(24.))
            .on_scroll(|_, _, cx| cx.stop_propagation())
            .occlude()
            .child(ui::scrim_backdrop())
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .bg(ui::scrim()),
            )
            .child(card)
    }
}

/// What the sheet shows of the sign-in.
#[derive(Clone, Copy)]
enum Shown<'a> {
    Idle,
    /// What the sign-in is doing.
    Starting(&'static str),
    /// The page and the code to enter there.
    Code(&'a str, &'a str),
    /// Browser authorization flow is open.
    Browser(&'a str),
    Error(&'a str),
}

/// `HORIZON_LAUNCHER_PAGE=sign-in` opens the sheet with a made-up code, for screenshots: nothing
/// asks Microsoft, and the gate itself is as it was (no account, no Play).
pub(super) fn sign_in_preview() -> bool {
    static PREVIEW: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PREVIEW.get_or_init(|| std::env::var("HORIZON_LAUNCHER_PAGE").as_deref() == Ok("sign-in"))
}

/// `https://www.microsoft.com/link` as `microsoft.com/link`.
fn short_link(uri: &str) -> &str {
    let uri = uri
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    uri.strip_prefix("www.").unwrap_or(uri)
}

#[cfg(test)]
mod tests {
    #[test]
    fn links_are_shown_as_typed() {
        assert_eq!(
            super::short_link("https://www.microsoft.com/link"),
            "microsoft.com/link"
        );
        assert_eq!(
            super::short_link("https://login.live.com/oauth20_remoteconnect.srf"),
            "login.live.com/oauth20_remoteconnect.srf"
        );
    }
}
