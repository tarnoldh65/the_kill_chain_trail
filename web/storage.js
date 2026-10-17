// Keeps small text values, like the top ten and the options, in the browser's localStorage.
miniquad_add_plugin({
    name: "storage",
    version: 1,
    register_plugin: function (importObject) {
        function key(ptr, length) {
            return new TextDecoder().decode(new Uint8Array(wasm_memory.buffer, ptr, length));
        }
        importObject.env.storage_read = function (keyPtr, keyLength, ptr, capacity) {
            var bytes = new TextEncoder().encode(localStorage.getItem(key(keyPtr, keyLength)) || "");
            var length = Math.min(bytes.length, capacity);
            new Uint8Array(wasm_memory.buffer, ptr, length).set(bytes.subarray(0, length));
            return length;
        };
        importObject.env.storage_write = function (keyPtr, keyLength, ptr, length) {
            var bytes = new Uint8Array(wasm_memory.buffer, ptr, length);
            localStorage.setItem(key(keyPtr, keyLength), new TextDecoder().decode(bytes));
        };
    }
});
