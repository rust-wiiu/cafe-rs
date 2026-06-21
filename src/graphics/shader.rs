use crate::prelude::*;
use num_enum::IntoPrimitive;
use sys::gx2;

use super::{buffer::ShaderProgram, types::AttributeFormat};

pub struct VertexShader {
    _data: OwnedVertexData,
    shader: gx2::shader::VertexShader,
}

struct OwnedVertexData {
    _strings: Box<[std::ffi::CString]>,
    uniform_blocks: Box<[gx2::shader::UniformBlock]>,
    uniform_vars: Box<[gx2::shader::UniformVar]>,
    initial_values: Box<[gx2::shader::UniformInitialValue]>,
    loop_vars: Box<[gx2::shader::LoopVar]>,
    sampler_vars: Box<[gx2::shader::SamplerVar]>,
    attrib_vars: Box<[gx2::shader::AttribVar]>,
    program: ShaderProgram,
}

impl VertexShader {
    pub fn as_raw(&self) -> &gx2::shader::VertexShader {
        &self.shader
    }
}

impl From<gfx2::VertexShader> for VertexShader {
    fn from(value: gfx2::VertexShader) -> Self {
        let owned = {
            let mut strings = Vec::new();

            let uniform_blocks = {
                let mut vec = Vec::new();

                for x in value.uniform_blocks.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::UniformBlock {
                        name: strings.last().unwrap().as_ptr(),
                        location: x.location,
                        size: x.size,
                    });
                }

                vec.into_boxed_slice()
            };

            let uniform_vars = {
                let mut vec = Vec::new();

                for x in value.uniform_vars.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::UniformVar {
                        name: strings.last().unwrap().as_ptr(),
                        r#type: (x.ty as u32).try_into().unwrap(),
                        count: x.count,
                        offset: x.offset,
                        index: x.index,
                    });
                }

                vec.into_boxed_slice()
            };

            let initial_values = {
                let mut vec = Vec::new();

                for x in value.initial_values.into_iter() {
                    vec.push(gx2::shader::UniformInitialValue {
                        value: x.value,
                        offset: x.offset,
                    });
                }

                vec.into_boxed_slice()
            };

            let loop_vars = {
                let mut vec = Vec::new();

                for x in value.loop_vars.into_iter() {
                    vec.push(gx2::shader::LoopVar {
                        offset: x.offset,
                        value: x.value,
                    });
                }

                vec.into_boxed_slice()
            };

            let sampler_vars = {
                let mut vec = Vec::new();

                for x in value.sampler_vars.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::SamplerVar {
                        name: strings.last().unwrap().as_ptr(),
                        r#type: (x.ty as u32).try_into().unwrap(),
                        location: x.location,
                    });
                }

                vec.into_boxed_slice()
            };

            let attrib_vars = {
                let mut vec = Vec::new();

                for x in value.attrib_vars.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::AttribVar {
                        name: strings.last().unwrap().as_ptr(),
                        r#type: (x.ty as u32).try_into().unwrap(),
                        count: x.count,
                        location: x.location,
                    });
                }

                vec.into_boxed_slice()
            };

            OwnedVertexData {
                _strings: strings.into_boxed_slice(),
                uniform_blocks,
                uniform_vars,
                initial_values,
                loop_vars,
                sampler_vars,
                attrib_vars,
                program: ShaderProgram::from(value.program),
            }
        };

        let shader = gx2::shader::VertexShader {
            regs: value.regs.into(),
            shader_size: owned.program.len() as u32,
            shader_ptr: owned.program.as_raw().ptr,
            shader_mode: (value.mode as u32).try_into().unwrap(),
            num_uniform_blocks: owned.uniform_blocks.len() as u32,
            uniform_blocks: owned.uniform_blocks.as_ptr(),
            num_uniforms: owned.uniform_vars.len() as u32,
            uniform_vars: owned.uniform_vars.as_ptr(),
            num_initial_values: owned.initial_values.len() as u32,
            initial_values: owned.initial_values.as_ptr(),
            num_loops: owned.loop_vars.len() as u32,
            loop_vars: owned.loop_vars.as_ptr(),
            num_samplers: owned.sampler_vars.len() as u32,
            sampler_vars: owned.sampler_vars.as_ptr(),
            num_attribs: owned.attrib_vars.len() as u32,
            attrib_vars: owned.attrib_vars.as_ptr(),
            ring_itemsize: value.ring_item_size,
            has_stream_output: value.has_stream_out as i32,
            stream_out_vertex_stride: value.stream_out_stride,
            program: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        };

        Self {
            _data: owned,
            shader,
        }
    }
}

pub struct PixelShader {
    _data: OwnedPixelData,
    shader: gx2::shader::PixelShader,
}

struct OwnedPixelData {
    _strings: Box<[std::ffi::CString]>,
    uniform_blocks: Box<[gx2::shader::UniformBlock]>,
    uniform_vars: Box<[gx2::shader::UniformVar]>,
    initial_values: Box<[gx2::shader::UniformInitialValue]>,
    loop_vars: Box<[gx2::shader::LoopVar]>,
    sampler_vars: Box<[gx2::shader::SamplerVar]>,
    program: ShaderProgram,
}

impl PixelShader {
    pub fn as_raw(&self) -> &gx2::shader::PixelShader {
        &self.shader
    }
}

impl From<gfx2::PixelShader> for PixelShader {
    fn from(value: gfx2::PixelShader) -> Self {
        let owned = {
            let mut strings = Vec::new();

            let uniform_blocks = {
                let mut vec = Vec::new();

                for x in value.uniform_blocks.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::UniformBlock {
                        name: strings.last().unwrap().as_ptr(),
                        location: x.location,
                        size: x.size,
                    });
                }

                vec.into_boxed_slice()
            };

