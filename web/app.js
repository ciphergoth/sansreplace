"use strict";

// Explore results/latest: pick a point (n, k) on a triangle of log n against
// log k, and see each algorithm's time/k there as a bar. The data comes from
// python/export_web.

const SVGNS = "http://www.w3.org/2000/svg";
const YMAXES = [5, 10, 20, 50, 100, 200, 500, 1000];
const SUPERSCRIPTS = "⁰¹²³⁴⁵⁶⁷⁸⁹";

const state = { caseIndex: 0, n: 1e7, k: 1e5, ymax: 50 };
let data, algs, cases;

function el(name, attrs, parent) {
  const e = document.createElementNS(SVGNS, name);
  for (const [a, v] of Object.entries(attrs || {})) e.setAttribute(a, v);
  if (parent) parent.appendChild(e);
  return e;
}

function text(parent, x, y, s, attrs) {
  const t = el("text", { x, y, ...attrs }, parent);
  t.textContent = s;
  return t;
}

function power10(p) {
  if (p === 0) return "1";
  if (p === 1) return "10";
  return "10" + String(p).split("").map(d => SUPERSCRIPTS[d]).join("");
}

function short(x) {
  if (x >= 1e4) {
    const [m, e] = x.toExponential(1).split("e+");
    return `${m}e${e}`;
  }
  if (x >= 100) return Math.round(x).toLocaleString("en");
  return x.toPrecision(3);
}

// Per case: the points, each algorithm's times, and the fastest at each point.
function prepare(c) {
  const fastest = c.points.map((_, i) => {
    let best = null;
    for (const [a, alg] of algs.entries()) {
      const t = c.times[alg.name]?.[i];
      if (t != null && (best === null || t < c.times[algs[best].name][i])) best = a;
    }
    return best;
  });
  return { ...c, fastest };
}

function nearest(c, n, k) {
  const ln = Math.log10(n), lk = Math.log10(k);
  let best = 0, bestD = Infinity;
  c.points.forEach(([pn, pk], i) => {
    const d = (Math.log10(pn) - ln) ** 2 + (Math.log10(pk) - lk) ** 2;
    if (d < bestD) { best = i; bestD = d; }
  });
  return best;
}

function selected() {
  const c = cases[state.caseIndex];
  return { c, i: nearest(c, state.n, state.k) };
}

// ---- The triangle ----

const T = { w: 520, l: 56, r: 16, t: 12, b: 44 };

function triangleGeometry() {
  const maxN = Math.max(...cases.flatMap(c => c.points.map(p => p[0])));
  const maxK = Math.max(...cases.flatMap(c => c.points.map(p => p[1])));
  const lx = Math.log10(maxN) * 1.02, ly = Math.log10(maxK) * 1.02;
  const s = T.w - T.l - T.r;
  const ph = s * ly / lx;
  return {
    lx, ly, ph, h: T.t + ph + T.b,
    x: n => T.l + Math.log10(n) / lx * s,
    y: k => T.t + ph - Math.log10(k) / lx * s,
  };
}

let tri;

function drawTriangle() {
  const svg = document.getElementById("triangle");
  svg.replaceChildren();
  svg.setAttribute("viewBox", `0 0 ${T.w} ${tri.h}`);
  const c = cases[state.caseIndex];
  const top = 10 ** tri.ly, right = 10 ** tri.lx;
  el("polygon", {
    points: [[1, 1], [right, 1], [right, top], [top, top]].map(([n, k]) => `${tri.x(n)},${tri.y(k)}`).join(" "),
    fill: "#f6f6f6",
  }, svg);
  const axis = el("g", { class: "axis" }, svg);
  for (let p = 0; p <= tri.lx; p++) {
    const x = tri.x(10 ** p);
    el("line", { class: "gridline", x1: x, x2: x, y1: T.t, y2: T.t + tri.ph }, axis);
    text(axis, x, T.t + tri.ph + 16, power10(p), { "text-anchor": "middle" });
  }
  for (let p = 0; p <= tri.ly; p++) {
    const y = tri.y(10 ** p);
    el("line", { class: "gridline", x1: T.l, x2: T.w - T.r, y1: y, y2: y }, axis);
    text(axis, T.l - 6, y + 4, power10(p), { "text-anchor": "end" });
  }
  text(axis, T.l + (T.w - T.l - T.r) / 2, T.t + tri.ph + 36, "n", { "text-anchor": "middle", "font-style": "italic" });
  text(axis, 14, T.t + tri.ph / 2, "k", { "text-anchor": "middle", "font-style": "italic" });
  const dots = el("g", {}, svg);
  c.points.forEach(([n, k], i) => {
    const f = c.fastest[i];
    el("circle", {
      cx: tri.x(n), cy: tri.y(k), r: 3.2,
      fill: f === null ? "#ccc" : algs[f].colour,
    }, dots);
  });
  const { i } = selected();
  const [n, k] = c.points[i];
  el("circle", { cx: tri.x(n), cy: tri.y(k), r: 7, fill: "none", stroke: "#000", "stroke-width": 2 }, svg);
  el("circle", { cx: tri.x(n), cy: tri.y(k), r: 9, fill: "none", stroke: "#fff", "stroke-width": 1.5 }, svg);
}

