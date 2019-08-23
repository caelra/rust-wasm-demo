const rust = import('./rust_wasm')

rust.then(func =>{
    func.dimmadome()
    func.greet("Ukko")
})