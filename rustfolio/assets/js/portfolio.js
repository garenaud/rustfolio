(function () {
    const DATA_URL = '/data/graphicPortfolio.json';
    const $ = (s) => document.querySelector(s);
    const grid = $('#pf-grid');
    const form = $('#pf-filters');
    const modal = $('#pf-modal');
    const modalContent = $('#pf-modal-content');
  
    let all = [];
  
    function cardHTML(w) {
      const isPdf = typeof w.image === 'string' && w.image.toLowerCase().endsWith('.pdf');
      const thumb = isPdf
        ? `<div class="card" style="display:grid;place-items:center;min-height:160px"><p>${w.title}</p><p class="meta">PDF</p></div>`
        : `<img src="${w.image}" alt="${w.title}" loading="lazy" style="width:100%;aspect-ratio:16/9;object-fit:cover;border-radius:12px">`;
  
      return `
        <article class="card project-card">
          ${thumb}
          <h3 style="margin-top:10px">${w.title}</h3>
          <p class="meta">${w.type ?? ''}</p>
          <p class="max-w-prose">${w.description ?? ''}</p>
          <div class="actions" style="margin-top:10px">
            <button class="btn ghost" data-open="${encodeURIComponent(w.image)}" data-title="${encodeURIComponent(w.title)}">Voir</button>
          </div>
        </article>
      `;
    }
  
    function render(list) {
      if (!list.length) {
        grid.innerHTML = `<article class="card"><p class="max-w-prose">Aucun résultat.</p></article>`;
        return;
      }
      grid.innerHTML = list.map(cardHTML).join('');
      // bind open buttons
      grid.querySelectorAll('button[data-open]').forEach(btn => {
        btn.addEventListener('click', (e) => {
          e.preventDefault();
          const src = decodeURIComponent(btn.dataset.open);
          const title = decodeURIComponent(btn.dataset.title);
          openModal(src, title);
        });
      });
    }
  
    function openModal(src, title) {
      const isPdf = src.toLowerCase().endsWith('.pdf');
      modalContent.innerHTML = `
        <h3>${title}</h3>
        ${isPdf
          ? `<iframe src="${src}" title="${title}" style="width:min(92vw,1000px); height:min(80vh,800px); border:0;"></iframe>`
          : `<img src="${src}" alt="${title}" style="max-width:min(92vw,1000px); height:auto; display:block;">`
        }
      `;
      if (!modal.open) modal.showModal();
    }
  
    function filterNow() {
      const type = form.elements.type.value || 'Tous';
      const q = (form.elements.q.value || '').toLowerCase().trim();
  
      let list = all;
      if (type !== 'Tous') list = list.filter(w => (w.type || '').toLowerCase() === type.toLowerCase());
      if (q) list = list.filter(w =>
        (w.title || '').toLowerCase().includes(q) ||
        (w.description || '').toLowerCase().includes(q)
      );
      render(list);
    }
  
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      filterNow();
    });
    $('#pf-reset').addEventListener('click', () => {
      form.reset();
      filterNow();
    });
  
    async function boot() {
      grid.innerHTML = `<article class="card"><p>Chargement…</p></article>`;
      try {
        const res = await fetch(DATA_URL, { cache: 'no-store' });
        if (!res.ok) throw new Error('HTTP ' + res.status);
        all = await res.json();
        filterNow();
      } catch (err) {
        grid.innerHTML = `<article class="card"><p>Erreur : ${String(err).replace(/</g,'&lt;')}</p></article>`;
      }
    }
  
    boot();
  })();
  