#![allow(non_snake_case, dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    pub applicationPath: Option<String>,

    // Custom applications
    pub isCustom: Option<bool>,
    pub posterPath: Option<String>,
}

impl Clone for Application {
    fn clone(&self) -> Self {
        Application {
            name: self.name.clone(),
            applicationId: self.applicationId.clone(),
            applicationPath: self.applicationPath.clone(),
            isCustom: self.isCustom.clone(),
            posterPath: self.posterPath.clone(),
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
        json!({
            "name": self.name,
            "applicationId": self.applicationId,
            "applicationPath": self.applicationPath,
            "isCustom": self.isCustom,
            "posterPath": self.posterPath,
        })
    }
}

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