            let uniform_vars = {
                let mut vec = Vec::new();

                for x in value.uniform_vars.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::UniformVar {
                        name: strings.last().unwrap().as_ptr(),
                        r#type: (x.ty as u32).try_into().unwrap(),
                        count: x.count,
                        offset: x.offset,
                        index: x.index,
                    });
                }

                vec.into_boxed_slice()
            };

            let initial_values = {
                let mut vec = Vec::new();

                for x in value.initial_values.into_iter() {
                    vec.push(gx2::shader::UniformInitialValue {
                        value: x.value,
                        offset: x.offset,
                    });
                }

                vec.into_boxed_slice()
            };

            let loop_vars = {
                let mut vec = Vec::new();

                for x in value.loop_vars.into_iter() {
                    vec.push(gx2::shader::LoopVar {
                        offset: x.offset,
                        value: x.value,
                    });
                }

                vec.into_boxed_slice()
            };

            let sampler_vars = {
                let mut vec = Vec::new();

                for x in value.sampler_vars.into_iter() {
                    strings.push(std::ffi::CString::new(x.name).unwrap());

                    vec.push(gx2::shader::SamplerVar {
                        name: strings.last().unwrap().as_ptr(),
                        r#type: (x.ty as u32).try_into().unwrap(),
                        location: x.location,
                    });
                }

                vec.into_boxed_slice()
            };

            OwnedPixelData {
                _strings: strings.into_boxed_slice(),
                uniform_blocks,
                uniform_vars,
                initial_values,
                loop_vars,
                sampler_vars,
                program: ShaderProgram::from(value.program),
            }
        };

        let shader = gx2::shader::PixelShader {
            regs: value.regs.into(),
            shader_size: owned.program.len() as u32,
            shader_ptr: owned.program.as_raw().ptr,
            shader_mode: (value.mode as u32).try_into().unwrap(),
            num_uniform_blocks: owned.uniform_blocks.len() as u32,
            uniform_blocks: owned.uniform_blocks.as_ptr(),
            num_uniforms: owned.uniform_vars.len() as u32,
            uniform_vars: owned.uniform_vars.as_ptr(),
            num_initial_values: owned.initial_values.len() as u32,
            initial_values: owned.initial_values.as_ptr(),
            num_loops: owned.loop_vars.len() as u32,
            loop_vars: owned.loop_vars.as_ptr(),
            num_samplers: owned.sampler_vars.len() as u32,
            sampler_vars: owned.sampler_vars.as_ptr(),
            program: unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        };

        Self {
            _data: owned,
            shader,
        }
    }
}

pub struct FetchShader {
    _data: ShaderProgram,
    shader: gx2::shader::FetchShader,
}

impl FetchShader {
    pub fn new(attribs: &[gx2::shader::AttribStream]) -> Self {
        let size = unsafe {
            gx2::shader::fetch_shader_program_size(
                attribs.len() as u32,
                gx2::shader::FetchShaderType::None,
                gx2::shader::TessellationMode::Discrete,
            )
        };

        let program = ShaderProgram::with_capacity(size as usize);

        let shader = gx2::shader::FetchShader::init(|shader| unsafe {
            gx2::shader::init_fetch_shader_ex(
                shader,
                program.lock().as_mut_ptr().cast(),
                attribs.len() as u32,
                attribs.as_ptr(),
                gx2::shader::FetchShaderType::None,
                gx2::shader::TessellationMode::Discrete,
            );
        });

        // unsafe {
        //     gx2::mem::invalidate(
        //         gx2::mem::Invalidate::Shader | gx2::mem::Invalidate::Cpu,
        //         shader.program.cast_mut(),
        //         size as u32,
        //     );
        // }

        Self {
            _data: program,
            shader,
        }
    }

    pub fn as_raw(&self) -> &gx2::shader::FetchShader {
        &self.shader
    }
}

pub struct ShaderGroup<A> {
    pub(crate) vertex: VertexShader,
    pub(crate) pixel: PixelShader,
    pub(crate) fetch: FetchShader,
    pub attrs: A,
}

#[repr(u32)]
#[derive(Debug, IntoPrimitive)]
pub enum Stream {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
}

#[repr(transparent)]
pub struct Attribute(pub(crate) gx2::shader::AttribStream);

impl Attribute {
    pub const fn new<T: AttributeFormat>(location: usize, stream: Stream) -> Self {
        Self(gx2::shader::AttribStream {
            location: location as u32,
            buffer: stream as u32,
            offset: 0,
            format: T::FORMAT,
            index_type: gx2::shader::AttribIndexType::PerVertex,
            alu_divisor: 0,
            mask: gx2::shader::ComponentSelection::default_for(T::FORMAT),
            endian_swap: gx2::shader::EndianSwapMode::Default,
        })
    }

    pub const fn offset(mut self, offset: usize) -> Self {
        self.0.offset = offset as u32;
        self
    }

    // add missing methods
}

impl<A: AsRef<[Attribute]>> ShaderGroup<A> {
    pub fn new(
        vertex: impl Into<VertexShader>,
        pixel: impl Into<PixelShader>,
        attributes: A,
    ) -> Self {
        Self {
            vertex: vertex.into(),
            pixel: pixel.into(),
            fetch: FetchShader::new(
                // SAFETY: Attribute is transparent to gx2::shader::AttribStream
                unsafe {
                    std::slice::from_raw_parts(
                        attributes.as_ref().as_ptr() as *const gx2::shader::AttribStream,
                        attributes.as_ref().len(),
                    )
                },
            ),
            attrs: attributes,
        }
    }
}