function pickAt(evt) {
  const svg = document.getElementById("triangle");
  const pt = new DOMPoint(evt.clientX, evt.clientY).matrixTransform(svg.getScreenCTM().inverse());
  const c = cases[state.caseIndex];
  let best = 0, bestD = Infinity;
  c.points.forEach(([n, k], i) => {
    const d = (tri.x(n) - pt.x) ** 2 + (tri.y(k) - pt.y) ** 2;
    if (d < bestD) { best = i; bestD = d; }
  });
  select(c.points[best]);
}

// Arrow keys: left and right move to the next n, keeping k as close as it
// can be; up and down move to the next k at the same n.
function step(key) {
  const { c, i } = selected();
  const [n, k] = c.points[i];
  if (key === "ArrowUp" || key === "ArrowDown") {
    const ks = c.points.filter(p => p[0] === n).map(p => p[1]);
    const j = ks.indexOf(k) + (key === "ArrowUp" ? 1 : -1);
    if (j >= 0 && j < ks.length) select([n, ks[j]]);
  } else {
    const ns = [...new Set(c.points.map(p => p[0]))];
    const j = ns.indexOf(n) + (key === "ArrowRight" ? 1 : -1);
    if (j < 0 || j >= ns.length) return;
    const ks = c.points.filter(p => p[0] === ns[j]).map(p => p[1]);
    const lk = Math.log10(k);
    const best = ks.reduce((a, b) => Math.abs(Math.log10(b) - lk) < Math.abs(Math.log10(a) - lk) ? b : a);
    select([ns[j], best]);
  }
}

// ---- The bars ----

const B = { w: 640, h: 420, l: 52, r: 8, t: 40, b: 100 };

function drawBars() {
  const svg = document.getElementById("bars");
  svg.replaceChildren();
  svg.setAttribute("viewBox", `0 0 ${B.w} ${B.h}`);
  const { c, i } = selected();
  const pw = B.w - B.l - B.r, ph = B.h - B.t - B.b;
  const y = v => B.t + ph - Math.min(v, state.ymax) / state.ymax * ph;
  const axis = el("g", { class: "axis" }, svg);
  const tick = state.ymax / 5;
  for (let v = 0; v <= state.ymax + 1e-9; v += tick) {
    el("line", { class: "gridline", x1: B.l, x2: B.w - B.r, y1: y(v), y2: y(v) }, axis);
    text(axis, B.l - 6, y(v) + 4, +v.toPrecision(3), { "text-anchor": "end" });
  }
  text(axis, 14, B.t + ph / 2, "ns per output", {
    "text-anchor": "middle", transform: `rotate(-90 14 ${B.t + ph / 2})`,
  });

  const groups = data.families.length;
  const unit = pw / (algs.length + 0.6 * (groups - 1));
  const bw = unit * 0.72;
  const f = c.fastest[i];
  const best = f === null ? null : c.times[algs[f].name][i];
  el("defs", {}, svg).innerHTML =
    `<pattern id="hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
       <line x1="0" y1="0" x2="0" y2="6" stroke="#999" stroke-width="2"/></pattern>`;
  for (const [a, alg] of algs.entries()) {
    const cx = B.l + unit * (a + 0.6 * alg.family + 0.5);
    const t = c.times[alg.name]?.[i];
    const base = B.t + ph;
    if (t == null) {
      el("rect", { x: cx - bw / 2, y: base - 14, width: bw, height: 14, fill: "url(#hatch)", stroke: "#999" }, svg);
      text(svg, cx, base - 20, "not timed", {
        class: "ratio", "text-anchor": "start", transform: `rotate(-90 ${cx} ${base - 20})`, dy: "0.35em",
      });
    } else {
      const top = y(t);
      el("rect", { x: cx - bw / 2, y: top, width: bw, height: base - top, fill: alg.colour }, svg);
      if (t > state.ymax) {
        // A break in the bar shows that it goes on off the top.
        const z = B.t + 10;
        el("polygon", {
          points: `${cx - bw / 2 - 1},${z} ${cx + bw / 2 + 1},${z - 5} ${cx + bw / 2 + 1},${z + 1} ${cx - bw / 2 - 1},${z + 6}`,
          fill: "var(--bg)",
        }, svg);
      }
      text(svg, cx, top - 4, short(t), { class: "value", "text-anchor": "middle" });
      text(svg, cx, top - 18, "×" + short(t / best), { class: "ratio", "text-anchor": "middle" });
    }
    text(svg, cx + 4, base + 10, alg.name, {
      class: "label", "text-anchor": "end", transform: `rotate(-45 ${cx + 4} ${base + 10})`,
      "font-weight": a === f ? "bold" : "normal",
    });
  }
}

