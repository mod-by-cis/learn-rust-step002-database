// deno:https://esm.sh/preact@10.28.2/denonext/preact.mjs
var D;
var d;
var Y;
var ce;
var C;
var K;
var Z;
var ee;
var _e;
var V;
var $;
var j;
var te;
var M = {};
var ne = [];
var pe = /acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;
var L = Array.isArray;
function k(_, e) {
  for (var t in e) _[t] = e[t];
  return _;
}
function z(_) {
  _ && _.parentNode && _.parentNode.removeChild(_);
}
function fe(_, e, t) {
  var r2, l2, n, i = {};
  for (n in e) n == "key" ? r2 = e[n] : n == "ref" ? l2 = e[n] : i[n] = e[n];
  if (arguments.length > 2 && (i.children = arguments.length > 3 ? D.call(arguments, 2) : t), typeof _ == "function" && _.defaultProps != null) for (n in _.defaultProps) i[n] === void 0 && (i[n] = _.defaultProps[n]);
  return E(_, i, r2, l2, null);
}
function E(_, e, t, r2, l2) {
  var n = {
    type: _,
    props: e,
    key: t,
    ref: r2,
    __k: null,
    __: null,
    __b: 0,
    __e: null,
    __c: null,
    constructor: void 0,
    __v: l2 ?? ++Y,
    __i: -1,
    __u: 0
  };
  return l2 == null && d.vnode != null && d.vnode(n), n;
}
function I(_) {
  return _.children;
}
function N(_, e) {
  this.props = _, this.context = e;
}
function S(_, e) {
  if (e == null) return _.__ ? S(_.__, _.__i + 1) : null;
  for (var t; e < _.__k.length; e++) if ((t = _.__k[e]) != null && t.__e != null) return t.__e;
  return typeof _.type == "function" ? S(_) : null;
}
function oe(_) {
  var e, t;
  if ((_ = _.__) != null && _.__c != null) {
    for (_.__e = _.__c.base = null, e = 0; e < _.__k.length; e++) if ((t = _.__k[e]) != null && t.__e != null) {
      _.__e = _.__c.base = t.__e;
      break;
    }
    return oe(_);
  }
}
function B(_) {
  (!_.__d && (_.__d = true) && C.push(_) && !H.__r++ || K != d.debounceRendering) && ((K = d.debounceRendering) || Z)(H);
}
function H() {
  for (var _, e, t, r2, l2, n, i, u = 1; C.length; ) C.length > u && C.sort(ee), _ = C.shift(), u = C.length, _.__d && (t = void 0, r2 = void 0, l2 = (r2 = (e = _).__v).__e, n = [], i = [], e.__P && ((t = k({}, r2)).__v = r2.__v + 1, d.vnode && d.vnode(t), q(e.__P, t, r2, e.__n, e.__P.namespaceURI, 32 & r2.__u ? [
    l2
  ] : null, n, l2 ?? S(r2), !!(32 & r2.__u), i), t.__v = r2.__v, t.__.__k[t.__i] = t, ie(n, t, i), r2.__e = r2.__ = null, t.__e != l2 && oe(t)));
  H.__r = 0;
}
function re(_, e, t, r2, l2, n, i, u, f, s, p2) {
  var o2, v2, c2, m, g2, y, h, a = r2 && r2.__k || ne, w = e.length;
  for (f = ae(t, e, a, f, w), o2 = 0; o2 < w; o2++) (c2 = t.__k[o2]) != null && (v2 = c2.__i == -1 ? M : a[c2.__i] || M, c2.__i = o2, y = q(_, c2, v2, l2, n, i, u, f, s, p2), m = c2.__e, c2.ref && v2.ref != c2.ref && (v2.ref && G(v2.ref, null, c2), p2.push(c2.ref, c2.__c || m, c2)), g2 == null && m != null && (g2 = m), (h = !!(4 & c2.__u)) || v2.__k === c2.__k ? f = le(c2, f, _, h) : typeof c2.type == "function" && y !== void 0 ? f = y : m && (f = m.nextSibling), c2.__u &= -7);
  return t.__e = g2, f;
}
function ae(_, e, t, r2, l2) {
  var n, i, u, f, s, p2 = t.length, o2 = p2, v2 = 0;
  for (_.__k = new Array(l2), n = 0; n < l2; n++) (i = e[n]) != null && typeof i != "boolean" && typeof i != "function" ? (typeof i == "string" || typeof i == "number" || typeof i == "bigint" || i.constructor == String ? i = _.__k[n] = E(null, i, null, null, null) : L(i) ? i = _.__k[n] = E(I, {
    children: i
  }, null, null, null) : i.constructor === void 0 && i.__b > 0 ? i = _.__k[n] = E(i.type, i.props, i.key, i.ref ? i.ref : null, i.__v) : _.__k[n] = i, f = n + v2, i.__ = _, i.__b = _.__b + 1, u = null, (s = i.__i = de(i, t, f, o2)) != -1 && (o2--, (u = t[s]) && (u.__u |= 2)), u == null || u.__v == null ? (s == -1 && (l2 > p2 ? v2-- : l2 < p2 && v2++), typeof i.type != "function" && (i.__u |= 4)) : s != f && (s == f - 1 ? v2-- : s == f + 1 ? v2++ : (s > f ? v2-- : v2++, i.__u |= 4))) : _.__k[n] = null;
  if (o2) for (n = 0; n < p2; n++) (u = t[n]) != null && (2 & u.__u) == 0 && (u.__e == r2 && (r2 = S(u)), ue(u, u));
  return r2;
}
function le(_, e, t, r2) {
  var l2, n;
  if (typeof _.type == "function") {
    for (l2 = _.__k, n = 0; l2 && n < l2.length; n++) l2[n] && (l2[n].__ = _, e = le(l2[n], e, t, r2));
    return e;
  }
  _.__e != e && (r2 && (e && _.type && !e.parentNode && (e = S(_)), t.insertBefore(_.__e, e || null)), e = _.__e);
  do
    e = e && e.nextSibling;
  while (e != null && e.nodeType == 8);
  return e;
}
function de(_, e, t, r2) {
  var l2, n, i, u = _.key, f = _.type, s = e[t], p2 = s != null && (2 & s.__u) == 0;
  if (s === null && u == null || p2 && u == s.key && f == s.type) return t;
  if (r2 > (p2 ? 1 : 0)) {
    for (l2 = t - 1, n = t + 1; l2 >= 0 || n < e.length; ) if ((s = e[i = l2 >= 0 ? l2-- : n++]) != null && (2 & s.__u) == 0 && u == s.key && f == s.type) return i;
  }
  return -1;
}
function Q(_, e, t) {
  e[0] == "-" ? _.setProperty(e, t ?? "") : _[e] = t == null ? "" : typeof t != "number" || pe.test(e) ? t : t + "px";
}
function F(_, e, t, r2, l2) {
  var n, i;
  e: if (e == "style") if (typeof t == "string") _.style.cssText = t;
  else {
    if (typeof r2 == "string" && (_.style.cssText = r2 = ""), r2) for (e in r2) t && e in t || Q(_.style, e, "");
    if (t) for (e in t) r2 && t[e] == r2[e] || Q(_.style, e, t[e]);
  }
  else if (e[0] == "o" && e[1] == "n") n = e != (e = e.replace(_e, "$1")), i = e.toLowerCase(), e = i in _ || e == "onFocusOut" || e == "onFocusIn" ? i.slice(2) : e.slice(2), _.l || (_.l = {}), _.l[e + n] = t, t ? r2 ? t.u = r2.u : (t.u = V, _.addEventListener(e, n ? j : $, n)) : _.removeEventListener(e, n ? j : $, n);
  else {
    if (l2 == "http://www.w3.org/2000/svg") e = e.replace(/xlink(H|:h)/, "h").replace(/sName$/, "s");
    else if (e != "width" && e != "height" && e != "href" && e != "list" && e != "form" && e != "tabIndex" && e != "download" && e != "rowSpan" && e != "colSpan" && e != "role" && e != "popover" && e in _) try {
      _[e] = t ?? "";
      break e;
    } catch {
    }
    typeof t == "function" || (t == null || t === false && e[4] != "-" ? _.removeAttribute(e) : _.setAttribute(e, e == "popover" && t == 1 ? "" : t));
  }
}
function X(_) {
  return function(e) {
    if (this.l) {
      var t = this.l[e.type + _];
      if (e.t == null) e.t = V++;
      else if (e.t < t.u) return;
      return t(d.event ? d.event(e) : e);
    }
  };
}
function q(_, e, t, r2, l2, n, i, u, f, s) {
  var p2, o2, v2, c2, m, g2, y, h, a, w, x2, W2, U2, J, A2, T, R, b2 = e.type;
  if (e.constructor !== void 0) return null;
  128 & t.__u && (f = !!(32 & t.__u), n = [
    u = e.__e = t.__e
  ]), (p2 = d.__b) && p2(e);
  e: if (typeof b2 == "function") try {
    if (h = e.props, a = "prototype" in b2 && b2.prototype.render, w = (p2 = b2.contextType) && r2[p2.__c], x2 = p2 ? w ? w.props.value : p2.__ : r2, t.__c ? y = (o2 = e.__c = t.__c).__ = o2.__E : (a ? e.__c = o2 = new b2(h, x2) : (e.__c = o2 = new N(h, x2), o2.constructor = b2, o2.render = me), w && w.sub(o2), o2.state || (o2.state = {}), o2.__n = r2, v2 = o2.__d = true, o2.__h = [], o2._sb = []), a && o2.__s == null && (o2.__s = o2.state), a && b2.getDerivedStateFromProps != null && (o2.__s == o2.state && (o2.__s = k({}, o2.__s)), k(o2.__s, b2.getDerivedStateFromProps(h, o2.__s))), c2 = o2.props, m = o2.state, o2.__v = e, v2) a && b2.getDerivedStateFromProps == null && o2.componentWillMount != null && o2.componentWillMount(), a && o2.componentDidMount != null && o2.__h.push(o2.componentDidMount);
    else {
      if (a && b2.getDerivedStateFromProps == null && h !== c2 && o2.componentWillReceiveProps != null && o2.componentWillReceiveProps(h, x2), e.__v == t.__v || !o2.__e && o2.shouldComponentUpdate != null && o2.shouldComponentUpdate(h, o2.__s, x2) === false) {
        for (e.__v != t.__v && (o2.props = h, o2.state = o2.__s, o2.__d = false), e.__e = t.__e, e.__k = t.__k, e.__k.some(function(P) {
          P && (P.__ = e);
        }), W2 = 0; W2 < o2._sb.length; W2++) o2.__h.push(o2._sb[W2]);
        o2._sb = [], o2.__h.length && i.push(o2);
        break e;
      }
      o2.componentWillUpdate != null && o2.componentWillUpdate(h, o2.__s, x2), a && o2.componentDidUpdate != null && o2.__h.push(function() {
        o2.componentDidUpdate(c2, m, g2);
      });
    }
    if (o2.context = x2, o2.props = h, o2.__P = _, o2.__e = false, U2 = d.__r, J = 0, a) {
      for (o2.state = o2.__s, o2.__d = false, U2 && U2(e), p2 = o2.render(o2.props, o2.state, o2.context), A2 = 0; A2 < o2._sb.length; A2++) o2.__h.push(o2._sb[A2]);
      o2._sb = [];
    } else do
      o2.__d = false, U2 && U2(e), p2 = o2.render(o2.props, o2.state, o2.context), o2.state = o2.__s;
    while (o2.__d && ++J < 25);
    o2.state = o2.__s, o2.getChildContext != null && (r2 = k(k({}, r2), o2.getChildContext())), a && !v2 && o2.getSnapshotBeforeUpdate != null && (g2 = o2.getSnapshotBeforeUpdate(c2, m)), T = p2, p2 != null && p2.type === I && p2.key == null && (T = se(p2.props.children)), u = re(_, L(T) ? T : [
      T
    ], e, t, r2, l2, n, i, u, f, s), o2.base = e.__e, e.__u &= -161, o2.__h.length && i.push(o2), y && (o2.__E = o2.__ = null);
  } catch (P) {
    if (e.__v = null, f || n != null) if (P.then) {
      for (e.__u |= f ? 160 : 128; u && u.nodeType == 8 && u.nextSibling; ) u = u.nextSibling;
      n[n.indexOf(u)] = null, e.__e = u;
    } else {
      for (R = n.length; R--; ) z(n[R]);
      O(e);
    }
    else e.__e = t.__e, e.__k = t.__k, P.then || O(e);
    d.__e(P, e, t);
  }
  else n == null && e.__v == t.__v ? (e.__k = t.__k, e.__e = t.__e) : u = e.__e = ve(t.__e, e, t, r2, l2, n, i, f, s);
  return (p2 = d.diffed) && p2(e), 128 & e.__u ? void 0 : u;
}
function O(_) {
  _ && _.__c && (_.__c.__e = true), _ && _.__k && _.__k.forEach(O);
}
function ie(_, e, t) {
  for (var r2 = 0; r2 < t.length; r2++) G(t[r2], t[++r2], t[++r2]);
  d.__c && d.__c(e, _), _.some(function(l2) {
    try {
      _ = l2.__h, l2.__h = [], _.some(function(n) {
        n.call(l2);
      });
    } catch (n) {
      d.__e(n, l2.__v);
    }
  });
}
function se(_) {
  return typeof _ != "object" || _ == null || _.__b && _.__b > 0 ? _ : L(_) ? _.map(se) : k({}, _);
}
function ve(_, e, t, r2, l2, n, i, u, f) {
  var s, p2, o2, v2, c2, m, g2, y = t.props || M, h = e.props, a = e.type;
  if (a == "svg" ? l2 = "http://www.w3.org/2000/svg" : a == "math" ? l2 = "http://www.w3.org/1998/Math/MathML" : l2 || (l2 = "http://www.w3.org/1999/xhtml"), n != null) {
    for (s = 0; s < n.length; s++) if ((c2 = n[s]) && "setAttribute" in c2 == !!a && (a ? c2.localName == a : c2.nodeType == 3)) {
      _ = c2, n[s] = null;
      break;
    }
  }
  if (_ == null) {
    if (a == null) return document.createTextNode(h);
    _ = document.createElementNS(l2, a, h.is && h), u && (d.__m && d.__m(e, n), u = false), n = null;
  }
  if (a == null) y === h || u && _.data == h || (_.data = h);
  else {
    if (n = n && D.call(_.childNodes), !u && n != null) for (y = {}, s = 0; s < _.attributes.length; s++) y[(c2 = _.attributes[s]).name] = c2.value;
    for (s in y) if (c2 = y[s], s != "children") {
      if (s == "dangerouslySetInnerHTML") o2 = c2;
      else if (!(s in h)) {
        if (s == "value" && "defaultValue" in h || s == "checked" && "defaultChecked" in h) continue;
        F(_, s, null, c2, l2);
      }
    }
    for (s in h) c2 = h[s], s == "children" ? v2 = c2 : s == "dangerouslySetInnerHTML" ? p2 = c2 : s == "value" ? m = c2 : s == "checked" ? g2 = c2 : u && typeof c2 != "function" || y[s] === c2 || F(_, s, c2, y[s], l2);
    if (p2) u || o2 && (p2.__html == o2.__html || p2.__html == _.innerHTML) || (_.innerHTML = p2.__html), e.__k = [];
    else if (o2 && (_.innerHTML = ""), re(e.type == "template" ? _.content : _, L(v2) ? v2 : [
      v2
    ], e, t, r2, a == "foreignObject" ? "http://www.w3.org/1999/xhtml" : l2, n, i, n ? n[0] : t.__k && S(t, 0), u, f), n != null) for (s = n.length; s--; ) z(n[s]);
    u || (s = "value", a == "progress" && m == null ? _.removeAttribute("value") : m != null && (m !== _[s] || a == "progress" && !m || a == "option" && m != y[s]) && F(_, s, m, y[s], l2), s = "checked", g2 != null && g2 != _[s] && F(_, s, g2, y[s], l2));
  }
  return _;
}
function G(_, e, t) {
  try {
    if (typeof _ == "function") {
      var r2 = typeof _.__u == "function";
      r2 && _.__u(), r2 && e == null || (_.__u = _(e));
    } else _.current = e;
  } catch (l2) {
    d.__e(l2, t);
  }
}
function ue(_, e, t) {
  var r2, l2;
  if (d.unmount && d.unmount(_), (r2 = _.ref) && (r2.current && r2.current != _.__e || G(r2, null, e)), (r2 = _.__c) != null) {
    if (r2.componentWillUnmount) try {
      r2.componentWillUnmount();
    } catch (n) {
      d.__e(n, e);
    }
    r2.base = r2.__P = null;
  }
  if (r2 = _.__k) for (l2 = 0; l2 < r2.length; l2++) r2[l2] && ue(r2[l2], e, t || typeof _.type != "function");
  t || z(_.__e), _.__c = _.__ = _.__e = void 0;
}
function me(_, e, t) {
  return this.constructor(_, t);
}
function ye(_, e, t) {
  var r2, l2, n, i;
  e == document && (e = document.documentElement), d.__ && d.__(_, e), l2 = (r2 = typeof t == "function") ? null : t && t.__k || e.__k, n = [], i = [], q(e, _ = (!r2 && t || e).__k = fe(I, null, [
    _
  ]), l2 || M, M, e.namespaceURI, !r2 && t ? [
    t
  ] : l2 ? null : e.firstChild ? D.call(e.childNodes) : null, n, !r2 && t ? t : l2 ? l2.__e : e.firstChild, r2, i), ie(n, _, i);
}
D = ne.slice, d = {
  __e: function(_, e, t, r2) {
    for (var l2, n, i; e = e.__; ) if ((l2 = e.__c) && !l2.__) try {
      if ((n = l2.constructor) && n.getDerivedStateFromError != null && (l2.setState(n.getDerivedStateFromError(_)), i = l2.__d), l2.componentDidCatch != null && (l2.componentDidCatch(_, r2 || {}), i = l2.__d), i) return l2.__E = l2;
    } catch (u) {
      _ = u;
    }
    throw _;
  }
}, Y = 0, ce = function(_) {
  return _ != null && _.constructor === void 0;
}, N.prototype.setState = function(_, e) {
  var t;
  t = this.__s != null && this.__s != this.state ? this.__s : this.__s = k({}, this.state), typeof _ == "function" && (_ = _(k({}, t), this.props)), _ && k(t, _), _ != null && this.__v && (e && this._sb.push(e), B(this));
}, N.prototype.forceUpdate = function(_) {
  this.__v && (this.__e = true, _ && this.__h.push(_), B(this));
}, N.prototype.render = I, C = [], Z = typeof Promise == "function" ? Promise.prototype.then.bind(Promise.resolve()) : setTimeout, ee = function(_, e) {
  return _.__v.__b - e.__v.__b;
}, H.__r = 0, _e = /(PointerCapture)$|Capture$/i, V = 0, $ = X(false), j = X(true), te = 0;

