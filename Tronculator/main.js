import init, { start } from "./pkg/Tronculator.js";

async function main() {
    await init();
    start();
}

main().catch((error) => {
    console.error("Failed to initialize:", error);
});