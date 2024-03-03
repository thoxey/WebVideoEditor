
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub(crate) struct ContextService {
    context: WebGl2RenderingContext,
}