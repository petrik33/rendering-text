<script lang="ts">
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { centerPixmap } from "$lib/utils/pixmap";
    import { onMount } from "svelte";
    import vertexShaderSource from "./shaders/glyph/vertex.glsl?raw";
    import fragmentShaderSource from "./shaders/glyph/fragment.glsl?raw";

    type PixelMap = number[][];

    let loadedFonts = $state<string[]>([]);
    let selectedFont = $state("");
    let glyphPixmaps = $state<PixelMap[]>([]);

    let startingGlyph = $state(65);
    let glyphCount = $state(26);
    let glyphSize = $state(48);
    let currentRenderSize = $state(48);
    let displayScale = $state(1);
    let displayScaleInput = $state(1);

    let canvas: HTMLCanvasElement;
    let gl: WebGLRenderingContext;

    async function renderGlyphsWebGL(pixmaps: PixelMap[]) {
        const VERTEX_SIZE = 4;
        const FLOAT_SIZE = 4;
        const STRIDE = VERTEX_SIZE * FLOAT_SIZE;
        const spacing = 8;

        if (!gl) return;

        function createShader(type: number, source: string) {
            const shader = gl.createShader(type)!;
            gl.shaderSource(shader, source);
            gl.compileShader(shader);
            if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
                throw new Error(
                    `Shader compile error: ${gl.getShaderInfoLog(shader)}`,
                );
            }
            return shader;
        }

        const vertexShader = createShader(gl.VERTEX_SHADER, vertexShaderSource);
        const fragmentShader = createShader(
            gl.FRAGMENT_SHADER,
            fragmentShaderSource,
        );

        const program = gl.createProgram()!;
        if (!program) {
            throw new Error("Failed to create WebGL program");
        }

        gl.attachShader(program, vertexShader);
        gl.attachShader(program, fragmentShader);
        gl.linkProgram(program);

        if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
            throw new Error(
                `Program link error: ${gl.getProgramInfoLog(program)}`,
            );
        }

        const pixelGlyphSize = glyphSize * displayScale;
        const glyphAdvance = pixelGlyphSize + spacing * 2;

        let width = canvas.clientWidth;

        const gridCols = Math.floor(width / glyphAdvance);
        const gridRows = Math.ceil(pixmaps.length / gridCols);

        let height = gridRows * glyphAdvance;

        canvas.width = width;
        canvas.height = height;
        canvas.style.height = `${height}px`;

        const textureWidth = glyphSize * gridCols;
        const textureHeight = glyphSize * gridRows;
        const textureData = new Uint8Array(textureWidth * textureHeight);

        pixmaps.forEach((pixmap, index) => {
            const gridX = (index % gridCols) * glyphSize;
            const gridY = Math.floor(index / gridCols) * glyphSize;

            for (let y = 0; y < glyphSize; y++) {
                for (let x = 0; x < glyphSize; x++) {
                    const texIndex = (gridY + y) * textureWidth + (gridX + x);
                    textureData[texIndex] = pixmap[y][x];
                }
            }
        });

        // Create and set up texture
        const texture = gl.createTexture();
        if (!texture) {
            throw new Error("Failed to create texture");
        }

        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texImage2D(
            gl.TEXTURE_2D,
            0,
            gl.LUMINANCE,
            textureWidth,
            textureHeight,
            0,
            gl.LUMINANCE,
            gl.UNSIGNED_BYTE,
            textureData,
        );

        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

        // Create vertex buffer
        const vertexBuffer = gl.createBuffer();
        if (!vertexBuffer) {
            throw new Error("Failed to create vertex buffer");
        }

        // Set up attributes
        const positionLocation = gl.getAttribLocation(program, "a_position");
        const texCoordLocation = gl.getAttribLocation(program, "a_texCoord");

        gl.enableVertexAttribArray(positionLocation);
        gl.enableVertexAttribArray(texCoordLocation);

        // Setup render state
        gl.viewport(0, 0, canvas.width, canvas.height);
        gl.clearColor(0, 0, 0, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);

        gl.useProgram(program);

        const textureLocation = gl.getUniformLocation(program, "u_texture");
        gl.uniform1i(textureLocation, 0);

        const glyphWidthClipSpace = (glyphAdvance / canvas.width) * 2;
        const glyphHeightClipSpace = (glyphAdvance / canvas.height) * 2;

        const texWidth = 1.0 / gridCols;
        const texHeight = 1.0 / gridRows;

        for (let i = 0; i < pixmaps.length; i++) {
            const gridX = i % gridCols;
            const gridY = Math.floor(i / gridCols);

            const pixelX = gridX * glyphAdvance;
            const pixelY = gridY * glyphAdvance;

            const x = (pixelX / canvas.width) * 2 - 1;
            const y = 1 - (2 * pixelY) / canvas.height;

            // Calculate texture coordinates for this glyph
            const texX = gridX / gridCols;
            const texY = gridY / gridRows;

            const glyphVertices = new Float32Array([
                // Position (x,y)    // Texcoord (u,v)
                x,
                y - glyphHeightClipSpace,
                texX,
                texY + texHeight, // bottom-left
                x + glyphWidthClipSpace,
                y - glyphHeightClipSpace,
                texX + texWidth,
                texY + texHeight, // bottom-right
                x + glyphWidthClipSpace,
                y,
                texX + texWidth,
                texY, // top-right
                x,
                y,
                texX,
                texY, // top-left
            ]);

            gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer);
            gl.bufferData(gl.ARRAY_BUFFER, glyphVertices, gl.STATIC_DRAW);

            gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 16, 0);
            gl.vertexAttribPointer(texCoordLocation, 2, gl.FLOAT, false, 16, 8);

            gl.drawArrays(gl.TRIANGLE_FAN, 0, 4);
        }

        // Cleanup
        gl.deleteBuffer(vertexBuffer);
        gl.deleteTexture(texture);
        gl.deleteShader(vertexShader);
        gl.deleteShader(fragmentShader);
        gl.deleteProgram(program);
    }

    onMount(() => {
        let resizeTimeout: number;
        let resizeObserver: ResizeObserver | null = null;
        const resizeRerenderDebounceMs = 250;

        async function initializeWebGL() {
            canvas = document.getElementById(
                "glyph-canvas",
            ) as HTMLCanvasElement;
            gl = canvas.getContext("webgl") as WebGLRenderingContext;

            if (!gl) {
                throw new Error("WebGL not supported");
            }
        }

        async function loadDefaultFont() {
            const fontName = await invoke<string>("load_font", {
                path: "assets/Roboto-Regular.ttf",
            });

            if (!loadedFonts.includes(fontName)) {
                loadedFonts.push(fontName);
                selectedFont = fontName;
            }
        }

        function observeWindowResize() {
            resizeObserver = new ResizeObserver((entries) => {
                for (const entry of entries) {
                    if (entry.target === canvas && glyphPixmaps.length > 0) {
                        window.clearTimeout(resizeTimeout);
                        resizeTimeout = window.setTimeout(async () => {
                            await renderGlyphsWebGL(glyphPixmaps);
                        }, resizeRerenderDebounceMs);
                    }
                }
            });

            resizeObserver.observe(canvas);
        }

        async function initialize() {
            try {
                await initializeWebGL();
                await loadDefaultFont();
                observeWindowResize();
            } catch (error) {
                console.error("Error loading default font:", error);
            }
        }

        initialize();

        return () => {
            window.clearTimeout(resizeTimeout);
            resizeObserver?.disconnect();
        };
    });

    async function handleLoadFont() {
        try {
            const selected = await open({
                filters: [
                    {
                        name: "Font File",
                        extensions: ["ttf"],
                    },
                ],
            });

            if (selected) {
                const fontName = await invoke("load_font", {
                    path: selected,
                });

                if (!loadedFonts.includes(fontName as string)) {
                    loadedFonts.push(fontName as string);
                }
            }
        } catch (error) {
            console.error("Error loading font:", error);
        }
    }

    function handleFontSelect(event: Event) {
        const target = event.target as HTMLSelectElement;
        selectedFont = target.value;
    }

    async function handleRasterize() {
        if (!selectedFont) return;

        glyphPixmaps = [];
        currentRenderSize = glyphSize;
        displayScale = displayScaleInput;

        const channel = new Channel<number[][]>();

        channel.onmessage = async (pixmap: number[][]) => {
            pixmap = centerPixmap(pixmap, glyphSize, glyphSize);
            glyphPixmaps.push(pixmap);

            if (glyphPixmaps.length === glyphCount) {
                await renderGlyphsWebGL(glyphPixmaps);
            }
        };

        try {
            invoke("rasterize_glyphs", {
                fontName: selectedFont,
                startingGlyph,
                glyphCount,
                glyphWidth: glyphSize,
                glyphHeight: glyphSize,
                channel,
            });
        } catch (error) {
            console.error("Error rasterizing glyphs:", error);
        }
    }
