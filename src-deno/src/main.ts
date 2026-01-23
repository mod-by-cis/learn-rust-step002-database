import { h, render, Fragment } from "preact";
import { useState, useEffect } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";

// --- GLOBALNE STYLE CSS (Hover, Active, Shadows) ---
const styleTag = document.createElement("style");
styleTag.innerHTML = `
  .calc-btn {
    border: none;
    border-radius: 10px;
    cursor: pointer;
    color: white;
    font-weight: bold;
    font-size: 1.2rem;
    transition: all 0.1s ease;
    /* Efekt wypukłości (Convex) */
    box-shadow: 
      0 4px #000, 
      inset 1px 1px 1px rgba(255,255,255,0.2),
      inset -1px -1px 1px rgba(0,0,0,0.2);
    display: flex;
    align-items: center;
    justify-content: center;
    user-select: none;
  }

  .calc-btn:hover {
    filter: brightness(1.2);
    transform: translateY(-1px);
  }

  .calc-btn:active {
    /* Efekt wklęsłości (Concave) */
    transform: translateY(3px);
    box-shadow: 0 1px #000;
  }

  .display-box {
    background: #000;
    padding: 30px 15px;
    text-align: right;
    font-size: 2.8rem;
    font-family: 'Courier New', monospace;
    border-radius: 12px;
    color: #2ecc71;
    min-height: 100px;
    word-break: break-all;
    box-shadow: inset 0 4px 10px rgba(0,0,0,0.8), 0 1px 1px rgba(255,255,255,0.1);
    margin-bottom: 10px;
  }
`;
document.head.appendChild(styleTag);

function App() {
  const [display, setDisplay] = useState("0");
  const [prevValue, setPrevValue] = useState<number | null>(null);
  const [operator, setOperator] = useState<string | null>(null);
  const [waitingForNext, setWaitingForNext] = useState(false);

  const runCalculation = async (a: number, b: number, op: string) => {
    try {
      const res = await invoke<number>("calculate_command", { a, b, op: operator });
      setDisplay(res.toString());
      setPrevValue(res);
      setOperator(null);
      setWaitingForNext(true);
    } catch (err) {
      setDisplay("Błąd");
    }
  };

  const handleNumber = (num: string) => {
    if (waitingForNext) {
      setDisplay(num);
      setWaitingForNext(false);
    } else {
      setDisplay(display === "0" ? num : display + num);
    }
  };

  const handleOperator = (op: string) => {
    setPrevValue(parseFloat(display));
    setOperator(op);
    setWaitingForNext(true);
  };

  const performEquals = () => {
    if (operator && prevValue !== null) {
      runCalculation(prevValue, parseFloat(display), operator);
    }
  };

  // Helper do renderowania przycisków
  const Button = (label: string, color: string, onClick: () => void, span = 1) => 
    h("button", { 
      className: "calc-btn", 
      style: `background: ${color}; grid-column: span ${span};`,
      onClick 
    }, label);

  return h("div", { 
    style: "display: flex; flex-direction: column; height: 100vh; padding: 15px; background: #1a1a1a;" 
  }, [
    h("div", { className: "display-box" }, display),

    h("div", { 
      style: "display: grid; grid-template-columns: repeat(5, 1fr); gap: 12px; flex-grow: 1;" 
    }, [
      // Wiersz 1: Dodane %, ^ i AC
      Button("AC", "#c0392b", () => setDisplay("0"), 2),
      Button("%", "#8e44ad", () => handleOperator("%")),
      Button("^", "#8e44ad", () => handleOperator("^")),
      Button("/", "#f39c12", () => handleOperator("/")),

      // Wiersz 2: Dodany pierwiastek "v"
      Button("7", "#333", () => handleNumber("7")),
      Button("8", "#333", () => handleNumber("8")),
      Button("9", "#333", () => handleNumber("9")),
      Button("v", "#8e44ad", () => handleOperator("v")),
      Button("*", "#f39c12", () => handleOperator("*")),

      // Wiersz 3
      Button("4", "#333", () => handleNumber("4")),
      Button("5", "#333", () => handleNumber("5")),
      Button("6", "#333", () => handleNumber("6")),
      h("div", { style: "grid-column: span 1;" }), // Puste miejsce
      Button("-", "#f39c12", () => handleOperator("-")),

      // Wiersz 4
      Button("1", "#333", () => handleNumber("1")),
      Button("2", "#333", () => handleNumber("2")),
      Button("3", "#333", () => handleNumber("3")),
      h("div", { style: "grid-column: span 1;" }), // Puste miejsce
      Button("+", "#f39c12", () => handleOperator("+")),

      // Wiersz 5
      Button("0", "#333", () => handleNumber("0"), 2),
      Button(".", "#333", () => handleNumber(".")),
      Button("=", "#27ae60", performEquals, 2)
    ])
  ]);
}

const root = document.getElementById("app");
if (root) render(h(App, null), root);