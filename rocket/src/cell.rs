// use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};


use schema::cells;

#[table_name = "cells"]
#[derive(Serialize, Deserialize, Queryable)]
pub struct Cell {
    pub id: i32,
    pub row: i32,
    pub column: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

//implement responder trait, so that the type can be used as response
impl<'r> Responder<'r> for Cell {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            //.sized_body(Cursor::new(format!("{}:{}", self.name, self.age)))
            //.raw_header("X-Person-Name", self.name)
            //.raw_header("X-Person-Age", self.age.to_string())
            //.header(ContentType::new("application", "x-person"))
            .ok()
    }
}
