import * as wasm from "wasm-cache"

let config = new wasm.CacheConfig(3, "asset_lifetime")
let handler = wasm.create(config)

let res = handler.encrypt("lalala")
console.log("encrypt result", res)

res = handler.decrypt("alalal")
console.log("decrypt result", res)

let asset = wasm.check_asset("asset_url")
console.log("check asset result:", asset)

wasm.delete_asset("asset_url")