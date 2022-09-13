

pub static QUAD_VSH_SRC: &str = r#"
    #version 140
    in vec2 position;
    in vec2 uv;
    out vec2 uvi;

    uniform float t;

    void main() {
        uvi = uv;
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

pub static QUAD_FSH_SRC: &str = r#"
    #version 140

    in vec2 uvi;
    out vec4 color;

    uniform sampler2D tex;

    void main() {
        // color = vec4(uvi, 0.0, 1.0);
        color = texture(tex, uvi);
    }
"#;


pub static TRIANGLE_VSH_SRC: &str = r#"
    #version 140
    in vec2 position;
    in vec2 uv;
    out vec2 uvi;

    uniform float t;

    void main() {
        uvi = uv;
        gl_Position = vec4(position + vec2(t,t), 0.0, 1.0);
    }
"#;

pub static TRIANGLE_FSH_SRC: &str = r#"
    #version 140

    in vec2 uvi;
    out vec4 color;

    void main() {
        color = vec4(uvi, 0.0, 1.0);
    }
"#;


pub static WIREFRAME_VSH_SRC: &str = r#"
    #version 140
    in vec3 position;

    void main() {
        gl_Position = vec4(position, 1.0);
    }
"#;

pub static WIREFRAME_FSH_SRC: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
"#;


// 

pub static DIFFUSE_VSH_SRC: &str = r#"
    #version 140
    in vec3 position;

    out float z_itpl;

    void main() {
        z_itpl = 0.5 + position.z/2.0;
        gl_Position = vec4(position, 1.0);
    }
"#;

pub static DIFFUSE_FSH_SRC: &str = r#"
    #version 140

    in float z_itpl;
    out vec4 color;

    void main() {
        color = vec4(z_itpl, z_itpl, z_itpl, 1.0);
    }
"#;