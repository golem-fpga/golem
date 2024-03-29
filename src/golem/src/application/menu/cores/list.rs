use crate::application::coordinator::GameStartInfo;
use crate::application::menu::cores::download::cores_download_panel;
use crate::application::menu::filesystem::{select_file_path_menu, FilesystemMenuOptions};
use crate::application::menu::style::MenuReturn;
use crate::application::menu::text_menu;
use crate::application::menu::TextMenuOptions;
use crate::application::panels::alert::{alert, show_error};
use crate::application::panels::core_loop::run_core_loop;
use crate::application::GoLEmApp;
use crate::platform::GoLEmPlatform;
use anyhow::anyhow;
use golem_db::models;
use golem_db::models::CoreOrder;
use thiserror::__private::AsDynError;
use tracing::{error, info};

#[derive(Default, Debug, Clone, Copy)]
pub enum MenuAction {
    #[default]
    Back,
    ChangeSort,
    Manage,
    ManualLoad,
    ExecuteCore(usize),
    SelectCore(usize),
    ShowCoreDetails(usize),
}

impl MenuReturn for MenuAction {
    fn back() -> Option<Self> {
        Some(MenuAction::Back)
    }

    fn into_details(self) -> Option<Self> {
        match self {
            MenuAction::ExecuteCore(i) => Some(MenuAction::ShowCoreDetails(i)),
            _ => None,
        }
    }

    fn sort() -> Option<Self> {
        Some(Self::ChangeSort)
    }
}

fn build_cores_items_(database: &mut golem_db::Connection, order: CoreOrder) -> Vec<models::Core> {
    let all_cores = models::Core::list(database, 0, 1000, order);
    all_cores.unwrap_or_else(|e| {
        error!("Database error: {e}");
        Vec::new()
    })
}

pub fn select_core(app: &mut GoLEmApp, title: &str) -> Option<models::Core> {
    let mut state = None;
    let mut core_order = CoreOrder::default();

    loop {
        // Refresh the cores.
        let mut all_cores = build_cores_items_(&mut app.database().lock().unwrap(), core_order);

        let menu_items = all_cores
            .iter()
            .enumerate()
            .map(|(i, core)| (core.name.as_str(), "", MenuAction::SelectCore(i)))
            .collect::<Vec<_>>();

        let (result, new_state) = text_menu(
            app,
            title,
            &menu_items,
            TextMenuOptions::default()
                .with_state(state)
                .with_sort(core_order.as_str()),
        );
        state = Some(new_state);

        match result {
            MenuAction::Back => {
                return None;
            }
            MenuAction::SelectCore(i) => {
                let core = all_cores.swap_remove(i);
                return Some(core);
            }
            MenuAction::ChangeSort => core_order = core_order.next(),
            _ => {}
        }
    }
}

pub fn cores_menu_panel(app: &mut GoLEmApp) {
    let mut state = None;
    let mut core_order = CoreOrder::default();

    loop {
        // Refresh the cores.
        let all_cores = build_cores_items_(&mut app.database().lock().unwrap(), core_order);

        let menu_items = all_cores
            .iter()
            .enumerate()
            .map(|(i, core)| (core.name.as_str(), "", MenuAction::ExecuteCore(i)))
            .collect::<Vec<_>>();

        let (result, new_state) = text_menu(
            app,
            "Cores",
            &menu_items,
            TextMenuOptions::default()
                .with_state(state)
                .with_sort(core_order.as_str())
                .with_details("Details")
                .with_suffix(&[
                    ("Load Core Manually", "", MenuAction::ManualLoad),
                    ("Manage Cores", "", MenuAction::Manage),
                ]),
        );
        state = Some(new_state);
        match result {
            MenuAction::Back => break,
            MenuAction::Manage => {
                cores_download_panel(app);
            }
            MenuAction::ChangeSort => core_order = core_order.next(),
            MenuAction::ExecuteCore(i) | MenuAction::SelectCore(i) => {
                let core = &all_cores[i];
                let path = &core.path;
                info!("Loading core from path {:?}", path);

                match app
                    .coordinator_mut()
                    .launch_game(app, GameStartInfo::default().with_core_id(core.id))
                {
                    Ok((_, core)) => {
                        run_core_loop(app, core, true);
                    }
                    Err(e) => {
                        show_error(app, anyhow!(e).as_dyn_error(), true);
                    }
                };
            }
            MenuAction::ShowCoreDetails(i) => {
                alert(
                    app,
                    "Core Details",
                    &format!("{:#?}", all_cores[i]),
                    &["Okay"],
                );
            }
            MenuAction::ManualLoad => {
                let path = select_file_path_menu(
                    app,
                    "Select Core Manually",
                    std::env::current_exe().unwrap().parent().unwrap(),
                    FilesystemMenuOptions::default().with_allow_back(true),
                );
                info!("Loading core from path {:?}", path);

                if let Ok(Some(path)) = path {
                    match app.platform_mut().core_manager_mut().load_core(&path) {
                        Ok(core) => {
                            run_core_loop(app, core, true);
                        }
                        Err(e) => {
                            show_error(app, anyhow!(e).as_dyn_error(), true);
                            return;
                        }
                    }

                    // TODO: reload the Menu core here.
                } else {
                    info!("No core selected.");
                }
            }
        }
    }
}
