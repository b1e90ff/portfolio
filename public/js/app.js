const root = document.documentElement;

let world = null;
const pages = new Map();
let navSeq = 0;
let currentView = null;
let currentPath = location.pathname;
let modalOpener = null;

function storeMood(mood) {
    try {
        localStorage.setItem('mood', mood);
    } catch {
        /* Private windows may block storage; the mood then lasts for this visit only. */
    }
}

function applyMood(mood) {
    root.dataset.mood = mood;
    document.querySelectorAll('[data-mood-option]').forEach((b) => b.setAttribute('aria-pressed', String(b.dataset.moodOption === mood)));
    document.querySelector('meta[name="theme-color"]')?.setAttribute('content', getComputedStyle(root).getPropertyValue('--sky-top').trim());
    world?.setMood(mood);
}

function initMoods() {
    const buttons = [...document.querySelectorAll('[data-mood-option]')];
    const moods = buttons.map((b) => b.dataset.moodOption);
    if (moods.length) applyMood(moods.includes(root.dataset.mood) ? root.dataset.mood : moods[0]);
    buttons.forEach((b) => b.addEventListener('click', () => {
        applyMood(b.dataset.moodOption);
        storeMood(b.dataset.moodOption);
    }));
}

function loadPage(url) {
    if (!pages.has(url)) {
        const request = fetch(url, { headers: { accept: 'text/html' } })
            .then((res) => {
                if (!res.ok && res.status !== 404) throw new Error(`HTTP ${res.status}`);
                return res.text();
            })
            .then((html) => new DOMParser().parseFromString(html, 'text/html'));
        request.catch(() => pages.delete(url));
        pages.set(url, request);
    }
    return pages.get(url);
}

function swapHead(doc) {
    document.title = doc.title;
    for (const selector of ['meta[name="description"]', 'link[rel="canonical"]']) {
        const next = doc.querySelector(selector);
        const cur = document.querySelector(selector);
        if (next && cur) cur.replaceWith(next.cloneNode(true));
    }
    document.querySelectorAll('link[rel="alternate"][hreflang]').forEach((el) => el.remove());
    const anchor = document.querySelector('link[rel="canonical"]');
    doc.querySelectorAll('link[rel="alternate"][hreflang]').forEach((el) => anchor?.after(el.cloneNode(true)));
    document.querySelectorAll('[data-swap]').forEach((el) => {
        const next = doc.querySelector(`[data-swap="${el.dataset.swap}"]`);
        if (next) el.replaceWith(document.importNode(next, true));
    });
}

function swapMain(doc) {
    const main = document.getElementById('main-content');
    const next = doc.getElementById('main-content');
    const curPanel = main.querySelector('[data-panel]');
    const nextPanel = next.querySelector('[data-panel]');
    const hadModal = Boolean(main.querySelector('[data-modal]'));
    if (curPanel && nextPanel && curPanel.dataset.panel === nextPanel.dataset.panel) {
        curPanel.inert = nextPanel.inert;
        curPanel.querySelector('#panel-title').replaceWith(document.importNode(nextPanel.querySelector('#panel-title'), true));
        main.querySelector('[data-modal]')?.remove();
        const modal = next.querySelector('[data-modal]');
        if (modal) main.append(document.importNode(modal, true));
    } else {
        main.replaceChildren(...[...next.childNodes].map((n) => document.importNode(n, true)));
        world?.setPanelCollapsed(false);
        const panel = main.querySelector('[data-panel]');
        if (panel) {
            panel.classList.add('is-entering');
            requestAnimationFrame(() => requestAnimationFrame(() => panel.classList.remove('is-entering')));
        }
    }
    main.dataset.island = next.dataset.island;
    document.body.dataset.island = next.dataset.island;
    initContent();
    const opener = hadModal && !main.querySelector('[data-modal]') && modalOpener && main.querySelector(`a[href="${CSS.escape(modalOpener)}"]`);
    if (!main.querySelector('[data-modal]')) modalOpener = null;
    const focusTarget = opener || main.querySelector('#modal-title, #panel-title[tabindex], .intro h1');
    focusTarget?.focus({ preventScroll: true });
}

async function navigate(url, { push = true } = {}) {
    const seq = ++navSeq;
    const path = new URL(url, location.href).pathname;
    try {
        const doc = await loadPage(url);
        if (seq !== navSeq) return;
        const next = doc.getElementById('main-content');
        if (!next?.dataset.island) throw new Error('unexpected page shape');
        const target = viewOf(next);
        if (push) history.pushState({}, '', url);
        currentPath = path;
        if (target !== currentView) {
            currentView = target;
            root.classList.add('is-flying');
            await world.fly(target);
            if (seq !== navSeq) return;
        }
        root.classList.remove('is-flying');
        swapHead(doc);
        swapMain(doc);
    } catch {
        location.assign(url);
    }
}