// deno:https://esm.sh/preact@10.28.2/denonext/hooks.mjs
var c;
var o;
var d2;
var b;
var v = 0;
var x = [];
var r = d;
var g = r.__b;
var C2 = r.__r;
var A = r.diffed;
var D2 = r.__c;
var F2 = r.unmount;
var k2 = r.__;
function l(t, _) {
  r.__h && r.__h(o, t, v || _), v = 0;
  var u = o.__H || (o.__H = {
    __: [],
    __h: []
  });
  return t >= u.__.length && u.__.push({}), u.__[t];
}
function B2(t) {
  return v = 1, I2(U, t);
}
function I2(t, _, u) {
  var n = l(c++, 2);
  if (n.t = t, !n.__c && (n.__ = [
    u ? u(_) : U(void 0, _),
    function(f) {
      var a = n.__N ? n.__N[0] : n.__[0], s = n.t(a, f);
      a !== s && (n.__N = [
        s,
        n.__[1]
      ], n.__c.setState({}));
    }
  ], n.__c = o, !o.__f)) {
    var i = function(f, a, s) {
      if (!n.__c.__H) return true;
      var m = n.__c.__H.__.filter(function(e) {
        return !!e.__c;
      });
      if (m.every(function(e) {
        return !e.__N;
      })) return !h || h.call(this, f, a, s);
      var N2 = n.__c.props !== f;
      return m.forEach(function(e) {
        if (e.__N) {
          var P = e.__[0];
          e.__ = e.__N, e.__N = void 0, P !== e.__[0] && (N2 = true);
        }
      }), h && h.call(this, f, a, s) || N2;
    };
    o.__f = true;
    var h = o.shouldComponentUpdate, E2 = o.componentWillUpdate;
    o.componentWillUpdate = function(f, a, s) {
      if (this.__e) {
        var m = h;
        h = void 0, i(f, a, s), h = m;
      }
      E2 && E2.call(this, f, a, s);
    }, o.shouldComponentUpdate = i;
  }
  return n.__N || n.__;
}
function S2() {
  for (var t; t = x.shift(); ) if (t.__P && t.__H) try {
    t.__H.__h.forEach(p), t.__H.__h.forEach(H2), t.__H.__h = [];
  } catch (_) {
    t.__H.__h = [], r.__e(_, t.__v);
  }
}
r.__b = function(t) {
  o = null, g && g(t);
}, r.__ = function(t, _) {
  t && _.__k && _.__k.__m && (t.__m = _.__k.__m), k2 && k2(t, _);
}, r.__r = function(t) {
  C2 && C2(t), c = 0;
  var _ = (o = t.__c).__H;
  _ && (d2 === o ? (_.__h = [], o.__h = [], _.__.forEach(function(u) {
    u.__N && (u.__ = u.__N), u.u = u.__N = void 0;
  })) : (_.__h.forEach(p), _.__h.forEach(H2), _.__h = [], c = 0)), d2 = o;
}, r.diffed = function(t) {
  A && A(t);
  var _ = t.__c;
  _ && _.__H && (_.__H.__h.length && (x.push(_) !== 1 && b === r.requestAnimationFrame || ((b = r.requestAnimationFrame) || W)(S2)), _.__H.__.forEach(function(u) {
    u.u && (u.__H = u.u), u.u = void 0;
  })), d2 = o = null;
}, r.__c = function(t, _) {
  _.some(function(u) {
    try {
      u.__h.forEach(p), u.__h = u.__h.filter(function(n) {
        return !n.__ || H2(n);
      });
    } catch (n) {
      _.some(function(i) {
        i.__h && (i.__h = []);
      }), _ = [], r.__e(n, u.__v);
    }
  }), D2 && D2(t, _);
}, r.unmount = function(t) {
  F2 && F2(t);
  var _, u = t.__c;
  u && u.__H && (u.__H.__.forEach(function(n) {
    try {
      p(n);
    } catch (i) {
      _ = i;
    }
  }), u.__H = void 0, _ && r.__e(_, u.__v));
};
var q2 = typeof requestAnimationFrame == "function";
function W(t) {
  var _, u = function() {
    clearTimeout(n), q2 && cancelAnimationFrame(_), setTimeout(t);
  }, n = setTimeout(u, 35);
  q2 && (_ = requestAnimationFrame(u));
}
function p(t) {
  var _ = o, u = t.__c;
  typeof u == "function" && (t.__c = void 0, u()), o = _;
}
function H2(t) {
  var _ = o;
  t.__c = t.__(), o = _;
}
function U(t, _) {
  return typeof _ == "function" ? _(t) : _;
}

