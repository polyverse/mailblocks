import * as wasm from "mailblocks-lib";

document.getElementById("input-form").addEventListener('submit', (event) => {
    event.preventDefault();
    let msg = document.getElementById("msg").value;
    let hash = wasm.generate_mailblock(msg, 2, 5);
    document.getElementById("hash-output").innerHTML = JSON.stringify(hash);
});