/* Legal and 404 pages have no island: keep the overview camera, shifted aside for the panel. */
function viewOf(main) {
    const island = main.dataset.island;
    return island === 'overview' && main.querySelector('[data-panel]') ? 'overview-aside' : island;
}

function setCollapsed(panel, collapsed) {
    panel.classList.toggle('is-collapsed', collapsed);
    const toggle = panel.querySelector('[data-panel-toggle]');
    const label = collapsed ? toggle.dataset.labelExpand : toggle.dataset.labelCollapse;
    toggle.setAttribute('aria-expanded', String(!collapsed));
    toggle.setAttribute('aria-label', label);
    toggle.title = label;
    world?.setPanelCollapsed(collapsed);
}

function islandHref(key) {
    return document.querySelector(`.dock a[data-island="${key}"]`)?.href;
}

function initNavigation() {
    document.addEventListener('click', (e) => {
        const link = e.target.closest('a[data-nav]');
        if (!link || e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
        if (link.origin !== location.origin) return;
        e.preventDefault();
        if (link.closest('.projects')) modalOpener = link.getAttribute('href');
        if (link.href !== location.href) navigate(link.href);
    });
    document.addEventListener('click', (e) => {
        const toggle = e.target.closest('[data-panel-toggle]');
        const panel = toggle?.closest('[data-panel]');
        if (panel) setCollapsed(panel, !panel.classList.contains('is-collapsed'));
    });
    document.addEventListener('pointerover', (e) => {
        const link = e.target.closest('a[data-nav]');
        if (link && link.origin === location.origin) loadPage(link.href);
    });
    addEventListener('popstate', () => {
        modalOpener = null;
        if (location.pathname !== currentPath) navigate(location.href, { push: false });
    });
    addEventListener('keydown', (e) => {
        if (e.key !== 'Escape' || e.repeat || e.isComposing || e.target.closest('input, textarea, select, [contenteditable]')) return;
        const close = document.querySelector('[data-modal] [data-modal-close]');
        const target = close?.href || (currentView !== 'overview' && islandHref('overview'));
        if (target) navigate(target);
    });
}

function initContactForm() {
    const form = document.querySelector('[data-contact-form]');
    if (!form || form.dataset.bound) return;
    form.dataset.bound = '1';
    const submit = form.querySelector('[data-contact-submit]');
    const idle = form.querySelector('[data-contact-label-idle]');
    const sending = form.querySelector('[data-contact-label-sending]');
    const error = form.querySelector('[data-contact-error]');
    const success = form.querySelector('[data-contact-success]');
    const setState = (state) => {
        submit.disabled = state === 'sending';
        submit.setAttribute('aria-busy', String(state === 'sending'));
        idle.hidden = state === 'sending';
        sending.hidden = state !== 'sending';
        error.hidden = state !== 'error';
        success.hidden = state !== 'success';
    };
    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        if (!form.reportValidity()) return;
        setState('sending');
        try {
            const res = await fetch(form.action, {
                method: 'POST',
                headers: { 'content-type': 'application/json' },
                body: JSON.stringify(Object.fromEntries(new FormData(form))),
            });
            if (!res.ok) throw new Error(`HTTP ${res.status}`);
            setState('success');
            form.reset();
            world?.celebrate();
        } catch {
            setState('error');
        }
    });
}

function initContent() {
    const modalOpen = Boolean(document.querySelector('[data-modal]'));
    document.querySelectorAll('.topbar, .dock').forEach((el) => { el.inert = modalOpen; });
    initContactForm();
}

function webglAvailable() {
    try {
        const probe = document.createElement('canvas');
        return Boolean(probe.getContext('webgl2') || probe.getContext('webgl'));
    } catch {
        return false;
    }
}

async function initWorld() {
    const canvas = document.getElementById('world');
    if (!canvas || !webglAvailable() || navigator.connection?.saveData) return;
    const loading = document.querySelector('[data-loading]');
    loading.hidden = false;
    try {
        const { createWorld } = await import('./world/world.js');
        currentView = viewOf(document.getElementById('main-content'));
        world = createWorld(canvas, {
            mood: root.dataset.mood,
            island: currentView,
            labels: JSON.parse(canvas.dataset.signposts || '[]'),
            models: '/models',
            onPick: (key) => {
                const href = islandHref(key);
                if (href) navigate(href);
            },
        });
        world.trackPins(document.querySelectorAll('[data-pin]'));
        root.classList.add('world-ready');
        initNavigation();
        await world.ready;
    } catch (err) {
        console.error('world failed to load', err);
    } finally {
        loading.hidden = true;
    }
}

initMoods();
initContent();
initWorld();
