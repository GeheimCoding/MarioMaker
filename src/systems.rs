use bevy::prelude::*;
use std::fmt::Debug;

pub fn error_handler<E: Debug>(In(result): In<Result<(), E>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}
