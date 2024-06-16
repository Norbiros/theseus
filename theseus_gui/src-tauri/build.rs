// fn main() {
//     // Build the Tauri app
//     tauri_build::build();
// }

fn main() {
    // Build the app
    tauri_build::try_build(
        tauri_build::Attributes::new()
            .codegen(tauri_build::CodegenContext::new())
            .plugin(
                "auth",
                tauri_build::InlinedPlugin::new().commands(&["auth_get_default_user", "auth_set_default_user", "auth_remove_user", "auth_users", "auth_get_user"]),
            )
            .plugin(
                "import",
                tauri_build::InlinedPlugin::new().commands(&["import_get_importable_instances", "import_import_instance", "import_is_valid_importable_instance", "import_get_default_launcher_path"]),
            )
            .plugin(
                "jre",
                tauri_build::InlinedPlugin::new().commands(&["jre_find_filtered_jres", "jre_get_jre", "jre_test_jre", "jre_auto_install_java", "jre_get_max_memory"]),
            )
            .plugin(
                "logs",
                tauri_build::InlinedPlugin::new().commands(&["logs_get_logs", "logs_get_logs_by_filename", "logs_get_output_by_filename", "logs_delete_logs", "logs_delete_logs_by_filename", "logs_get_latest_log_cursor"]),
            )
            .plugin(
                "metadata",
                tauri_build::InlinedPlugin::new().commands(&["metadata_get_game_versions", "metadata_get_fabric_versions", "metadata_get_forge_versions", "metadata_get_quilt_versions", "metadata_get_neoforge_versions"])
            )
            .plugin(
                "mr-auth",
                tauri_build::InlinedPlugin::new().commands(&["authenticate_begin_flow", "authenticate_await_completion", "cancel_flow", "login_pass", "login_2fa", "create_account", "refresh", "logout", "get"])
            )
            .plugin(
                "pack",
                tauri_build::InlinedPlugin::new().commands(&["pack_install", "pack_get_profile_from_pack"])
            )
            .plugin(
                "process",
                tauri_build::InlinedPlugin::new().commands(&["process_has_finished_by_uuid", "process_get_exit_status_by_uuid", "process_get_all_uuids", "process_get_all_running_uuids", "process_get_uuids_by_profile_path", "process_get_all_running_profile_paths", "process_get_all_running_profiles", "process_kill_by_uuid", "process_wait_for_by_uuid"])
            )
            .plugin(
                "profile",
                tauri_build::InlinedPlugin::new().commands(&["profile_remove", "profile_get", "profile_get_optimal_jre_key", "profile_get_full_path", "profile_get_mod_full_path", "profile_list", "profile_check_installed", "profile_install", "profile_update_all", "profile_update_project", "profile_add_project_from_version", "profile_add_project_from_path", "profile_toggle_disable_project", "profile_remove_project", "profile_update_managed_modrinth_version", "profile_repair_managed_modrinth", "profile_run", "profile_run_wait", "profile_run_credentials", "profile_run_wait_credentials", "profile_edit", "profile_edit_icon", "profile_export_mrpack", "profile_get_pack_export_candidates"])
            )
            .plugin(
                "profile-create",
                tauri_build::InlinedPlugin::new().commands(&["profile_create", "profile_duplicate"])
            )
            .plugin(
                "settings",
                tauri_build::InlinedPlugin::new().commands(&["settings_get", "settings_set", "settings_change_config_dir", "settings_is_dir_writeable"])
            )
            .plugin(
                "tags",
                tauri_build::InlinedPlugin::new().commands(&["tags_get_categories", "tags_get_report_types", "tags_get_loaders", "tags_get_game_versions", "tags_get_donation_platforms", "tags_get_tag_bundle"])
            )
            .plugin(
                "utils",
                tauri_build::InlinedPlugin::new().commands(&["get_os", "should_disable_mouseover", "show_in_folder", "show_launcher_logs_folder", "progress_bars_list", "safety_check_safe_loading_bars", "get_opening_command", "await_sync", "is_offline", "refresh_offline"])
            )
    )
    .expect("Failed to run tauri-build");
}