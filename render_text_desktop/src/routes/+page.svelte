<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    let fonts: string[] = [];
    let selectedFont: string = "";
    let userText: string = "Sample Text";

    async function loadFonts(): Promise<void> {
        fonts = await invoke<string[]>("list_fonts");
    }

    async function importFont(): Promise<void> {
        const path = await open({
            filters: [{ name: "Font Files", extensions: ["ttf"] }],
        });
        if (path) {
            await invoke("import_font", { path });
            loadFonts();
        }
    }

    async function render(): Promise<void> {
        const result = await invoke("render_text", {
            text: userText,
            fontName: selectedFont,
        });
        console.log(result);
    }

    loadFonts();
</script>

<div class="input">
    <input type="text" bind:value={userText} on:input={render} />
</div>

<div class="font-list">
    {#each fonts as font}
        <button on:click={() => (selectedFont = font)}>{font}</button>
    {/each}
    <button on:click={importFont}>+</button>
</div>

<style>
    .font-list {
        margin-top: 20px;
        display: flex;
        gap: 10px;
        flex-wrap: wrap;
    }
    .input {
        margin: 20px 0;
    }
</style>
