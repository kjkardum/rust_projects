pub mod DTOs;
pub mod controllers;
pub mod cors;
pub mod data;
pub mod entities;
pub mod helpers;
pub mod services;
pub mod swagger;

#[macro_use]
extern crate rocket;

extern crate rocket_okapi;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate rocket_sync_db_pools;

#[macro_use]
extern crate diesel_migrations;
