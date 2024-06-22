import { heavy_computation } from 'wasm-rust';

async function initWasm() {
    try {
        if (heavy_computation) {
            self.onmessage = (event) => {
                try {
                    const result = heavy_computation(BigInt(event.data));
                    self.postMessage(result);
                } catch (e) {
                    self.postMessage({ error: e.message });
                }
            };
        } else {
            console.error("Failed to load WebAssembly function heavy_computation");
        }
    } catch (error) {
        console.error("Error initializing WebAssembly module:", error);
    }
}

initWasm().catch((error) => {
    console.error("Error initializing WebAssembly module:", error);
});
