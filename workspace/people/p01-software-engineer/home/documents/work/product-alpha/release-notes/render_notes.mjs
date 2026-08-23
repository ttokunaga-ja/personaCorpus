const escapeHtml = value => String(value).replace(/[&<>"']/g, char => ({"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"})[char]);

export function renderNotes(markdown) {
  const lines = markdown.split(/\r?\n/);
  return lines.map(line => {
    const heading = /^(#{1,3})\s+(.+)$/.exec(line);
    if (heading) { const level = heading[1].length; return `<h${level}>${escapeHtml(heading[2])}</h${level}>`; }
    if (!line.trim()) return "";
    return `<p>${escapeHtml(line)}</p>`;
  }).join("\n");
}