</script>

<main>
    <div class="controls">
        <button onclick={handleLoadFont}> Load Font </button>

        <select
            value={selectedFont}
            onchange={handleFontSelect}
            disabled={loadedFonts.length === 0}
        >
            <option value="" disabled>Select a font</option>
            {#each loadedFonts as font}
                <option value={font}>{font}</option>
            {/each}
        </select>

        <div class="number-input">
            <label for="starting-glyph">Start at:</label>
            <input
                type="number"
                id="starting-glyph"
                min="0"
                bind:value={startingGlyph}
            />
        </div>

        <div class="number-input">
            <label for="glyph-count">Count:</label>
            <input
                type="number"
                id="glyph-count"
                min="1"
                max="1000"
                bind:value={glyphCount}
            />
        </div>

        <div class="number-input">
            <label for="glyph-size">Size:</label>
            <input
                type="number"
                id="glyph-size"
                min="8"
                max="128"
                step="8"
                bind:value={glyphSize}
            />
        </div>

        <div class="scale-controls">
            <button
                onclick={() =>
                    (displayScaleInput = Math.max(
                        displayScaleInput - 0.5,
                        0.5,
                    ))}
                disabled={displayScaleInput <= 0.5}
            >
                -
            </button>
            <span>{displayScaleInput}x</span>
            <button
                onclick={() =>
                    (displayScaleInput = Math.min(displayScaleInput + 0.5, 4))}
                disabled={displayScaleInput >= 4}
            >
                +
            </button>
        </div>

        <button onclick={handleRasterize} disabled={!selectedFont}>
            Rasterize
        </button>
    </div>

    <div class="glyph-container">
        <div
            class="glyph-grid"
            style:grid-template-columns="repeat(auto-fill, {currentRenderSize *
                1.2 *
                displayScale}px)"
        >
            {#each glyphPixmaps as pixmap}
                <div
                    class="glyph-card"
                    style:width="{currentRenderSize * 1.2 * displayScale}px"
                    style:height="{currentRenderSize * 1.2 * displayScale}px"
                >
                    <div
                        class="pixel-grid"
                        style:width="{currentRenderSize * displayScale}px"
                        style:height="{currentRenderSize * displayScale}px"
                        style:grid-template-columns="repeat({pixmap[0].length}, {displayScale}px)"
                        style:grid-template-rows="repeat({pixmap.length}, {displayScale}px)"
                    >
                        {#each pixmap as row}
                            {#each row as pixel}
                                <div
                                    class="pixel"
                                    style:background-color="rgb({pixel},{pixel},{pixel})"
                                    style:width="{displayScale}px"
                                    style:height="{displayScale}px"
                                ></div>
                            {/each}
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
        <canvas id="glyph-canvas"></canvas>
    </div>
</main>

<style>
    main {
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 24px;
    }

    .controls {
        display: flex;
        gap: 16px;
        align-items: center;
        flex-wrap: wrap;
    }

    .number-input {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .number-input input {
        width: 70px;
        padding: 8px;
    }

    .number-input label {
        font-size: 14px;
        white-space: nowrap;
    }

    button {
        padding: 8px 16px;
        cursor: pointer;
    }

    button:disabled {
        cursor: not-allowed;
        opacity: 0.6;
    }

    select {
        padding: 8px;
        width: 200px;
    }

    .glyph-container {
        display: flex;
        width: 100%;
    }

    .glyph-grid {
        flex: 1;
        min-width: 0;
        display: grid;
        gap: 16px;
        grid-row-gap: 4px;
    }

    #glyph-canvas {
        flex: 1;
        min-width: 0;
    }

    .glyph-card {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
        box-sizing: border-box;
    }

    .pixel-grid {
        display: grid;
    }

    .pixel {
        width: 1px;
        height: 1px;
    }
</style>
