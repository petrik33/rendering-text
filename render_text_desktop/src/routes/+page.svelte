<script lang="ts">
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { centerPixmap } from "$lib/utils/pixmap";
    import { onMount } from "svelte";

    type PixelMap = number[][];

    let loadedFonts = $state<string[]>([]);
    let selectedFont = $state("");
    let glyphPixmaps = $state<PixelMap[]>([]);

    // Add new state variables for the controls
    let startingGlyph = $state(65);
    let glyphCount = $state(26);
    let glyphSize = $state(48);
    let currentRenderSize = $state(48);
    let displayScale = $state(1);

    onMount(async () => {
        try {
            const fontName = await invoke<string>("load_font", {
                path: "assets/Roboto-Regular.ttf",
            });

            if (!loadedFonts.includes(fontName)) {
                loadedFonts.push(fontName);
                selectedFont = fontName;
            }
        } catch (error) {
            console.error("Error loading default font:", error);
        }
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

        const channel = new Channel<number[][]>();

        channel.onmessage = async (pixmap: number[][]) => {
            console.log(pixmap);
            pixmap = centerPixmap(pixmap, glyphSize, glyphSize);
            glyphPixmaps.push(pixmap);
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
                    (displayScale = Math.max(displayScale - 0.5, 0.5))}
                disabled={displayScale <= 0.5}
            >
                -
            </button>
            <span>{displayScale}x</span>
            <button
                onclick={() => (displayScale = Math.min(displayScale + 0.5, 4))}
                disabled={displayScale >= 4}
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
        width: 50%;
        display: grid;
        gap: 16px;
        grid-row-gap: 4px;
    }

    #glyph-canvas {
        flex: 1;
        width: 50%;
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
