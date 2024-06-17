const worker = new Worker(new URL('../workers/heavyComputation.js', import.meta.url), { type: 'module' });

export function heavyComputation(data) {
    return new Promise((resolve, reject) => {
        worker.onmessage = (event) => {
            if (event.data.error) {
                reject(event.data.error);
            } else {
                resolve(event.data);
            }
        };
        worker.postMessage(data);
    });
}
