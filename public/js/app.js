(() => {
    'use strict';

    const prefersReducedMotion = window.matchMedia('(prefers-color-scheme: reduce)').matches;

    const themeToggle = () => {
        const btn = document.querySelector('[data-theme-toggle]');
        if (!btn) return;
        const apply = (theme) => {
            document.documentElement.setAttribute('data-theme', theme);
            try { localStorage.setItem('theme', theme); } catch (e) { /* opaque storage */ }
            document
                .querySelectorAll('[data-theme-icon]')
                .forEach((el) => el.classList.toggle('hidden', el.dataset.themeIcon !== (theme === 'dark' ? 'dark' : 'light')));
        };
        const initial = document.documentElement.getAttribute('data-theme') || 'dark';
        apply(initial);
        btn.addEventListener('click', () => {
            const next = document.documentElement.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
            apply(next);
        });
    };

    const mobileMenu = () => {
        const trigger = document.querySelector('[data-mobile-menu-toggle]');
        const menu = document.querySelector('[data-mobile-menu]');
        if (!trigger || !menu) return;
        const setOpen = (open) => {
            menu.classList.toggle('hidden', !open);
            trigger.setAttribute('aria-expanded', String(open));
            const closed = trigger.querySelector('[data-menu-icon="closed"]');
            const opened = trigger.querySelector('[data-menu-icon="open"]');
            if (closed) closed.classList.toggle('hidden', open);
            if (opened) opened.classList.toggle('hidden', !open);
        };
        trigger.addEventListener('click', () => {
            setOpen(menu.classList.contains('hidden'));
        });
        document.querySelectorAll('[data-mobile-link]').forEach((link) => {
            link.addEventListener('click', () => setOpen(false));
        });
    };

    const backToTop = () => {
        const btn = document.querySelector('[data-back-to-top]');
        if (!btn) return;
        const onScroll = () => {
            const visible = window.scrollY > 400;
            btn.style.opacity = visible ? '1' : '0';
            btn.style.transform = visible ? 'translateY(0)' : 'translateY(1rem)';
            btn.style.pointerEvents = visible ? 'auto' : 'none';
        };
        window.addEventListener('scroll', onScroll, { passive: true });
        btn.addEventListener('click', () => {
            window.scrollTo({ top: 0, behavior: prefersReducedMotion ? 'auto' : 'smooth' });
        });
        onScroll();
    };

    const fadeIn = () => {
        const elements = document.querySelectorAll('[data-fade]');
        if (!elements.length) return;
        if (!('IntersectionObserver' in window) || prefersReducedMotion) {
            return;
        }
        const io = new IntersectionObserver(
            (entries) => {
                for (const e of entries) {
                    if (e.isIntersecting) {
                        e.target.classList.add('is-visible');
                        io.unobserve(e.target);
                    }
                }
            },
            { threshold: 0.08, rootMargin: '0px 0px -8% 0px' },
        );
        elements.forEach((el) => {
            el.setAttribute('data-fade-init', '');
            io.observe(el);
        });
    };

    const magnetic = () => {
        if (prefersReducedMotion) return;
        const isTouch = matchMedia('(hover: none)').matches;
        if (isTouch) return;
        document.querySelectorAll('[data-magnetic]').forEach((el) => {
            const move = (e) => {
                const r = el.getBoundingClientRect();
                const x = e.clientX - (r.left + r.width / 2);
                const y = e.clientY - (r.top + r.height / 2);
                el.style.transform = `translate(${x * 0.18}px, ${y * 0.18}px)`;
                el.style.transition = 'transform 0.15s ease-out';
            };
            const reset = () => {
                el.style.transform = 'translate(0, 0)';
                el.style.transition = 'transform 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94)';
            };
            el.addEventListener('mousemove', move);
            el.addEventListener('mouseleave', reset);
        });
    };

    const cardSpotlight = () => {
        if (prefersReducedMotion) return;
        document.querySelectorAll('.card-interactive').forEach((card) => {
            card.addEventListener('mousemove', (e) => {
                const r = card.getBoundingClientRect();
                card.style.setProperty('--mx', `${e.clientX - r.left}px`);
                card.style.setProperty('--my', `${e.clientY - r.top}px`);
            });
        });
    };

    const projectsFilter = () => {
        const grid = document.querySelector('[data-projects-grid]');
        if (!grid) return;
        const cards = Array.from(grid.querySelectorAll('[data-project-card]'));
        const search = document.querySelector('[data-projects-search]');
        const statusSel = document.querySelector('[data-projects-status]');
        const techSel = document.querySelector('[data-projects-technology]');
        const reset = document.querySelector('[data-projects-reset]');
        const empty = document.querySelector('[data-projects-empty]');

        const apply = () => {
            const q = (search?.value || '').toLowerCase().trim();
            const status = statusSel?.value || '';
            const tech = techSel?.value || '';
            let visible = 0;
            for (const card of cards) {
                const title = card.dataset.title || '';
                const desc = card.dataset.description || '';
                const cardStatus = card.dataset.status || '';
                const techs = (card.dataset.technologies || '').split(',');
                const matchesQ = !q || title.includes(q) || desc.includes(q);
                const matchesStatus = !status || cardStatus === status;
                const matchesTech = !tech || techs.includes(tech);
                const show = matchesQ && matchesStatus && matchesTech;
                card.classList.toggle('hidden', !show);
                if (show) visible += 1;
            }
            if (empty) empty.classList.toggle('hidden', visible !== 0);
        };

        [search, statusSel, techSel].forEach((el) => el?.addEventListener('input', apply));
        statusSel?.addEventListener('change', apply);
        techSel?.addEventListener('change', apply);
        reset?.addEventListener('click', () => {
            if (search) search.value = '';
            if (statusSel) statusSel.value = '';
            if (techSel) techSel.value = '';
            apply();
        });
    };

    const contactFormToggle = () => {
        const toggle = document.querySelector('[data-contact-form-toggle]');
        const panel = document.querySelector('[data-contact-form-panel]');
        if (!toggle || !panel) return;
        toggle.addEventListener('click', () => {
            const open = panel.classList.toggle('hidden');
            toggle.setAttribute('aria-expanded', String(!open));
            if (!open) {
                const first = panel.querySelector('input, textarea');
                first?.focus();
            }
        });
    };

    const contactForm = () => {
        const form = document.querySelector('[data-contact-form]');
        if (!form) return;
        const submit = form.querySelector('[data-contact-submit]');
        const idle = form.querySelector('[data-contact-label-idle]');
        const sending = form.querySelector('[data-contact-label-sending]');
        const errBox = form.querySelector('[data-contact-error]');
        const okBox = form.querySelector('[data-contact-success]');
        const hint = document.querySelector('[data-contact-hint]');

        const setState = (state) => {
            submit?.setAttribute('aria-busy', state === 'sending' ? 'true' : 'false');
            submit && (submit.disabled = state === 'sending');
            idle?.classList.toggle('hidden', state === 'sending');
            sending?.classList.toggle('hidden', state !== 'sending');
            errBox?.classList.toggle('hidden', state !== 'error');
            okBox?.classList.toggle('hidden', state !== 'success');
        };

        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            setState('sending');
            const data = Object.fromEntries(new FormData(form).entries());
            try {
                const res = await fetch('/api/contact', {
                    method: 'POST',
                    headers: { 'content-type': 'application/json' },
                    body: JSON.stringify(data),
                });
                if (res.ok) {
                    setState('success');
                    form.reset();
                    hint?.classList.add('hidden');
                } else {
                    setState('error');
                }
            } catch (_) {
                setState('error');
            }
        });
    };

    const init = () => {
        themeToggle();
        mobileMenu();
        backToTop();
        fadeIn();
        magnetic();
        cardSpotlight();
        projectsFilter();
        contactFormToggle();
        contactForm();
    };

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init, { once: true });
    } else {
        init();
    }
})();
