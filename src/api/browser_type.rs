pub use crate::imp::{
    browser_type::{RecordHar, RecordVideo},
    utils::{ColorScheme, Geolocation, HttpCredentials, ProxySettings, Viewport}
};
use crate::{
    api::{browser::Browser, browser_context::BrowserContext},
    imp::{
        self,
        browser_type::{LaunchArgs, LaunchPersistentContextArgs},
        core::*,
        prelude::*
    },
    Error
};

#[derive(Debug, Clone)]
pub struct BrowserType {
    inner: Weak<imp::browser_type::BrowserType>
}

impl BrowserType {
    pub(crate) fn new(inner: Weak<imp::browser_type::BrowserType>) -> Self { Self { inner } }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn name(&self) -> Result<String, Error> { Ok(upgrade(&self.inner)?.name().into()) }

    /// # Errors
    /// Returns error only if this function is called after object is disposed.
    pub fn executable(&self) -> Result<PathBuf, Error> {
        Ok(upgrade(&self.inner)?.executable().into())
    }

    pub fn launcher(&mut self) -> Launcher<'_, '_, '_> { Launcher::new(self.inner.clone()) }

    pub fn persistent_context_launcher<'a>(
        &mut self,
        user_data_dir: &'a Path
    ) -> PersistentContextLauncher<'a, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
        PersistentContextLauncher::new(self.inner.clone(), user_data_dir)
    }
}

/// [`BrowserType::launcher`]
pub struct Launcher<'a, 'b, 'c> {
    inner: Weak<imp::browser_type::BrowserType>,
    args: LaunchArgs<'a, 'b, 'c>
}

impl<'a, 'b, 'c> Launcher<'a, 'b, 'c> {
    pub async fn launch(self) -> Result<Browser, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch(args).await?;
        Ok(Browser::new(r))
    }

    fn new(inner: Weak<imp::browser_type::BrowserType>) -> Self {
        Launcher {
            inner,
            args: LaunchArgs::default()
        }
    }

    optional_setter!(
        executable, &'a Path;
        args, &'b [String];
        ignore_all_default_args, bool;
        handle_sigint, bool;
        handle_sigterm, bool;
        handle_sighup, bool;
        timeout, f64;
        env, Map<String, Value>;
        headless, bool;
        devtools, bool;
        proxy, ProxySettings;
        downloads, &'c Path;
        slowmo, f64;
        chromium_sandbox, f64;
        firefox_user_prefs, Map<String, Value>);
}

/// [`BrowserType::persistent_context_launcher`]
///
/// Has launch args and context args
pub struct PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    inner: Weak<imp::browser_type::BrowserType>,
    args: LaunchPersistentContextArgs<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
    PersistentContextLauncher<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k>
{
    pub async fn launch(self) -> Result<BrowserContext, Arc<Error>> {
        let Self { inner, args } = self;
        let r = upgrade(&inner)?.launch_persistent_context(args).await?;
        Ok(BrowserContext::new(r))
    }

    fn new(inner: Weak<imp::browser_type::BrowserType>, user_data_dir: &'a Path) -> Self {
        Self {
            inner,
            args: LaunchPersistentContextArgs::new(user_data_dir)
        }
    }

    optional_setter!(
        executable, &'b Path;
        args, &'c [String];
        ignore_all_default_args, bool;
        handle_sigint, bool;
        handle_sigterm, bool;
        handle_sighup, bool;
        timeout, f64;
        env, Map<String, Value>;
        headless, bool;
        devtools, bool;
        proxy, ProxySettings;
        downloads, &'d Path;
        slowmo, f64;
        viewport, Viewport;
        no_default_viewport, bool;
        ignore_http_errors, bool;
        js_enabled, bool;
        bypass_csp, bool;
        user_agent, &'e str;
        locale, &'f str;
        timezone_id, &'g str;
        geolocation, Geolocation;
        permissions, &'h [String];
        extra_http_headers, HashMap<String, String>;
        offline, bool;
        http_credentials, &'i HttpCredentials;
        device_scale_factor, f64;
        is_mobile, bool;
        has_touch, bool;
        color_scheme, ColorScheme;
        accept_downloads, bool;
        chromium_sandbox, bool;
        record_video, RecordVideo<'j>;
        record_har, RecordHar<'k>);
}
