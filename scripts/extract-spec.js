// agent-browser eval --stdin로 실행될 JS.
// 현재 열려있는 API 상세 페이지에서 구조화된 마크다운 스펙을 추출한다.
// 한 번 실행 = 한 개 API 스펙 추출.
(() => {
  const collapseWs = (s) => (s || '').replace(/[ \t]+/g, ' ').replace(/\s*\n\s*/g, '\n').trim();
  const rows = (tbl) => Array.from(tbl.querySelectorAll('tbody tr')).map(tr =>
    Array.from(tr.querySelectorAll('th, td')).map(c => collapseWs(c.textContent))
  );
  const root = document.querySelector('.contents') || document.body;
  const tables = Array.from(root.querySelectorAll('table'));
  if (tables.length === 0) return '<!-- NO_TABLES -->\n';

  // First table: info about the API (Method/URL/TR ID)
  const infoTable = tables[0];
  const info = {};
  Array.from(infoTable.querySelectorAll('tbody tr')).forEach(tr => {
    const cells = Array.from(tr.querySelectorAll('th, td')).map(c => collapseWs(c.textContent));
    for (let i = 0; i + 1 < cells.length; i += 2) {
      const k = cells[i].replace(/\s/g, '');
      const v = cells[i + 1];
      if (k && v) info[k] = v;
    }
  });

  // Caption of info table contains title like "주식현재가 시세[v1_국내주식-008]정보 ..."
  const infoCaption = collapseWs(infoTable.caption?.textContent || '').split('\n')[0];
  const titleMatch = infoCaption.match(/^(.*?)\s*정보\s*$/) || [null, infoCaption];
  const title = titleMatch[1] || 'UNKNOWN';

  // Overview / 개요 — scoped to main content only
  let overview = '';
  const rootText = root.innerText || '';
  const ovMatch = rootText.match(/개요\s*\n([\s\S]*?)(?=\n(Header|Query Parameter|Path Parameter|Body|Response|요청)\s*\n|$)/);
  if (ovMatch) overview = collapseWs(ovMatch[1]).slice(0, 1500);

  // Subsequent tables: look at preceding heading/section to know their role.
  // In KIS portal these tables appear in order:
  //   Header (request)
  //   Query Parameter / Path Parameter / Body (request)
  //   Header (response)
  //   Body (response) — may be split into Body/output/output1/output2
  // We'll detect role by scanning preceding siblings within the same section.
  const getPrecedingHeading = (t) => {
    // Walk up to parents and look for the nearest heading-like element before this table
    let node = t;
    const seen = new Set();
    while (node && !seen.has(node)) {
      seen.add(node);
      let prev = node.previousElementSibling;
      while (prev) {
        const txt = collapseWs(prev.textContent).split('\n')[0];
        if (prev.matches('h1,h2,h3,h4,h5,h6,dt,.tit,.title,strong')) {
          if (txt) return txt;
        }
        prev = prev.previousElementSibling;
      }
      node = node.parentElement;
    }
    return '';
  };

  const sections = [];
  // Track whether we've passed a response-side marker
  let responseMode = false;
  tables.slice(1).forEach((tbl, idx) => {
    const heading = getPrecedingHeading(tbl);
    // KIS portal uses same "Header" text for both request & response; disambiguate by position
    // Heuristic: once we've seen a non-Header request section (Query/Path/Body), the NEXT Header is response.
    const isHeader = /^Header$/i.test(heading);
    const isBody = /^Body$/i.test(heading);
    const isQuery = /Query\s*Parameter/i.test(heading);
    const isPath = /Path\s*Parameter/i.test(heading);

    let role = heading || `Section${idx}`;
    if (isHeader) {
      role = responseMode ? 'Response Header' : 'Request Header';
      if (!responseMode && idx > 0) {
        // no change
      }
    } else if (isBody) {
      role = responseMode ? 'Response Body' : 'Request Body';
    } else if (isQuery) {
      role = 'Query Parameter';
    } else if (isPath) {
      role = 'Path Parameter';
    }

    // After a request body/query/path, switch to response mode for the next Header
    if (!responseMode && (isQuery || isPath || isBody)) responseMode = true;

    const headerCells = Array.from(tbl.querySelectorAll('thead th')).map(c => collapseWs(c.textContent));
    const body = rows(tbl);
    sections.push({ role, header: headerCells, rows: body });
  });

  // Build markdown
  const lines = [];
  lines.push(`# ${title}`);
  lines.push('');
  lines.push('## Info');
  Object.keys(info).forEach(k => lines.push(`- **${k}**: ${info[k]}`));
  lines.push('');
  if (overview) {
    lines.push('## 개요');
    lines.push(overview);
    lines.push('');
  }
  sections.forEach(sec => {
    lines.push(`## ${sec.role}`);
    if (sec.header.length) {
      lines.push('| ' + sec.header.join(' | ') + ' |');
      lines.push('| ' + sec.header.map(() => '---').join(' | ') + ' |');
    }
    sec.rows.forEach(r => {
      // Escape pipes inside cells
      const cells = r.map(c => c.replace(/\|/g, '\\|').replace(/\n/g, '<br>'));
      lines.push('| ' + cells.join(' | ') + ' |');
    });
    lines.push('');
  });
  return lines.join('\n');
})()
