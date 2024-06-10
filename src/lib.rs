use zed_extension_api as zed;

struct ValaExtension {}

impl zed::Extension for ValaExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let path = worktree
            .which("vala-language-server")
            .expect("vala-language-server not found");

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ValaExtension);
