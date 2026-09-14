(function () {
    var HINT_CHARS = 'sadfjklewcmpgh';
    var SCROLL_STEP = 60;
    var TARGETS = 'a[href], button, input:not([type=hidden]), select, textarea,' +
        '[role="button"], [role="link"], [data-action], [tabindex]:not([tabindex="-1"])';
    var REVEALED = '.cardOverlayContainer, .cardOverlayButton, .cardOverlayFab';

    var host = null;
    var layer = null;
    var revealStyle = null;
    var hints = [];
    var typed = '';

    function videoActive() {
        return !!document.querySelector('.videoPlayerContainer');
    }

    function isEditable(el) {
        if (!el) return false;
        var tag = el.tagName;
        return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || el.isContentEditable;
    }

    function scroller() {
        var doc = document.scrollingElement || document.documentElement;
        if (doc && doc.scrollHeight > doc.clientHeight + 4) return doc;
        var nodes = document.querySelectorAll('.mainAnimatedPage, .skinBody, main, .scrollY, .itemsContainer');
        for (var i = 0; i < nodes.length; i++) {
            var n = nodes[i];
            if (n.scrollHeight <= n.clientHeight + 4) continue;
            var oy = getComputedStyle(n).overflowY;
            if (oy === 'auto' || oy === 'scroll' || oy === 'overlay') return n;
        }
        return doc;
    }

    function scroll(delta) {
        var el = scroller();
        if (el === document.scrollingElement || el === document.documentElement) {
            window.scrollBy(0, delta);
        } else {
            el.scrollTop += delta;
        }
    }

    function labels(n) {
        var width = 1;
        for (var cap = HINT_CHARS.length; cap < n; cap *= HINT_CHARS.length) width++;
        var out = [];
        for (var i = 0; i < n; i++) {
            var s = '';
            var x = i;
            for (var k = 0; k < width; k++) {
                s = HINT_CHARS[x % HINT_CHARS.length] + s;
                x = Math.floor(x / HINT_CHARS.length);
            }
            out.push(s);
        }
        return out;
    }

    function styleOf(el, cache) {
        var st = cache.get(el);
        if (!st) {
            st = getComputedStyle(el);
            cache.set(el, st);
        }
        return st;
    }

    function clipRect(el, cache, skipOpacity) {
        var r = el.getBoundingClientRect();
        var left = r.left, top = r.top, right = r.right, bottom = r.bottom;
        var clipping = styleOf(el, cache).position !== 'fixed';
        for (var p = el.parentElement; p; p = p.parentElement) {
            var st = styleOf(p, cache);
            if (!skipOpacity && (st.opacity === '0' || st.visibility === 'hidden')) return null;
            if (clipping && (st.overflowX !== 'visible' || st.overflowY !== 'visible')) {
                var pr = p.getBoundingClientRect();
                if (st.overflowX !== 'visible') {
                    left = Math.max(left, pr.left);
                    right = Math.min(right, pr.right);
                }
                if (st.overflowY !== 'visible') {
                    top = Math.max(top, pr.top);
                    bottom = Math.min(bottom, pr.bottom);
                }
            }
            if (st.position === 'fixed') clipping = false;
        }
        return {
            left: Math.max(left, 0),
            top: Math.max(top, 0),
            right: Math.min(right, window.innerWidth),
            bottom: Math.min(bottom, window.innerHeight)
        };
    }

    function candidates() {
        var vw = window.innerWidth;
        var vh = window.innerHeight;
        var nodes = document.querySelectorAll(TARGETS);
        var cache = new Map();
        var seen = new Set();
        var out = [];
        for (var i = 0; i < nodes.length; i++) {
            var el = nodes[i];
            if (el.disabled) continue;
            var st = styleOf(el, cache);
            if (st.display === 'none') continue;
            var revealed = el.closest ? !!el.closest(REVEALED) : false;
            if (!revealed && (st.visibility === 'hidden' || st.opacity === '0')) continue;
            var box = clipRect(el, cache, revealed);
            if (!box) continue;
            var w = box.right - box.left;
            var h = box.bottom - box.top;
            if (w < 8 || h < 8) continue;
            if (w > vw * 0.9 && h > vh * 0.9) continue;
            var key = Math.round(box.left) + ':' + Math.round(box.top) + ':' +
                Math.round(w) + ':' + Math.round(h);
            if (seen.has(key)) continue;
            seen.add(key);
            out.push({ el: el, x: box.left, y: box.top });
        }
        out.sort(function (a, b) {
            return (Math.round(a.y / 20) - Math.round(b.y / 20)) || (a.x - b.x);
        });
        return out;
    }

    function buildHost() {
        if (host) {
            if (!host.isConnected) (document.body || document.documentElement).appendChild(host);
            return;
        }
        host = document.createElement('div');
        host.id = '_jvim';
        host.style.cssText = 'position:fixed;left:0;top:0;width:0;height:0;z-index:2147483647;pointer-events:none';
        var shadow = host.attachShadow({ mode: 'closed' });
        var style = document.createElement('style');
        style.textContent =
            '.h{position:fixed;background:#a6e3a1;color:#11111b;border:1px solid #40a02b;' +
              'border-radius:3px;padding:0 3px;font:bold 11px/1.5 monospace;' +
              'text-transform:uppercase;box-shadow:0 1px 3px rgba(17,17,27,.6)}' +
            '.h .d{opacity:.4}';
        shadow.appendChild(style);
        layer = document.createElement('div');
        shadow.appendChild(layer);
        (document.body || document.documentElement).appendChild(host);
    }

    function render() {
        var shown = 0;
        for (var i = 0; i < hints.length; i++) {
            var h = hints[i];
            var match = typed === '' || h.label.indexOf(typed) === 0;
            h.node.style.display = match ? '' : 'none';
            if (!match) continue;
            shown++;
            h.node.textContent = '';
            if (typed) {
                var done = document.createElement('span');
                done.className = 'd';
                done.textContent = typed;
                h.node.appendChild(done);
            }
            h.node.appendChild(document.createTextNode(h.label.slice(typed.length)));
        }
        if (shown === 0) clearHints();
    }

    function reveal(on) {
        if (on) {
            if (revealStyle) return;
            revealStyle = document.createElement('style');
            revealStyle.textContent = '.cardOverlayContainer,.cardOverlayButton,.cardOverlayFab' +
                '{opacity:1!important;visibility:visible!important;transition:none!important}';
            (document.head || document.documentElement).appendChild(revealStyle);
        } else if (revealStyle) {
            revealStyle.remove();
            revealStyle = null;
        }
    }

    function showHints() {
        clearHints();
        reveal(true);
        var found = candidates();
        if (!found.length) {
            reveal(false);
            return;
        }
        buildHost();
        var names = labels(found.length);
        for (var i = 0; i < found.length; i++) {
            var node = document.createElement('div');
            node.className = 'h';
            node.style.left = found[i].x + 'px';
            node.style.top = found[i].y + 'px';
            layer.appendChild(node);
            hints.push({ el: found[i].el, label: names[i], node: node });
        }
        typed = '';
        render();
        window.addEventListener('scroll', clearHints, true);
        window.addEventListener('resize', clearHints, true);
        window.addEventListener('mousedown', clearHints, true);
    }

    function clearHints() {
        reveal(false);
        if (!hints.length) return;
        hints = [];
        typed = '';
        if (layer) layer.textContent = '';
        window.removeEventListener('scroll', clearHints, true);
        window.removeEventListener('resize', clearHints, true);
        window.removeEventListener('mousedown', clearHints, true);
    }

    function activate(el) {
        clearHints();
        if (isEditable(el)) {
            el.focus();
            return;
        }
        el.click();
    }

    function goBack() {
        var page = window.Emby && window.Emby.Page;
        if (page && page.back && (!page.canGoBack || page.canGoBack())) {
            page.back();
            return;
        }
        history.back();
    }

    function focusSearchInput(tries) {
        var input = document.querySelector(
            '.searchfields input, .searchFields input, input[type=search], .searchFieldsInner input');
        if (input) {
            input.focus();
            if (input.select) input.select();
            return;
        }
        if (tries < 20) setTimeout(function () { focusSearchInput(tries + 1); }, 50);
    }

    function openSearch() {
        var btn = document.querySelector('.headerSearchButton');
        if (btn) {
            btn.click();
        } else {
            location.hash = '#/search.html';
        }
        focusSearchInput(0);
    }

    function onKey(e) {
        if (e.ctrlKey || e.altKey || e.metaKey) return;

        if (hints.length) {
            e.preventDefault();
            e.stopPropagation();
            if (e.key === 'Escape') {
                clearHints();
            } else if (e.key === 'Backspace') {
                typed = typed.slice(0, -1);
                render();
            } else if (e.key.length === 1 && HINT_CHARS.indexOf(e.key.toLowerCase()) >= 0) {
                typed += e.key.toLowerCase();
                var exact = hints.filter(function (h) { return h.label === typed; });
                if (exact.length === 1) activate(exact[0].el); else render();
            } else if (e.key.length === 1 || e.key === 'Enter') {
                clearHints();
            }
            return;
        }

        if (isEditable(e.target) || isEditable(document.activeElement)) {
            if (e.key === 'Escape') {
                e.preventDefault();
                e.stopPropagation();
                var active = document.activeElement;
                if (active && active.blur) active.blur();
            }
            return;
        }

        if (e.key === 'Escape') {
            if (window._isFullscreen) return;
            e.preventDefault();
            e.stopPropagation();
            goBack();
            return;
        }

        if (videoActive()) return;

        if (e.key === 'f') {
            e.preventDefault();
            e.stopPropagation();
            showHints();
        } else if (e.key === 'j' || e.key === 'k') {
            e.preventDefault();
            e.stopPropagation();
            scroll(e.key === 'j' ? SCROLL_STEP : -SCROLL_STEP);
        } else if (e.key === '/') {
            e.preventDefault();
            e.stopPropagation();
            openSearch();
        }
    }

    document.addEventListener('keydown', onKey, true);
})();
