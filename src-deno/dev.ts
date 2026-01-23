import { serveDir } from "jsr:@std/http/file-server";

const PORT = 4237;
const DIST_DIR = "dist-temp";
const WATCH_DIR = "src";

// Zbiór aktywnych połączeń z przeglądarkami (do wysyłania sygnału reload)
const sockets = new Set<WebSocket>();

// 1. Funkcja Budująca (uruchamia Twoje polecenie z deno.json)
async function runBuild() {
  console.log("🔨 Budowanie (dev-gui)...");
  const command = new Deno.Command("deno", {
    args: ["task", "dev-gui"],
    stdout: "inherit",
    stderr: "inherit",
  });
  const { code } = await command.output();
  return code === 0;
}

// 2. Serwer HTTP + WebSocket
Deno.serve({ port: PORT }, (req) => {
  const url = new URL(req.url);

  // A. Obsługa WebSocket (Live Reload)
  if (url.pathname === "/_reload") {
    const { socket, response } = Deno.upgradeWebSocket(req);
    socket.onopen = () => sockets.add(socket);
    socket.onclose = () => sockets.delete(socket);
    return response;
  }

  // B. Serwowanie index.html z wstrzykniętym skryptem
  // Jeśli żądamy głównej strony, musimy dodać do niej kod JS, który nasłuchuje zmian
  if (url.pathname === "/" || url.pathname === "/index.html") {
    try {
      let html = Deno.readTextFileSync(`${DIST_DIR}/index.html`);
      // Wstrzyknięcie skryptu przed zamknięciem body
      const reloadScript = `
        <script>
          const ws = new WebSocket("ws://localhost:${PORT}/_reload");
          ws.onmessage = (msg) => {
            if (msg.data === "reload") window.location.reload();
          };
          console.log("🔌 Live Reload podłączony");
        </script>
      </body>`;
      html = html.replace("</body>", reloadScript);
      
      return new Response(html, {
        headers: { "content-type": "text/html" },
      });
    } catch {
      return new Response("Nie znaleziono index.html (uruchom build)", { status: 404 });
    }
  }

  // C. Serwowanie reszty plików statycznych (js, css, itp.) z folderu dist-temp
  return serveDir(req, {
    fsRoot: DIST_DIR,
    quiet: true,
  });
});

console.log(`🚀 Dev Serwer: http://localhost:${PORT}`);

// 3. Wstępne budowanie
await runBuild();

// 4. Obserwator plików (Watcher)
let timeout: number | undefined;
const watcher = Deno.watchFs(WATCH_DIR);

console.log(`👀 Obserwuję zmiany w: ./${WATCH_DIR}`);

for await (const event of watcher) {
  // Reagujemy tylko na zmiany w plikach .ts i .tsx
  const relevantFile = event.paths.some(path => /\.(ts|tsx)$/.test(path));
  
  if ((event.kind === "modify" || event.kind === "create") && relevantFile) {
    // Debounce (żeby nie budować 10 razy na sekundę)
    clearTimeout(timeout);
    timeout = setTimeout(async () => {
      console.log("♻️ Wykryto zmianę!");
      
      const success = await runBuild();
      
      if (success) {
        console.log("📡 Odświeżam przeglądarkę...");
        for (const socket of sockets) {
          socket.send("reload");
        }
      }
    }, 100);
  }
}