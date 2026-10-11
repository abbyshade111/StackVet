// The stand-in name server for the fenced network (backlog 0240, ADR-085). It answers every DNS question it is asked with
// SERVFAIL, the answer a name gets on the fenced network today, so the app sees the same result as before. It writes one
// JSON line per question to its output: the time, the name, and the query type. Built-in modules only: the fence has no
// route to a package registry. Names are written as asked; `sv` redacts them before anything is kept.
import dgram from "node:dgram";

const server = dgram.createSocket("udp4");

// The name in a question, from the byte after the 12-byte header, and where the question ends.
function question(message) {
  const labels = [];
  let at = 12;
  while (at < message.length && message[at] !== 0) {
    const length = message[at];
    if (length > 63 || at + 1 + length > message.length) return null;
    labels.push(message.subarray(at + 1, at + 1 + length).toString("latin1"));
    at += 1 + length;
  }
  if (at + 5 > message.length) return null;
  const type = message.readUInt16BE(at + 1);
  return { name: labels.join("."), type, end: at + 5 };
}

server.on("message", (message, from) => {
  if (message.length < 12) return;
  const parsed = question(message);
  if (!parsed) return;
  console.log(
    JSON.stringify({ time: new Date().toISOString(), name: parsed.name, type: parsed.type }),
  );
  // The reply: the same id, QR set with the recursion bits copied, RCODE SERVFAIL (2), and the question as it was asked.
  const reply = Buffer.alloc(parsed.end);
  message.copy(reply, 0, 0, parsed.end);
  reply.writeUInt16BE(0x8000 | (message.readUInt16BE(2) & 0x0100) | 0x0080 | 0x0002, 2);
  reply.writeUInt16BE(0, 6);
  reply.writeUInt16BE(0, 8);
  reply.writeUInt16BE(0, 10);
  server.send(reply, from.port, from.address);
});

server.bind(53, () => console.log(JSON.stringify({ time: new Date().toISOString(), ready: true })));
