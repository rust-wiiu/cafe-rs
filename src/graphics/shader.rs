use crate::prelude::*;
use std::marker::PhantomData;
use sys::gx2;

use super::buffer::{ShaderProgram, VertexBuffer};

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
                program: ShaderProgram::from(&value.program),
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
                program: ShaderProgram::from(&value.program),
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
                program.as_raw().ptr,
                attribs.len() as u32,
                attribs.as_ptr(),
                gx2::shader::FetchShaderType::None,
                gx2::shader::TessellationMode::Discrete,
            );
        });

        // invalidate

        Self {
            _data: program,
            shader,
        }
    }

    pub fn as_raw(&self) -> &gx2::shader::FetchShader {
        &self.shader
    }
}

pub trait FormatList {
    type Streams: AsRef<[gx2::shader::AttribStream]>;
}

pub trait AttributeList {
    type Formats: FormatList;
    fn into_streams(self) -> <Self::Formats as FormatList>::Streams;
}

pub trait BufferList {
    type Formats: FormatList;
    type Bindings: IntoIterator<Item = (*const sys::gx2::mem::Buffer, AttributeBinding)>;

    fn bindings(&self, streams: &<Self::Formats as FormatList>::Streams) -> Self::Bindings;
}

pub struct AttributeBinding {
    pub slot: u32,
    pub stride: u32,
    pub offset: u32,
}

pub struct ShaderGroup<Formats: FormatList> {
    pub(crate) vertex: VertexShader,
    pub(crate) pixel: PixelShader,
    pub(crate) fetch: FetchShader,
    pub(crate) attributes: Formats::Streams,
}

impl<Formats: FormatList> ShaderGroup<Formats> {
    pub fn new<A: AttributeList<Formats = Formats>>(
        vertex: impl Into<VertexShader>,
        pixel: impl Into<PixelShader>,
        attributes: A,
    ) -> Self {
        let attributes = attributes.into_streams();

        let fetch = FetchShader::new(attributes.as_ref());

        Self {
            vertex: vertex.into(),
            pixel: pixel.into(),
            fetch,
            attributes,
        }
    }

    pub fn foo<B: BufferList<Formats = Formats>>(&self, _buffers: B) {
        todo!()
    }
}

macro_rules! impl_format_lists {
    ($N:literal, $(($Ti:ident, $i:tt)),+) => {
        impl<$($Ti: AttributeFormat),+> FormatList for ($($Ti,)+) {
            type Streams = [gx2::shader::AttribStream; $N];
        }

        impl<$($Ti: AttributeFormat),+> AttributeList for ($(Attribute<$Ti>,)+) {
            type Formats = ($($Ti,)+);

            fn into_streams(self) -> [gx2::shader::AttribStream; $N] {
                [$(self.$i.0),+]
            }
        }

        impl<'a, $($Ti: AttributeFormat),+> BufferList for ($(&'a VertexBuffer<$Ti>,)+) {
            type Formats = ($($Ti,)+);
            type Bindings = [(*const sys::gx2::mem::Buffer, AttributeBinding); $N];

            fn bindings(&self, streams: &[gx2::shader::AttribStream; $N]) -> Self::Bindings {
                [
                    $(
                        (
                            self.$i.as_raw(),
                            AttributeBinding {
                                slot: streams[$i].location,
                                stride: std::mem::size_of::<$Ti>() as u32,
                                offset: streams[$i].offset,
                            }
                        )
                    ),+
                ]
            }
        }
    }
}

impl_format_lists!(1, (T0, 0));
impl_format_lists!(2, (T0, 0), (T1, 1));
impl_format_lists!(3, (T0, 0), (T1, 1), (T2, 2));
impl_format_lists!(4, (T0, 0), (T1, 1), (T2, 2), (T3, 3));
// …

pub trait AttributeFormat {
    const FORMAT: gx2::shader::AttribFormat;
}

impl AttributeFormat for (f32, f32) {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Float32_32;
}

impl AttributeFormat for (f32, f32, f32) {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Float32_32_32;
}

impl AttributeFormat for (f32, f32, f32, f32) {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Float32_32_32_32;
}

impl AttributeFormat for [f32; 4] {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Float32_32_32_32;
}

impl AttributeFormat for u32 {
    const FORMAT: gx2::shader::AttribFormat = gx2::shader::AttribFormat::Uint32;
}

pub struct Attribute<T>(gx2::shader::AttribStream, PhantomData<T>);

impl<T: AttributeFormat> Attribute<T> {
    pub fn location(location: usize) -> Self {
        Self(
            gx2::shader::AttribStream {
                location: location as u32,
                buffer: location as u32,
                offset: 0,
                format: T::FORMAT,
                index_type: gx2::shader::AttribIndexType::PerVertex,
                alu_divisor: 0,
                mask: gx2::shader::ComponentSelection::from(T::FORMAT),
                endian_swap: gx2::shader::EndianSwapMode::Default,
            },
            PhantomData,
        )
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.0.offset = offset as u32;
        self
    }
}
