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

let input = document.getElementById("fileinput")

async function run() {
    await init()
    let vm = new wasm.VmWasm()

    document.addEventListener("keydown", function (event) {
        vm.keypress(event, true)
    })

    document.addEventListener("keyup", function (event) {
        vm.keypress(event, false)
    })

    input.addEventListener("change", function (event) {
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

function rom_selector() {
    let select_tag = document.getElementById("rom-selector")
    for(let i=0;i < 10; i++){
        let option = document.createElement("option")
        option.setAttribute("value", i)
        option.text = "rom " + i
        select_tag.appendChild(option)
    }
}

rom_selector()