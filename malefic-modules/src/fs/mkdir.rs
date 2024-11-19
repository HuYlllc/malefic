use async_trait::async_trait;
use malefic_trait::module_impl;
use crate::{ check_request, Module, Result, check_field, TaskResult};
use malefic_proto::proto::implantpb::spite::Body;

pub struct Mkdir{}

#[async_trait]
#[module_impl("mkdir")]
impl Module for Mkdir {
    #[allow(unused_variables)]
    async fn run(&mut self, id: u32, receiver: &mut crate::Input, sender: &mut crate::Output) -> Result {
        let request = check_request!(receiver, Body::Request)?;

        let dir = check_field!(request.input)?;

        std::fs::create_dir_all(dir)?;

        Ok(TaskResult::new(id))
    }
}