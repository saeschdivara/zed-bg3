use zed_extension_api as zed;

struct Bg3Extension {
    // ... state
}

impl zed::Extension for Bg3Extension {
    fn new() -> Self
    where
        Self: Sized
    {
        Self {}
    }

    fn language_server_command(&mut self, _language_server_id: &zed::LanguageServerId, worktree: &zed::Worktree) -> zed::Result<zed::Command> {

        Ok(zed::Command {
            command: "/Users/saschahaeusler/Development/Rust/bg3-project-lsp/target/debug/bg3-project-lsp".to_string(),
            args: vec!["--use-printf-debugging".to_string()],
            env: worktree.shell_env(),
        })
    }

    // fn language_server_initialization_options(&mut self, _language_server_id: &LanguageServerId, _worktree: &Worktree) -> zed_extension_api::Result<Option<Value>> {
    //     todo!()
    // }
    // 
    // fn language_server_workspace_configuration(&mut self, _language_server_id: &LanguageServerId, _worktree: &Worktree) -> zed_extension_api::Result<Option<Value>> {
    //     todo!()
    // }
    // 
    // fn complete_slash_command_argument(&self, _command: SlashCommand, _args: Vec<String>) -> zed_extension_api::Result<Vec<SlashCommandArgumentCompletion>, String> {
    //     todo!()
    // }
    // 
    // fn run_slash_command(&self, _command: SlashCommand, _args: Vec<String>, _worktree: Option<&Worktree>) -> zed_extension_api::Result<SlashCommandOutput, String> {
    //     todo!()
    // }
    // 
    // fn suggest_docs_packages(&self, _provider: String) -> zed_extension_api::Result<Vec<String>, String> {
    //     todo!()
    // }
    // 
    // fn index_docs(&self, _provider: String, _package: String, _database: &KeyValueStore) -> zed_extension_api::Result<(), String> {
    //     todo!()
    // }
    // ...
}

zed::register_extension!(Bg3Extension);