((function(){
  console.log('downloading rust wasm...');
  fetch('./target/wasm32-unknown-unknown/release/web.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes, {}))
  .then(wasmContainer => {
    console.log('downloaded...');
    const {add,subtract,multiply} = wasmContainer.instance.exports;
    console.log('4 + 2 = ', add(4, 2));
    console.log('4 - 2 = ', subtract(4, 2));
    console.log('4 * 2 = ', multiply(4, 2));
  }).catch(err=>console.log(err));
})())


