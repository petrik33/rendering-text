precision mediump float;
uniform sampler2D u_texture;
uniform vec2 u_gridSize;
uniform vec2 u_glyphIndex;
varying vec2 v_texCoord;

void main() {
    vec2 finalTexCoord = (v_texCoord + u_glyphIndex) / u_gridSize;
    gl_FragColor = texture2D(u_texture, finalTexCoord);
}
