// Keeps the top-ten scoreboard string in the browser's localStorage.
miniquad_add_plugin({
    name: "top_ten",
    version: 1,
    register_plugin: function (importObject) {
        importObject.env.top_ten_read = function (ptr, capacity) {
            var bytes = new TextEncoder().encode(localStorage.getItem("top_ten") || "");
            var length = Math.min(bytes.length, capacity);
            new Uint8Array(wasm_memory.buffer, ptr, length).set(bytes.subarray(0, length));
            return length;
        };
        importObject.env.top_ten_write = function (ptr, length) {
            var bytes = new Uint8Array(wasm_memory.buffer, ptr, length);
            localStorage.setItem("top_ten", new TextDecoder().decode(bytes));
        };
    }
});
