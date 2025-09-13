// /assets/js/projects.js

// Helpers
const $ = (s)=>document.querySelector(s);
const paramsFromForm = (formSel)=>{
  const fd = new FormData($(formSel));
  const p = new URLSearchParams();
  for (const [k,v] of fd.entries()) if (v) p.set(k,v);
  return p;
};

// Hydrate le formulaire depuis l’URL
(function syncForm(){
  const p = new URLSearchParams(location.search);
  const f = $('#filters');
  for (const [k,v] of p.entries()){
    const el = f.elements.namedItem(k);
    if (el) el.value = v;
  }
})();

async function loadProjects(){
  const qs = location.search;
  const url = '/api/projects' + (qs || '');
  const grid = $('#grid');

  grid.setAttribute('aria-busy', 'true');
  grid.innerHTML = `
    <article class="card project-card" role="status">
      <p class="max-w-prose">Chargement…</p>
    </article>`;

  try{
    const res = await fetch(url, { headers: { 'Accept': 'application/json' } });
    if(!res.ok) throw new Error('HTTP '+res.status);
    const items = await res.json();

    if (!Array.isArray(items) || items.length === 0){
      grid.innerHTML = `
        <article class="card project-card">
          <p class="max-w-prose">Aucun projet trouvé avec ces filtres.</p>
        </article>`;
      return;
    }

    grid.innerHTML = '';
    for (const p of items){
      const hasImg  = typeof p.image    === 'string' && p.image.length > 0;
      const hasRepo = typeof p.repoLink === 'string' && p.repoLink.length > 0;
      const hasPdf  = typeof p.pdfLink  === 'string' && p.pdfLink.length  > 0;

      const article = document.createElement('article');
      article.className = 'card project-card';

      article.innerHTML = `
        ${hasImg ? `<img src="${p.image}" alt="${p.title}" loading="lazy">` : ``}
        <h3>${p.title}</h3>
        ${p.category ? `<p class="meta">${p.category}</p>` : ``}
        ${p.description ? `<p class="max-w-prose">${p.description}</p>` : ``}
        ${
          Array.isArray(p.technologies) && p.technologies.length
          ? `<div class="tags">${p.technologies.map(t=>`<span class="tag">${t}</span>`).join('')}</div>`
          : ``
        }
        <div class="actions">
          ${hasRepo ? `<a class="btn" href="${p.repoLink}" target="_blank" rel="noopener">Code</a>` : ``}
          ${hasPdf  ? `<a class="btn ghost" href="${p.pdfLink}" target="_blank" rel="noopener">PDF</a>` : ``}
        </div>
      `;
      grid.appendChild(article);
    }
  }catch(err){
    grid.innerHTML = `
      <article class="card project-card">
        <p class="max-w-prose">Erreur de chargement : ${String(err).replace(/</g,'&lt;')}</p>
      </article>`;
  }finally{
    grid.setAttribute('aria-busy', 'false');
  }
}

// Handlers du formulaire
$('#filters').addEventListener('submit', (e)=>{
  e.preventDefault();
  const p = paramsFromForm('#filters');
  const next = location.pathname + (p.toString() ? ('?'+p.toString()) : '');
  history.replaceState(null,'',next);
  loadProjects();
});
$('#reset').addEventListener('click', ()=>{
  history.replaceState(null,'',location.pathname);
  $('#filters').reset();
  loadProjects();
});

// Initial load
loadProjects();