function drawReadout() {
  const { c, i } = selected();
  const [n, k] = c.points[i];
  const f = c.fastest[i];
  const fmt = x => x.toLocaleString("en");
  const fastest = f === null ? "" :
    `Fastest: <b>${algs[f].name}</b>, ${short(c.times[algs[f].name][i])} ns per output`;
  document.getElementById("readout").innerHTML =
    `<i>n</i> = ${fmt(n)}, <i>k</i> = ${fmt(k)}, <i>k</i>/<i>n</i> = ${(k / n).toPrecision(3)}<br>${fastest}`;
}

// ---- State ----

function select([n, k]) {
  state.n = n;
  state.k = k;
  draw();
}

function draw() {
  for (const [j, b] of document.querySelectorAll("#cases button").entries())
    b.setAttribute("aria-checked", j === state.caseIndex);
  document.getElementById("ymax").value = state.ymax;
  drawTriangle();
  drawBars();
  drawReadout();
  const { c, i } = selected();
  const [n, k] = c.points[i];
  history.replaceState(null, "", `#case=${c.name}&n=${n}&k=${k}&ymax=${state.ymax}`);
}

function readHash() {
  const h = new URLSearchParams(location.hash.slice(1));
  const ci = cases.findIndex(c => c.name === h.get("case"));
  if (ci >= 0) state.caseIndex = ci;
  if (+h.get("n") > 0) state.n = +h.get("n");
  if (+h.get("k") > 0) state.k = +h.get("k");
  if (YMAXES.includes(+h.get("ymax"))) state.ymax = +h.get("ymax");
}

async function main() {
  data = await (await fetch("generated/data.json")).json();
  algs = data.families.flatMap((fam, family) => fam.map(([name, colour]) => ({ name, colour, family })));
  cases = data.cases.map(prepare);
  tri = triangleGeometry();
  readHash();

  const caseButtons = document.getElementById("cases");
  cases.forEach((c, j) => {
    const b = document.createElement("button");
    b.textContent = c.title;
    b.setAttribute("role", "radio");
    b.addEventListener("click", () => { state.caseIndex = j; draw(); });
    caseButtons.appendChild(b);
  });
  const ymax = document.getElementById("ymax");
  for (const v of YMAXES) {
    const o = document.createElement("option");
    o.value = o.textContent = v;
    ymax.appendChild(o);
  }
  ymax.addEventListener("change", () => { state.ymax = +ymax.value; draw(); });

  const svg = document.getElementById("triangle");
  svg.addEventListener("pointerdown", e => { svg.setPointerCapture(e.pointerId); svg.focus(); pickAt(e); });
  svg.addEventListener("pointermove", e => { if (e.buttons) pickAt(e); });
  svg.addEventListener("keydown", e => {
    if (e.key.startsWith("Arrow")) { e.preventDefault(); step(e.key); }
  });
  draw();
}

main();
