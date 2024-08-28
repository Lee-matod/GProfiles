#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::{APPLICATION_NAME_DESKTOP, PROFILE_NAME_DEFAULT};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
    pub applications: InnerApplications,
    pub profiles: InnerProfiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerApplications {
    pub applications: Vec<Application>,
}

impl Into<Value> for InnerApplications {
    fn into(self) -> Value {
        json!(self.applications)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerProfiles {
    pub profiles: Vec<Profile>,
}

impl Into<Value> for InnerProfiles {
    fn into(self) -> Value {
        json!(self.profiles)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub applicationId: String,

    // Non-desktop profiles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicationPath: Option<String>,

    // Non-custom applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databaseId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categoryColors: Option<Vec<CategoryColors>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<usize>,

    // Custom applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isCustom: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posterPath: Option<String>,

    // Installed applications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicationFolder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isInstalled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posterTitlePosition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posterUrl: Option<String>,
}

impl Application {
    pub fn new(name: &String, applicationId: &String) -> Application {
        Application {
            name: name.clone(),
            applicationId: applicationId.clone(),
            applicationPath: None,
            databaseId: None,
            categoryColors: None,
            commands: None,
            version: None,
            isCustom: None,
            posterPath: None,
            applicationFolder: None,
            isInstalled: None,
            posterTitlePosition: None,
            posterUrl: None,
        }
    }

    pub fn custom(
        name: String,
        applicationId: String,
        applicationPath: String,
        posterPath: String,
    ) -> Application {
        let mut app = Application::new(&name, &applicationId);
        app.applicationPath = Some(applicationPath);
        app.posterPath = Some(posterPath);
        app.isCustom = Some(true);
        app
    }

    pub fn installed(
        name: String,
        applicationId: String,
        applicationPath: String,
        applicationFolder: String,
        categoryColors: Vec<CategoryColors>,
        commands: Vec<Command>,
        posterTitlePosition: String,
        posterUrl: String,
        version: usize,
    ) -> Application {
        let mut app = Application::new(&name, &applicationId);
        app.applicationPath = Some(applicationPath);
        app.databaseId = Some(applicationId);
        app.categoryColors = Some(categoryColors);
        app.commands = Some(commands);
        app.version = Some(version);
        app.applicationFolder = Some(applicationFolder);
        app.isInstalled = Some(true);
        app.posterTitlePosition = Some(posterTitlePosition);
        app.posterUrl = Some(posterUrl);
        app
    }

    pub fn desktop(
        applicationId: String,
        categoryColors: Vec<CategoryColors>,
        commands: Vec<Command>,
        version: usize,
    ) -> Application {
        let mut app = Application::new(&String::from(APPLICATION_NAME_DESKTOP), &applicationId);
        app.categoryColors = Some(categoryColors);
        app.commands = Some(commands);
        app.databaseId = Some(applicationId);
        app.version = Some(version);
        app
    }

    pub fn update(&mut self, app: Application) -> () {
        self.name = app.name;
        self.applicationId = app.applicationId;
        self.applicationPath = app.applicationPath;
        self.databaseId = app.databaseId;
        self.categoryColors = app.categoryColors;
        self.commands = app.commands;
        self.version = app.version;
        self.isCustom = app.isCustom;
        self.posterPath = app.posterPath;
        self.applicationFolder = app.applicationFolder;
        self.isInstalled = app.isInstalled;
        self.posterTitlePosition = app.posterTitlePosition;
        self.posterUrl = app.posterUrl;
    }
}

impl Clone for Application {
    fn clone(&self) -> Self {
        Application {
            name: self.name.clone(),
            applicationId: self.applicationId.clone(),
            applicationPath: self.applicationPath.clone(),
            isCustom: self.isCustom.clone(),
            posterPath: self.posterPath.clone(),
            applicationFolder: self.applicationFolder.clone(),
            categoryColors: self.categoryColors.clone(),
            commands: self.commands.clone(),
            databaseId: self.databaseId.clone(),
            isInstalled: self.isInstalled.clone(),
            posterTitlePosition: self.posterTitlePosition.clone(),
            posterUrl: self.posterUrl.clone(),
            version: self.version.clone(),
        }
    }
}

impl PartialEq for Application {
    fn eq(&self, other: &Self) -> bool {
        self.applicationId == other.applicationId
    }
}

impl Into<Value> for Application {
    fn into(self) -> Value {
        let mut data = json!({
            "name": self.name,
            "applicationId": self.applicationId
        });
        if let Some(applicationPath) = self.applicationPath {
            data["applicationPath"] = applicationPath.into();
        }
        if let Some(isCustom) = self.isCustom {
            data["isCustom"] = isCustom.into();
            data["posterPath"] = self.posterPath.unwrap().into();
        }
        if let Some(isInstalled) = self.isInstalled {
            data["isInstalled"] = isInstalled.into();
            data["applicationFolder"] = self.applicationFolder.unwrap().into();
            data["posterTitlePosition"] = self.posterTitlePosition.unwrap().into();
            data["posterUrl"] = self.posterUrl.unwrap().into();
        }
        if self.name == APPLICATION_NAME_DESKTOP || self.isInstalled.is_some() {
            data["categoryColors"] = self.categoryColors.unwrap().into();
            data["commands"] = self.commands.unwrap().into();
            data["databaseId"] = self.databaseId.unwrap().into();
            data["version"] = self.version.unwrap().into();
        }
        data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryColors {
    pub hex: String,
    pub tag: String,
}

impl Clone for CategoryColors {
    fn clone(&self) -> Self {
        CategoryColors {
            hex: self.hex.clone(),
            tag: self.tag.clone(),
        }
    }
}

impl Into<Value> for CategoryColors {
    fn into(self) -> Value {
        json!({
            "hex": self.hex,
            "tag": self.tag
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub cardId: String,
    pub category: String,
    pub name: String,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            cardId: self.cardId.clone(),
            category: self.category.clone(),
            name: self.name.clone(),
        }
    }
}

impl Into<Value> for Command {
    fn into(self) -> Value {
        json!({
            "cardId": self.cardId,
            "category": self.category,
            "name": self.name
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activeForApplication: Option<bool>,
    pub applicationId: String,
    pub id: String,
    pub name: String,
    pub assignments: Vec<Assignment>,
}

impl Profile {
    pub fn default(id: &String) -> Profile {
        Profile {
            activeForApplication: Some(true),
            applicationId: id.clone(),
            id: id.clone(),
            name: PROFILE_NAME_DEFAULT.to_string(),
            assignments: Vec::new(),
        }
    }
}

impl Clone for Profile {
    fn clone(&self) -> Self {
        Profile {
            activeForApplication: self.activeForApplication.clone(),
            applicationId: self.applicationId.clone(),
            id: self.id.clone(),
            name: self.name.clone(),
            assignments: self.assignments.clone(),
        }
    }
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Into<Value> for Profile {
    fn into(self) -> Value {
        let mut data = json!({
            "applicationId": self.applicationId,
            "id": self.id,
            "name": self.name,
            "assignments": self.assignments,
        });
        if self.activeForApplication.is_some() {
            data["activeForApplication"] = true.into();
        }
        data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    cardId: String,
    slotId: String,
}

impl Clone for Assignment {
    fn clone(&self) -> Self {
        Assignment {
            cardId: self.cardId.clone(),
            slotId: self.slotId.clone(),
        }
    }
}

impl Into<Value> for Assignment {
    fn into(self) -> Value {
        json!({
            "cardId": self.cardId,
            "slotId": self.slotId,
        })
    }
}
