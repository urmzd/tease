use crate::browser::{self, BrowserPage};

/// Install the DOM MutationObserver that tracks page activity (idempotent).
pub(crate) async fn install_idle_tracker(page: &dyn BrowserPage) {
    page.execute(
        r#"(() => {
            if (!window.__teasrIdle) {
                window.__teasrIdle = { changed: true };
                new MutationObserver(() => { window.__teasrIdle.changed = true; })
                    .observe(document.documentElement, {
                        childList: true, subtree: true, attributes: true, characterData: true
                    });
            } else {
                window.__teasrIdle.changed = true;
            }
        })()"#,
    )
    .await;
}

/// Check whether the page has had DOM mutations since the last check.
pub(crate) fn page_has_activity(
    page: &dyn BrowserPage,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = bool> + Send + '_>> {
    Box::pin(async move {
        browser::try_evaluate::<bool>(
            page,
            r#"(() => {
                const t = window.__teasrIdle;
                if (!t) return false;
                const c = t.changed;
                t.changed = false;
                return c;
            })()"#,
        )
        .await
        .unwrap_or(false)
    })
}
