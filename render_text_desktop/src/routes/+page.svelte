<script lang="ts">
    import { invoke, Channel } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    type PixelMap = number[][];

    let loadedFonts = $state<string[]>([]);
    let selectedFont = $state("");
    let glyphPixmaps = $state<PixelMap[]>([]);

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

                // Add the new font to the list if not already present
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
            glyphPixmaps = [...glyphPixmaps, pixmap];
        };

        try {
            await invoke("rasterize_glyphs", {
                fontName: selectedFont,
                startingGlyph: 20,
                glyphCount: 100,
                glyphWidth: 48.0,
                glyphHeight: 48.0,
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
