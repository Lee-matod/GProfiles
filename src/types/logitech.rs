#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::APPLICATION_NAME_DESKTOP;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogitechData {
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

impl Clone for Application {
    fn clone(&self) -> Self {
        Application {
            name: self.name.clone(),
            applicationId: self.applicationId.clone(),
            applicationPath: self.applicationPath.clone(),
            isCustom: self.isCustom.clone(),
            posterPath: self.posterPath.clone(),
            applicationFolder: self.applicationFolder.clone(),
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
            data["databaseId"] = self.databaseId.unwrap().into();
            data["version"] = self.version.unwrap().into();
        }
        data
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CategoryColors {
//     pub hex: String,
//     pub tag: String,
// }

// impl Clone for CategoryColors {
//     fn clone(&self) -> Self {
//         CategoryColors {
//             hex: self.hex.clone(),
//             tag: self.tag.clone(),
//         }
//     }
// }

// impl Into<Value> for CategoryColors {
//     fn into(self) -> Value {
//         json!({
//             "hex": self.hex,
//             "tag": self.tag
//         })
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Command {
//     pub cardId: String,
//     pub category: String,
//     pub name: String,
// }

// impl Clone for Command {
//     fn clone(&self) -> Self {
//         Command {
//             cardId: self.cardId.clone(),
//             category: self.category.clone(),
//             name: self.name.clone(),
//         }
//     }
// }

// impl Into<Value> for Command {
//     fn into(self) -> Value {
//         json!({
//             "cardId": self.cardId,
//             "category": self.category,
//             "name": self.name
//         })
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub activeForApplication: bool,
    pub applicationId: String,
    pub id: String,
    pub name: String,
    pub assignments: Vec<Assignment>,
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
        json!({
            "activeForApplication": self.activeForApplication,
            "applicationId": self.applicationId,
            "id": self.id,
            "name": self.name,
            "assignments": self.assignments,
        })
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
