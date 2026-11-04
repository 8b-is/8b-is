const STORAGE_KEY = "uoze.progress.v1";

const boxes = Array.from(document.querySelectorAll("[data-progress]"));
const cards = Array.from(document.querySelectorAll("[data-module]"));
const progress = document.getElementById("progress");
const progressText = document.getElementById("progress-text");
const resetButton = document.getElementById("reset-progress");

let memoryState = {};

function readState() {
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    const parsed = raw ? JSON.parse(raw) : {};
    return parsed && typeof parsed === "object" ? parsed : {};
  } catch {
    return memoryState;
  }
}

function writeState(state) {
  memoryState = state;
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } catch {
    return;
  }
}

function renderProgress() {
  const total = boxes.length;
  const done = boxes.filter((box) => box.checked).length;

  if (progress) {
    progress.max = total;
    progress.value = done;
    progress.textContent = `${done} of ${total} modules marked.`;
  }

  if (progressText) {
    const suffix = done === total && total > 0
      ? "Lap complete — begin another."
      : "Keep the loop gentle.";
    progressText.textContent = `${done} of ${total} modules marked. ${suffix}`;
  }

  cards.forEach((card) => {
    const box = card.querySelector("[data-progress]");
    card.classList.toggle("is-done", Boolean(box && box.checked));
  });
}

function saveProgress() {
  const state = {};
  boxes.forEach((box) => {
    state[box.dataset.progress] = box.checked;
  });
  writeState(state);
}

function loadProgress() {
  const state = readState();
  boxes.forEach((box) => {
    box.checked = Boolean(state[box.dataset.progress]);
  });
  renderProgress();
}

boxes.forEach((box) => {
  box.addEventListener("change", () => {
    saveProgress();
    renderProgress();
  });
});

if (resetButton) {
  resetButton.addEventListener("click", () => {
    boxes.forEach((box) => {
      box.checked = false;
    });
    writeState({});
    renderProgress();
  });
}

const learningStopped = document.getElementById("input-learning");
const livingStopped = document.getElementById("input-living");
const gateOutput = document.getElementById("nand-output");

function renderGate() {
  if (!learningStopped || !livingStopped || !gateOutput) {
    return;
  }

  const alive = !(learningStopped.checked && livingStopped.checked);
  gateOutput.textContent = alive
    ? "Loop alive"
    : "Loop stopped — restart with one small act.";
  gateOutput.classList.toggle("is-stopped", !alive);
}

[learningStopped, livingStopped].forEach((input) => {
  if (input) {
    input.addEventListener("change", renderGate);
  }
});

loadProgress();
renderGate();
