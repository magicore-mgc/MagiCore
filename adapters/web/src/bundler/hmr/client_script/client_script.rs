//! Embedded browser client for the WebSocket HMR endpoint.
//! Browser client nhúng cho WebSocket HMR endpoint.

/// Client payload served at `/@magicore/hmr.js`.
/// Payload client được phục vụ tại `/@magicore/hmr.js`.
pub const HMR_CLIENT_SCRIPT: &str = r#"
(function() {
    console.log('[MgDevServer] Connecting to HMR WebSocket...');
    const ws = new WebSocket(`ws://${window.location.host}/@magicore/hmr`);
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.type === 'connected') {
            console.log('[MgDevServer] HMR connected.');
            return;
        }
        if (data.type === 'reload') {
            console.log('[MgDevServer] File changed, reloading...');
            window.location.reload();
        }
    };
    ws.onclose = () => {
        console.warn('[MgDevServer] HMR connection lost. Server might be restarting.');
        setTimeout(() => window.location.reload(), 2000);
    };
})();
"#;

#[cfg(test)]
#[path = "test/client_script.rs"]
mod tests;
