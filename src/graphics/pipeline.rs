use gfx;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub fn create_pipeline() -> Result<>{
    /*factory.create_pipeline_simple(
            include_bytes!("shader/myshader_150.glslv"),
            include_bytes!("shader/myshader_150.glslf"),
            pipe::new()
        )?*/
}