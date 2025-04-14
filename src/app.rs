use crate::api::{StashClient, Scene, Performer, Tag};
use std::error::Error;

pub enum AppTab {
    Scenes,
    Performers,
    Tags,
    Studios,
}

#[derive(PartialEq)]
pub enum ViewMode {
    List,
    Detail,
}

pub struct App {
    pub stash_client: StashClient,
    pub current_tab: AppTab,
    pub scenes: Vec<Scene>,
    pub performers: Vec<Performer>,
    pub tags: Vec<Tag>,
    pub selected_index: usize,
    pub status_message: String,
    pub loading: bool,
    pub page: i32,
    pub per_page: i32,
    pub total_count: i32,
    pub view_mode: ViewMode,
}

impl App {
    pub fn new(stash_url: &str) -> Self {
        Self {
            stash_client: StashClient::new(stash_url),
            current_tab: AppTab::Scenes,
            scenes: Vec::new(),
            performers: Vec::new(),
            tags: Vec::new(),
            selected_index: 0,
            status_message: String::from("Welcome to Stash TUI"),
            loading: false,
            page: 1,
            per_page: 25,
            total_count: 0,
            view_mode: ViewMode::List,
        }
    }

    pub async fn fetch_system_status(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        self.status_message = "Fetching system status...".to_string();
        
        match self.stash_client.get_system_status().await {
            Ok(status) => {
                self.status_message = format!(
                    "Connected to Stash. DB Schema: {}, Status: {}",
                    status.database_schema,
                    status.status
                );
                self.loading = false;
                Ok(())
            },
            Err(e) => {
                self.status_message = format!("Error fetching system status: {}", e);
                self.loading = false;
                Err(e)
            }
        }
    }

    pub async fn fetch_current_data(&mut self) -> Result<(), Box<dyn Error>> {
        match self.current_tab {
            AppTab::Scenes => self.fetch_scenes().await,
            AppTab::Performers => self.fetch_performers().await,
            AppTab::Tags => self.fetch_tags().await,
            AppTab::Studios => Ok(()),
        }
    }

    pub async fn fetch_scenes(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        self.status_message = "Fetching scenes...".to_string();
        
        match self.stash_client.find_scenes(self.page, self.per_page).await {
            Ok(scenes_response) => {
                self.scenes = scenes_response.scenes;
                self.total_count = scenes_response.count;
                self.status_message = format!("Loaded {} of {} scenes", self.scenes.len(), self.total_count);
                self.loading = false;
                Ok(())
            },
            Err(e) => {
                self.status_message = format!("Error fetching scenes: {}", e);
                self.loading = false;
                Err(e)
            }
        }
    }

    pub async fn fetch_performers(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        self.status_message = "Fetching performers...".to_string();
        
        match self.stash_client.find_performers(self.page, self.per_page).await {
            Ok(performers_response) => {
                self.performers = performers_response.performers;
                self.total_count = performers_response.count;
                self.status_message = format!("Loaded {} of {} performers", self.performers.len(), self.total_count);
                self.loading = false;
                Ok(())
            },
            Err(e) => {
                self.status_message = format!("Error fetching performers: {}", e);
                self.loading = false;
                Err(e)
            }
        }
    }

    pub async fn fetch_tags(&mut self) -> Result<(), Box<dyn Error>> {
        self.loading = true;
        self.status_message = "Fetching tags...".to_string();
        
        match self.stash_client.find_tags(self.page, self.per_page).await {
            Ok(tags_response) => {
                self.tags = tags_response.tags;
                self.total_count = tags_response.count;
                self.status_message = format!("Loaded {} of {} tags", self.tags.len(), self.total_count);
                self.loading = false;
                Ok(())
            },
            Err(e) => {
                self.status_message = format!("Error fetching tags: {}", e);
                self.loading = false;
                Err(e)
            }
        }
    }

    pub fn next_item(&mut self) {
        let list_len = match self.current_tab {
            AppTab::Scenes => self.scenes.len(),
            AppTab::Performers => self.performers.len(),
            AppTab::Tags => self.tags.len(),
            AppTab::Studios => 0,
        };

        if list_len > 0 {
            self.selected_index = (self.selected_index + 1) % list_len;
        }
    }

    pub fn previous_item(&mut self) {
        let list_len = match self.current_tab {
            AppTab::Scenes => self.scenes.len(),
            AppTab::Performers => self.performers.len(),
            AppTab::Tags => self.tags.len(),
            AppTab::Studios => 0,
        };

        if list_len > 0 {
            self.selected_index = if self.selected_index > 0 {
                self.selected_index - 1
            } else {
                list_len - 1
            };
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Scenes => AppTab::Performers,
            AppTab::Performers => AppTab::Tags,
            AppTab::Tags => AppTab::Studios,
            AppTab::Studios => AppTab::Scenes,
        };
        self.selected_index = 0;
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            AppTab::Scenes => AppTab::Studios,
            AppTab::Performers => AppTab::Scenes,
            AppTab::Tags => AppTab::Performers,
            AppTab::Studios => AppTab::Tags,
        };
        self.selected_index = 0;
    }

    pub async fn next_page(&mut self) -> Result<(), Box<dyn Error>> {
        let max_page = (self.total_count + self.per_page - 1) / self.per_page;
        if self.page < max_page {
            self.page += 1;
            self.selected_index = 0;
            self.fetch_current_data().await?;
        }
        Ok(())
    }

    pub async fn previous_page(&mut self) -> Result<(), Box<dyn Error>> {
        if self.page > 1 {
            self.page -= 1;
            self.selected_index = 0;
            self.fetch_current_data().await?;
        }
        Ok(())
    }
    
    pub fn toggle_detail_view(&mut self) {
        match self.view_mode {
            ViewMode::List => {
                // Only enter detail view if there are items to show
                let has_items = match self.current_tab {
                    AppTab::Scenes => !self.scenes.is_empty(),
                    AppTab::Performers => !self.performers.is_empty(),
                    AppTab::Tags => !self.tags.is_empty(),
                    AppTab::Studios => false, // Not implemented yet
                };
                
                if has_items {
                    self.view_mode = ViewMode::Detail;
                    self.status_message = "Viewing details. Press Escape to return to list view.".to_string();
                }
            },
            ViewMode::Detail => {
                self.view_mode = ViewMode::List;
                self.status_message = "Returned to list view.".to_string();
            },
        }
    }
}
