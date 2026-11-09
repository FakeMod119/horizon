//! Offline play control on Settings > Account. The original dev_offline setting and owner
//! gate fallback are reused; this fork makes them visible and enabled by default. Minecraft
//! resources must already be available from the player's local installation or preparation.
use super::*;

/// Clicks on the launcher's version that reveal the section.
const CLICKS: u8 = 7;

impl Launcher {
    /// A click on the sidebar's launcher version; the last one opens Settings on the section.
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    /// Always-visible offline-mode setting (also present in the Glass look).
    pub(super) fn dev_options_section(&self, cx: &mut Context<Self>) -> Option<Div> {
        let t = self.theme;
        let toggle = switch("dev-offline", self.settings.dev_offline, &t).on_click(cx.listener(
            |this, _: &ClickEvent, _, cx| {
                this.settings.dev_offline = !this.settings.dev_offline;
                this.save_settings();
                cx.notify();
            },
        ));
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Offline play", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Offline names without an account"),
                    Some(
                        div()
                            .child(tr(
                                "Offline names play without a Microsoft account that owns the game. \
                                 Minecraft still comes from your own install.",
                            ))
                            .into_any_element(),
                    ),
                    Some(toggle.into_any_element()),
                    &t,
                )),
        )
    }
}
