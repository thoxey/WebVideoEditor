mod utils;
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject};
use std::sync::Mutex;
use once_cell::sync::Lazy;

//This came from the boilerplate from the wasm-pack-template
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct WebGLResources {
    context: WebGl2RenderingContext,
    vao: Option<WebGlVertexArrayObject>,
    program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl WebGLResources {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<WebGLResources, JsValue> {
        let context = get_context(canvas_id)?;
        Ok(WebGLResources {
            context,
            vao: None,
            program: None,
        })
    }

    pub fn setup_resources(&mut self) -> Result<(), JsValue> {
        // Example of setting up a VAO and a shader program
        let vert_shader = compile_shader(
            &self.context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
            in vec4 position;
            void main() {
                gl_Position = position;
            }
            "##,
        )?;

        let frag_shader = compile_shader(
            &self.context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
        precision highp float;
        out vec4 outColor;
        void main() {
            outColor = vec4(1, 1, 1, 1); // White color
        }
        "##,
        )?;

        let program = link_program(&self.context, &vert_shader, &frag_shader)?;
        self.context.use_program(Some(&program));

        // Square vertices and indices
        let vertices: [f32; 12] = [
            -0.5, 0.5, 0.0,  // Top Left
            -0.5, -0.5, 0.0, // Bottom Left
            0.5, -0.5, 0.0,  // Bottom Right
            0.5, 0.5, 0.0,   // Top Right
        ];
        let indices: [u16; 6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        // Vertex buffer
        let vertex_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe
            {
                let vert_array = js_sys::Float32Array::view(&vertices);
                self.context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &vert_array, WebGl2RenderingContext::STATIC_DRAW);
            }

        // Index buffer
        let index_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        unsafe
            {
                let index_array = js_sys::Uint16Array::view(&indices);
                self.context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &index_array, WebGl2RenderingContext::STATIC_DRAW);
            }

        // Vertex Array Object (VAO)
        let vao = self.context.create_vertex_array().ok_or("Could not create vertex array object")?;
        self.context.bind_vertex_array(Some(&vao));

        // Position attribute setup
        let position_attribute_location = self.context.get_attrib_location(&program, "position");
        if position_attribute_location == -1
        {
            return Err(JsValue::from_str("Position attribute not found"));
        }

        self.context.enable_vertex_attrib_array(position_attribute_location as u32);
        self.context.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

        self.vao = Some(vao);
        self.program = Some(program);

        Ok(())
    }

    pub fn draw(&self) -> Result<(), JsValue>
    {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        if let Some(program) = &self.program
        {
            self.context.use_program(Some(program));
        }

        if let Some(vao) = &self.vao
        {
            self.context.bind_vertex_array(Some(vao));
            self.context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, 6, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
        }

        let error = self.context.get_error();
        if error != WebGl2RenderingContext::NO_ERROR
        {
            return Err(JsValue::from_str(&format!("WebGL error: {}", error)));
        }
        else
        {
            Ok(())
        }
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Bonjour!");
}

fn get_context(canvas_id: &str) -> Result<WebGl2RenderingContext, JsValue>
{
    let window = web_sys::window().ok_or("should have a Window")?;
    let document = window.document().ok_or("should have a Document")?;
    let canvas = document.get_element_by_id(canvas_id).ok_or("should have a canvas element")?;
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<WebGl2RenderingContext>()?;
    Ok(context)
}

pub fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str, ) -> Result<WebGlShader, String>
{
    let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false)
    {
        Ok(shader)
    }
    else
    {
        Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(context: &WebGl2RenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader, ) -> Result<WebGlProgram, String>
{
    let program = context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false)
    {
        Ok(program)
    }
    else
    {
        Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}