use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

pub struct CSharpLs {}

impl CSharpLs {
    pub const LANGUAGE_SERVER_ID: &'static str = "csharp-ls";

    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_cmd(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(zed::Command {
                command: path,
                args: binary_args.unwrap_or_default(),
                env: Default::default(),
            });
        }

        // Check if csharp-ls is available in PATH
        if let Some(path) = worktree.which("csharp-ls") {
            return Ok(zed::Command {
                command: path,
                args: binary_args.unwrap_or_default(),
                env: Default::default(),
            });
        }

        Err("csharp-ls is not installed. Please run 'dotnet tool install --global csharp-ls' to install it.".into())
    }
}
