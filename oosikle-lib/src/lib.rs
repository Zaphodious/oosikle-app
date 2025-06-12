pub mod db;
pub mod lua_api;
pub mod miko;
pub mod facadefs;
use crate::db::init_db;
use hypertext::{html_elements, maud, rsx, GlobalAttributes, Renderable};
use std::fmt;


pub fn run() {
    // lua_api::demotest().unwrap();
    // lua_api::mt_test().unwrap();
}
