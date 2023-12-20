const PUZZLE_DATA = `
%lh -> mj
%nd -> qf
&pn -> dh, dk, bg, qs, rp, bk, gs
%bk -> rs
%nh -> lh
%hc -> jg, ks
%pt -> gv, jg
&dh -> jz
%jq -> nd
%gv -> jg, mr
%gm -> jv
&zt -> jq, rn, nd, bt, jh, gm
%mz -> dc, zt
%nf -> dm, pn
%bg -> bk
%qt -> qx, xk
%dc -> zt, db
%rc -> gz, jg
%kx -> pn, gj
%mj -> zm
%rs -> pn, dk
%lv -> tb, jg
&mk -> jz
%bt -> pv, zt
%cg -> mz, zt
%pk -> qx
%jd -> lv, jg
%jv -> jh, zt
%ks -> jg, jd
%gs -> bg
broadcaster -> bt, rc, qs, qt
%dm -> rm, pn
%pv -> jq, zt
%db -> zt
%dv -> sl, qx
%qs -> rp, pn
%sr -> hf
%qf -> gm, zt
&jz -> rx
&vf -> jz
%gz -> vj, jg
%mr -> jg
%dk -> kx
&jg -> rc, mk, vj
%qh -> hc, jg
%vj -> qh
%tb -> pt, jg
%rm -> pn
%gj -> pn, nf
%rp -> gs
%xk -> td, qx
%hf -> nh
&rn -> jz
&qx -> lh, vf, hf, nh, sr, mj, qt
%td -> sr, qx
%sl -> pk, qx
%jh -> cg
%zm -> dv, qx
`

/** @type HTMLCanvasElement */
let canvas = C
/** @type CanvasRenderingContext2D */
let ctx = C.getContext("2d")

let nodes = {}
let edges = []

for (let line of PUZZLE_DATA.trim().split("\n")) {
    let parts = line.split(" -> ")
    let kind = ""
    let name = "broadcaster"
    if (!parts[0].startsWith("broadcaster")) {
        kind = parts[0][0]
        name = parts[0].substring(1)
    }
    for (let child of parts[1].split(", ")) {
        edges.push([name, child])
    }
    nodes[name] = {
        kind,
        pos: [100 + 800 * Math.random(), 100 + 800 * Math.random()],
    }
}
nodes["rx"] = {
    kind: "",
    pos: [100 + 800 * Math.random(), 100 + 800 * Math.random()],
}
let nodeKeys = Object.keys(nodes)

const draw = () => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)
    ctx.strokeStyle = "#000"
    ctx.font = "10pt sans-serif"
    ctx.lineWidth = 2
    ctx.textAlign = "center"
    ctx.textBaseline = "middle"
    for (const edge of edges) {
        let pos0 = nodes[edge[0]].pos
        let pos1 = nodes[edge[1]].pos
        let mid = [0.5 * pos0[0] + 0.5 * pos1[0], 0.5 * pos0[1] + 0.5 * pos1[1]]
        ctx.strokeStyle = "#007"
        ctx.beginPath()
        ctx.moveTo(pos0[0], pos0[1])
        ctx.lineTo(mid[0], mid[1])
        ctx.stroke()
        ctx.strokeStyle = "#07d"
        ctx.beginPath()
        ctx.moveTo(mid[0], mid[1])
        ctx.lineTo(pos1[0], pos1[1])
        ctx.stroke()
    }
    for (const name in nodes) {
        let node = nodes[name]
        ctx.beginPath()
        ctx.arc(node.pos[0], node.pos[1], 20, 0, 2 * Math.PI)
        ctx.stroke()
        ctx.fillStyle = "#eee"
        ctx.fill()
        ctx.fillStyle = "#000"
        ctx.fillText(node.kind + name, node.pos[0], node.pos[1])
    }
}

const relax = () => {
    for (let i = 0; i < nodeKeys.length - 1; ++i) {
        for (let j = i + 1; j < nodeKeys.length; ++j) {
            let a = nodes[nodeKeys[i]]
            let b = nodes[nodeKeys[j]]

            let dx = b.pos[0] - a.pos[0]
            let dy = b.pos[1] - a.pos[1]
            let dist = Math.sqrt(dx * dx + dy * dy)
            let ndx = dx / dist
            let ndy = dy / dist
            let force = 20 / Math.max(1, dist)

            b.pos[0] += ndx * force
            b.pos[1] += ndy * force
            a.pos[0] -= ndx * force
            a.pos[1] -= ndy * force
        }
    }

    for (const edge of edges) {
        let a = nodes[edge[0]]
        let b = nodes[edge[1]]

        let dx = b.pos[0] - a.pos[0]
        let dy = b.pos[1] - a.pos[1]
        let dist = Math.sqrt(dx * dx + dy * dy)
        let ndx = dx / dist
        let ndy = dy / dist
        let force = 0.04 * (75 - dist)

        b.pos[0] += ndx * force
        b.pos[1] += ndy * force
        a.pos[0] -= ndx * force
        a.pos[1] -= ndy * force
    }
}

let mouseTarget = null
let mouseDelta = [0, 0]
ctx.canvas.onmousedown = e => {
    let rect = canvas.getBoundingClientRect()
    let mouse = [e.clientX - rect.left, e.clientY - rect.top]
    for (const k in nodes) {
        let node = nodes[k]
        let a = node.pos
        let b = mouse
        let dx = b[0] - a[0]
        let dy = b[1] - a[1]
        let dist = Math.sqrt(dx * dx + dy * dy)
        if (dist < 20) {
            mouseTarget = node
            mouseDelta = [dx, dy]
            return
        }
    }
}
ctx.canvas.onmousemove = e => {
    let rect = canvas.getBoundingClientRect()
    let mouse = [e.clientX - rect.left, e.clientY - rect.top]
    if (mouseTarget !== null) {
        mouseTarget.pos[0] = mouse[0] - mouseDelta[0]
        mouseTarget.pos[1] = mouse[1] - mouseDelta[1]
    }
}
ctx.canvas.onmouseup = () => {
    mouseTarget = null
}

const frame = () => {
    requestAnimationFrame(frame)
    if (chkRelax.checked) {
        relax()
    }
    draw()
}
frame()
