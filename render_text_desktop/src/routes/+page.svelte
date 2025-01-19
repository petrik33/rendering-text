<script lang="ts">
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";
    import { centerPixmap } from "$lib/utils/pixmap";

    type PixelMap = number[][];

    let loadedFonts = $state<string[]>([]);
    let selectedFont = $state("");
    let glyphPixmaps = $state<PixelMap[]>([]);

    // Add new state variables for the controls
    let startingGlyph = $state(20);
    let glyphCount = $state(100);
    let glyphSize = $state(48);

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

        const channel = new Channel<number[][]>();

        channel.onmessage = (pixmap: number[][]) => {
            console.log(pixmap);
            pixmap = centerPixmap(pixmap, glyphSize, glyphSize);
            glyphPixmaps = [...glyphPixmaps, pixmap];
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

        <button onclick={handleRasterize} disabled={!selectedFont}>
            Rasterize
        </button>
    </div>

    <div class="glyph-grid">
        {#each glyphPixmaps as pixmap}
            <div class="glyph-card">
                <div
                    class="pixel-grid"
                    style:grid-template-columns="repeat({pixmap[0].length}, 1fr)"
                    style:grid-template-rows="repeat({pixmap.length}, 1fr)"
                >
                    {#each pixmap as row}
                        {#each row as pixel}
                            <div
                                class="pixel"
                                style:background-color="rgb({pixel},{pixel},{pixel})"
                            ></div>
                        {/each}
                    {/each}
                </div>
            </div>
        {/each}
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

    .glyph-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
        gap: 16px;
    }

    .glyph-card {
        aspect-ratio: 1;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .pixel-grid {
        width: 100%;
        height: 100%;
        display: grid;
    }

    .pixel {
        width: 100%;
        height: 100%;
    }
</style>