// ../../../../AppData/Local/deno/npm/registry.npmjs.org/@tauri-apps/api/2.9.1/external/tslib/tslib.es6.js
function __classPrivateFieldGet(receiver, state, kind, f) {
  if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a getter");
  if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
  return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
}
function __classPrivateFieldSet(receiver, state, value, kind, f) {
  if (kind === "m") throw new TypeError("Private method is not writable");
  if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a setter");
  if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
  return kind === "a" ? f.call(receiver, value) : f ? f.value = value : state.set(receiver, value), value;
}

// ../../../../AppData/Local/deno/npm/registry.npmjs.org/@tauri-apps/api/2.9.1/core.js
var _Channel_onmessage;
var _Channel_nextMessageIndex;
var _Channel_pendingMessages;
var _Channel_messageEndIndex;
var _Resource_rid;
var SERIALIZE_TO_IPC_FN = "__TAURI_TO_IPC_KEY__";
function transformCallback(callback, once = false) {
  return window.__TAURI_INTERNALS__.transformCallback(callback, once);
}
var Channel = class {
  constructor(onmessage) {
    _Channel_onmessage.set(this, void 0);
    _Channel_nextMessageIndex.set(this, 0);
    _Channel_pendingMessages.set(this, []);
    _Channel_messageEndIndex.set(this, void 0);
    __classPrivateFieldSet(this, _Channel_onmessage, onmessage || (() => {
    }), "f");
    this.id = transformCallback((rawMessage) => {
      const index = rawMessage.index;
      if ("end" in rawMessage) {
        if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
          this.cleanupCallback();
        } else {
          __classPrivateFieldSet(this, _Channel_messageEndIndex, index, "f");
        }
        return;
      }
      const message = rawMessage.message;
      if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
        __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message);
        __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
        while (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") in __classPrivateFieldGet(this, _Channel_pendingMessages, "f")) {
          const message2 = __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
          __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message2);
          delete __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
          __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
        }
        if (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") === __classPrivateFieldGet(this, _Channel_messageEndIndex, "f")) {
          this.cleanupCallback();
        }
      } else {
        __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[index] = message;
      }
    });
  }
  cleanupCallback() {
    window.__TAURI_INTERNALS__.unregisterCallback(this.id);
  }
  set onmessage(handler) {
    __classPrivateFieldSet(this, _Channel_onmessage, handler, "f");
  }
  get onmessage() {
    return __classPrivateFieldGet(this, _Channel_onmessage, "f");
  }
  [(_Channel_onmessage = /* @__PURE__ */ new WeakMap(), _Channel_nextMessageIndex = /* @__PURE__ */ new WeakMap(), _Channel_pendingMessages = /* @__PURE__ */ new WeakMap(), _Channel_messageEndIndex = /* @__PURE__ */ new WeakMap(), SERIALIZE_TO_IPC_FN)]() {
    return `__CHANNEL__:${this.id}`;
  }
  toJSON() {
    return this[SERIALIZE_TO_IPC_FN]();
  }
};
async function invoke(cmd, args = {}, options) {
  return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
}
_Resource_rid = /* @__PURE__ */ new WeakMap();

