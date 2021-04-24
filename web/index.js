import init, * as wasm from "./chimp_wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
const TICKS_PER_FRAME = 10
let anim_frame = 0

let canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

let ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

let file_input = document.getElementById("file-input")
let file_input_div = document.getElementById("file-input-div")
let rom_selector = document.getElementById("rom-selector")

async function populate_rom_selector() {
    let file_url = new URL("roms/rom_list.txt",
        import.meta.url)

    fetch(file_url).then(file => file.text()).then(text => {
        let roms = text.split(/\r?\n/)

        for (let i = 0; i < roms.length; i++) {
            if (roms[i] == "") {
                continue
            }
            let r = "roms/" + roms[i]
            let option = document.createElement("option")
            option.setAttribute("value", r)
            option.text = roms[i]
            rom_selector.appendChild(option)
        }
    })
}

populate_rom_selector()

async function run() {
    await init()
    let vm = new wasm.VmWasm()

    document.addEventListener("keydown", function (event) {
        vm.keypress(event, true)
    })

    document.addEventListener("keyup", function (event) {
        vm.keypress(event, false)
    })

    file_input_div.addEventListener("change", function (event) {
        rom_selector.selectedIndex = 0;

        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        let file = event.target.files[0]
        if (!file) {
            alert("failed to read file")
            return
        }

        let reader = new FileReader()
        reader.onload = function () {
            let buffer = reader.result
            const rom = new Uint8Array(buffer)
            vm.reset()
            vm.load_game(rom)
            main_loop(vm)

            console.log(vm)
        }
        reader.readAsArrayBuffer(file)

    }, false)

    rom_selector.addEventListener("change", function (event) {
        reset_input()

        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        let selector_value = event.target.value
        let rom_url = new URL(selector_value,
            import.meta.url)

        fetch(rom_url).then(file => file.arrayBuffer()).then(buffer => {
            const rom = new Uint8Array(buffer)
            console.log(buffer)
            vm.reset()
            vm.load_game(rom)
            main_loop(vm)

            console.log(vm)
        })
    }, false)
}

function reset_input() {
    file_input.remove()
    let new_input = document.createElement('input')
    new_input.setAttribute("type", "file")
    new_input.setAttribute("id", "file-input")
    file_input_div.appendChild(new_input)
    file_input = new_input
}

function main_loop(vm) {
    for (let i = 0; i < TICKS_PER_FRAME; i++) {
        vm.tick()
    }
    vm.tick_timers()

    ctx.fillStyle = "black"
    ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)
    ctx.fillStyle = "white"
    vm.draw(SCALE)

    anim_frame = window.requestAnimationFrame(() => {
        main_loop(vm)
    })
}

run().catch(console.error)