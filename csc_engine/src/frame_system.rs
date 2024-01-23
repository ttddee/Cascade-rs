// Copyright (c) 2017 The vulkano developers <=== !
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use std::{convert::TryFrom, sync::Arc};

use cgmath::Matrix4;
use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        RenderPassBeginInfo, SecondaryCommandBufferAbstract, SubpassBeginInfo, SubpassContents,
    },
    device::Queue,
    format::Format,
    image::view::ImageView,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::GpuFuture,
};

use crate::renderer::Allocators;

/// System that contains the necessary facilities for rendering a single frame.
pub struct FrameSystem {
    gfx_queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    allocators: Allocators,
}

impl FrameSystem {
    pub fn new(
        gfx_queue: Arc<Queue>,
        final_output_format: Format,
        allocators: Allocators,
    ) -> FrameSystem {
        let render_pass = vulkano::ordered_passes_renderpass!(gfx_queue.device().clone(),
            attachments: {
                final_color: {
                    format: final_output_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            passes: [
                {
                    color: [final_color],
                    depth_stencil: {},
                    input: []
                }
            ]
        )
        .unwrap();

        FrameSystem {
            gfx_queue,
            render_pass,
            allocators,
        }
    }

    #[inline]
    pub fn deferred_subpass(&self) -> Subpass {
        Subpass::from(self.render_pass.clone(), 0).unwrap()
    }

    pub fn frame<F>(
        &mut self,
        before_future: F,
        final_image: Arc<ImageView>,
        world_to_framebuffer: Matrix4<f32>,
    ) -> Frame
    where
        F: GpuFuture + 'static,
    {
        let framebuffer = Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![final_image],
                ..Default::default()
            },
        )
        .unwrap();
        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            self.allocators.command_buffers.as_ref(),
            self.gfx_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        command_buffer_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.05, 0.05, 0.05, 0.0].into()), Some(1.0f32.into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassBeginInfo {
                    contents: SubpassContents::SecondaryCommandBuffers,
                    ..Default::default()
                },
            )
            .unwrap();

        Frame {
            system: self,
            before_main_cb_future: Some(Box::new(before_future)),
            framebuffer,
            num_pass: 0,
            command_buffer_builder: Some(command_buffer_builder),
            world_to_framebuffer,
        }
    }
}

pub struct Frame<'a> {
    system: &'a mut FrameSystem,
    num_pass: u8,
    before_main_cb_future: Option<Box<dyn GpuFuture>>,
    framebuffer: Arc<Framebuffer>,
    command_buffer_builder: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    #[allow(dead_code)]
    world_to_framebuffer: Matrix4<f32>,
}

impl<'a> Frame<'a> {
    pub fn next_pass<'f>(&'f mut self) -> Option<Pass<'f, 'a>> {
        match {
            let current_pass = self.num_pass;
            self.num_pass += 1;
            current_pass
        } {
            0 => Some(Pass::Deferred(DrawPass { frame: self })),
            1 => {
                self.command_buffer_builder
                    .as_mut()
                    .unwrap()
                    .end_render_pass(Default::default())
                    .unwrap();
                let command_buffer = self.command_buffer_builder.take().unwrap().build().unwrap();
                let after_main_cb = self
                    .before_main_cb_future
                    .take()
                    .unwrap()
                    .then_execute(self.system.gfx_queue.clone(), command_buffer)
                    .unwrap();
                Some(Pass::Finished(Box::new(after_main_cb)))
            }
            _ => None,
        }
    }
}

pub enum Pass<'f, 's: 'f> {
    Deferred(DrawPass<'f, 's>),
    Finished(Box<dyn GpuFuture>),
}

pub struct DrawPass<'f, 's: 'f> {
    frame: &'f mut Frame<'s>,
}

impl<'f, 's: 'f> DrawPass<'f, 's> {
    #[inline]
    pub fn execute<C>(&mut self, command_buffer: Arc<C>)
    where
        C: SecondaryCommandBufferAbstract + Send + Sync + 'static,
    {
        self.frame
            .command_buffer_builder
            .as_mut()
            .unwrap()
            .execute_commands(command_buffer)
            .unwrap();
    }

    #[allow(dead_code)]
    #[inline]
    pub fn viewport_dimensions(&self) -> [u32; 2] {
        self.frame.framebuffer.extent()
    }

    #[allow(dead_code)]
    #[inline]
    pub fn world_to_framebuffer_matrix(&self) -> Matrix4<f32> {
        self.frame.world_to_framebuffer
    }
}