// src/main.ts
var styleTag = document.createElement("style");
styleTag.innerHTML = `
  .calc-btn {
    border: none;
    border-radius: 10px;
    cursor: pointer;
    color: white;
    font-weight: bold;
    font-size: 1.2rem;
    transition: all 0.1s ease;
    /* Efekt wypuk\u0142o\u015Bci (Convex) */
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
    /* Efekt wkl\u0119s\u0142o\u015Bci (Concave) */
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
  const [display, setDisplay] = B2("0");
  const [prevValue, setPrevValue] = B2(null);
  const [operator, setOperator] = B2(null);
  const [waitingForNext, setWaitingForNext] = B2(false);
  const runCalculation = async (a, b2, op) => {
    try {
      const res = await invoke("calculate_command", {
        a,
        b: b2,
        op: operator
      });
      setDisplay(res.toString());
      setPrevValue(res);
      setOperator(null);
      setWaitingForNext(true);
    } catch (err) {
      setDisplay("B\u0142\u0105d");
    }
  };
  const handleNumber = (num) => {
    if (waitingForNext) {
      setDisplay(num);
      setWaitingForNext(false);
    } else {
      setDisplay(display === "0" ? num : display + num);
    }
  };
  const handleOperator = (op) => {
    setPrevValue(parseFloat(display));
    setOperator(op);
    setWaitingForNext(true);
  };
  const performEquals = () => {
    if (operator && prevValue !== null) {
      runCalculation(prevValue, parseFloat(display), operator);
    }
  };
  const Button = (label, color, onClick, span = 1) => fe("button", {
    className: "calc-btn",
    style: `background: ${color}; grid-column: span ${span};`,
    onClick
  }, label);
  return fe("div", {
    style: "display: flex; flex-direction: column; height: 100vh; padding: 15px; background: #1a1a1a;"
  }, [
    fe("div", {
      className: "display-box"
    }, display),
    fe("div", {
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
      fe("div", {
        style: "grid-column: span 1;"
      }),
      Button("-", "#f39c12", () => handleOperator("-")),
      // Wiersz 4
      Button("1", "#333", () => handleNumber("1")),
      Button("2", "#333", () => handleNumber("2")),
      Button("3", "#333", () => handleNumber("3")),
      fe("div", {
        style: "grid-column: span 1;"
      }),
      Button("+", "#f39c12", () => handleOperator("+")),
      // Wiersz 5
      Button("0", "#333", () => handleNumber("0"), 2),
      Button(".", "#333", () => handleNumber(".")),
      Button("=", "#27ae60", performEquals, 2)
    ])
  ]);
}
var root = document.getElementById("app");
if (root) ye(fe(App, null), root);
