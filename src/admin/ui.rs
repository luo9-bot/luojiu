pub const HTML: &str = r##"<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Luo9 AI Chat Admin</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;
      -webkit-font-smoothing: antialiased;
      -moz-osx-font-smoothing: grayscale;
    }
  </style>
  <script type="module" crossorigin>(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const s of document.querySelectorAll('link[rel="modulepreload"]'))i(s);new MutationObserver(s=>{for(const r of s)if(r.type==="childList")for(const a of r.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&i(a)}).observe(document,{childList:!0,subtree:!0});function e(s){const r={};return s.integrity&&(r.integrity=s.integrity),s.referrerPolicy&&(r.referrerPolicy=s.referrerPolicy),s.crossOrigin==="use-credentials"?r.credentials="include":s.crossOrigin==="anonymous"?r.credentials="omit":r.credentials="same-origin",r}function i(s){if(s.ep)return;s.ep=!0;const r=e(s);fetch(s.href,r)}})();/**
* @vue/shared v3.5.34
* (c) 2018-present Yuxi (Evan) You and Vue contributors
* @license MIT
**/function Ha(n){const t=Object.create(null);for(const e of n.split(","))t[e]=1;return e=>e in t}const he={},us=[],Nn=()=>{},Uu=()=>!1,ao=n=>n.charCodeAt(0)===111&&n.charCodeAt(1)===110&&(n.charCodeAt(2)>122||n.charCodeAt(2)<97),lo=n=>n.startsWith("onUpdate:"),$e=Object.assign,Va=(n,t)=>{const e=n.indexOf(t);e>-1&&n.splice(e,1)},Eh=Object.prototype.hasOwnProperty,ae=(n,t)=>Eh.call(n,t),zt=Array.isArray,ds=n=>nr(n)==="[object Map]",Es=n=>nr(n)==="[object Set]",bl=n=>nr(n)==="[object Date]",Xt=n=>typeof n=="function",be=n=>typeof n=="string",Fn=n=>typeof n=="symbol",de=n=>n!==null&&typeof n=="object",Iu=n=>(de(n)||Xt(n))&&Xt(n.then)&&Xt(n.catch),Nu=Object.prototype.toString,nr=n=>Nu.call(n),Th=n=>nr(n).slice(8,-1),ku=n=>nr(n)==="[object Object]",Ga=n=>be(n)&&n!=="NaN"&&n[0]!=="-"&&""+parseInt(n,10)===n,zs=Ha(",key,ref,ref_for,ref_key,onVnodeBeforeMount,onVnodeMounted,onVnodeBeforeUpdate,onVnodeUpdated,onVnodeBeforeUnmount,onVnodeUnmounted"),co=n=>{const t=Object.create(null);return(e=>t[e]||(t[e]=n(e)))},wh=/-\w/g,Qe=co(n=>n.replace(wh,t=>t.slice(1).toUpperCase())),Ah=/\B([A-Z])/g,Mi=co(n=>n.replace(Ah,"-$1").toLowerCase()),uo=co(n=>n.charAt(0).toUpperCase()+n.slice(1)),Ro=co(n=>n?`on${uo(n)}`:""),In=(n,t)=>!Object.is(n,t),Vr=(n,...t)=>{for(let e=0;e<n.length;e++)n[e](...t)},Fu=(n,t,e,i=!1)=>{Object.defineProperty(n,t,{configurable:!0,enumerable:!1,writable:i,value:e})},ho=n=>{const t=parseFloat(n);return isNaN(t)?n:t};let Ml;const fo=()=>Ml||(Ml=typeof globalThis<"u"?globalThis:typeof self<"u"?self:typeof window<"u"?window:typeof global<"u"?global:{});function se(n){if(zt(n)){const t={};for(let e=0;e<n.length;e++){const i=n[e],s=be(i)?Ph(i):se(i);if(s)for(const r in s)t[r]=s[r]}return t}else if(be(n)||de(n))return n}const Ch=/;(?![^(]*\))/g,Rh=/:([^]+)/,Lh=/\/\*[^]*?\*\//g;function Ph(n){const t={};return n.replace(Lh,"").split(Ch).forEach(e=>{if(e){const i=e.split(Rh);i.length>1&&(t[i[0].trim()]=i[1].trim())}}),t}function ie(n){let t="";if(be(n))t=n;else if(zt(n))for(let e=0;e<n.length;e++){const i=ie(n[e]);i&&(t+=i+" ")}else if(de(n))for(const e in n)n[e]&&(t+=e+" ");return t.trim()}const Dh="itemscope,allowfullscreen,formnovalidate,ismap,nomodule,novalidate,readonly",Uh=Ha(Dh);function Ou(n){return!!n||n===""}function Ih(n,t){if(n.length!==t.length)return!1;let e=!0;for(let i=0;e&&i<n.length;i++)e=vi(n[i],t[i]);return e}function vi(n,t){if(n===t)return!0;let e=bl(n),i=bl(t);if(e||i)return e&&i?n.getTime()===t.getTime():!1;if(e=Fn(n),i=Fn(t),e||i)return n===t;if(e=zt(n),i=zt(t),e||i)return e&&i?Ih(n,t):!1;if(e=de(n),i=de(t),e||i){if(!e||!i)return!1;const s=Object.keys(n).length,r=Object.keys(t).length;if(s!==r)return!1;for(const a in n){const o=n.hasOwnProperty(a),l=t.hasOwnProperty(a);if(o&&!l||!o&&l||!vi(n[a],t[a]))return!1}}return String(n)===String(t)}function Wa(n,t){return n.findIndex(e=>vi(e,t))}const Bu=n=>!!(n&&n.__v_isRef===!0),I=n=>be(n)?n:n==null?"":zt(n)||de(n)&&(n.toString===Nu||!Xt(n.toString))?Bu(n)?I(n.value):JSON.stringify(n,zu,2):String(n),zu=(n,t)=>Bu(t)?zu(n,t.value):ds(t)?{[`Map(${t.size})`]:[...t.entries()].reduce((e,[i,s],r)=>(e[Lo(i,r)+" =>"]=s,e),{})}:Es(t)?{[`Set(${t.size})`]:[...t.values()].map(e=>Lo(e))}:Fn(t)?Lo(t):de(t)&&!zt(t)&&!ku(t)?String(t):t,Lo=(n,t="")=>{var e;return Fn(n)?`Symbol(${(e=n.description)!=null?e:t})`:n};/**
* @vue/reactivity v3.5.34
* (c) 2018-present Yuxi (Evan) You and Vue contributors
* @license MIT
**/let Ue;class Nh{constructor(t=!1){this.detached=t,this._active=!0,this._on=0,this.effects=[],this.cleanups=[],this._isPaused=!1,this._warnOnRun=!0,this.__v_skip=!0,!t&&Ue&&(Ue.active?(this.parent=Ue,this.index=(Ue.scopes||(Ue.scopes=[])).push(this)-1):(this._active=!1,this._warnOnRun=!1))}get active(){return this._active}pause(){if(this._active){this._isPaused=!0;let t,e;if(this.scopes)for(t=0,e=this.scopes.length;t<e;t++)this.scopes[t].pause();for(t=0,e=this.effects.length;t<e;t++)this.effects[t].pause()}}resume(){if(this._active&&this._isPaused){this._isPaused=!1;let t,e;if(this.scopes)for(t=0,e=this.scopes.length;t<e;t++)this.scopes[t].resume();for(t=0,e=this.effects.length;t<e;t++)this.effects[t].resume()}}run(t){if(this._active){const e=Ue;try{return Ue=this,t()}finally{Ue=e}}}on(){++this._on===1&&(this.prevScope=Ue,Ue=this)}off(){if(this._on>0&&--this._on===0){if(Ue===this)Ue=this.prevScope;else{let t=Ue;for(;t;){if(t.prevScope===this){t.prevScope=this.prevScope;break}t=t.prevScope}}this.prevScope=void 0}}stop(t){if(this._active){this._active=!1;let e,i;for(e=0,i=this.effects.length;e<i;e++)this.effects[e].stop();for(this.effects.length=0,e=0,i=this.cleanups.length;e<i;e++)this.cleanups[e]();if(this.cleanups.length=0,this.scopes){for(e=0,i=this.scopes.length;e<i;e++)this.scopes[e].stop(!0);this.scopes.length=0}if(!this.detached&&this.parent&&!t){const s=this.parent.scopes.pop();s&&s!==this&&(this.parent.scopes[this.index]=s,s.index=this.index)}this.parent=void 0}}}function kh(){return Ue}let _e;const Po=new WeakSet;class Hu{constructor(t){this.fn=t,this.deps=void 0,this.depsTail=void 0,this.flags=5,this.next=void 0,this.cleanup=void 0,this.scheduler=void 0,Ue&&(Ue.active?Ue.effects.push(this):this.flags&=-2)}pause(){this.flags|=64}resume(){this.flags&64&&(this.flags&=-65,Po.has(this)&&(Po.delete(this),this.trigger()))}notify(){this.flags&2&&!(this.flags&32)||this.flags&8||Gu(this)}run(){if(!(this.flags&1))return this.fn();this.flags|=2,Sl(this),Wu(this);const t=_e,e=Tn;_e=this,Tn=!0;try{return this.fn()}finally{$u(this),_e=t,Tn=e,this.flags&=-3}}stop(){if(this.flags&1){for(let t=this.deps;t;t=t.nextDep)qa(t);this.deps=this.depsTail=void 0,Sl(this),this.onStop&&this.onStop(),this.flags&=-2}}trigger(){this.flags&64?Po.add(this):this.scheduler?this.scheduler():this.runIfDirty()}runIfDirty(){xa(this)&&this.run()}get dirty(){return xa(this)}}let Vu=0,Hs,Vs;function Gu(n,t=!1){if(n.flags|=8,t){n.next=Vs,Vs=n;return}n.next=Hs,Hs=n}function $a(){Vu++}function Xa(){if(--Vu>0)return;if(Vs){let t=Vs;for(Vs=void 0;t;){const e=t.next;t.next=void 0,t.flags&=-9,t=e}}let n;for(;Hs;){let t=Hs;for(Hs=void 0;t;){const e=t.next;if(t.next=void 0,t.flags&=-9,t.flags&1)try{t.trigger()}catch(i){n||(n=i)}t=e}}if(n)throw n}function Wu(n){for(let t=n.deps;t;t=t.nextDep)t.version=-1,t.prevActiveLink=t.dep.activeLink,t.dep.activeLink=t}function $u(n){let t,e=n.depsTail,i=e;for(;i;){const s=i.prevDep;i.version===-1?(i===e&&(e=s),qa(i),Fh(i)):t=i,i.dep.activeLink=i.prevActiveLink,i.prevActiveLink=void 0,i=s}n.deps=t,n.depsTail=e}function xa(n){for(let t=n.deps;t;t=t.nextDep)if(t.dep.version!==t.version||t.dep.computed&&(Xu(t.dep.computed)||t.dep.version!==t.version))return!0;return!!n._dirty}function Xu(n){if(n.flags&4&&!(n.flags&16)||(n.flags&=-17,n.globalVersion===Ys)||(n.globalVersion=Ys,!n.isSSR&&n.flags&128&&(!n.deps&&!n._dirty||!xa(n))))return;n.flags|=2;const t=n.dep,e=_e,i=Tn;_e=n,Tn=!0;try{Wu(n);const s=n.fn(n._value);(t.version===0||In(s,n._value))&&(n.flags|=128,n._value=s,t.version++)}catch(s){throw t.version++,s}finally{_e=e,Tn=i,$u(n),n.flags&=-3}}function qa(n,t=!1){const{dep:e,prevSub:i,nextSub:s}=n;if(i&&(i.nextSub=s,n.prevSub=void 0),s&&(s.prevSub=i,n.nextSub=void 0),e.subs===n&&(e.subs=i,!i&&e.computed)){e.computed.flags&=-5;for(let r=e.computed.deps;r;r=r.nextDep)qa(r,!0)}!t&&!--e.sc&&e.map&&e.map.delete(e.key)}function Fh(n){const{prevDep:t,nextDep:e}=n;t&&(t.nextDep=e,n.prevDep=void 0),e&&(e.prevDep=t,n.nextDep=void 0)}let Tn=!0;const qu=[];function ei(){qu.push(Tn),Tn=!1}function ni(){const n=qu.pop();Tn=n===void 0?!0:n}function Sl(n){const{cleanup:t}=n;if(n.cleanup=void 0,t){const e=_e;_e=void 0;try{t()}finally{_e=e}}}let Ys=0;class Oh{constructor(t,e){this.sub=t,this.dep=e,this.version=e.version,this.nextDep=this.prevDep=this.nextSub=this.prevSub=this.prevActiveLink=void 0}}class ja{constructor(t){this.computed=t,this.version=0,this.activeLink=void 0,this.subs=void 0,this.map=void 0,this.key=void 0,this.sc=0,this.__v_skip=!0}track(t){if(!_e||!Tn||_e===this.computed)return;let e=this.activeLink;if(e===void 0||e.sub!==_e)e=this.activeLink=new Oh(_e,this),_e.deps?(e.prevDep=_e.depsTail,_e.depsTail.nextDep=e,_e.depsTail=e):_e.deps=_e.depsTail=e,ju(e);else if(e.version===-1&&(e.version=this.version,e.nextDep)){const i=e.nextDep;i.prevDep=e.prevDep,e.prevDep&&(e.prevDep.nextDep=i),e.prevDep=_e.depsTail,e.nextDep=void 0,_e.depsTail.nextDep=e,_e.depsTail=e,_e.deps===e&&(_e.deps=i)}return e}trigger(t){this.version++,Ys++,this.notify(t)}notify(t){$a();try{for(let e=this.subs;e;e=e.prevSub)e.sub.notify()&&e.sub.dep.notify()}finally{Xa()}}}function ju(n){if(n.dep.sc++,n.sub.flags&4){const t=n.dep.computed;if(t&&!n.dep.subs){t.flags|=20;for(let i=t.deps;i;i=i.nextDep)ju(i)}const e=n.dep.subs;e!==n&&(n.prevSub=e,e&&(e.nextSub=n)),n.dep.subs=n}}const ya=new WeakMap,ki=Symbol(""),ba=Symbol(""),Ks=Symbol("");function ze(n,t,e){if(Tn&&_e){let i=ya.get(n);i||ya.set(n,i=new Map);let s=i.get(e);s||(i.set(e,s=new ja),s.map=i,s.key=e),s.track()}}function Kn(n,t,e,i,s,r){const a=ya.get(n);if(!a){Ys++;return}const o=l=>{l&&l.trigger()};if($a(),t==="clear")a.forEach(o);else{const l=zt(n),c=l&&Ga(e);if(l&&e==="length"){const d=Number(i);a.forEach((h,p)=>{(p==="length"||p===Ks||!Fn(p)&&p>=d)&&o(h)})}else switch((e!==void 0||a.has(void 0))&&o(a.get(e)),c&&o(a.get(Ks)),t){case"add":l?c&&o(a.get("length")):(o(a.get(ki)),ds(n)&&o(a.get(ba)));break;case"delete":l||(o(a.get(ki)),ds(n)&&o(a.get(ba)));break;case"set":ds(n)&&o(a.get(ki));break}}Xa()}function Gi(n){const t=oe(n);return t===n?t:(ze(t,"iterate",Ks),gn(n)?t:t.map(wn))}function po(n){return ze(n=oe(n),"iterate",Ks),n}function Pn(n,t){return ii(n)?_s(Fi(n)?wn(t):t):wn(t)}const Bh={__proto__:null,[Symbol.iterator](){return Do(this,Symbol.iterator,n=>Pn(this,n))},concat(...n){return Gi(this).concat(...n.map(t=>zt(t)?Gi(t):t))},entries(){return Do(this,"entries",n=>(n[1]=Pn(this,n[1]),n))},every(n,t){return zn(this,"every",n,t,void 0,arguments)},filter(n,t){return zn(this,"filter",n,t,e=>e.map(i=>Pn(this,i)),arguments)},find(n,t){return zn(this,"find",n,t,e=>Pn(this,e),arguments)},findIndex(n,t){return zn(this,"findIndex",n,t,void 0,arguments)},findLast(n,t){return zn(this,"findLast",n,t,e=>Pn(this,e),arguments)},findLastIndex(n,t){return zn(this,"findLastIndex",n,t,void 0,arguments)},forEach(n,t){return zn(this,"forEach",n,t,void 0,arguments)},includes(...n){return Uo(this,"includes",n)},indexOf(...n){return Uo(this,"indexOf",n)},join(n){return Gi(this).join(n)},lastIndexOf(...n){return Uo(this,"lastIndexOf",n)},map(n,t){return zn(this,"map",n,t,void 0,arguments)},pop(){return Rs(this,"pop")},push(...n){return Rs(this,"push",n)},reduce(n,...t){return El(this,"reduce",n,t)},reduceRight(n,...t){return El(this,"reduceRight",n,t)},shift(){return Rs(this,"shift")},some(n,t){return zn(this,"some",n,t,void 0,arguments)},splice(...n){return Rs(this,"splice",n)},toReversed(){return Gi(this).toReversed()},toSorted(n){return Gi(this).toSorted(n)},toSpliced(...n){return Gi(this).toSpliced(...n)},unshift(...n){return Rs(this,"unshift",n)},values(){return Do(this,"values",n=>Pn(this,n))}};function Do(n,t,e){const i=po(n),s=i[t]();return i!==n&&!gn(n)&&(s._next=s.next,s.next=()=>{const r=s._next();return r.done||(r.value=e(r.value)),r}),s}const zh=Array.prototype;function zn(n,t,e,i,s,r){const a=po(n),o=a!==n&&!gn(n),l=a[t];if(l!==zh[t]){const h=l.apply(n,r);return o?wn(h):h}let c=e;a!==n&&(o?c=function(h,p){return e.call(this,Pn(n,h),p,n)}:e.length>2&&(c=function(h,p){return e.call(this,h,p,n)}));const d=l.call(a,c,i);return o&&s?s(d):d}function El(n,t,e,i){const s=po(n),r=s!==n&&!gn(n);let a=e,o=!1;s!==n&&(r?(o=i.length===0,a=function(c,d,h){return o&&(o=!1,c=Pn(n,c)),e.call(this,c,Pn(n,d),h,n)}):e.length>3&&(a=function(c,d,h){return e.call(this,c,d,h,n)}));const l=s[t](a,...i);return o?Pn(n,l):l}function Uo(n,t,e){const i=oe(n);ze(i,"iterate",Ks);const s=i[t](...e);return(s===-1||s===!1)&&Za(e[0])?(e[0]=oe(e[0]),i[t](...e)):s}function Rs(n,t,e=[]){ei(),$a();const i=oe(n)[t].apply(n,e);return Xa(),ni(),i}const Hh=Ha("__proto__,__v_isRef,__isVue"),Yu=new Set(Object.getOwnPropertyNames(Symbol).filter(n=>n!=="arguments"&&n!=="caller").map(n=>Symbol[n]).filter(Fn));function Vh(n){Fn(n)||(n=String(n));const t=oe(this);return ze(t,"has",n),t.hasOwnProperty(n)}class Ku{constructor(t=!1,e=!1){this._isReadonly=t,this._isShallow=e}get(t,e,i){if(e==="__v_skip")return t.__v_skip;const s=this._isReadonly,r=this._isShallow;if(e==="__v_isReactive")return!s;if(e==="__v_isReadonly")return s;if(e==="__v_isShallow")return r;if(e==="__v_raw")return i===(s?r?Jh:td:r?Qu:Ju).get(t)||Object.getPrototypeOf(t)===Object.getPrototypeOf(i)?t:void 0;const a=zt(t);if(!s){let l;if(a&&(l=Bh[e]))return l;if(e==="hasOwnProperty")return Vh}const o=Reflect.get(t,e,Ge(t)?t:i);if((Fn(e)?Yu.has(e):Hh(e))||(s||ze(t,"get",e),r))return o;if(Ge(o)){const l=a&&Ga(e)?o:o.value;return s&&de(l)?Sa(l):l}return de(o)?s?Sa(o):ir(o):o}}class Zu extends Ku{constructor(t=!1){super(!1,t)}set(t,e,i,s){let r=t[e];const a=zt(t)&&Ga(e);if(!this._isShallow){const c=ii(r);if(!gn(i)&&!ii(i)&&(r=oe(r),i=oe(i)),!a&&Ge(r)&&!Ge(i))return c||(r.value=i),!0}const o=a?Number(e)<t.length:ae(t,e),l=Reflect.set(t,e,i,Ge(t)?t:s);return t===oe(s)&&(o?In(i,r)&&Kn(t,"set",e,i):Kn(t,"add",e,i)),l}deleteProperty(t,e){const i=ae(t,e);t[e];const s=Reflect.deleteProperty(t,e);return s&&i&&Kn(t,"delete",e,void 0),s}has(t,e){const i=Reflect.has(t,e);return(!Fn(e)||!Yu.has(e))&&ze(t,"has",e),i}ownKeys(t){return ze(t,"iterate",zt(t)?"length":ki),Reflect.ownKeys(t)}}class Gh extends Ku{constructor(t=!1){super(!0,t)}set(t,e){return!0}deleteProperty(t,e){return!0}}const Wh=new Zu,$h=new Gh,Xh=new Zu(!0);const Ma=n=>n,fr=n=>Reflect.getPrototypeOf(n);function qh(n,t,e){return function(...i){const s=this.__v_raw,r=oe(s),a=ds(r),o=n==="entries"||n===Symbol.iterator&&a,l=n==="keys"&&a,c=s[n](...i),d=e?Ma:t?_s:wn;return!t&&ze(r,"iterate",l?ba:ki),$e(Object.create(c),{next(){const{value:h,done:p}=c.next();return p?{value:h,done:p}:{value:o?[d(h[0]),d(h[1])]:d(h),done:p}}})}}function pr(n){return function(...t){return n==="delete"?!1:n==="clear"?void 0:this}}function jh(n,t){const e={get(s){const r=this.__v_raw,a=oe(r),o=oe(s);n||(In(s,o)&&ze(a,"get",s),ze(a,"get",o));const{has:l}=fr(a),c=t?Ma:n?_s:wn;if(l.call(a,s))return c(r.get(s));if(l.call(a,o))return c(r.get(o));r!==a&&r.get(s)},get size(){const s=this.__v_raw;return!n&&ze(oe(s),"iterate",ki),s.size},has(s){const r=this.__v_raw,a=oe(r),o=oe(s);return n||(In(s,o)&&ze(a,"has",s),ze(a,"has",o)),s===o?r.has(s):r.has(s)||r.has(o)},forEach(s,r){const a=this,o=a.__v_raw,l=oe(o),c=t?Ma:n?_s:wn;return!n&&ze(l,"iterate",ki),o.forEach((d,h)=>s.call(r,c(d),c(h),a))}};return $e(e,n?{add:pr("add"),set:pr("set"),delete:pr("delete"),clear:pr("clear")}:{add(s){const r=oe(this),a=fr(r),o=oe(s),l=!t&&!gn(s)&&!ii(s)?o:s;return a.has.call(r,l)||In(s,l)&&a.has.call(r,s)||In(o,l)&&a.has.call(r,o)||(r.add(l),Kn(r,"add",l,l)),this},set(s,r){!t&&!gn(r)&&!ii(r)&&(r=oe(r));const a=oe(this),{has:o,get:l}=fr(a);let c=o.call(a,s);c||(s=oe(s),c=o.call(a,s));const d=l.call(a,s);return a.set(s,r),c?In(r,d)&&Kn(a,"set",s,r):Kn(a,"add",s,r),this},delete(s){const r=oe(this),{has:a,get:o}=fr(r);let l=a.call(r,s);l||(s=oe(s),l=a.call(r,s)),o&&o.call(r,s);const c=r.delete(s);return l&&Kn(r,"delete",s,void 0),c},clear(){const s=oe(this),r=s.size!==0,a=s.clear();return r&&Kn(s,"clear",void 0,void 0),a}}),["keys","values","entries",Symbol.iterator].forEach(s=>{e[s]=qh(s,n,t)}),e}function Ya(n,t){const e=jh(n,t);return(i,s,r)=>s==="__v_isReactive"?!n:s==="__v_isReadonly"?n:s==="__v_raw"?i:Reflect.get(ae(e,s)&&s in i?e:i,s,r)}const Yh={get:Ya(!1,!1)},Kh={get:Ya(!1,!0)},Zh={get:Ya(!0,!1)};const Ju=new WeakMap,Qu=new WeakMap,td=new WeakMap,Jh=new WeakMap;function Qh(n){switch(n){case"Object":case"Array":return 1;case"Map":case"Set":case"WeakMap":case"WeakSet":return 2;default:return 0}}function tf(n){return n.__v_skip||!Object.isExtensible(n)?0:Qh(Th(n))}function ir(n){return ii(n)?n:Ka(n,!1,Wh,Yh,Ju)}function ef(n){return Ka(n,!1,Xh,Kh,Qu)}function Sa(n){return Ka(n,!0,$h,Zh,td)}function Ka(n,t,e,i,s){if(!de(n)||n.__v_raw&&!(t&&n.__v_isReactive))return n;const r=tf(n);if(r===0)return n;const a=s.get(n);if(a)return a;const o=new Proxy(n,r===2?i:e);return s.set(n,o),o}function Fi(n){return ii(n)?Fi(n.__v_raw):!!(n&&n.__v_isReactive)}function ii(n){return!!(n&&n.__v_isReadonly)}function gn(n){return!!(n&&n.__v_isShallow)}function Za(n){return n?!!n.__v_raw:!1}function oe(n){const t=n&&n.__v_raw;return t?oe(t):n}function nf(n){return!ae(n,"__v_skip")&&Object.isExtensible(n)&&Fu(n,"__v_skip",!0),n}const wn=n=>de(n)?ir(n):n,_s=n=>de(n)?Sa(n):n;function Ge(n){return n?n.__v_isRef===!0:!1}function yt(n){return sf(n,!1)}function sf(n,t){return Ge(n)?n:new rf(n,t)}class rf{constructor(t,e){this.dep=new ja,this.__v_isRef=!0,this.__v_isShallow=!1,this._rawValue=e?t:oe(t),this._value=e?t:wn(t),this.__v_isShallow=e}get value(){return this.dep.track(),this._value}set value(t){const e=this._rawValue,i=this.__v_isShallow||gn(t)||ii(t);t=i?t:oe(t),In(t,e)&&(this._rawValue=t,this._value=i?t:wn(t),this.dep.trigger())}}function of(n){return Ge(n)?n.value:n}const af={get:(n,t,e)=>t==="__v_raw"?n:of(Reflect.get(n,t,e)),set:(n,t,e,i)=>{const s=n[t];return Ge(s)&&!Ge(e)?(s.value=e,!0):Reflect.set(n,t,e,i)}};function ed(n){return Fi(n)?n:new Proxy(n,af)}class lf{constructor(t,e,i){this.fn=t,this.setter=e,this._value=void 0,this.dep=new ja(this),this.__v_isRef=!0,this.deps=void 0,this.depsTail=void 0,this.flags=16,this.globalVersion=Ys-1,this.next=void 0,this.effect=this,this.__v_isReadonly=!e,this.isSSR=i}notify(){if(this.flags|=16,!(this.flags&8)&&_e!==this)return Gu(this,!0),!0}get value(){const t=this.dep.track();return Xu(this),t&&(t.version=this.dep.version),this._value}set value(t){this.setter&&this.setter(t)}}function cf(n,t,e=!1){let i,s;return Xt(n)?i=n:(i=n.get,s=n.set),new lf(i,s,e)}const mr={},Yr=new WeakMap;let Pi;function uf(n,t=!1,e=Pi){if(e){let i=Yr.get(e);i||Yr.set(e,i=[]),i.push(n)}}function df(n,t,e=he){const{immediate:i,deep:s,once:r,scheduler:a,augmentJob:o,call:l}=e,c=x=>s?x:gn(x)||s===!1||s===0?Zn(x,1):Zn(x);let d,h,p,f,v=!1,g=!1;if(Ge(n)?(h=()=>n.value,v=gn(n)):Fi(n)?(h=()=>c(n),v=!0):zt(n)?(g=!0,v=n.some(x=>Fi(x)||gn(x)),h=()=>n.map(x=>{if(Ge(x))return x.value;if(Fi(x))return c(x);if(Xt(x))return l?l(x,2):x()})):Xt(n)?t?h=l?()=>l(n,2):n:h=()=>{if(p){ei();try{p()}finally{ni()}}const x=Pi;Pi=d;try{return l?l(n,3,[f]):n(f)}finally{Pi=x}}:h=Nn,t&&s){const x=h,C=s===!0?1/0:s;h=()=>Zn(x(),C)}const _=kh(),m=()=>{d.stop(),_&&_.active&&Va(_.effects,d)};if(r&&t){const x=t;t=(...C)=>{x(...C),m()}}let E=g?new Array(n.length).fill(mr):mr;const b=x=>{if(!(!(d.flags&1)||!d.dirty&&!x))if(t){const C=d.run();if(s||v||(g?C.some((A,D)=>In(A,E[D])):In(C,E))){p&&p();const A=Pi;Pi=d;try{const D=[C,E===mr?void 0:g&&E[0]===mr?[]:E,f];E=C,l?l(t,3,D):t(...D)}finally{Pi=A}}}else d.run()};return o&&o(b),d=new Hu(h),d.scheduler=a?()=>a(b,!1):b,f=x=>uf(x,!1,d),p=d.onStop=()=>{const x=Yr.get(d);if(x){if(l)l(x,4);else for(const C of x)C();Yr.delete(d)}},t?i?b(!0):E=d.run():a?a(b.bind(null,!0),!0):d.run(),m.pause=d.pause.bind(d),m.resume=d.resume.bind(d),m.stop=m,m}function Zn(n,t=1/0,e){if(t<=0||!de(n)||n.__v_skip||(e=e||new Map,(e.get(n)||0)>=t))return n;if(e.set(n,t),t--,Ge(n))Zn(n.value,t,e);else if(zt(n))for(let i=0;i<n.length;i++)Zn(n[i],t,e);else if(Es(n)||ds(n))n.forEach(i=>{Zn(i,t,e)});else if(ku(n)){for(const i in n)Zn(n[i],t,e);for(const i of Object.getOwnPropertySymbols(n))Object.prototype.propertyIsEnumerable.call(n,i)&&Zn(n[i],t,e)}return n}/**
* @vue/runtime-core v3.5.34
* (c) 2018-present Yuxi (Evan) You and Vue contributors
* @license MIT
**/function sr(n,t,e,i){try{return i?n(...i):n()}catch(s){mo(s,t,e)}}function On(n,t,e,i){if(Xt(n)){const s=sr(n,t,e,i);return s&&Iu(s)&&s.catch(r=>{mo(r,t,e)}),s}if(zt(n)){const s=[];for(let r=0;r<n.length;r++)s.push(On(n[r],t,e,i));return s}}function mo(n,t,e,i=!0){const s=t?t.vnode:null,{errorHandler:r,throwUnhandledErrorInProduction:a}=t&&t.appContext.config||he;if(t){let o=t.parent;const l=t.proxy,c=`https://vuejs.org/error-reference/#runtime-${e}`;for(;o;){const d=o.ec;if(d){for(let h=0;h<d.length;h++)if(d[h](n,l,c)===!1)return}o=o.parent}if(r){ei(),sr(r,null,10,[n,l,c]),ni();return}}hf(n,e,s,i,a)}function hf(n,t,e,i=!0,s=!1){if(s)throw n;console.error(n)}const Ze=[];let Ln=-1;const hs=[];let hi=null,os=0;const nd=Promise.resolve();let Kr=null;function id(n){const t=Kr||nd;return n?t.then(this?n.bind(this):n):t}function ff(n){let t=Ln+1,e=Ze.length;for(;t<e;){const i=t+e>>>1,s=Ze[i],r=Zs(s);r<n||r===n&&s.flags&2?t=i+1:e=i}return t}function Ja(n){if(!(n.flags&1)){const t=Zs(n),e=Ze[Ze.length-1];!e||!(n.flags&2)&&t>=Zs(e)?Ze.push(n):Ze.splice(ff(t),0,n),n.flags|=1,sd()}}function sd(){Kr||(Kr=nd.then(od))}function pf(n){zt(n)?hs.push(...n):hi&&n.id===-1?hi.splice(os+1,0,n):n.flags&1||(hs.push(n),n.flags|=1),sd()}function Tl(n,t,e=Ln+1){for(;e<Ze.length;e++){const i=Ze[e];if(i&&i.flags&2){if(n&&i.id!==n.uid)continue;Ze.splice(e,1),e--,i.flags&4&&(i.flags&=-2),i(),i.flags&4||(i.flags&=-2)}}}function rd(n){if(hs.length){const t=[...new Set(hs)].sort((e,i)=>Zs(e)-Zs(i));if(hs.length=0,hi){hi.push(...t);return}for(hi=t,os=0;os<hi.length;os++){const e=hi[os];e.flags&4&&(e.flags&=-2),e.flags&8||e(),e.flags&=-2}hi=null,os=0}}const Zs=n=>n.id==null?n.flags&2?-1:1/0:n.id;function od(n){try{for(Ln=0;Ln<Ze.length;Ln++){const t=Ze[Ln];t&&!(t.flags&8)&&(t.flags&4&&(t.flags&=-2),sr(t,t.i,t.i?15:14),t.flags&4||(t.flags&=-2))}}finally{for(;Ln<Ze.length;Ln++){const t=Ze[Ln];t&&(t.flags&=-2)}Ln=-1,Ze.length=0,rd(),Kr=null,(Ze.length||hs.length)&&od()}}let cn=null,ad=null;function Zr(n){const t=cn;return cn=n,ad=n&&n.type.__scopeId||null,t}function mf(n,t=cn,e){if(!t||n._n)return n;const i=(...s)=>{i._d&&Fl(-1);const r=Zr(t);let a;try{a=n(...s)}finally{Zr(r),i._d&&Fl(1)}return a};return i._n=!0,i._c=!0,i._d=!0,i}function Ee(n,t){if(cn===null)return n;const e=xo(cn),i=n.dirs||(n.dirs=[]);for(let s=0;s<t.length;s++){let[r,a,o,l=he]=t[s];r&&(Xt(r)&&(r={mounted:r,updated:r}),r.deep&&Zn(a),i.push({dir:r,instance:e,value:a,oldValue:void 0,arg:o,modifiers:l}))}return n}function Ti(n,t,e,i){const s=n.dirs,r=t&&t.dirs;for(let a=0;a<s.length;a++){const o=s[a];r&&(o.oldValue=r[a].value);let l=o.dir[i];l&&(ei(),On(l,e,8,[n.el,o,n,t]),ni())}}function _f(n,t){if(He){let e=He.provides;const i=He.parent&&He.parent.provides;i===e&&(e=He.provides=Object.create(i)),e[n]=t}}function Gr(n,t,e=!1){const i=_p();if(i||fs){let s=fs?fs._context.provides:i?i.parent==null||i.ce?i.vnode.appContext&&i.vnode.appContext.provides:i.parent.provides:void 0;if(s&&n in s)return s[n];if(arguments.length>1)return e&&Xt(t)?t.call(i&&i.proxy):t}}const gf=Symbol.for("v-scx"),vf=()=>Gr(gf);function Wr(n,t,e){return ld(n,t,e)}function ld(n,t,e=he){const{immediate:i,deep:s,flush:r,once:a}=e,o=$e({},e),l=t&&i||!t&&r!=="post";let c;if(Qs){if(r==="sync"){const f=vf();c=f.__watcherHandles||(f.__watcherHandles=[])}else if(!l){const f=()=>{};return f.stop=Nn,f.resume=Nn,f.pause=Nn,f}}const d=He;o.call=(f,v,g)=>On(f,d,v,g);let h=!1;r==="post"?o.scheduler=f=>{nn(f,d&&d.suspense)}:r!=="sync"&&(h=!0,o.scheduler=(f,v)=>{v?f():Ja(f)}),o.augmentJob=f=>{t&&(f.flags|=4),h&&(f.flags|=2,d&&(f.id=d.uid,f.i=d))};const p=df(n,t,o);return Qs&&(c?c.push(p):l&&p()),p}function xf(n,t,e){const i=this.proxy,s=be(n)?n.includes(".")?cd(i,n):()=>i[n]:n.bind(i,i);let r;Xt(t)?r=t:(r=t.handler,e=t);const a=rr(this),o=ld(s,r.bind(i),e);return a(),o}function cd(n,t){const e=t.split(".");return()=>{let i=n;for(let s=0;s<e.length&&i;s++)i=i[e[s]];return i}}const yf=Symbol("_vte"),bf=n=>n.__isTeleport,Mf=Symbol("_leaveCb");function Qa(n,t){n.shapeFlag&6&&n.component?(n.transition=t,Qa(n.component.subTree,t)):n.shapeFlag&128?(n.ssContent.transition=t.clone(n.ssContent),n.ssFallback.transition=t.clone(n.ssFallback)):n.transition=t}function ud(n){n.ids=[n.ids[0]+n.ids[2]+++"-",0,0]}function wl(n,t){let e;return!!((e=Object.getOwnPropertyDescriptor(n,t))&&!e.configurable)}const Jr=new WeakMap;function Gs(n,t,e,i,s=!1){if(zt(n)){n.forEach((g,_)=>Gs(g,t&&(zt(t)?t[_]:t),e,i,s));return}if(Ws(i)&&!s){i.shapeFlag&512&&i.type.__asyncResolved&&i.component.subTree.component&&Gs(n,t,e,i.component.subTree);return}const r=i.shapeFlag&4?xo(i.component):i.el,a=s?null:r,{i:o,r:l}=n,c=t&&t.r,d=o.refs===he?o.refs={}:o.refs,h=o.setupState,p=oe(h),f=h===he?Uu:g=>wl(d,g)?!1:ae(p,g),v=(g,_)=>!(_&&wl(d,_));if(c!=null&&c!==l){if(Al(t),be(c))d[c]=null,f(c)&&(h[c]=null);else if(Ge(c)){const g=t;v(c,g.k)&&(c.value=null),g.k&&(d[g.k]=null)}}if(Xt(l))sr(l,o,12,[a,d]);else{const g=be(l),_=Ge(l);if(g||_){const m=()=>{if(n.f){const E=g?f(l)?h[l]:d[l]:v()||!n.k?l.value:d[n.k];if(s)zt(E)&&Va(E,r);else if(zt(E))E.includes(r)||E.push(r);else if(g)d[l]=[r],f(l)&&(h[l]=d[l]);else{const b=[r];v(l,n.k)&&(l.value=b),n.k&&(d[n.k]=b)}}else g?(d[l]=a,f(l)&&(h[l]=a)):_&&(v(l,n.k)&&(l.value=a),n.k&&(d[n.k]=a))};if(a){const E=()=>{m(),Jr.delete(n)};E.id=-1,Jr.set(n,E),nn(E,e)}else Al(n),m()}}}function Al(n){const t=Jr.get(n);t&&(t.flags|=8,Jr.delete(n))}fo().requestIdleCallback;fo().cancelIdleCallback;const Ws=n=>!!n.type.__asyncLoader,dd=n=>n.type.__isKeepAlive;function Sf(n,t){hd(n,"a",t)}function Ef(n,t){hd(n,"da",t)}function hd(n,t,e=He){const i=n.__wdc||(n.__wdc=()=>{let s=e;for(;s;){if(s.isDeactivated)return;s=s.parent}return n()});if(_o(t,i,e),e){let s=e.parent;for(;s&&s.parent;)dd(s.parent.vnode)&&Tf(i,t,e,s),s=s.parent}}function Tf(n,t,e,i){const s=_o(t,n,i,!0);An(()=>{Va(i[t],s)},e)}function _o(n,t,e=He,i=!1){if(e){const s=e[n]||(e[n]=[]),r=t.__weh||(t.__weh=(...a)=>{ei();const o=rr(e),l=On(t,e,n,a);return o(),ni(),l});return i?s.unshift(r):s.push(r),r}}const oi=n=>(t,e=He)=>{(!Qs||n==="sp")&&_o(n,(...i)=>t(...i),e)},wf=oi("bm"),ve=oi("m"),Af=oi("bu"),Cf=oi("u"),Rf=oi("bum"),An=oi("um"),Lf=oi("sp"),Pf=oi("rtg"),Df=oi("rtc");function Uf(n,t=He){_o("ec",n,t)}const If="components",fd=Symbol.for("v-ndc");function Nf(n){return be(n)?kf(If,n,!1)||n:n||fd}function kf(n,t,e=!0,i=!1){const s=cn||He;if(s){const r=s.type;{const o=bp(r,!1);if(o&&(o===t||o===Qe(t)||o===uo(Qe(t))))return r}const a=Cl(s[n]||r[n],t)||Cl(s.appContext[n],t);return!a&&i?r:a}}function Cl(n,t){return n&&(n[t]||n[Qe(t)]||n[uo(Qe(t))])}function It(n,t,e,i){let s;const r=e,a=zt(n);if(a||be(n)){const o=a&&Fi(n);let l=!1,c=!1;o&&(l=!gn(n),c=ii(n),n=po(n)),s=new Array(n.length);for(let d=0,h=n.length;d<h;d++)s[d]=t(l?c?_s(wn(n[d])):wn(n[d]):n[d],d,void 0,r)}else if(typeof n=="number"){s=new Array(n);for(let o=0;o<n;o++)s[o]=t(o+1,o,void 0,r)}else if(de(n))if(n[Symbol.iterator])s=Array.from(n,(o,l)=>t(o,l,void 0,r));else{const o=Object.keys(n);s=new Array(o.length);for(let l=0,c=o.length;l<c;l++){const d=o[l];s[l]=t(n[d],d,l,r)}}else s=[];return s}const Ea=n=>n?Nd(n)?xo(n):Ea(n.parent):null,$s=$e(Object.create(null),{$:n=>n,$el:n=>n.vnode.el,$data:n=>n.data,$props:n=>n.props,$attrs:n=>n.attrs,$slots:n=>n.slots,$refs:n=>n.refs,$parent:n=>Ea(n.parent),$root:n=>Ea(n.root),$host:n=>n.ce,$emit:n=>n.emit,$options:n=>md(n),$forceUpdate:n=>n.f||(n.f=()=>{Ja(n.update)}),$nextTick:n=>n.n||(n.n=id.bind(n.proxy)),$watch:n=>xf.bind(n)}),Io=(n,t)=>n!==he&&!n.__isScriptSetup&&ae(n,t),Ff={get({_:n},t){if(t==="__v_skip")return!0;const{ctx:e,setupState:i,data:s,props:r,accessCache:a,type:o,appContext:l}=n;if(t[0]!=="$"){const p=a[t];if(p!==void 0)switch(p){case 1:return i[t];case 2:return s[t];case 4:return e[t];case 3:return r[t]}else{if(Io(i,t))return a[t]=1,i[t];if(s!==he&&ae(s,t))return a[t]=2,s[t];if(ae(r,t))return a[t]=3,r[t];if(e!==he&&ae(e,t))return a[t]=4,e[t];Ta&&(a[t]=0)}}const c=$s[t];let d,h;if(c)return t==="$attrs"&&ze(n.attrs,"get",""),c(n);if((d=o.__cssModules)&&(d=d[t]))return d;if(e!==he&&ae(e,t))return a[t]=4,e[t];if(h=l.config.globalProperties,ae(h,t))return h[t]},set({_:n},t,e){const{data:i,setupState:s,ctx:r}=n;return Io(s,t)?(s[t]=e,!0):i!==he&&ae(i,t)?(i[t]=e,!0):ae(n.props,t)||t[0]==="$"&&t.slice(1)in n?!1:(r[t]=e,!0)},has({_:{data:n,setupState:t,accessCache:e,ctx:i,appContext:s,props:r,type:a}},o){let l;return!!(e[o]||n!==he&&o[0]!=="$"&&ae(n,o)||Io(t,o)||ae(r,o)||ae(i,o)||ae($s,o)||ae(s.config.globalProperties,o)||(l=a.__cssModules)&&l[o])},defineProperty(n,t,e){return e.get!=null?n._.accessCache[t]=0:ae(e,"value")&&this.set(n,t,e.value,null),Reflect.defineProperty(n,t,e)}};function Rl(n){return zt(n)?n.reduce((t,e)=>(t[e]=null,t),{}):n}let Ta=!0;function Of(n){const t=md(n),e=n.proxy,i=n.ctx;Ta=!1,t.beforeCreate&&Ll(t.beforeCreate,n,"bc");const{data:s,computed:r,methods:a,watch:o,provide:l,inject:c,created:d,beforeMount:h,mounted:p,beforeUpdate:f,updated:v,activated:g,deactivated:_,beforeDestroy:m,beforeUnmount:E,destroyed:b,unmounted:x,render:C,renderTracked:A,renderTriggered:D,errorCaptured:G,serverPrefetch:T,expose:R,inheritAttrs:J,components:rt,directives:mt,filters:O}=t;if(c&&Bf(c,i,null),a)for(const st in a){const et=a[st];Xt(et)&&(i[st]=et.bind(e))}if(s){const st=s.call(e,e);de(st)&&(n.data=ir(st))}if(Ta=!0,r)for(const st in r){const et=r[st],ot=Xt(et)?et.bind(e,e):Xt(et.get)?et.get.bind(e,e):Nn,ut=!Xt(et)&&Xt(et.set)?et.set.bind(e):Nn,pt=ee({get:ot,set:ut});Object.defineProperty(i,st,{enumerable:!0,configurable:!0,get:()=>pt.value,set:ft=>pt.value=ft})}if(o)for(const st in o)pd(o[st],i,e,st);if(l){const st=Xt(l)?l.call(e):l;Reflect.ownKeys(st).forEach(et=>{_f(et,st[et])})}d&&Ll(d,n,"c");function tt(st,et){zt(et)?et.forEach(ot=>st(ot.bind(e))):et&&st(et.bind(e))}if(tt(wf,h),tt(ve,p),tt(Af,f),tt(Cf,v),tt(Sf,g),tt(Ef,_),tt(Uf,G),tt(Df,A),tt(Pf,D),tt(Rf,E),tt(An,x),tt(Lf,T),zt(R))if(R.length){const st=n.exposed||(n.exposed={});R.forEach(et=>{Object.defineProperty(st,et,{get:()=>e[et],set:ot=>e[et]=ot,enumerable:!0})})}else n.exposed||(n.exposed={});C&&n.render===Nn&&(n.render=C),J!=null&&(n.inheritAttrs=J),rt&&(n.components=rt),mt&&(n.directives=mt),T&&ud(n)}function Bf(n,t,e=Nn){zt(n)&&(n=wa(n));for(const i in n){const s=n[i];let r;de(s)?"default"in s?r=Gr(s.from||i,s.default,!0):r=Gr(s.from||i):r=Gr(s),Ge(r)?Object.defineProperty(t,i,{enumerable:!0,configurable:!0,get:()=>r.value,set:a=>r.value=a}):t[i]=r}}function Ll(n,t,e){On(zt(n)?n.map(i=>i.bind(t.proxy)):n.bind(t.proxy),t,e)}function pd(n,t,e,i){let s=i.includes(".")?cd(e,i):()=>e[i];if(be(n)){const r=t[n];Xt(r)&&Wr(s,r)}else if(Xt(n))Wr(s,n.bind(e));else if(de(n))if(zt(n))n.forEach(r=>pd(r,t,e,i));else{const r=Xt(n.handler)?n.handler.bind(e):t[n.handler];Xt(r)&&Wr(s,r,n)}}function md(n){const t=n.type,{mixins:e,extends:i}=t,{mixins:s,optionsCache:r,config:{optionMergeStrategies:a}}=n.appContext,o=r.get(t);let l;return o?l=o:!s.length&&!e&&!i?l=t:(l={},s.length&&s.forEach(c=>Qr(l,c,a,!0)),Qr(l,t,a)),de(t)&&r.set(t,l),l}function Qr(n,t,e,i=!1){const{mixins:s,extends:r}=t;r&&Qr(n,r,e,!0),s&&s.forEach(a=>Qr(n,a,e,!0));for(const a in t)if(!(i&&a==="expose")){const o=zf[a]||e&&e[a];n[a]=o?o(n[a],t[a]):t[a]}return n}const zf={data:Pl,props:Dl,emits:Dl,methods:Fs,computed:Fs,beforeCreate:je,created:je,beforeMount:je,mounted:je,beforeUpdate:je,updated:je,beforeDestroy:je,beforeUnmount:je,destroyed:je,unmounted:je,activated:je,deactivated:je,errorCaptured:je,serverPrefetch:je,components:Fs,directives:Fs,watch:Vf,provide:Pl,inject:Hf};function Pl(n,t){return t?n?function(){return $e(Xt(n)?n.call(this,this):n,Xt(t)?t.call(this,this):t)}:t:n}function Hf(n,t){return Fs(wa(n),wa(t))}function wa(n){if(zt(n)){const t={};for(let e=0;e<n.length;e++)t[n[e]]=n[e];return t}return n}function je(n,t){return n?[...new Set([].concat(n,t))]:t}function Fs(n,t){return n?$e(Object.create(null),n,t):t}function Dl(n,t){return n?zt(n)&&zt(t)?[...new Set([...n,...t])]:$e(Object.create(null),Rl(n),Rl(t??{})):t}function Vf(n,t){if(!n)return t;if(!t)return n;const e=$e(Object.create(null),n);for(const i in t)e[i]=je(n[i],t[i]);return e}function _d(){return{app:null,config:{isNativeTag:Uu,performance:!1,globalProperties:{},optionMergeStrategies:{},errorHandler:void 0,warnHandler:void 0,compilerOptions:{}},mixins:[],components:{},directives:{},provides:Object.create(null),optionsCache:new WeakMap,propsCache:new WeakMap,emitsCache:new WeakMap}}let Gf=0;function Wf(n,t){return function(i,s=null){Xt(i)||(i=$e({},i)),s!=null&&!de(s)&&(s=null);const r=_d(),a=new WeakSet,o=[];let l=!1;const c=r.app={_uid:Gf++,_component:i,_props:s,_container:null,_context:r,_instance:null,version:Sp,get config(){return r.config},set config(d){},use(d,...h){return a.has(d)||(d&&Xt(d.install)?(a.add(d),d.install(c,...h)):Xt(d)&&(a.add(d),d(c,...h))),c},mixin(d){return r.mixins.includes(d)||r.mixins.push(d),c},component(d,h){return h?(r.components[d]=h,c):r.components[d]},directive(d,h){return h?(r.directives[d]=h,c):r.directives[d]},mount(d,h,p){if(!l){const f=c._ceVNode||kn(i,s);return f.appContext=r,p===!0?p="svg":p===!1&&(p=void 0),n(f,d,p),l=!0,c._container=d,d.__vue_app__=c,xo(f.component)}},onUnmount(d){o.push(d)},unmount(){l&&(On(o,c._instance,16),n(null,c._container),delete c._container.__vue_app__)},provide(d,h){return r.provides[d]=h,c},runWithContext(d){const h=fs;fs=c;try{return d()}finally{fs=h}}};return c}}let fs=null;const $f=(n,t)=>t==="modelValue"||t==="model-value"?n.modelModifiers:n[`${t}Modifiers`]||n[`${Qe(t)}Modifiers`]||n[`${Mi(t)}Modifiers`];function Xf(n,t,...e){if(n.isUnmounted)return;const i=n.vnode.props||he;let s=e;const r=t.startsWith("update:"),a=r&&$f(i,t.slice(7));a&&(a.trim&&(s=e.map(d=>be(d)?d.trim():d)),a.number&&(s=e.map(ho)));let o,l=i[o=Ro(t)]||i[o=Ro(Qe(t))];!l&&r&&(l=i[o=Ro(Mi(t))]),l&&On(l,n,6,s);const c=i[o+"Once"];if(c){if(!n.emitted)n.emitted={};else if(n.emitted[o])return;n.emitted[o]=!0,On(c,n,6,s)}}const qf=new WeakMap;function gd(n,t,e=!1){const i=e?qf:t.emitsCache,s=i.get(n);if(s!==void 0)return s;const r=n.emits;let a={},o=!1;if(!Xt(n)){const l=c=>{const d=gd(c,t,!0);d&&(o=!0,$e(a,d))};!e&&t.mixins.length&&t.mixins.forEach(l),n.extends&&l(n.extends),n.mixins&&n.mixins.forEach(l)}return!r&&!o?(de(n)&&i.set(n,null),null):(zt(r)?r.forEach(l=>a[l]=null):$e(a,r),de(n)&&i.set(n,a),a)}function go(n,t){return!n||!ao(t)?!1:(t=t.slice(2).replace(/Once$/,""),ae(n,t[0].toLowerCase()+t.slice(1))||ae(n,Mi(t))||ae(n,t))}function Ul(n){const{type:t,vnode:e,proxy:i,withProxy:s,propsOptions:[r],slots:a,attrs:o,emit:l,render:c,renderCache:d,props:h,data:p,setupState:f,ctx:v,inheritAttrs:g}=n,_=Zr(n);let m,E;try{if(e.shapeFlag&4){const x=s||i,C=x;m=Dn(c.call(C,x,d,h,f,p,v)),E=o}else{const x=t;m=Dn(x.length>1?x(h,{attrs:o,slots:a,emit:l}):x(h,null)),E=t.props?o:jf(o)}}catch(x){Xs.length=0,mo(x,n,1),m=kn(xi)}let b=m;if(E&&g!==!1){const x=Object.keys(E),{shapeFlag:C}=b;x.length&&C&7&&(r&&x.some(lo)&&(E=Yf(E,r)),b=gs(b,E,!1,!0))}return e.dirs&&(b=gs(b,null,!1,!0),b.dirs=b.dirs?b.dirs.concat(e.dirs):e.dirs),e.transition&&Qa(b,e.transition),m=b,Zr(_),m}const jf=n=>{let t;for(const e in n)(e==="class"||e==="style"||ao(e))&&((t||(t={}))[e]=n[e]);return t},Yf=(n,t)=>{const e={};for(const i in n)(!lo(i)||!(i.slice(9)in t))&&(e[i]=n[i]);return e};function Kf(n,t,e){const{props:i,children:s,component:r}=n,{props:a,children:o,patchFlag:l}=t,c=r.emitsOptions;if(t.dirs||t.transition)return!0;if(e&&l>=0){if(l&1024)return!0;if(l&16)return i?Il(i,a,c):!!a;if(l&8){const d=t.dynamicProps;for(let h=0;h<d.length;h++){const p=d[h];if(vd(a,i,p)&&!go(c,p))return!0}}}else return(s||o)&&(!o||!o.$stable)?!0:i===a?!1:i?a?Il(i,a,c):!0:!!a;return!1}function Il(n,t,e){const i=Object.keys(t);if(i.length!==Object.keys(n).length)return!0;for(let s=0;s<i.length;s++){const r=i[s];if(vd(t,n,r)&&!go(e,r))return!0}return!1}function vd(n,t,e){const i=n[e],s=t[e];return e==="style"&&de(i)&&de(s)?!vi(i,s):i!==s}function Zf({vnode:n,parent:t,suspense:e},i){for(;t;){const s=t.subTree;if(s.suspense&&s.suspense.activeBranch===n&&(s.suspense.vnode.el=s.el=i,n=s),s===n)(n=t.vnode).el=i,t=t.parent;else break}e&&e.activeBranch===n&&(e.vnode.el=i)}const xd={},yd=()=>Object.create(xd),bd=n=>Object.getPrototypeOf(n)===xd;function Jf(n,t,e,i=!1){const s={},r=yd();n.propsDefaults=Object.create(null),Md(n,t,s,r);for(const a in n.propsOptions[0])a in s||(s[a]=void 0);e?n.props=i?s:ef(s):n.type.props?n.props=s:n.props=r,n.attrs=r}function Qf(n,t,e,i){const{props:s,attrs:r,vnode:{patchFlag:a}}=n,o=oe(s),[l]=n.propsOptions;let c=!1;if((i||a>0)&&!(a&16)){if(a&8){const d=n.vnode.dynamicProps;for(let h=0;h<d.length;h++){let p=d[h];if(go(n.emitsOptions,p))continue;const f=t[p];if(l)if(ae(r,p))f!==r[p]&&(r[p]=f,c=!0);else{const v=Qe(p);s[v]=Aa(l,o,v,f,n,!1)}else f!==r[p]&&(r[p]=f,c=!0)}}}else{Md(n,t,s,r)&&(c=!0);let d;for(const h in o)(!t||!ae(t,h)&&((d=Mi(h))===h||!ae(t,d)))&&(l?e&&(e[h]!==void 0||e[d]!==void 0)&&(s[h]=Aa(l,o,h,void 0,n,!0)):delete s[h]);if(r!==o)for(const h in r)(!t||!ae(t,h))&&(delete r[h],c=!0)}c&&Kn(n.attrs,"set","")}function Md(n,t,e,i){const[s,r]=n.propsOptions;let a=!1,o;if(t)for(let l in t){if(zs(l))continue;const c=t[l];let d;s&&ae(s,d=Qe(l))?!r||!r.includes(d)?e[d]=c:(o||(o={}))[d]=c:go(n.emitsOptions,l)||(!(l in i)||c!==i[l])&&(i[l]=c,a=!0)}if(r){const l=oe(e),c=o||he;for(let d=0;d<r.length;d++){const h=r[d];e[h]=Aa(s,l,h,c[h],n,!ae(c,h))}}return a}function Aa(n,t,e,i,s,r){const a=n[e];if(a!=null){const o=ae(a,"default");if(o&&i===void 0){const l=a.default;if(a.type!==Function&&!a.skipFactory&&Xt(l)){const{propsDefaults:c}=s;if(e in c)i=c[e];else{const d=rr(s);i=c[e]=l.call(null,t),d()}}else i=l;s.ce&&s.ce._setProp(e,i)}a[0]&&(r&&!o?i=!1:a[1]&&(i===""||i===Mi(e))&&(i=!0))}return i}const tp=new WeakMap;function Sd(n,t,e=!1){const i=e?tp:t.propsCache,s=i.get(n);if(s)return s;const r=n.props,a={},o=[];let l=!1;if(!Xt(n)){const d=h=>{l=!0;const[p,f]=Sd(h,t,!0);$e(a,p),f&&o.push(...f)};!e&&t.mixins.length&&t.mixins.forEach(d),n.extends&&d(n.extends),n.mixins&&n.mixins.forEach(d)}if(!r&&!l)return de(n)&&i.set(n,us),us;if(zt(r))for(let d=0;d<r.length;d++){const h=Qe(r[d]);Nl(h)&&(a[h]=he)}else if(r)for(const d in r){const h=Qe(d);if(Nl(h)){const p=r[d],f=a[h]=zt(p)||Xt(p)?{type:p}:$e({},p),v=f.type;let g=!1,_=!0;if(zt(v))for(let m=0;m<v.length;++m){const E=v[m],b=Xt(E)&&E.name;if(b==="Boolean"){g=!0;break}else b==="String"&&(_=!1)}else g=Xt(v)&&v.name==="Boolean";f[0]=g,f[1]=_,(g||ae(f,"default"))&&o.push(h)}}const c=[a,o];return de(n)&&i.set(n,c),c}function Nl(n){return n[0]!=="$"&&!zs(n)}const tl=n=>n==="_"||n==="_ctx"||n==="$stable",el=n=>zt(n)?n.map(Dn):[Dn(n)],ep=(n,t,e)=>{if(t._n)return t;const i=mf((...s)=>el(t(...s)),e);return i._c=!1,i},Ed=(n,t,e)=>{const i=n._ctx;for(const s in n){if(tl(s))continue;const r=n[s];if(Xt(r))t[s]=ep(s,r,i);else if(r!=null){const a=el(r);t[s]=()=>a}}},Td=(n,t)=>{const e=el(t);n.slots.default=()=>e},wd=(n,t,e)=>{for(const i in t)(e||!tl(i))&&(n[i]=t[i])},np=(n,t,e)=>{const i=n.slots=yd();if(n.vnode.shapeFlag&32){const s=t._;s?(wd(i,t,e),e&&Fu(i,"_",s,!0)):Ed(t,i)}else t&&Td(n,t)},ip=(n,t,e)=>{const{vnode:i,slots:s}=n;let r=!0,a=he;if(i.shapeFlag&32){const o=t._;o?e&&o===1?r=!1:wd(s,t,e):(r=!t.$stable,Ed(t,s)),a=t}else t&&(Td(n,t),a={default:1});if(r)for(const o in s)!tl(o)&&a[o]==null&&delete s[o]},nn=lp;function sp(n){return rp(n)}function rp(n,t){const e=fo();e.__VUE__=!0;const{insert:i,remove:s,patchProp:r,createElement:a,createText:o,createComment:l,setText:c,setElementText:d,parentNode:h,nextSibling:p,setScopeId:f=Nn,insertStaticContent:v}=n,g=(M,k,B,Y=null,N=null,V=null,j=void 0,S=null,y=!!k.dynamicChildren)=>{if(M===k)return;M&&!Ls(M,k)&&(Y=Pt(M),ft(M,N,V,!0),M=null),k.patchFlag===-2&&(y=!1,k.dynamicChildren=null);const{type:U,ref:q,shapeFlag:H}=k;switch(U){case vo:_(M,k,B,Y);break;case xi:m(M,k,B,Y);break;case $r:M==null&&E(k,B,Y,j);break;case Et:rt(M,k,B,Y,N,V,j,S,y);break;default:H&1?C(M,k,B,Y,N,V,j,S,y):H&6?mt(M,k,B,Y,N,V,j,S,y):(H&64||H&128)&&U.process(M,k,B,Y,N,V,j,S,y,Nt)}q!=null&&N?Gs(q,M&&M.ref,V,k||M,!k):q==null&&M&&M.ref!=null&&Gs(M.ref,null,V,M,!0)},_=(M,k,B,Y)=>{if(M==null)i(k.el=o(k.children),B,Y);else{const N=k.el=M.el;k.children!==M.children&&c(N,k.children)}},m=(M,k,B,Y)=>{M==null?i(k.el=l(k.children||""),B,Y):k.el=M.el},E=(M,k,B,Y)=>{[M.el,M.anchor]=v(M.children,k,B,Y,M.el,M.anchor)},b=({el:M,anchor:k},B,Y)=>{let N;for(;M&&M!==k;)N=p(M),i(M,B,Y),M=N;i(k,B,Y)},x=({el:M,anchor:k})=>{let B;for(;M&&M!==k;)B=p(M),s(M),M=B;s(k)},C=(M,k,B,Y,N,V,j,S,y)=>{if(k.type==="svg"?j="svg":k.type==="math"&&(j="mathml"),M==null)A(k,B,Y,N,V,j,S,y);else{const U=M.el&&M.el._isVueCE?M.el:null;try{U&&U._beginPatch(),T(M,k,N,V,j,S,y)}finally{U&&U._endPatch()}}},A=(M,k,B,Y,N,V,j,S)=>{let y,U;const{props:q,shapeFlag:H,transition:X,dirs:lt}=M;if(y=M.el=a(M.type,V,q&&q.is,q),H&8?d(y,M.children):H&16&&G(M.children,y,null,Y,N,No(M,V),j,S),lt&&Ti(M,null,Y,"created"),D(y,M,M.scopeId,j,Y),q){for(const dt in q)dt!=="value"&&!zs(dt)&&r(y,dt,null,q[dt],V,Y);"value"in q&&r(y,"value",null,q.value,V),(U=q.onVnodeBeforeMount)&&Rn(U,Y,M)}lt&&Ti(M,null,Y,"beforeMount");const at=op(N,X);at&&X.beforeEnter(y),i(y,k,B),((U=q&&q.onVnodeMounted)||at||lt)&&nn(()=>{try{U&&Rn(U,Y,M),at&&X.enter(y),lt&&Ti(M,null,Y,"mounted")}finally{}},N)},D=(M,k,B,Y,N)=>{if(B&&f(M,B),Y)for(let V=0;V<Y.length;V++)f(M,Y[V]);if(N){let V=N.subTree;if(k===V||Ld(V.type)&&(V.ssContent===k||V.ssFallback===k)){const j=N.vnode;D(M,j,j.scopeId,j.slotScopeIds,N.parent)}}},G=(M,k,B,Y,N,V,j,S,y=0)=>{for(let U=y;U<M.length;U++){const q=M[U]=S?Yn(M[U]):Dn(M[U]);g(null,q,k,B,Y,N,V,j,S)}},T=(M,k,B,Y,N,V,j)=>{const S=k.el=M.el;let{patchFlag:y,dynamicChildren:U,dirs:q}=k;y|=M.patchFlag&16;const H=M.props||he,X=k.props||he;let lt;if(B&&wi(B,!1),(lt=X.onVnodeBeforeUpdate)&&Rn(lt,B,k,M),q&&Ti(k,M,B,"beforeUpdate"),B&&wi(B,!0),(H.innerHTML&&X.innerHTML==null||H.textContent&&X.textContent==null)&&d(S,""),U?R(M.dynamicChildren,U,S,B,Y,No(k,N),V):j||et(M,k,S,null,B,Y,No(k,N),V,!1),y>0){if(y&16)J(S,H,X,B,N);else if(y&2&&H.class!==X.class&&r(S,"class",null,X.class,N),y&4&&r(S,"style",H.style,X.style,N),y&8){const at=k.dynamicProps;for(let dt=0;dt<at.length;dt++){const ct=at[dt],_t=H[ct],$=X[ct];($!==_t||ct==="value")&&r(S,ct,_t,$,N,B)}}y&1&&M.children!==k.children&&d(S,k.children)}else!j&&U==null&&J(S,H,X,B,N);((lt=X.onVnodeUpdated)||q)&&nn(()=>{lt&&Rn(lt,B,k,M),q&&Ti(k,M,B,"updated")},Y)},R=(M,k,B,Y,N,V,j)=>{for(let S=0;S<k.length;S++){const y=M[S],U=k[S],q=y.el&&(y.type===Et||!Ls(y,U)||y.shapeFlag&198)?h(y.el):B;g(y,U,q,null,Y,N,V,j,!0)}},J=(M,k,B,Y,N)=>{if(k!==B){if(k!==he)for(const V in k)!zs(V)&&!(V in B)&&r(M,V,k[V],null,N,Y);for(const V in B){if(zs(V))continue;const j=B[V],S=k[V];j!==S&&V!=="value"&&r(M,V,S,j,N,Y)}"value"in B&&r(M,"value",k.value,B.value,N)}},rt=(M,k,B,Y,N,V,j,S,y)=>{const U=k.el=M?M.el:o(""),q=k.anchor=M?M.anchor:o("");let{patchFlag:H,dynamicChildren:X,slotScopeIds:lt}=k;lt&&(S=S?S.concat(lt):lt),M==null?(i(U,B,Y),i(q,B,Y),G(k.children||[],B,q,N,V,j,S,y)):H>0&&H&64&&X&&M.dynamicChildren&&M.dynamicChildren.length===X.length?(R(M.dynamicChildren,X,B,N,V,j,S),(k.key!=null||N&&k===N.subTree)&&Ad(M,k,!0)):et(M,k,B,q,N,V,j,S,y)},mt=(M,k,B,Y,N,V,j,S,y)=>{k.slotScopeIds=S,M==null?k.shapeFlag&512?N.ctx.activate(k,B,Y,j,y):O(k,B,Y,N,V,j,y):Z(M,k,y)},O=(M,k,B,Y,N,V,j)=>{const S=M.component=mp(M,Y,N);if(dd(M)&&(S.ctx.renderer=Nt),gp(S,!1,j),S.asyncDep){if(N&&N.registerDep(S,tt,j),!M.el){const y=S.subTree=kn(xi);m(null,y,k,B),M.placeholder=y.el}}else tt(S,M,k,B,N,V,j)},Z=(M,k,B)=>{const Y=k.component=M.component;if(Kf(M,k,B))if(Y.asyncDep&&!Y.asyncResolved){st(Y,k,B);return}else Y.next=k,Y.update();else k.el=M.el,Y.vnode=k},tt=(M,k,B,Y,N,V,j)=>{const S=()=>{if(M.isMounted){let{next:H,bu:X,u:lt,parent:at,vnode:dt}=M;{const At=Cd(M);if(At){H&&(H.el=dt.el,st(M,H,j)),At.asyncDep.then(()=>{nn(()=>{M.isUnmounted||U()},N)});return}}let ct=H,_t;wi(M,!1),H?(H.el=dt.el,st(M,H,j)):H=dt,X&&Vr(X),(_t=H.props&&H.props.onVnodeBeforeUpdate)&&Rn(_t,at,H,dt),wi(M,!0);const $=Ul(M),Rt=M.subTree;M.subTree=$,g(Rt,$,h(Rt.el),Pt(Rt),M,N,V),H.el=$.el,ct===null&&Zf(M,$.el),lt&&nn(lt,N),(_t=H.props&&H.props.onVnodeUpdated)&&nn(()=>Rn(_t,at,H,dt),N)}else{let H;const{el:X,props:lt}=k,{bm:at,m:dt,parent:ct,root:_t,type:$}=M,Rt=Ws(k);wi(M,!1),at&&Vr(at),!Rt&&(H=lt&&lt.onVnodeBeforeMount)&&Rn(H,ct,k),wi(M,!0);{_t.ce&&_t.ce._hasShadowRoot()&&_t.ce._injectChildStyle($,M.parent?M.parent.type:void 0);const At=M.subTree=Ul(M);g(null,At,B,Y,M,N,V),k.el=At.el}if(dt&&nn(dt,N),!Rt&&(H=lt&&lt.onVnodeMounted)){const At=k;nn(()=>Rn(H,ct,At),N)}(k.shapeFlag&256||ct&&Ws(ct.vnode)&&ct.vnode.shapeFlag&256)&&M.a&&nn(M.a,N),M.isMounted=!0,k=B=Y=null}};M.scope.on();const y=M.effect=new Hu(S);M.scope.off();const U=M.update=y.run.bind(y),q=M.job=y.runIfDirty.bind(y);q.i=M,q.id=M.uid,y.scheduler=()=>Ja(q),wi(M,!0),U()},st=(M,k,B)=>{k.component=M;const Y=M.vnode.props;M.vnode=k,M.next=null,Qf(M,k.props,Y,B),ip(M,k.children,B),ei(),Tl(M),ni()},et=(M,k,B,Y,N,V,j,S,y=!1)=>{const U=M&&M.children,q=M?M.shapeFlag:0,H=k.children,{patchFlag:X,shapeFlag:lt}=k;if(X>0){if(X&128){ut(U,H,B,Y,N,V,j,S,y);return}else if(X&256){ot(U,H,B,Y,N,V,j,S,y);return}}lt&8?(q&16&&Lt(U,N,V),H!==U&&d(B,H)):q&16?lt&16?ut(U,H,B,Y,N,V,j,S,y):Lt(U,N,V,!0):(q&8&&d(B,""),lt&16&&G(H,B,Y,N,V,j,S,y))},ot=(M,k,B,Y,N,V,j,S,y)=>{M=M||us,k=k||us;const U=M.length,q=k.length,H=Math.min(U,q);let X;for(X=0;X<H;X++){const lt=k[X]=y?Yn(k[X]):Dn(k[X]);g(M[X],lt,B,null,N,V,j,S,y)}U>q?Lt(M,N,V,!0,!1,H):G(k,B,Y,N,V,j,S,y,H)},ut=(M,k,B,Y,N,V,j,S,y)=>{let U=0;const q=k.length;let H=M.length-1,X=q-1;for(;U<=H&&U<=X;){const lt=M[U],at=k[U]=y?Yn(k[U]):Dn(k[U]);if(Ls(lt,at))g(lt,at,B,null,N,V,j,S,y);else break;U++}for(;U<=H&&U<=X;){const lt=M[H],at=k[X]=y?Yn(k[X]):Dn(k[X]);if(Ls(lt,at))g(lt,at,B,null,N,V,j,S,y);else break;H--,X--}if(U>H){if(U<=X){const lt=X+1,at=lt<q?k[lt].el:Y;for(;U<=X;)g(null,k[U]=y?Yn(k[U]):Dn(k[U]),B,at,N,V,j,S,y),U++}}else if(U>X)for(;U<=H;)ft(M[U],N,V,!0),U++;else{const lt=U,at=U,dt=new Map;for(U=at;U<=X;U++){const xt=k[U]=y?Yn(k[U]):Dn(k[U]);xt.key!=null&&dt.set(xt.key,U)}let ct,_t=0;const $=X-at+1;let Rt=!1,At=0;const Tt=new Array($);for(U=0;U<$;U++)Tt[U]=0;for(U=lt;U<=H;U++){const xt=M[U];if(_t>=$){ft(xt,N,V,!0);continue}let Dt;if(xt.key!=null)Dt=dt.get(xt.key);else for(ct=at;ct<=X;ct++)if(Tt[ct-at]===0&&Ls(xt,k[ct])){Dt=ct;break}Dt===void 0?ft(xt,N,V,!0):(Tt[Dt-at]=U+1,Dt>=At?At=Dt:Rt=!0,g(xt,k[Dt],B,null,N,V,j,S,y),_t++)}const Ct=Rt?ap(Tt):us;for(ct=Ct.length-1,U=$-1;U>=0;U--){const xt=at+U,Dt=k[xt],Yt=k[xt+1],le=xt+1<q?Yt.el||Rd(Yt):Y;Tt[U]===0?g(null,Dt,B,le,N,V,j,S,y):Rt&&(ct<0||U!==Ct[ct]?pt(Dt,B,le,2):ct--)}}},pt=(M,k,B,Y,N=null)=>{const{el:V,type:j,transition:S,children:y,shapeFlag:U}=M;if(U&6){pt(M.component.subTree,k,B,Y);return}if(U&128){M.suspense.move(k,B,Y);return}if(U&64){j.move(M,k,B,Nt);return}if(j===Et){i(V,k,B);for(let H=0;H<y.length;H++)pt(y[H],k,B,Y);i(M.anchor,k,B);return}if(j===$r){b(M,k,B);return}if(Y!==2&&U&1&&S)if(Y===0)S.beforeEnter(V),i(V,k,B),nn(()=>S.enter(V),N);else{const{leave:H,delayLeave:X,afterLeave:lt}=S,at=()=>{M.ctx.isUnmounted?s(V):i(V,k,B)},dt=()=>{V._isLeaving&&V[Mf](!0),H(V,()=>{at(),lt&&lt()})};X?X(V,at,dt):dt()}else i(V,k,B)},ft=(M,k,B,Y=!1,N=!1)=>{const{type:V,props:j,ref:S,children:y,dynamicChildren:U,shapeFlag:q,patchFlag:H,dirs:X,cacheIndex:lt,memo:at}=M;if(H===-2&&(N=!1),S!=null&&(ei(),Gs(S,null,B,M,!0),ni()),lt!=null&&(k.renderCache[lt]=void 0),q&256){k.ctx.deactivate(M);return}const dt=q&1&&X,ct=!Ws(M);let _t;if(ct&&(_t=j&&j.onVnodeBeforeUnmount)&&Rn(_t,k,M),q&6)bt(M.component,B,Y);else{if(q&128){M.suspense.unmount(B,Y);return}dt&&Ti(M,null,k,"beforeUnmount"),q&64?M.type.remove(M,k,B,Nt,Y):U&&!U.hasOnce&&(V!==Et||H>0&&H&64)?Lt(U,k,B,!1,!0):(V===Et&&H&384||!N&&q&16)&&Lt(y,k,B),Y&&it(M)}const $=at!=null&&lt==null;(ct&&(_t=j&&j.onVnodeUnmounted)||dt||$)&&nn(()=>{_t&&Rn(_t,k,M),dt&&Ti(M,null,k,"unmounted"),$&&(M.el=null)},B)},it=M=>{const{type:k,el:B,anchor:Y,transition:N}=M;if(k===Et){ht(B,Y);return}if(k===$r){x(M);return}const V=()=>{s(B),N&&!N.persisted&&N.afterLeave&&N.afterLeave()};if(M.shapeFlag&1&&N&&!N.persisted){const{leave:j,delayLeave:S}=N,y=()=>j(B,V);S?S(M.el,V,y):y()}else V()},ht=(M,k)=>{let B;for(;M!==k;)B=p(M),s(M),M=B;s(k)},bt=(M,k,B)=>{const{bum:Y,scope:N,job:V,subTree:j,um:S,m:y,a:U}=M;kl(y),kl(U),Y&&Vr(Y),N.stop(),V&&(V.flags|=8,ft(j,M,k,B)),S&&nn(S,k),nn(()=>{M.isUnmounted=!0},k)},Lt=(M,k,B,Y=!1,N=!1,V=0)=>{for(let j=V;j<M.length;j++)ft(M[j],k,B,Y,N)},Pt=M=>{if(M.shapeFlag&6)return Pt(M.component.subTree);if(M.shapeFlag&128)return M.suspense.next();const k=p(M.anchor||M.el),B=k&&k[yf];return B?p(B):k};let Ht=!1;const Gt=(M,k,B)=>{let Y;M==null?k._vnode&&(ft(k._vnode,null,null,!0),Y=k._vnode.component):g(k._vnode||null,M,k,null,null,null,B),k._vnode=M,Ht||(Ht=!0,Tl(Y),rd(),Ht=!1)},Nt={p:g,um:ft,m:pt,r:it,mt:O,mc:G,pc:et,pbc:R,n:Pt,o:n};return{render:Gt,hydrate:void 0,createApp:Wf(Gt)}}function No({type:n,props:t},e){return e==="svg"&&n==="foreignObject"||e==="mathml"&&n==="annotation-xml"&&t&&t.encoding&&t.encoding.includes("html")?void 0:e}function wi({effect:n,job:t},e){e?(n.flags|=32,t.flags|=4):(n.flags&=-33,t.flags&=-5)}function op(n,t){return(!n||n&&!n.pendingBranch)&&t&&!t.persisted}function Ad(n,t,e=!1){const i=n.children,s=t.children;if(zt(i)&&zt(s))for(let r=0;r<i.length;r++){const a=i[r];let o=s[r];o.shapeFlag&1&&!o.dynamicChildren&&((o.patchFlag<=0||o.patchFlag===32)&&(o=s[r]=Yn(s[r]),o.el=a.el),!e&&o.patchFlag!==-2&&Ad(a,o)),o.type===vo&&(o.patchFlag===-1&&(o=s[r]=Yn(o)),o.el=a.el),o.type===xi&&!o.el&&(o.el=a.el)}}function ap(n){const t=n.slice(),e=[0];let i,s,r,a,o;const l=n.length;for(i=0;i<l;i++){const c=n[i];if(c!==0){if(s=e[e.length-1],n[s]<c){t[i]=s,e.push(i);continue}for(r=0,a=e.length-1;r<a;)o=r+a>>1,n[e[o]]<c?r=o+1:a=o;c<n[e[r]]&&(r>0&&(t[i]=e[r-1]),e[r]=i)}}for(r=e.length,a=e[r-1];r-- >0;)e[r]=a,a=t[a];return e}function Cd(n){const t=n.subTree.component;if(t)return t.asyncDep&&!t.asyncResolved?t:Cd(t)}function kl(n){if(n)for(let t=0;t<n.length;t++)n[t].flags|=8}function Rd(n){if(n.placeholder)return n.placeholder;const t=n.component;return t?Rd(t.subTree):null}const Ld=n=>n.__isSuspense;function lp(n,t){t&&t.pendingBranch?zt(n)?t.effects.push(...n):t.effects.push(n):pf(n)}const Et=Symbol.for("v-fgt"),vo=Symbol.for("v-txt"),xi=Symbol.for("v-cmt"),$r=Symbol.for("v-stc"),Xs=[];let un=null;function L(n=!1){Xs.push(un=n?null:[])}function cp(){Xs.pop(),un=Xs[Xs.length-1]||null}let Js=1;function Fl(n,t=!1){Js+=n,n<0&&un&&t&&(un.hasOnce=!0)}function Pd(n){return n.dynamicChildren=Js>0?un||us:null,cp(),Js>0&&un&&un.push(n),n}function P(n,t,e,i,s,r){return Pd(u(n,t,e,i,s,r,!0))}function Dd(n,t,e,i,s){return Pd(kn(n,t,e,i,s,!0))}function Ud(n){return n?n.__v_isVNode===!0:!1}function Ls(n,t){return n.type===t.type&&n.key===t.key}const Id=({key:n})=>n??null,Xr=({ref:n,ref_key:t,ref_for:e})=>(typeof n=="number"&&(n=""+n),n!=null?be(n)||Ge(n)||Xt(n)?{i:cn,r:n,k:t,f:!!e}:n:null);function u(n,t=null,e=null,i=0,s=null,r=n===Et?0:1,a=!1,o=!1){const l={__v_isVNode:!0,__v_skip:!0,type:n,props:t,key:t&&Id(t),ref:t&&Xr(t),scopeId:ad,slotScopeIds:null,children:e,component:null,suspense:null,ssContent:null,ssFallback:null,dirs:null,transition:null,el:null,anchor:null,target:null,targetStart:null,targetAnchor:null,staticCount:0,shapeFlag:r,patchFlag:i,dynamicProps:s,dynamicChildren:null,appContext:null,ctx:cn};return o?(nl(l,e),r&128&&n.normalize(l)):e&&(l.shapeFlag|=be(e)?8:16),Js>0&&!a&&un&&(l.patchFlag>0||r&6)&&l.patchFlag!==32&&un.push(l),l}const kn=up;function up(n,t=null,e=null,i=0,s=null,r=!1){if((!n||n===fd)&&(n=xi),Ud(n)){const o=gs(n,t,!0);return e&&nl(o,e),Js>0&&!r&&un&&(o.shapeFlag&6?un[un.indexOf(n)]=o:un.push(o)),o.patchFlag=-2,o}if(Mp(n)&&(n=n.__vccOpts),t){t=dp(t);let{class:o,style:l}=t;o&&!be(o)&&(t.class=ie(o)),de(l)&&(Za(l)&&!zt(l)&&(l=$e({},l)),t.style=se(l))}const a=be(n)?1:Ld(n)?128:bf(n)?64:de(n)?4:Xt(n)?2:0;return u(n,t,e,i,s,a,r,!0)}function dp(n){return n?Za(n)||bd(n)?$e({},n):n:null}function gs(n,t,e=!1,i=!1){const{props:s,ref:r,patchFlag:a,children:o,transition:l}=n,c=t?hp(s||{},t):s,d={__v_isVNode:!0,__v_skip:!0,type:n.type,props:c,key:c&&Id(c),ref:t&&t.ref?e&&r?zt(r)?r.concat(Xr(t)):[r,Xr(t)]:Xr(t):r,scopeId:n.scopeId,slotScopeIds:n.slotScopeIds,children:o,target:n.target,targetStart:n.targetStart,targetAnchor:n.targetAnchor,staticCount:n.staticCount,shapeFlag:n.shapeFlag,patchFlag:t&&n.type!==Et?a===-1?16:a|16:a,dynamicProps:n.dynamicProps,dynamicChildren:n.dynamicChildren,appContext:n.appContext,dirs:n.dirs,transition:l,component:n.component,suspense:n.suspense,ssContent:n.ssContent&&gs(n.ssContent),ssFallback:n.ssFallback&&gs(n.ssFallback),placeholder:n.placeholder,el:n.el,anchor:n.anchor,ctx:n.ctx,ce:n.ce};return l&&i&&Qa(d,l.clone(d)),d}function ue(n=" ",t=0){return kn(vo,null,n,t)}function vs(n,t){const e=kn($r,null,n);return e.staticCount=t,e}function Ut(n="",t=!1){return t?(L(),Dd(xi,null,n)):kn(xi,null,n)}function Dn(n){return n==null||typeof n=="boolean"?kn(xi):zt(n)?kn(Et,null,n.slice()):Ud(n)?Yn(n):kn(vo,null,String(n))}function Yn(n){return n.el===null&&n.patchFlag!==-1||n.memo?n:gs(n)}function nl(n,t){let e=0;const{shapeFlag:i}=n;if(t==null)t=null;else if(zt(t))e=16;else if(typeof t=="object")if(i&65){const s=t.default;s&&(s._c&&(s._d=!1),nl(n,s()),s._c&&(s._d=!0));return}else{e=32;const s=t._;!s&&!bd(t)?t._ctx=cn:s===3&&cn&&(cn.slots._===1?t._=1:(t._=2,n.patchFlag|=1024))}else Xt(t)?(t={default:t,_ctx:cn},e=32):(t=String(t),i&64?(e=16,t=[ue(t)]):e=8);n.children=t,n.shapeFlag|=e}function hp(...n){const t={};for(let e=0;e<n.length;e++){const i=n[e];for(const s in i)if(s==="class")t.class!==i.class&&(t.class=ie([t.class,i.class]));else if(s==="style")t.style=se([t.style,i.style]);else if(ao(s)){const r=t[s],a=i[s];a&&r!==a&&!(zt(r)&&r.includes(a))?t[s]=r?[].concat(r,a):a:a==null&&r==null&&!lo(s)&&(t[s]=a)}else s!==""&&(t[s]=i[s])}return t}function Rn(n,t,e,i=null){On(n,t,7,[e,i])}const fp=_d();let pp=0;function mp(n,t,e){const i=n.type,s=(t?t.appContext:n.appContext)||fp,r={uid:pp++,vnode:n,type:i,parent:t,appContext:s,root:null,next:null,subTree:null,effect:null,update:null,job:null,scope:new Nh(!0),render:null,proxy:null,exposed:null,exposeProxy:null,withProxy:null,provides:t?t.provides:Object.create(s.provides),ids:t?t.ids:["",0,0],accessCache:null,renderCache:[],components:null,directives:null,propsOptions:Sd(i,s),emitsOptions:gd(i,s),emit:null,emitted:null,propsDefaults:he,inheritAttrs:i.inheritAttrs,ctx:he,data:he,props:he,attrs:he,slots:he,refs:he,setupState:he,setupContext:null,suspense:e,suspenseId:e?e.pendingId:0,asyncDep:null,asyncResolved:!1,isMounted:!1,isUnmounted:!1,isDeactivated:!1,bc:null,c:null,bm:null,m:null,bu:null,u:null,um:null,bum:null,da:null,a:null,rtg:null,rtc:null,ec:null,sp:null};return r.ctx={_:r},r.root=t?t.root:r,r.emit=Xf.bind(null,r),n.ce&&n.ce(r),r}let He=null;const _p=()=>He||cn;let to,Ca;{const n=fo(),t=(e,i)=>{let s;return(s=n[e])||(s=n[e]=[]),s.push(i),r=>{s.length>1?s.forEach(a=>a(r)):s[0](r)}};to=t("__VUE_INSTANCE_SETTERS__",e=>He=e),Ca=t("__VUE_SSR_SETTERS__",e=>Qs=e)}const rr=n=>{const t=He;return to(n),n.scope.on(),()=>{n.scope.off(),to(t)}},Ol=()=>{He&&He.scope.off(),to(null)};function Nd(n){return n.vnode.shapeFlag&4}let Qs=!1;function gp(n,t=!1,e=!1){t&&Ca(t);const{props:i,children:s}=n.vnode,r=Nd(n);Jf(n,i,r,t),np(n,s,e||t);const a=r?vp(n,t):void 0;return t&&Ca(!1),a}function vp(n,t){const e=n.type;n.accessCache=Object.create(null),n.proxy=new Proxy(n.ctx,Ff);const{setup:i}=e;if(i){ei();const s=n.setupContext=i.length>1?yp(n):null,r=rr(n),a=sr(i,n,0,[n.props,s]),o=Iu(a);if(ni(),r(),(o||n.sp)&&!Ws(n)&&ud(n),o){if(a.then(Ol,Ol),t)return a.then(l=>{Bl(n,l)}).catch(l=>{mo(l,n,0)});n.asyncDep=a}else Bl(n,a)}else kd(n)}function Bl(n,t,e){Xt(t)?n.type.__ssrInlineRender?n.ssrRender=t:n.render=t:de(t)&&(n.setupState=ed(t)),kd(n)}function kd(n,t,e){const i=n.type;n.render||(n.render=i.render||Nn);{const s=rr(n);ei();try{Of(n)}finally{ni(),s()}}}const xp={get(n,t){return ze(n,"get",""),n[t]}};function yp(n){const t=e=>{n.exposed=e||{}};return{attrs:new Proxy(n.attrs,xp),slots:n.slots,emit:n.emit,expose:t}}function xo(n){return n.exposed?n.exposeProxy||(n.exposeProxy=new Proxy(ed(nf(n.exposed)),{get(t,e){if(e in t)return t[e];if(e in $s)return $s[e](n)},has(t,e){return e in t||e in $s}})):n.proxy}function bp(n,t=!0){return Xt(n)?n.displayName||n.name:n.name||t&&n.__name}function Mp(n){return Xt(n)&&"__vccOpts"in n}const ee=(n,t)=>cf(n,t,Qs),Sp="3.5.34";/**
* @vue/runtime-dom v3.5.34
* (c) 2018-present Yuxi (Evan) You and Vue contributors
* @license MIT
**/let Ra;const zl=typeof window<"u"&&window.trustedTypes;if(zl)try{Ra=zl.createPolicy("vue",{createHTML:n=>n})}catch{}const Fd=Ra?n=>Ra.createHTML(n):n=>n,Ep="http://www.w3.org/2000/svg",Tp="http://www.w3.org/1998/Math/MathML",qn=typeof document<"u"?document:null,Hl=qn&&qn.createElement("template"),wp={insert:(n,t,e)=>{t.insertBefore(n,e||null)},remove:n=>{const t=n.parentNode;t&&t.removeChild(n)},createElement:(n,t,e,i)=>{const s=t==="svg"?qn.createElementNS(Ep,n):t==="mathml"?qn.createElementNS(Tp,n):e?qn.createElement(n,{is:e}):qn.createElement(n);return n==="select"&&i&&i.multiple!=null&&s.setAttribute("multiple",i.multiple),s},createText:n=>qn.createTextNode(n),createComment:n=>qn.createComment(n),setText:(n,t)=>{n.nodeValue=t},setElementText:(n,t)=>{n.textContent=t},parentNode:n=>n.parentNode,nextSibling:n=>n.nextSibling,querySelector:n=>qn.querySelector(n),setScopeId(n,t){n.setAttribute(t,"")},insertStaticContent(n,t,e,i,s,r){const a=e?e.previousSibling:t.lastChild;if(s&&(s===r||s.nextSibling))for(;t.insertBefore(s.cloneNode(!0),e),!(s===r||!(s=s.nextSibling)););else{Hl.innerHTML=Fd(i==="svg"?`<svg>${n}</svg>`:i==="mathml"?`<math>${n}</math>`:n);const o=Hl.content;if(i==="svg"||i==="mathml"){const l=o.firstChild;for(;l.firstChild;)o.appendChild(l.firstChild);o.removeChild(l)}t.insertBefore(o,e)}return[a?a.nextSibling:t.firstChild,e?e.previousSibling:t.lastChild]}},Ap=Symbol("_vtc");function Cp(n,t,e){const i=n[Ap];i&&(t=(t?[t,...i]:[...i]).join(" ")),t==null?n.removeAttribute("class"):e?n.setAttribute("class",t):n.className=t}const Vl=Symbol("_vod"),Rp=Symbol("_vsh"),Lp=Symbol(""),Pp=/(?:^|;)\s*display\s*:/;function Dp(n,t,e){const i=n.style,s=be(e);let r=!1;if(e&&!s){if(t)if(be(t))for(const a of t.split(";")){const o=a.slice(0,a.indexOf(":")).trim();e[o]==null&&Os(i,o,"")}else for(const a in t)e[a]==null&&Os(i,a,"");for(const a in e){a==="display"&&(r=!0);const o=e[a];o!=null?Ip(n,a,!be(t)&&t?t[a]:void 0,o)||Os(i,a,o):Os(i,a,"")}}else if(s){if(t!==e){const a=i[Lp];a&&(e+=";"+a),i.cssText=e,r=Pp.test(e)}}else t&&n.removeAttribute("style");Vl in n&&(n[Vl]=r?i.display:"",n[Rp]&&(i.display="none"))}const Gl=/\s*!important$/;function Os(n,t,e){if(zt(e))e.forEach(i=>Os(n,t,i));else if(e==null&&(e=""),t.startsWith("--"))n.setProperty(t,e);else{const i=Up(n,t);Gl.test(e)?n.setProperty(Mi(i),e.replace(Gl,""),"important"):n[i]=e}}const Wl=["Webkit","Moz","ms"],ko={};function Up(n,t){const e=ko[t];if(e)return e;let i=Qe(t);if(i!=="filter"&&i in n)return ko[t]=i;i=uo(i);for(let s=0;s<Wl.length;s++){const r=Wl[s]+i;if(r in n)return ko[t]=r}return t}function Ip(n,t,e,i){return n.tagName==="TEXTAREA"&&(t==="width"||t==="height")&&be(i)&&e===i}const $l="http://www.w3.org/1999/xlink";function Xl(n,t,e,i,s,r=Uh(t)){i&&t.startsWith("xlink:")?e==null?n.removeAttributeNS($l,t.slice(6,t.length)):n.setAttributeNS($l,t,e):e==null||r&&!Ou(e)?n.removeAttribute(t):n.setAttribute(t,r?"":Fn(e)?String(e):e)}function ql(n,t,e,i,s){if(t==="innerHTML"||t==="textContent"){e!=null&&(n[t]=t==="innerHTML"?Fd(e):e);return}const r=n.tagName;if(t==="value"&&r!=="PROGRESS"&&!r.includes("-")){const o=r==="OPTION"?n.getAttribute("value")||"":n.value,l=e==null?n.type==="checkbox"?"on":"":String(e);(o!==l||!("_value"in n))&&(n.value=l),e==null&&n.removeAttribute(t),n._value=e;return}let a=!1;if(e===""||e==null){const o=typeof n[t];o==="boolean"?e=Ou(e):e==null&&o==="string"?(e="",a=!0):o==="number"&&(e=0,a=!0)}try{n[t]=e}catch{}a&&n.removeAttribute(s||t)}function Jn(n,t,e,i){n.addEventListener(t,e,i)}function Np(n,t,e,i){n.removeEventListener(t,e,i)}const jl=Symbol("_vei");function kp(n,t,e,i,s=null){const r=n[jl]||(n[jl]={}),a=r[t];if(i&&a)a.value=i;else{const[o,l]=Fp(t);if(i){const c=r[t]=zp(i,s);Jn(n,o,c,l)}else a&&(Np(n,o,a,l),r[t]=void 0)}}const Yl=/(?:Once|Passive|Capture)$/;function Fp(n){let t;if(Yl.test(n)){t={};let i;for(;i=n.match(Yl);)n=n.slice(0,n.length-i[0].length),t[i[0].toLowerCase()]=!0}return[n[2]===":"?n.slice(3):Mi(n.slice(2)),t]}let Fo=0;const Op=Promise.resolve(),Bp=()=>Fo||(Op.then(()=>Fo=0),Fo=Date.now());function zp(n,t){const e=i=>{if(!i._vts)i._vts=Date.now();else if(i._vts<=e.attached)return;On(Hp(i,e.value),t,5,[i])};return e.value=n,e.attached=Bp(),e}function Hp(n,t){if(zt(t)){const e=n.stopImmediatePropagation;return n.stopImmediatePropagation=()=>{e.call(n),n._stopped=!0},t.map(i=>s=>!s._stopped&&i&&i(s))}else return t}const Kl=n=>n.charCodeAt(0)===111&&n.charCodeAt(1)===110&&n.charCodeAt(2)>96&&n.charCodeAt(2)<123,Vp=(n,t,e,i,s,r)=>{const a=s==="svg";t==="class"?Cp(n,i,a):t==="style"?Dp(n,e,i):ao(t)?lo(t)||kp(n,t,e,i,r):(t[0]==="."?(t=t.slice(1),!0):t[0]==="^"?(t=t.slice(1),!1):Gp(n,t,i,a))?(ql(n,t,i),!n.tagName.includes("-")&&(t==="value"||t==="checked"||t==="selected")&&Xl(n,t,i,a,r,t!=="value")):n._isVueCE&&(Wp(n,t)||n._def.__asyncLoader&&(/[A-Z]/.test(t)||!be(i)))?ql(n,Qe(t),i,r,t):(t==="true-value"?n._trueValue=i:t==="false-value"&&(n._falseValue=i),Xl(n,t,i,a))};function Gp(n,t,e,i){if(i)return!!(t==="innerHTML"||t==="textContent"||t in n&&Kl(t)&&Xt(e));if(t==="spellcheck"||t==="draggable"||t==="translate"||t==="autocorrect"||t==="sandbox"&&n.tagName==="IFRAME"||t==="form"||t==="list"&&n.tagName==="INPUT"||t==="type"&&n.tagName==="TEXTAREA")return!1;if(t==="width"||t==="height"){const s=n.tagName;if(s==="IMG"||s==="VIDEO"||s==="CANVAS"||s==="SOURCE")return!1}return Kl(t)&&be(e)?!1:t in n}function Wp(n,t){const e=n._def.props;if(!e)return!1;const i=Qe(t);return Array.isArray(e)?e.some(s=>Qe(s)===i):Object.keys(e).some(s=>Qe(s)===i)}const yi=n=>{const t=n.props["onUpdate:modelValue"]||!1;return zt(t)?e=>Vr(t,e):t};function $p(n){n.target.composing=!0}function Zl(n){const t=n.target;t.composing&&(t.composing=!1,t.dispatchEvent(new Event("input")))}const vn=Symbol("_assign");function Jl(n,t,e){return t&&(n=n.trim()),e&&(n=ho(n)),n}const Je={created(n,{modifiers:{lazy:t,trim:e,number:i}},s){n[vn]=yi(s);const r=i||s.props&&s.props.type==="number";Jn(n,t?"change":"input",a=>{a.target.composing||n[vn](Jl(n.value,e,r))}),(e||r)&&Jn(n,"change",()=>{n.value=Jl(n.value,e,r)}),t||(Jn(n,"compositionstart",$p),Jn(n,"compositionend",Zl),Jn(n,"change",Zl))},mounted(n,{value:t}){n.value=t??""},beforeUpdate(n,{value:t,oldValue:e,modifiers:{lazy:i,trim:s,number:r}},a){if(n[vn]=yi(a),n.composing)return;const o=(r||n.type==="number")&&!/^0\d/.test(n.value)?ho(n.value):n.value,l=t??"";if(o===l)return;const c=n.getRootNode();(c instanceof Document||c instanceof ShadowRoot)&&c.activeElement===n&&n.type!=="range"&&(i&&t===e||s&&n.value.trim()===l)||(n.value=l)}},Xp={deep:!0,created(n,t,e){n[vn]=yi(e),Jn(n,"change",()=>{const i=n._modelValue,s=xs(n),r=n.checked,a=n[vn];if(zt(i)){const o=Wa(i,s),l=o!==-1;if(r&&!l)a(i.concat(s));else if(!r&&l){const c=[...i];c.splice(o,1),a(c)}}else if(Es(i)){const o=new Set(i);r?o.add(s):o.delete(s),a(o)}else a(Od(n,r))})},mounted:Ql,beforeUpdate(n,t,e){n[vn]=yi(e),Ql(n,t,e)}};function Ql(n,{value:t,oldValue:e},i){n._modelValue=t;let s;if(zt(t))s=Wa(t,i.props.value)>-1;else if(Es(t))s=t.has(i.props.value);else{if(t===e)return;s=vi(t,Od(n,!0))}n.checked!==s&&(n.checked=s)}const qp={created(n,{value:t},e){n.checked=vi(t,e.props.value),n[vn]=yi(e),Jn(n,"change",()=>{n[vn](xs(n))})},beforeUpdate(n,{value:t,oldValue:e},i){n[vn]=yi(i),t!==e&&(n.checked=vi(t,i.props.value))}},si={deep:!0,created(n,{value:t,modifiers:{number:e}},i){const s=Es(t);Jn(n,"change",()=>{const r=Array.prototype.filter.call(n.options,a=>a.selected).map(a=>e?ho(xs(a)):xs(a));n[vn](n.multiple?s?new Set(r):r:r[0]),n._assigning=!0,id(()=>{n._assigning=!1})}),n[vn]=yi(i)},mounted(n,{value:t}){tc(n,t)},beforeUpdate(n,t,e){n[vn]=yi(e)},updated(n,{value:t}){n._assigning||tc(n,t)}};function tc(n,t){const e=n.multiple,i=zt(t);if(!(e&&!i&&!Es(t))){for(let s=0,r=n.options.length;s<r;s++){const a=n.options[s],o=xs(a);if(e)if(i){const l=typeof o;l==="string"||l==="number"?a.selected=t.some(c=>String(c)===String(o)):a.selected=Wa(t,o)>-1}else a.selected=t.has(o);else if(vi(xs(a),t)){n.selectedIndex!==s&&(n.selectedIndex=s);return}}!e&&n.selectedIndex!==-1&&(n.selectedIndex=-1)}}function xs(n){return"_value"in n?n._value:n.value}function Od(n,t){const e=t?"_trueValue":"_falseValue";return e in n?n[e]:t}const jp={created(n,t,e){_r(n,t,e,null,"created")},mounted(n,t,e){_r(n,t,e,null,"mounted")},beforeUpdate(n,t,e,i){_r(n,t,e,i,"beforeUpdate")},updated(n,t,e,i){_r(n,t,e,i,"updated")}};function Yp(n,t){switch(n){case"SELECT":return si;case"TEXTAREA":return Je;default:switch(t){case"checkbox":return Xp;case"radio":return qp;default:return Je}}}function _r(n,t,e,i,s){const a=Yp(n.tagName,e.props&&e.props.type)[s];a&&a(n,t,e,i)}const Kp=["ctrl","shift","alt","meta"],Zp={stop:n=>n.stopPropagation(),prevent:n=>n.preventDefault(),self:n=>n.target!==n.currentTarget,ctrl:n=>!n.ctrlKey,shift:n=>!n.shiftKey,alt:n=>!n.altKey,meta:n=>!n.metaKey,left:n=>"button"in n&&n.button!==0,middle:n=>"button"in n&&n.button!==1,right:n=>"button"in n&&n.button!==2,exact:(n,t)=>Kp.some(e=>n[`${e}Key`]&&!t.includes(e))},il=(n,t)=>{if(!n)return n;const e=n._withMods||(n._withMods={}),i=t.join(".");return e[i]||(e[i]=((s,...r)=>{for(let a=0;a<t.length;a++){const o=Zp[t[a]];if(o&&o(s,t))return}return n(s,...r)}))},Jp={esc:"escape",space:" ",up:"arrow-up",left:"arrow-left",right:"arrow-right",down:"arrow-down",delete:"backspace"},Qp=(n,t)=>{const e=n._withKeys||(n._withKeys={}),i=t.join(".");return e[i]||(e[i]=(s=>{if(!("key"in s))return;const r=Mi(s.key);if(t.some(a=>a===r||Jp[a]===r))return n(s)}))},tm=$e({patchProp:Vp},wp);let ec;function em(){return ec||(ec=sp(tm))}const nm=((...n)=>{const t=em().createApp(...n),{mount:e}=t;return t.mount=i=>{const s=sm(i);if(!s)return;const r=t._component;!Xt(r)&&!r.render&&!r.template&&(r.template=s.innerHTML),s.nodeType===1&&(s.textContent="");const a=e(s,!1,im(s));return s instanceof Element&&(s.removeAttribute("v-cloak"),s.setAttribute("data-v-app","")),a},t});function im(n){if(n instanceof SVGElement)return"svg";if(typeof MathMLElement=="function"&&n instanceof MathMLElement)return"mathml"}function sm(n){return be(n)?document.querySelector(n):n}let yo=localStorage.getItem("admin_token")||"";function rm(){return yo}function om(n){yo=n,localStorage.setItem("admin_token",n)}function Bd(){yo="",localStorage.removeItem("admin_token")}function am(){return{Authorization:"Bearer "+yo,"Content-Type":"application/json"}}async function Ot(n,t={}){const e=await fetch(n,{headers:am(),...t});if(e.status===401)throw Bd(),new Error("unauthorized");const i=await e.json();if(!e.ok)throw new Error(i.error||"request failed");return i}async function nc(n){return(await fetch("/api/login",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({token:n})})).ok?(om(n),!0):!1}/**
 * @license
 * Copyright 2010-2023 Three.js Authors
 * SPDX-License-Identifier: MIT
 */const sl="160",lm=0,ic=1,cm=2,zd=1,um=2,Xn=3,bi=0,tn=1,Qn=2,mi=0,ps=1,jn=2,sc=3,rc=4,dm=5,Ii=100,hm=101,fm=102,oc=103,ac=104,pm=200,mm=201,_m=202,gm=203,La=204,Pa=205,vm=206,xm=207,ym=208,bm=209,Mm=210,Sm=211,Em=212,Tm=213,wm=214,Am=0,Cm=1,Rm=2,eo=3,Lm=4,Pm=5,Dm=6,Um=7,Hd=0,Im=1,Nm=2,_i=0,km=1,Fm=2,Om=3,Bm=4,zm=5,Hm=6,Vd=300,ys=301,bs=302,Da=303,Ua=304,bo=306,Ia=1e3,Sn=1001,Na=1002,Ke=1003,lc=1004,Oo=1005,pn=1006,Vm=1007,tr=1008,gi=1009,Gm=1010,Wm=1011,rl=1012,Gd=1013,fi=1014,pi=1015,er=1016,Wd=1017,$d=1018,Oi=1020,$m=1021,En=1023,Xm=1024,qm=1025,Bi=1026,Ms=1027,jm=1028,Xd=1029,Ym=1030,qd=1031,jd=1033,Bo=33776,zo=33777,Ho=33778,Vo=33779,cc=35840,uc=35841,dc=35842,hc=35843,Yd=36196,fc=37492,pc=37496,mc=37808,_c=37809,gc=37810,vc=37811,xc=37812,yc=37813,bc=37814,Mc=37815,Sc=37816,Ec=37817,Tc=37818,wc=37819,Ac=37820,Cc=37821,Go=36492,Rc=36494,Lc=36495,Km=36283,Pc=36284,Dc=36285,Uc=36286,Kd=3e3,zi=3001,Zm=3200,Jm=3201,Qm=0,t_=1,mn="",Ie="srgb",ri="srgb-linear",ol="display-p3",Mo="display-p3-linear",no="linear",ge="srgb",io="rec709",so="p3",Wi=7680,Ic=519,e_=512,n_=513,i_=514,Zd=515,s_=516,r_=517,o_=518,a_=519,Nc=35044,kc="300 es",ka=1035,ti=2e3,ro=2001;class Ts{addEventListener(t,e){this._listeners===void 0&&(this._listeners={});const i=this._listeners;i[t]===void 0&&(i[t]=[]),i[t].indexOf(e)===-1&&i[t].push(e)}hasEventListener(t,e){if(this._listeners===void 0)return!1;const i=this._listeners;return i[t]!==void 0&&i[t].indexOf(e)!==-1}removeEventListener(t,e){if(this._listeners===void 0)return;const s=this._listeners[t];if(s!==void 0){const r=s.indexOf(e);r!==-1&&s.splice(r,1)}}dispatchEvent(t){if(this._listeners===void 0)return;const i=this._listeners[t.type];if(i!==void 0){t.target=this;const s=i.slice(0);for(let r=0,a=s.length;r<a;r++)s[r].call(this,t);t.target=null}}}const Oe=["00","01","02","03","04","05","06","07","08","09","0a","0b","0c","0d","0e","0f","10","11","12","13","14","15","16","17","18","19","1a","1b","1c","1d","1e","1f","20","21","22","23","24","25","26","27","28","29","2a","2b","2c","2d","2e","2f","30","31","32","33","34","35","36","37","38","39","3a","3b","3c","3d","3e","3f","40","41","42","43","44","45","46","47","48","49","4a","4b","4c","4d","4e","4f","50","51","52","53","54","55","56","57","58","59","5a","5b","5c","5d","5e","5f","60","61","62","63","64","65","66","67","68","69","6a","6b","6c","6d","6e","6f","70","71","72","73","74","75","76","77","78","79","7a","7b","7c","7d","7e","7f","80","81","82","83","84","85","86","87","88","89","8a","8b","8c","8d","8e","8f","90","91","92","93","94","95","96","97","98","99","9a","9b","9c","9d","9e","9f","a0","a1","a2","a3","a4","a5","a6","a7","a8","a9","aa","ab","ac","ad","ae","af","b0","b1","b2","b3","b4","b5","b6","b7","b8","b9","ba","bb","bc","bd","be","bf","c0","c1","c2","c3","c4","c5","c6","c7","c8","c9","ca","cb","cc","cd","ce","cf","d0","d1","d2","d3","d4","d5","d6","d7","d8","d9","da","db","dc","dd","de","df","e0","e1","e2","e3","e4","e5","e6","e7","e8","e9","ea","eb","ec","ed","ee","ef","f0","f1","f2","f3","f4","f5","f6","f7","f8","f9","fa","fb","fc","fd","fe","ff"],Wo=Math.PI/180,Fa=180/Math.PI;function or(){const n=Math.random()*4294967295|0,t=Math.random()*4294967295|0,e=Math.random()*4294967295|0,i=Math.random()*4294967295|0;return(Oe[n&255]+Oe[n>>8&255]+Oe[n>>16&255]+Oe[n>>24&255]+"-"+Oe[t&255]+Oe[t>>8&255]+"-"+Oe[t>>16&15|64]+Oe[t>>24&255]+"-"+Oe[e&63|128]+Oe[e>>8&255]+"-"+Oe[e>>16&255]+Oe[e>>24&255]+Oe[i&255]+Oe[i>>8&255]+Oe[i>>16&255]+Oe[i>>24&255]).toLowerCase()}function sn(n,t,e){return Math.max(t,Math.min(e,n))}function l_(n,t){return(n%t+t)%t}function $o(n,t,e){return(1-e)*n+e*t}function Fc(n){return(n&n-1)===0&&n!==0}function Oa(n){return Math.pow(2,Math.floor(Math.log(n)/Math.LN2))}function Ps(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return n/4294967295;case Uint16Array:return n/65535;case Uint8Array:return n/255;case Int32Array:return Math.max(n/2147483647,-1);case Int16Array:return Math.max(n/32767,-1);case Int8Array:return Math.max(n/127,-1);default:throw new Error("Invalid component type.")}}function en(n,t){switch(t.constructor){case Float32Array:return n;case Uint32Array:return Math.round(n*4294967295);case Uint16Array:return Math.round(n*65535);case Uint8Array:return Math.round(n*255);case Int32Array:return Math.round(n*2147483647);case Int16Array:return Math.round(n*32767);case Int8Array:return Math.round(n*127);default:throw new Error("Invalid component type.")}}class re{constructor(t=0,e=0){re.prototype.isVector2=!0,this.x=t,this.y=e}get width(){return this.x}set width(t){this.x=t}get height(){return this.y}set height(t){this.y=t}set(t,e){return this.x=t,this.y=e,this}setScalar(t){return this.x=t,this.y=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y)}copy(t){return this.x=t.x,this.y=t.y,this}add(t){return this.x+=t.x,this.y+=t.y,this}addScalar(t){return this.x+=t,this.y+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this}subScalar(t){return this.x-=t,this.y-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this}multiply(t){return this.x*=t.x,this.y*=t.y,this}multiplyScalar(t){return this.x*=t,this.y*=t,this}divide(t){return this.x/=t.x,this.y/=t.y,this}divideScalar(t){return this.multiplyScalar(1/t)}applyMatrix3(t){const e=this.x,i=this.y,s=t.elements;return this.x=s[0]*e+s[3]*i+s[6],this.y=s[1]*e+s[4]*i+s[7],this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this}negate(){return this.x=-this.x,this.y=-this.y,this}dot(t){return this.x*t.x+this.y*t.y}cross(t){return this.x*t.y-this.y*t.x}lengthSq(){return this.x*this.x+this.y*this.y}length(){return Math.sqrt(this.x*this.x+this.y*this.y)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)}normalize(){return this.divideScalar(this.length()||1)}angle(){return Math.atan2(-this.y,-this.x)+Math.PI}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(sn(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y;return e*e+i*i}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this}equals(t){return t.x===this.x&&t.y===this.y}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this}rotateAround(t,e){const i=Math.cos(e),s=Math.sin(e),r=this.x-t.x,a=this.y-t.y;return this.x=r*i-a*s+t.x,this.y=r*s+a*i+t.y,this}random(){return this.x=Math.random(),this.y=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y}}class te{constructor(t,e,i,s,r,a,o,l,c){te.prototype.isMatrix3=!0,this.elements=[1,0,0,0,1,0,0,0,1],t!==void 0&&this.set(t,e,i,s,r,a,o,l,c)}set(t,e,i,s,r,a,o,l,c){const d=this.elements;return d[0]=t,d[1]=s,d[2]=o,d[3]=e,d[4]=r,d[5]=l,d[6]=i,d[7]=a,d[8]=c,this}identity(){return this.set(1,0,0,0,1,0,0,0,1),this}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],this}extractBasis(t,e,i){return t.setFromMatrix3Column(this,0),e.setFromMatrix3Column(this,1),i.setFromMatrix3Column(this,2),this}setFromMatrix4(t){const e=t.elements;return this.set(e[0],e[4],e[8],e[1],e[5],e[9],e[2],e[6],e[10]),this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,s=e.elements,r=this.elements,a=i[0],o=i[3],l=i[6],c=i[1],d=i[4],h=i[7],p=i[2],f=i[5],v=i[8],g=s[0],_=s[3],m=s[6],E=s[1],b=s[4],x=s[7],C=s[2],A=s[5],D=s[8];return r[0]=a*g+o*E+l*C,r[3]=a*_+o*b+l*A,r[6]=a*m+o*x+l*D,r[1]=c*g+d*E+h*C,r[4]=c*_+d*b+h*A,r[7]=c*m+d*x+h*D,r[2]=p*g+f*E+v*C,r[5]=p*_+f*b+v*A,r[8]=p*m+f*x+v*D,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[3]*=t,e[6]*=t,e[1]*=t,e[4]*=t,e[7]*=t,e[2]*=t,e[5]*=t,e[8]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],l=t[6],c=t[7],d=t[8];return e*a*d-e*o*c-i*r*d+i*o*l+s*r*c-s*a*l}invert(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],l=t[6],c=t[7],d=t[8],h=d*a-o*c,p=o*l-d*r,f=c*r-a*l,v=e*h+i*p+s*f;if(v===0)return this.set(0,0,0,0,0,0,0,0,0);const g=1/v;return t[0]=h*g,t[1]=(s*c-d*i)*g,t[2]=(o*i-s*a)*g,t[3]=p*g,t[4]=(d*e-s*l)*g,t[5]=(s*r-o*e)*g,t[6]=f*g,t[7]=(i*l-c*e)*g,t[8]=(a*e-i*r)*g,this}transpose(){let t;const e=this.elements;return t=e[1],e[1]=e[3],e[3]=t,t=e[2],e[2]=e[6],e[6]=t,t=e[5],e[5]=e[7],e[7]=t,this}getNormalMatrix(t){return this.setFromMatrix4(t).invert().transpose()}transposeIntoArray(t){const e=this.elements;return t[0]=e[0],t[1]=e[3],t[2]=e[6],t[3]=e[1],t[4]=e[4],t[5]=e[7],t[6]=e[2],t[7]=e[5],t[8]=e[8],this}setUvTransform(t,e,i,s,r,a,o){const l=Math.cos(r),c=Math.sin(r);return this.set(i*l,i*c,-i*(l*a+c*o)+a+t,-s*c,s*l,-s*(-c*a+l*o)+o+e,0,0,1),this}scale(t,e){return this.premultiply(Xo.makeScale(t,e)),this}rotate(t){return this.premultiply(Xo.makeRotation(-t)),this}translate(t,e){return this.premultiply(Xo.makeTranslation(t,e)),this}makeTranslation(t,e){return t.isVector2?this.set(1,0,t.x,0,1,t.y,0,0,1):this.set(1,0,t,0,1,e,0,0,1),this}makeRotation(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,i,e,0,0,0,1),this}makeScale(t,e){return this.set(t,0,0,0,e,0,0,0,1),this}equals(t){const e=this.elements,i=t.elements;for(let s=0;s<9;s++)if(e[s]!==i[s])return!1;return!0}fromArray(t,e=0){for(let i=0;i<9;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t}clone(){return new this.constructor().fromArray(this.elements)}}const Xo=new te;function Jd(n){for(let t=n.length-1;t>=0;--t)if(n[t]>=65535)return!0;return!1}function oo(n){return document.createElementNS("http://www.w3.org/1999/xhtml",n)}function c_(){const n=oo("canvas");return n.style.display="block",n}const Oc={};function qs(n){n in Oc||(Oc[n]=!0,console.warn(n))}const Bc=new te().set(.8224621,.177538,0,.0331941,.9668058,0,.0170827,.0723974,.9105199),zc=new te().set(1.2249401,-.2249404,0,-.0420569,1.0420571,0,-.0196376,-.0786361,1.0982735),gr={[ri]:{transfer:no,primaries:io,toReference:n=>n,fromReference:n=>n},[Ie]:{transfer:ge,primaries:io,toReference:n=>n.convertSRGBToLinear(),fromReference:n=>n.convertLinearToSRGB()},[Mo]:{transfer:no,primaries:so,toReference:n=>n.applyMatrix3(zc),fromReference:n=>n.applyMatrix3(Bc)},[ol]:{transfer:ge,primaries:so,toReference:n=>n.convertSRGBToLinear().applyMatrix3(zc),fromReference:n=>n.applyMatrix3(Bc).convertLinearToSRGB()}},u_=new Set([ri,Mo]),ce={enabled:!0,_workingColorSpace:ri,get workingColorSpace(){return this._workingColorSpace},set workingColorSpace(n){if(!u_.has(n))throw new Error(`Unsupported working color space, "${n}".`);this._workingColorSpace=n},convert:function(n,t,e){if(this.enabled===!1||t===e||!t||!e)return n;const i=gr[t].toReference,s=gr[e].fromReference;return s(i(n))},fromWorkingColorSpace:function(n,t){return this.convert(n,this._workingColorSpace,t)},toWorkingColorSpace:function(n,t){return this.convert(n,t,this._workingColorSpace)},getPrimaries:function(n){return gr[n].primaries},getTransfer:function(n){return n===mn?no:gr[n].transfer}};function ms(n){return n<.04045?n*.0773993808:Math.pow(n*.9478672986+.0521327014,2.4)}function qo(n){return n<.0031308?n*12.92:1.055*Math.pow(n,.41666)-.055}let $i;class Qd{static getDataURL(t){if(/^data:/i.test(t.src)||typeof HTMLCanvasElement>"u")return t.src;let e;if(t instanceof HTMLCanvasElement)e=t;else{$i===void 0&&($i=oo("canvas")),$i.width=t.width,$i.height=t.height;const i=$i.getContext("2d");t instanceof ImageData?i.putImageData(t,0,0):i.drawImage(t,0,0,t.width,t.height),e=$i}return e.width>2048||e.height>2048?(console.warn("THREE.ImageUtils.getDataURL: Image converted to jpg for performance reasons",t),e.toDataURL("image/jpeg",.6)):e.toDataURL("image/png")}static sRGBToLinear(t){if(typeof HTMLImageElement<"u"&&t instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&t instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&t instanceof ImageBitmap){const e=oo("canvas");e.width=t.width,e.height=t.height;const i=e.getContext("2d");i.drawImage(t,0,0,t.width,t.height);const s=i.getImageData(0,0,t.width,t.height),r=s.data;for(let a=0;a<r.length;a++)r[a]=ms(r[a]/255)*255;return i.putImageData(s,0,0),e}else if(t.data){const e=t.data.slice(0);for(let i=0;i<e.length;i++)e instanceof Uint8Array||e instanceof Uint8ClampedArray?e[i]=Math.floor(ms(e[i]/255)*255):e[i]=ms(e[i]);return{data:e,width:t.width,height:t.height}}else return console.warn("THREE.ImageUtils.sRGBToLinear(): Unsupported image type. No color space conversion applied."),t}}let d_=0;class th{constructor(t=null){this.isSource=!0,Object.defineProperty(this,"id",{value:d_++}),this.uuid=or(),this.data=t,this.version=0}set needsUpdate(t){t===!0&&this.version++}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.images[this.uuid]!==void 0)return t.images[this.uuid];const i={uuid:this.uuid,url:""},s=this.data;if(s!==null){let r;if(Array.isArray(s)){r=[];for(let a=0,o=s.length;a<o;a++)s[a].isDataTexture?r.push(jo(s[a].image)):r.push(jo(s[a]))}else r=jo(s);i.url=r}return e||(t.images[this.uuid]=i),i}}function jo(n){return typeof HTMLImageElement<"u"&&n instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&n instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&n instanceof ImageBitmap?Qd.getDataURL(n):n.data?{data:Array.from(n.data),width:n.width,height:n.height,type:n.data.constructor.name}:(console.warn("THREE.Texture: Unable to serialize Texture."),{})}let h_=0;class dn extends Ts{constructor(t=dn.DEFAULT_IMAGE,e=dn.DEFAULT_MAPPING,i=Sn,s=Sn,r=pn,a=tr,o=En,l=gi,c=dn.DEFAULT_ANISOTROPY,d=mn){super(),this.isTexture=!0,Object.defineProperty(this,"id",{value:h_++}),this.uuid=or(),this.name="",this.source=new th(t),this.mipmaps=[],this.mapping=e,this.channel=0,this.wrapS=i,this.wrapT=s,this.magFilter=r,this.minFilter=a,this.anisotropy=c,this.format=o,this.internalFormat=null,this.type=l,this.offset=new re(0,0),this.repeat=new re(1,1),this.center=new re(0,0),this.rotation=0,this.matrixAutoUpdate=!0,this.matrix=new te,this.generateMipmaps=!0,this.premultiplyAlpha=!1,this.flipY=!0,this.unpackAlignment=4,typeof d=="string"?this.colorSpace=d:(qs("THREE.Texture: Property .encoding has been replaced by .colorSpace."),this.colorSpace=d===zi?Ie:mn),this.userData={},this.version=0,this.onUpdate=null,this.isRenderTargetTexture=!1,this.needsPMREMUpdate=!1}get image(){return this.source.data}set image(t=null){this.source.data=t}updateMatrix(){this.matrix.setUvTransform(this.offset.x,this.offset.y,this.repeat.x,this.repeat.y,this.rotation,this.center.x,this.center.y)}clone(){return new this.constructor().copy(this)}copy(t){return this.name=t.name,this.source=t.source,this.mipmaps=t.mipmaps.slice(0),this.mapping=t.mapping,this.channel=t.channel,this.wrapS=t.wrapS,this.wrapT=t.wrapT,this.magFilter=t.magFilter,this.minFilter=t.minFilter,this.anisotropy=t.anisotropy,this.format=t.format,this.internalFormat=t.internalFormat,this.type=t.type,this.offset.copy(t.offset),this.repeat.copy(t.repeat),this.center.copy(t.center),this.rotation=t.rotation,this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrix.copy(t.matrix),this.generateMipmaps=t.generateMipmaps,this.premultiplyAlpha=t.premultiplyAlpha,this.flipY=t.flipY,this.unpackAlignment=t.unpackAlignment,this.colorSpace=t.colorSpace,this.userData=JSON.parse(JSON.stringify(t.userData)),this.needsUpdate=!0,this}toJSON(t){const e=t===void 0||typeof t=="string";if(!e&&t.textures[this.uuid]!==void 0)return t.textures[this.uuid];const i={metadata:{version:4.6,type:"Texture",generator:"Texture.toJSON"},uuid:this.uuid,name:this.name,image:this.source.toJSON(t).uuid,mapping:this.mapping,channel:this.channel,repeat:[this.repeat.x,this.repeat.y],offset:[this.offset.x,this.offset.y],center:[this.center.x,this.center.y],rotation:this.rotation,wrap:[this.wrapS,this.wrapT],format:this.format,internalFormat:this.internalFormat,type:this.type,colorSpace:this.colorSpace,minFilter:this.minFilter,magFilter:this.magFilter,anisotropy:this.anisotropy,flipY:this.flipY,generateMipmaps:this.generateMipmaps,premultiplyAlpha:this.premultiplyAlpha,unpackAlignment:this.unpackAlignment};return Object.keys(this.userData).length>0&&(i.userData=this.userData),e||(t.textures[this.uuid]=i),i}dispose(){this.dispatchEvent({type:"dispose"})}transformUv(t){if(this.mapping!==Vd)return t;if(t.applyMatrix3(this.matrix),t.x<0||t.x>1)switch(this.wrapS){case Ia:t.x=t.x-Math.floor(t.x);break;case Sn:t.x=t.x<0?0:1;break;case Na:Math.abs(Math.floor(t.x)%2)===1?t.x=Math.ceil(t.x)-t.x:t.x=t.x-Math.floor(t.x);break}if(t.y<0||t.y>1)switch(this.wrapT){case Ia:t.y=t.y-Math.floor(t.y);break;case Sn:t.y=t.y<0?0:1;break;case Na:Math.abs(Math.floor(t.y)%2)===1?t.y=Math.ceil(t.y)-t.y:t.y=t.y-Math.floor(t.y);break}return this.flipY&&(t.y=1-t.y),t}set needsUpdate(t){t===!0&&(this.version++,this.source.needsUpdate=!0)}get encoding(){return qs("THREE.Texture: Property .encoding has been replaced by .colorSpace."),this.colorSpace===Ie?zi:Kd}set encoding(t){qs("THREE.Texture: Property .encoding has been replaced by .colorSpace."),this.colorSpace=t===zi?Ie:mn}}dn.DEFAULT_IMAGE=null;dn.DEFAULT_MAPPING=Vd;dn.DEFAULT_ANISOTROPY=1;class xe{constructor(t=0,e=0,i=0,s=1){xe.prototype.isVector4=!0,this.x=t,this.y=e,this.z=i,this.w=s}get width(){return this.z}set width(t){this.z=t}get height(){return this.w}set height(t){this.w=t}set(t,e,i,s){return this.x=t,this.y=e,this.z=i,this.w=s,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this.w=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setW(t){return this.w=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;case 3:this.w=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;case 3:return this.w;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z,this.w)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this.w=t.w!==void 0?t.w:1,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this.w+=t.w,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this.w+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this.w=t.w+e.w,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this.w+=t.w*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this.w-=t.w,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this.w-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this.w=t.w-e.w,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this.w*=t.w,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this.w*=t,this}applyMatrix4(t){const e=this.x,i=this.y,s=this.z,r=this.w,a=t.elements;return this.x=a[0]*e+a[4]*i+a[8]*s+a[12]*r,this.y=a[1]*e+a[5]*i+a[9]*s+a[13]*r,this.z=a[2]*e+a[6]*i+a[10]*s+a[14]*r,this.w=a[3]*e+a[7]*i+a[11]*s+a[15]*r,this}divideScalar(t){return this.multiplyScalar(1/t)}setAxisAngleFromQuaternion(t){this.w=2*Math.acos(t.w);const e=Math.sqrt(1-t.w*t.w);return e<1e-4?(this.x=1,this.y=0,this.z=0):(this.x=t.x/e,this.y=t.y/e,this.z=t.z/e),this}setAxisAngleFromRotationMatrix(t){let e,i,s,r;const l=t.elements,c=l[0],d=l[4],h=l[8],p=l[1],f=l[5],v=l[9],g=l[2],_=l[6],m=l[10];if(Math.abs(d-p)<.01&&Math.abs(h-g)<.01&&Math.abs(v-_)<.01){if(Math.abs(d+p)<.1&&Math.abs(h+g)<.1&&Math.abs(v+_)<.1&&Math.abs(c+f+m-3)<.1)return this.set(1,0,0,0),this;e=Math.PI;const b=(c+1)/2,x=(f+1)/2,C=(m+1)/2,A=(d+p)/4,D=(h+g)/4,G=(v+_)/4;return b>x&&b>C?b<.01?(i=0,s=.707106781,r=.707106781):(i=Math.sqrt(b),s=A/i,r=D/i):x>C?x<.01?(i=.707106781,s=0,r=.707106781):(s=Math.sqrt(x),i=A/s,r=G/s):C<.01?(i=.707106781,s=.707106781,r=0):(r=Math.sqrt(C),i=D/r,s=G/r),this.set(i,s,r,e),this}let E=Math.sqrt((_-v)*(_-v)+(h-g)*(h-g)+(p-d)*(p-d));return Math.abs(E)<.001&&(E=1),this.x=(_-v)/E,this.y=(h-g)/E,this.z=(p-d)/E,this.w=Math.acos((c+f+m-1)/2),this}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this.w=Math.min(this.w,t.w),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this.w=Math.max(this.w,t.w),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this.z=Math.max(t.z,Math.min(e.z,this.z)),this.w=Math.max(t.w,Math.min(e.w,this.w)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this.z=Math.max(t,Math.min(e,this.z)),this.w=Math.max(t,Math.min(e,this.w)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this.w=Math.floor(this.w),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this.w=Math.ceil(this.w),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this.w=Math.round(this.w),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this.w=Math.trunc(this.w),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this.w=-this.w,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z+this.w*t.w}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z+this.w*this.w)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)+Math.abs(this.w)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this.w+=(t.w-this.w)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this.w=t.w+(e.w-t.w)*i,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z&&t.w===this.w}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this.w=t[e+3],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t[e+3]=this.w,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this.w=t.getW(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this.w=Math.random(),this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z,yield this.w}}class f_ extends Ts{constructor(t=1,e=1,i={}){super(),this.isRenderTarget=!0,this.width=t,this.height=e,this.depth=1,this.scissor=new xe(0,0,t,e),this.scissorTest=!1,this.viewport=new xe(0,0,t,e);const s={width:t,height:e,depth:1};i.encoding!==void 0&&(qs("THREE.WebGLRenderTarget: option.encoding has been replaced by option.colorSpace."),i.colorSpace=i.encoding===zi?Ie:mn),i=Object.assign({generateMipmaps:!1,internalFormat:null,minFilter:pn,depthBuffer:!0,stencilBuffer:!1,depthTexture:null,samples:0},i),this.texture=new dn(s,i.mapping,i.wrapS,i.wrapT,i.magFilter,i.minFilter,i.format,i.type,i.anisotropy,i.colorSpace),this.texture.isRenderTargetTexture=!0,this.texture.flipY=!1,this.texture.generateMipmaps=i.generateMipmaps,this.texture.internalFormat=i.internalFormat,this.depthBuffer=i.depthBuffer,this.stencilBuffer=i.stencilBuffer,this.depthTexture=i.depthTexture,this.samples=i.samples}setSize(t,e,i=1){(this.width!==t||this.height!==e||this.depth!==i)&&(this.width=t,this.height=e,this.depth=i,this.texture.image.width=t,this.texture.image.height=e,this.texture.image.depth=i,this.dispose()),this.viewport.set(0,0,t,e),this.scissor.set(0,0,t,e)}clone(){return new this.constructor().copy(this)}copy(t){this.width=t.width,this.height=t.height,this.depth=t.depth,this.scissor.copy(t.scissor),this.scissorTest=t.scissorTest,this.viewport.copy(t.viewport),this.texture=t.texture.clone(),this.texture.isRenderTargetTexture=!0;const e=Object.assign({},t.texture.image);return this.texture.source=new th(e),this.depthBuffer=t.depthBuffer,this.stencilBuffer=t.stencilBuffer,t.depthTexture!==null&&(this.depthTexture=t.depthTexture.clone()),this.samples=t.samples,this}dispose(){this.dispatchEvent({type:"dispose"})}}class Hi extends f_{constructor(t=1,e=1,i={}){super(t,e,i),this.isWebGLRenderTarget=!0}}class eh extends dn{constructor(t=null,e=1,i=1,s=1){super(null),this.isDataArrayTexture=!0,this.image={data:t,width:e,height:i,depth:s},this.magFilter=Ke,this.minFilter=Ke,this.wrapR=Sn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class p_ extends dn{constructor(t=null,e=1,i=1,s=1){super(null),this.isData3DTexture=!0,this.image={data:t,width:e,height:i,depth:s},this.magFilter=Ke,this.minFilter=Ke,this.wrapR=Sn,this.generateMipmaps=!1,this.flipY=!1,this.unpackAlignment=1}}class ar{constructor(t=0,e=0,i=0,s=1){this.isQuaternion=!0,this._x=t,this._y=e,this._z=i,this._w=s}static slerpFlat(t,e,i,s,r,a,o){let l=i[s+0],c=i[s+1],d=i[s+2],h=i[s+3];const p=r[a+0],f=r[a+1],v=r[a+2],g=r[a+3];if(o===0){t[e+0]=l,t[e+1]=c,t[e+2]=d,t[e+3]=h;return}if(o===1){t[e+0]=p,t[e+1]=f,t[e+2]=v,t[e+3]=g;return}if(h!==g||l!==p||c!==f||d!==v){let _=1-o;const m=l*p+c*f+d*v+h*g,E=m>=0?1:-1,b=1-m*m;if(b>Number.EPSILON){const C=Math.sqrt(b),A=Math.atan2(C,m*E);_=Math.sin(_*A)/C,o=Math.sin(o*A)/C}const x=o*E;if(l=l*_+p*x,c=c*_+f*x,d=d*_+v*x,h=h*_+g*x,_===1-o){const C=1/Math.sqrt(l*l+c*c+d*d+h*h);l*=C,c*=C,d*=C,h*=C}}t[e]=l,t[e+1]=c,t[e+2]=d,t[e+3]=h}static multiplyQuaternionsFlat(t,e,i,s,r,a){const o=i[s],l=i[s+1],c=i[s+2],d=i[s+3],h=r[a],p=r[a+1],f=r[a+2],v=r[a+3];return t[e]=o*v+d*h+l*f-c*p,t[e+1]=l*v+d*p+c*h-o*f,t[e+2]=c*v+d*f+o*p-l*h,t[e+3]=d*v-o*h-l*p-c*f,t}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get w(){return this._w}set w(t){this._w=t,this._onChangeCallback()}set(t,e,i,s){return this._x=t,this._y=e,this._z=i,this._w=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._w)}copy(t){return this._x=t.x,this._y=t.y,this._z=t.z,this._w=t.w,this._onChangeCallback(),this}setFromEuler(t,e=!0){const i=t._x,s=t._y,r=t._z,a=t._order,o=Math.cos,l=Math.sin,c=o(i/2),d=o(s/2),h=o(r/2),p=l(i/2),f=l(s/2),v=l(r/2);switch(a){case"XYZ":this._x=p*d*h+c*f*v,this._y=c*f*h-p*d*v,this._z=c*d*v+p*f*h,this._w=c*d*h-p*f*v;break;case"YXZ":this._x=p*d*h+c*f*v,this._y=c*f*h-p*d*v,this._z=c*d*v-p*f*h,this._w=c*d*h+p*f*v;break;case"ZXY":this._x=p*d*h-c*f*v,this._y=c*f*h+p*d*v,this._z=c*d*v+p*f*h,this._w=c*d*h-p*f*v;break;case"ZYX":this._x=p*d*h-c*f*v,this._y=c*f*h+p*d*v,this._z=c*d*v-p*f*h,this._w=c*d*h+p*f*v;break;case"YZX":this._x=p*d*h+c*f*v,this._y=c*f*h+p*d*v,this._z=c*d*v-p*f*h,this._w=c*d*h-p*f*v;break;case"XZY":this._x=p*d*h-c*f*v,this._y=c*f*h-p*d*v,this._z=c*d*v+p*f*h,this._w=c*d*h+p*f*v;break;default:console.warn("THREE.Quaternion: .setFromEuler() encountered an unknown order: "+a)}return e===!0&&this._onChangeCallback(),this}setFromAxisAngle(t,e){const i=e/2,s=Math.sin(i);return this._x=t.x*s,this._y=t.y*s,this._z=t.z*s,this._w=Math.cos(i),this._onChangeCallback(),this}setFromRotationMatrix(t){const e=t.elements,i=e[0],s=e[4],r=e[8],a=e[1],o=e[5],l=e[9],c=e[2],d=e[6],h=e[10],p=i+o+h;if(p>0){const f=.5/Math.sqrt(p+1);this._w=.25/f,this._x=(d-l)*f,this._y=(r-c)*f,this._z=(a-s)*f}else if(i>o&&i>h){const f=2*Math.sqrt(1+i-o-h);this._w=(d-l)/f,this._x=.25*f,this._y=(s+a)/f,this._z=(r+c)/f}else if(o>h){const f=2*Math.sqrt(1+o-i-h);this._w=(r-c)/f,this._x=(s+a)/f,this._y=.25*f,this._z=(l+d)/f}else{const f=2*Math.sqrt(1+h-i-o);this._w=(a-s)/f,this._x=(r+c)/f,this._y=(l+d)/f,this._z=.25*f}return this._onChangeCallback(),this}setFromUnitVectors(t,e){let i=t.dot(e)+1;return i<Number.EPSILON?(i=0,Math.abs(t.x)>Math.abs(t.z)?(this._x=-t.y,this._y=t.x,this._z=0,this._w=i):(this._x=0,this._y=-t.z,this._z=t.y,this._w=i)):(this._x=t.y*e.z-t.z*e.y,this._y=t.z*e.x-t.x*e.z,this._z=t.x*e.y-t.y*e.x,this._w=i),this.normalize()}angleTo(t){return 2*Math.acos(Math.abs(sn(this.dot(t),-1,1)))}rotateTowards(t,e){const i=this.angleTo(t);if(i===0)return this;const s=Math.min(1,e/i);return this.slerp(t,s),this}identity(){return this.set(0,0,0,1)}invert(){return this.conjugate()}conjugate(){return this._x*=-1,this._y*=-1,this._z*=-1,this._onChangeCallback(),this}dot(t){return this._x*t._x+this._y*t._y+this._z*t._z+this._w*t._w}lengthSq(){return this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w}length(){return Math.sqrt(this._x*this._x+this._y*this._y+this._z*this._z+this._w*this._w)}normalize(){let t=this.length();return t===0?(this._x=0,this._y=0,this._z=0,this._w=1):(t=1/t,this._x=this._x*t,this._y=this._y*t,this._z=this._z*t,this._w=this._w*t),this._onChangeCallback(),this}multiply(t){return this.multiplyQuaternions(this,t)}premultiply(t){return this.multiplyQuaternions(t,this)}multiplyQuaternions(t,e){const i=t._x,s=t._y,r=t._z,a=t._w,o=e._x,l=e._y,c=e._z,d=e._w;return this._x=i*d+a*o+s*c-r*l,this._y=s*d+a*l+r*o-i*c,this._z=r*d+a*c+i*l-s*o,this._w=a*d-i*o-s*l-r*c,this._onChangeCallback(),this}slerp(t,e){if(e===0)return this;if(e===1)return this.copy(t);const i=this._x,s=this._y,r=this._z,a=this._w;let o=a*t._w+i*t._x+s*t._y+r*t._z;if(o<0?(this._w=-t._w,this._x=-t._x,this._y=-t._y,this._z=-t._z,o=-o):this.copy(t),o>=1)return this._w=a,this._x=i,this._y=s,this._z=r,this;const l=1-o*o;if(l<=Number.EPSILON){const f=1-e;return this._w=f*a+e*this._w,this._x=f*i+e*this._x,this._y=f*s+e*this._y,this._z=f*r+e*this._z,this.normalize(),this}const c=Math.sqrt(l),d=Math.atan2(c,o),h=Math.sin((1-e)*d)/c,p=Math.sin(e*d)/c;return this._w=a*h+this._w*p,this._x=i*h+this._x*p,this._y=s*h+this._y*p,this._z=r*h+this._z*p,this._onChangeCallback(),this}slerpQuaternions(t,e,i){return this.copy(t).slerp(e,i)}random(){const t=Math.random(),e=Math.sqrt(1-t),i=Math.sqrt(t),s=2*Math.PI*Math.random(),r=2*Math.PI*Math.random();return this.set(e*Math.cos(s),i*Math.sin(r),i*Math.cos(r),e*Math.sin(s))}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._w===this._w}fromArray(t,e=0){return this._x=t[e],this._y=t[e+1],this._z=t[e+2],this._w=t[e+3],this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._w,t}fromBufferAttribute(t,e){return this._x=t.getX(e),this._y=t.getY(e),this._z=t.getZ(e),this._w=t.getW(e),this._onChangeCallback(),this}toJSON(){return this.toArray()}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._w}}class z{constructor(t=0,e=0,i=0){z.prototype.isVector3=!0,this.x=t,this.y=e,this.z=i}set(t,e,i){return i===void 0&&(i=this.z),this.x=t,this.y=e,this.z=i,this}setScalar(t){return this.x=t,this.y=t,this.z=t,this}setX(t){return this.x=t,this}setY(t){return this.y=t,this}setZ(t){return this.z=t,this}setComponent(t,e){switch(t){case 0:this.x=e;break;case 1:this.y=e;break;case 2:this.z=e;break;default:throw new Error("index is out of range: "+t)}return this}getComponent(t){switch(t){case 0:return this.x;case 1:return this.y;case 2:return this.z;default:throw new Error("index is out of range: "+t)}}clone(){return new this.constructor(this.x,this.y,this.z)}copy(t){return this.x=t.x,this.y=t.y,this.z=t.z,this}add(t){return this.x+=t.x,this.y+=t.y,this.z+=t.z,this}addScalar(t){return this.x+=t,this.y+=t,this.z+=t,this}addVectors(t,e){return this.x=t.x+e.x,this.y=t.y+e.y,this.z=t.z+e.z,this}addScaledVector(t,e){return this.x+=t.x*e,this.y+=t.y*e,this.z+=t.z*e,this}sub(t){return this.x-=t.x,this.y-=t.y,this.z-=t.z,this}subScalar(t){return this.x-=t,this.y-=t,this.z-=t,this}subVectors(t,e){return this.x=t.x-e.x,this.y=t.y-e.y,this.z=t.z-e.z,this}multiply(t){return this.x*=t.x,this.y*=t.y,this.z*=t.z,this}multiplyScalar(t){return this.x*=t,this.y*=t,this.z*=t,this}multiplyVectors(t,e){return this.x=t.x*e.x,this.y=t.y*e.y,this.z=t.z*e.z,this}applyEuler(t){return this.applyQuaternion(Hc.setFromEuler(t))}applyAxisAngle(t,e){return this.applyQuaternion(Hc.setFromAxisAngle(t,e))}applyMatrix3(t){const e=this.x,i=this.y,s=this.z,r=t.elements;return this.x=r[0]*e+r[3]*i+r[6]*s,this.y=r[1]*e+r[4]*i+r[7]*s,this.z=r[2]*e+r[5]*i+r[8]*s,this}applyNormalMatrix(t){return this.applyMatrix3(t).normalize()}applyMatrix4(t){const e=this.x,i=this.y,s=this.z,r=t.elements,a=1/(r[3]*e+r[7]*i+r[11]*s+r[15]);return this.x=(r[0]*e+r[4]*i+r[8]*s+r[12])*a,this.y=(r[1]*e+r[5]*i+r[9]*s+r[13])*a,this.z=(r[2]*e+r[6]*i+r[10]*s+r[14])*a,this}applyQuaternion(t){const e=this.x,i=this.y,s=this.z,r=t.x,a=t.y,o=t.z,l=t.w,c=2*(a*s-o*i),d=2*(o*e-r*s),h=2*(r*i-a*e);return this.x=e+l*c+a*h-o*d,this.y=i+l*d+o*c-r*h,this.z=s+l*h+r*d-a*c,this}project(t){return this.applyMatrix4(t.matrixWorldInverse).applyMatrix4(t.projectionMatrix)}unproject(t){return this.applyMatrix4(t.projectionMatrixInverse).applyMatrix4(t.matrixWorld)}transformDirection(t){const e=this.x,i=this.y,s=this.z,r=t.elements;return this.x=r[0]*e+r[4]*i+r[8]*s,this.y=r[1]*e+r[5]*i+r[9]*s,this.z=r[2]*e+r[6]*i+r[10]*s,this.normalize()}divide(t){return this.x/=t.x,this.y/=t.y,this.z/=t.z,this}divideScalar(t){return this.multiplyScalar(1/t)}min(t){return this.x=Math.min(this.x,t.x),this.y=Math.min(this.y,t.y),this.z=Math.min(this.z,t.z),this}max(t){return this.x=Math.max(this.x,t.x),this.y=Math.max(this.y,t.y),this.z=Math.max(this.z,t.z),this}clamp(t,e){return this.x=Math.max(t.x,Math.min(e.x,this.x)),this.y=Math.max(t.y,Math.min(e.y,this.y)),this.z=Math.max(t.z,Math.min(e.z,this.z)),this}clampScalar(t,e){return this.x=Math.max(t,Math.min(e,this.x)),this.y=Math.max(t,Math.min(e,this.y)),this.z=Math.max(t,Math.min(e,this.z)),this}clampLength(t,e){const i=this.length();return this.divideScalar(i||1).multiplyScalar(Math.max(t,Math.min(e,i)))}floor(){return this.x=Math.floor(this.x),this.y=Math.floor(this.y),this.z=Math.floor(this.z),this}ceil(){return this.x=Math.ceil(this.x),this.y=Math.ceil(this.y),this.z=Math.ceil(this.z),this}round(){return this.x=Math.round(this.x),this.y=Math.round(this.y),this.z=Math.round(this.z),this}roundToZero(){return this.x=Math.trunc(this.x),this.y=Math.trunc(this.y),this.z=Math.trunc(this.z),this}negate(){return this.x=-this.x,this.y=-this.y,this.z=-this.z,this}dot(t){return this.x*t.x+this.y*t.y+this.z*t.z}lengthSq(){return this.x*this.x+this.y*this.y+this.z*this.z}length(){return Math.sqrt(this.x*this.x+this.y*this.y+this.z*this.z)}manhattanLength(){return Math.abs(this.x)+Math.abs(this.y)+Math.abs(this.z)}normalize(){return this.divideScalar(this.length()||1)}setLength(t){return this.normalize().multiplyScalar(t)}lerp(t,e){return this.x+=(t.x-this.x)*e,this.y+=(t.y-this.y)*e,this.z+=(t.z-this.z)*e,this}lerpVectors(t,e,i){return this.x=t.x+(e.x-t.x)*i,this.y=t.y+(e.y-t.y)*i,this.z=t.z+(e.z-t.z)*i,this}cross(t){return this.crossVectors(this,t)}crossVectors(t,e){const i=t.x,s=t.y,r=t.z,a=e.x,o=e.y,l=e.z;return this.x=s*l-r*o,this.y=r*a-i*l,this.z=i*o-s*a,this}projectOnVector(t){const e=t.lengthSq();if(e===0)return this.set(0,0,0);const i=t.dot(this)/e;return this.copy(t).multiplyScalar(i)}projectOnPlane(t){return Yo.copy(this).projectOnVector(t),this.sub(Yo)}reflect(t){return this.sub(Yo.copy(t).multiplyScalar(2*this.dot(t)))}angleTo(t){const e=Math.sqrt(this.lengthSq()*t.lengthSq());if(e===0)return Math.PI/2;const i=this.dot(t)/e;return Math.acos(sn(i,-1,1))}distanceTo(t){return Math.sqrt(this.distanceToSquared(t))}distanceToSquared(t){const e=this.x-t.x,i=this.y-t.y,s=this.z-t.z;return e*e+i*i+s*s}manhattanDistanceTo(t){return Math.abs(this.x-t.x)+Math.abs(this.y-t.y)+Math.abs(this.z-t.z)}setFromSpherical(t){return this.setFromSphericalCoords(t.radius,t.phi,t.theta)}setFromSphericalCoords(t,e,i){const s=Math.sin(e)*t;return this.x=s*Math.sin(i),this.y=Math.cos(e)*t,this.z=s*Math.cos(i),this}setFromCylindrical(t){return this.setFromCylindricalCoords(t.radius,t.theta,t.y)}setFromCylindricalCoords(t,e,i){return this.x=t*Math.sin(e),this.y=i,this.z=t*Math.cos(e),this}setFromMatrixPosition(t){const e=t.elements;return this.x=e[12],this.y=e[13],this.z=e[14],this}setFromMatrixScale(t){const e=this.setFromMatrixColumn(t,0).length(),i=this.setFromMatrixColumn(t,1).length(),s=this.setFromMatrixColumn(t,2).length();return this.x=e,this.y=i,this.z=s,this}setFromMatrixColumn(t,e){return this.fromArray(t.elements,e*4)}setFromMatrix3Column(t,e){return this.fromArray(t.elements,e*3)}setFromEuler(t){return this.x=t._x,this.y=t._y,this.z=t._z,this}setFromColor(t){return this.x=t.r,this.y=t.g,this.z=t.b,this}equals(t){return t.x===this.x&&t.y===this.y&&t.z===this.z}fromArray(t,e=0){return this.x=t[e],this.y=t[e+1],this.z=t[e+2],this}toArray(t=[],e=0){return t[e]=this.x,t[e+1]=this.y,t[e+2]=this.z,t}fromBufferAttribute(t,e){return this.x=t.getX(e),this.y=t.getY(e),this.z=t.getZ(e),this}random(){return this.x=Math.random(),this.y=Math.random(),this.z=Math.random(),this}randomDirection(){const t=(Math.random()-.5)*2,e=Math.random()*Math.PI*2,i=Math.sqrt(1-t**2);return this.x=i*Math.cos(e),this.y=i*Math.sin(e),this.z=t,this}*[Symbol.iterator](){yield this.x,yield this.y,yield this.z}}const Yo=new z,Hc=new ar;class lr{constructor(t=new z(1/0,1/0,1/0),e=new z(-1/0,-1/0,-1/0)){this.isBox3=!0,this.min=t,this.max=e}set(t,e){return this.min.copy(t),this.max.copy(e),this}setFromArray(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e+=3)this.expandByPoint(xn.fromArray(t,e));return this}setFromBufferAttribute(t){this.makeEmpty();for(let e=0,i=t.count;e<i;e++)this.expandByPoint(xn.fromBufferAttribute(t,e));return this}setFromPoints(t){this.makeEmpty();for(let e=0,i=t.length;e<i;e++)this.expandByPoint(t[e]);return this}setFromCenterAndSize(t,e){const i=xn.copy(e).multiplyScalar(.5);return this.min.copy(t).sub(i),this.max.copy(t).add(i),this}setFromObject(t,e=!1){return this.makeEmpty(),this.expandByObject(t,e)}clone(){return new this.constructor().copy(this)}copy(t){return this.min.copy(t.min),this.max.copy(t.max),this}makeEmpty(){return this.min.x=this.min.y=this.min.z=1/0,this.max.x=this.max.y=this.max.z=-1/0,this}isEmpty(){return this.max.x<this.min.x||this.max.y<this.min.y||this.max.z<this.min.z}getCenter(t){return this.isEmpty()?t.set(0,0,0):t.addVectors(this.min,this.max).multiplyScalar(.5)}getSize(t){return this.isEmpty()?t.set(0,0,0):t.subVectors(this.max,this.min)}expandByPoint(t){return this.min.min(t),this.max.max(t),this}expandByVector(t){return this.min.sub(t),this.max.add(t),this}expandByScalar(t){return this.min.addScalar(-t),this.max.addScalar(t),this}expandByObject(t,e=!1){t.updateWorldMatrix(!1,!1);const i=t.geometry;if(i!==void 0){const r=i.getAttribute("position");if(e===!0&&r!==void 0&&t.isInstancedMesh!==!0)for(let a=0,o=r.count;a<o;a++)t.isMesh===!0?t.getVertexPosition(a,xn):xn.fromBufferAttribute(r,a),xn.applyMatrix4(t.matrixWorld),this.expandByPoint(xn);else t.boundingBox!==void 0?(t.boundingBox===null&&t.computeBoundingBox(),vr.copy(t.boundingBox)):(i.boundingBox===null&&i.computeBoundingBox(),vr.copy(i.boundingBox)),vr.applyMatrix4(t.matrixWorld),this.union(vr)}const s=t.children;for(let r=0,a=s.length;r<a;r++)this.expandByObject(s[r],e);return this}containsPoint(t){return!(t.x<this.min.x||t.x>this.max.x||t.y<this.min.y||t.y>this.max.y||t.z<this.min.z||t.z>this.max.z)}containsBox(t){return this.min.x<=t.min.x&&t.max.x<=this.max.x&&this.min.y<=t.min.y&&t.max.y<=this.max.y&&this.min.z<=t.min.z&&t.max.z<=this.max.z}getParameter(t,e){return e.set((t.x-this.min.x)/(this.max.x-this.min.x),(t.y-this.min.y)/(this.max.y-this.min.y),(t.z-this.min.z)/(this.max.z-this.min.z))}intersectsBox(t){return!(t.max.x<this.min.x||t.min.x>this.max.x||t.max.y<this.min.y||t.min.y>this.max.y||t.max.z<this.min.z||t.min.z>this.max.z)}intersectsSphere(t){return this.clampPoint(t.center,xn),xn.distanceToSquared(t.center)<=t.radius*t.radius}intersectsPlane(t){let e,i;return t.normal.x>0?(e=t.normal.x*this.min.x,i=t.normal.x*this.max.x):(e=t.normal.x*this.max.x,i=t.normal.x*this.min.x),t.normal.y>0?(e+=t.normal.y*this.min.y,i+=t.normal.y*this.max.y):(e+=t.normal.y*this.max.y,i+=t.normal.y*this.min.y),t.normal.z>0?(e+=t.normal.z*this.min.z,i+=t.normal.z*this.max.z):(e+=t.normal.z*this.max.z,i+=t.normal.z*this.min.z),e<=-t.constant&&i>=-t.constant}intersectsTriangle(t){if(this.isEmpty())return!1;this.getCenter(Ds),xr.subVectors(this.max,Ds),Xi.subVectors(t.a,Ds),qi.subVectors(t.b,Ds),ji.subVectors(t.c,Ds),ai.subVectors(qi,Xi),li.subVectors(ji,qi),Ai.subVectors(Xi,ji);let e=[0,-ai.z,ai.y,0,-li.z,li.y,0,-Ai.z,Ai.y,ai.z,0,-ai.x,li.z,0,-li.x,Ai.z,0,-Ai.x,-ai.y,ai.x,0,-li.y,li.x,0,-Ai.y,Ai.x,0];return!Ko(e,Xi,qi,ji,xr)||(e=[1,0,0,0,1,0,0,0,1],!Ko(e,Xi,qi,ji,xr))?!1:(yr.crossVectors(ai,li),e=[yr.x,yr.y,yr.z],Ko(e,Xi,qi,ji,xr))}clampPoint(t,e){return e.copy(t).clamp(this.min,this.max)}distanceToPoint(t){return this.clampPoint(t,xn).distanceTo(t)}getBoundingSphere(t){return this.isEmpty()?t.makeEmpty():(this.getCenter(t.center),t.radius=this.getSize(xn).length()*.5),t}intersect(t){return this.min.max(t.min),this.max.min(t.max),this.isEmpty()&&this.makeEmpty(),this}union(t){return this.min.min(t.min),this.max.max(t.max),this}applyMatrix4(t){return this.isEmpty()?this:(Hn[0].set(this.min.x,this.min.y,this.min.z).applyMatrix4(t),Hn[1].set(this.min.x,this.min.y,this.max.z).applyMatrix4(t),Hn[2].set(this.min.x,this.max.y,this.min.z).applyMatrix4(t),Hn[3].set(this.min.x,this.max.y,this.max.z).applyMatrix4(t),Hn[4].set(this.max.x,this.min.y,this.min.z).applyMatrix4(t),Hn[5].set(this.max.x,this.min.y,this.max.z).applyMatrix4(t),Hn[6].set(this.max.x,this.max.y,this.min.z).applyMatrix4(t),Hn[7].set(this.max.x,this.max.y,this.max.z).applyMatrix4(t),this.setFromPoints(Hn),this)}translate(t){return this.min.add(t),this.max.add(t),this}equals(t){return t.min.equals(this.min)&&t.max.equals(this.max)}}const Hn=[new z,new z,new z,new z,new z,new z,new z,new z],xn=new z,vr=new lr,Xi=new z,qi=new z,ji=new z,ai=new z,li=new z,Ai=new z,Ds=new z,xr=new z,yr=new z,Ci=new z;function Ko(n,t,e,i,s){for(let r=0,a=n.length-3;r<=a;r+=3){Ci.fromArray(n,r);const o=s.x*Math.abs(Ci.x)+s.y*Math.abs(Ci.y)+s.z*Math.abs(Ci.z),l=t.dot(Ci),c=e.dot(Ci),d=i.dot(Ci);if(Math.max(-Math.max(l,c,d),Math.min(l,c,d))>o)return!1}return!0}const m_=new lr,Us=new z,Zo=new z;class cr{constructor(t=new z,e=-1){this.isSphere=!0,this.center=t,this.radius=e}set(t,e){return this.center.copy(t),this.radius=e,this}setFromPoints(t,e){const i=this.center;e!==void 0?i.copy(e):m_.setFromPoints(t).getCenter(i);let s=0;for(let r=0,a=t.length;r<a;r++)s=Math.max(s,i.distanceToSquared(t[r]));return this.radius=Math.sqrt(s),this}copy(t){return this.center.copy(t.center),this.radius=t.radius,this}isEmpty(){return this.radius<0}makeEmpty(){return this.center.set(0,0,0),this.radius=-1,this}containsPoint(t){return t.distanceToSquared(this.center)<=this.radius*this.radius}distanceToPoint(t){return t.distanceTo(this.center)-this.radius}intersectsSphere(t){const e=this.radius+t.radius;return t.center.distanceToSquared(this.center)<=e*e}intersectsBox(t){return t.intersectsSphere(this)}intersectsPlane(t){return Math.abs(t.distanceToPoint(this.center))<=this.radius}clampPoint(t,e){const i=this.center.distanceToSquared(t);return e.copy(t),i>this.radius*this.radius&&(e.sub(this.center).normalize(),e.multiplyScalar(this.radius).add(this.center)),e}getBoundingBox(t){return this.isEmpty()?(t.makeEmpty(),t):(t.set(this.center,this.center),t.expandByScalar(this.radius),t)}applyMatrix4(t){return this.center.applyMatrix4(t),this.radius=this.radius*t.getMaxScaleOnAxis(),this}translate(t){return this.center.add(t),this}expandByPoint(t){if(this.isEmpty())return this.center.copy(t),this.radius=0,this;Us.subVectors(t,this.center);const e=Us.lengthSq();if(e>this.radius*this.radius){const i=Math.sqrt(e),s=(i-this.radius)*.5;this.center.addScaledVector(Us,s/i),this.radius+=s}return this}union(t){return t.isEmpty()?this:this.isEmpty()?(this.copy(t),this):(this.center.equals(t.center)===!0?this.radius=Math.max(this.radius,t.radius):(Zo.subVectors(t.center,this.center).setLength(t.radius),this.expandByPoint(Us.copy(t.center).add(Zo)),this.expandByPoint(Us.copy(t.center).sub(Zo))),this)}equals(t){return t.center.equals(this.center)&&t.radius===this.radius}clone(){return new this.constructor().copy(this)}}const Vn=new z,Jo=new z,br=new z,ci=new z,Qo=new z,Mr=new z,ta=new z;class al{constructor(t=new z,e=new z(0,0,-1)){this.origin=t,this.direction=e}set(t,e){return this.origin.copy(t),this.direction.copy(e),this}copy(t){return this.origin.copy(t.origin),this.direction.copy(t.direction),this}at(t,e){return e.copy(this.origin).addScaledVector(this.direction,t)}lookAt(t){return this.direction.copy(t).sub(this.origin).normalize(),this}recast(t){return this.origin.copy(this.at(t,Vn)),this}closestPointToPoint(t,e){e.subVectors(t,this.origin);const i=e.dot(this.direction);return i<0?e.copy(this.origin):e.copy(this.origin).addScaledVector(this.direction,i)}distanceToPoint(t){return Math.sqrt(this.distanceSqToPoint(t))}distanceSqToPoint(t){const e=Vn.subVectors(t,this.origin).dot(this.direction);return e<0?this.origin.distanceToSquared(t):(Vn.copy(this.origin).addScaledVector(this.direction,e),Vn.distanceToSquared(t))}distanceSqToSegment(t,e,i,s){Jo.copy(t).add(e).multiplyScalar(.5),br.copy(e).sub(t).normalize(),ci.copy(this.origin).sub(Jo);const r=t.distanceTo(e)*.5,a=-this.direction.dot(br),o=ci.dot(this.direction),l=-ci.dot(br),c=ci.lengthSq(),d=Math.abs(1-a*a);let h,p,f,v;if(d>0)if(h=a*l-o,p=a*o-l,v=r*d,h>=0)if(p>=-v)if(p<=v){const g=1/d;h*=g,p*=g,f=h*(h+a*p+2*o)+p*(a*h+p+2*l)+c}else p=r,h=Math.max(0,-(a*p+o)),f=-h*h+p*(p+2*l)+c;else p=-r,h=Math.max(0,-(a*p+o)),f=-h*h+p*(p+2*l)+c;else p<=-v?(h=Math.max(0,-(-a*r+o)),p=h>0?-r:Math.min(Math.max(-r,-l),r),f=-h*h+p*(p+2*l)+c):p<=v?(h=0,p=Math.min(Math.max(-r,-l),r),f=p*(p+2*l)+c):(h=Math.max(0,-(a*r+o)),p=h>0?r:Math.min(Math.max(-r,-l),r),f=-h*h+p*(p+2*l)+c);else p=a>0?-r:r,h=Math.max(0,-(a*p+o)),f=-h*h+p*(p+2*l)+c;return i&&i.copy(this.origin).addScaledVector(this.direction,h),s&&s.copy(Jo).addScaledVector(br,p),f}intersectSphere(t,e){Vn.subVectors(t.center,this.origin);const i=Vn.dot(this.direction),s=Vn.dot(Vn)-i*i,r=t.radius*t.radius;if(s>r)return null;const a=Math.sqrt(r-s),o=i-a,l=i+a;return l<0?null:o<0?this.at(l,e):this.at(o,e)}intersectsSphere(t){return this.distanceSqToPoint(t.center)<=t.radius*t.radius}distanceToPlane(t){const e=t.normal.dot(this.direction);if(e===0)return t.distanceToPoint(this.origin)===0?0:null;const i=-(this.origin.dot(t.normal)+t.constant)/e;return i>=0?i:null}intersectPlane(t,e){const i=this.distanceToPlane(t);return i===null?null:this.at(i,e)}intersectsPlane(t){const e=t.distanceToPoint(this.origin);return e===0||t.normal.dot(this.direction)*e<0}intersectBox(t,e){let i,s,r,a,o,l;const c=1/this.direction.x,d=1/this.direction.y,h=1/this.direction.z,p=this.origin;return c>=0?(i=(t.min.x-p.x)*c,s=(t.max.x-p.x)*c):(i=(t.max.x-p.x)*c,s=(t.min.x-p.x)*c),d>=0?(r=(t.min.y-p.y)*d,a=(t.max.y-p.y)*d):(r=(t.max.y-p.y)*d,a=(t.min.y-p.y)*d),i>a||r>s||((r>i||isNaN(i))&&(i=r),(a<s||isNaN(s))&&(s=a),h>=0?(o=(t.min.z-p.z)*h,l=(t.max.z-p.z)*h):(o=(t.max.z-p.z)*h,l=(t.min.z-p.z)*h),i>l||o>s)||((o>i||i!==i)&&(i=o),(l<s||s!==s)&&(s=l),s<0)?null:this.at(i>=0?i:s,e)}intersectsBox(t){return this.intersectBox(t,Vn)!==null}intersectTriangle(t,e,i,s,r){Qo.subVectors(e,t),Mr.subVectors(i,t),ta.crossVectors(Qo,Mr);let a=this.direction.dot(ta),o;if(a>0){if(s)return null;o=1}else if(a<0)o=-1,a=-a;else return null;ci.subVectors(this.origin,t);const l=o*this.direction.dot(Mr.crossVectors(ci,Mr));if(l<0)return null;const c=o*this.direction.dot(Qo.cross(ci));if(c<0||l+c>a)return null;const d=-o*ci.dot(ta);return d<0?null:this.at(d/a,r)}applyMatrix4(t){return this.origin.applyMatrix4(t),this.direction.transformDirection(t),this}equals(t){return t.origin.equals(this.origin)&&t.direction.equals(this.direction)}clone(){return new this.constructor().copy(this)}}class Te{constructor(t,e,i,s,r,a,o,l,c,d,h,p,f,v,g,_){Te.prototype.isMatrix4=!0,this.elements=[1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1],t!==void 0&&this.set(t,e,i,s,r,a,o,l,c,d,h,p,f,v,g,_)}set(t,e,i,s,r,a,o,l,c,d,h,p,f,v,g,_){const m=this.elements;return m[0]=t,m[4]=e,m[8]=i,m[12]=s,m[1]=r,m[5]=a,m[9]=o,m[13]=l,m[2]=c,m[6]=d,m[10]=h,m[14]=p,m[3]=f,m[7]=v,m[11]=g,m[15]=_,this}identity(){return this.set(1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1),this}clone(){return new Te().fromArray(this.elements)}copy(t){const e=this.elements,i=t.elements;return e[0]=i[0],e[1]=i[1],e[2]=i[2],e[3]=i[3],e[4]=i[4],e[5]=i[5],e[6]=i[6],e[7]=i[7],e[8]=i[8],e[9]=i[9],e[10]=i[10],e[11]=i[11],e[12]=i[12],e[13]=i[13],e[14]=i[14],e[15]=i[15],this}copyPosition(t){const e=this.elements,i=t.elements;return e[12]=i[12],e[13]=i[13],e[14]=i[14],this}setFromMatrix3(t){const e=t.elements;return this.set(e[0],e[3],e[6],0,e[1],e[4],e[7],0,e[2],e[5],e[8],0,0,0,0,1),this}extractBasis(t,e,i){return t.setFromMatrixColumn(this,0),e.setFromMatrixColumn(this,1),i.setFromMatrixColumn(this,2),this}makeBasis(t,e,i){return this.set(t.x,e.x,i.x,0,t.y,e.y,i.y,0,t.z,e.z,i.z,0,0,0,0,1),this}extractRotation(t){const e=this.elements,i=t.elements,s=1/Yi.setFromMatrixColumn(t,0).length(),r=1/Yi.setFromMatrixColumn(t,1).length(),a=1/Yi.setFromMatrixColumn(t,2).length();return e[0]=i[0]*s,e[1]=i[1]*s,e[2]=i[2]*s,e[3]=0,e[4]=i[4]*r,e[5]=i[5]*r,e[6]=i[6]*r,e[7]=0,e[8]=i[8]*a,e[9]=i[9]*a,e[10]=i[10]*a,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromEuler(t){const e=this.elements,i=t.x,s=t.y,r=t.z,a=Math.cos(i),o=Math.sin(i),l=Math.cos(s),c=Math.sin(s),d=Math.cos(r),h=Math.sin(r);if(t.order==="XYZ"){const p=a*d,f=a*h,v=o*d,g=o*h;e[0]=l*d,e[4]=-l*h,e[8]=c,e[1]=f+v*c,e[5]=p-g*c,e[9]=-o*l,e[2]=g-p*c,e[6]=v+f*c,e[10]=a*l}else if(t.order==="YXZ"){const p=l*d,f=l*h,v=c*d,g=c*h;e[0]=p+g*o,e[4]=v*o-f,e[8]=a*c,e[1]=a*h,e[5]=a*d,e[9]=-o,e[2]=f*o-v,e[6]=g+p*o,e[10]=a*l}else if(t.order==="ZXY"){const p=l*d,f=l*h,v=c*d,g=c*h;e[0]=p-g*o,e[4]=-a*h,e[8]=v+f*o,e[1]=f+v*o,e[5]=a*d,e[9]=g-p*o,e[2]=-a*c,e[6]=o,e[10]=a*l}else if(t.order==="ZYX"){const p=a*d,f=a*h,v=o*d,g=o*h;e[0]=l*d,e[4]=v*c-f,e[8]=p*c+g,e[1]=l*h,e[5]=g*c+p,e[9]=f*c-v,e[2]=-c,e[6]=o*l,e[10]=a*l}else if(t.order==="YZX"){const p=a*l,f=a*c,v=o*l,g=o*c;e[0]=l*d,e[4]=g-p*h,e[8]=v*h+f,e[1]=h,e[5]=a*d,e[9]=-o*d,e[2]=-c*d,e[6]=f*h+v,e[10]=p-g*h}else if(t.order==="XZY"){const p=a*l,f=a*c,v=o*l,g=o*c;e[0]=l*d,e[4]=-h,e[8]=c*d,e[1]=p*h+g,e[5]=a*d,e[9]=f*h-v,e[2]=v*h-f,e[6]=o*d,e[10]=g*h+p}return e[3]=0,e[7]=0,e[11]=0,e[12]=0,e[13]=0,e[14]=0,e[15]=1,this}makeRotationFromQuaternion(t){return this.compose(__,t,g_)}lookAt(t,e,i){const s=this.elements;return on.subVectors(t,e),on.lengthSq()===0&&(on.z=1),on.normalize(),ui.crossVectors(i,on),ui.lengthSq()===0&&(Math.abs(i.z)===1?on.x+=1e-4:on.z+=1e-4,on.normalize(),ui.crossVectors(i,on)),ui.normalize(),Sr.crossVectors(on,ui),s[0]=ui.x,s[4]=Sr.x,s[8]=on.x,s[1]=ui.y,s[5]=Sr.y,s[9]=on.y,s[2]=ui.z,s[6]=Sr.z,s[10]=on.z,this}multiply(t){return this.multiplyMatrices(this,t)}premultiply(t){return this.multiplyMatrices(t,this)}multiplyMatrices(t,e){const i=t.elements,s=e.elements,r=this.elements,a=i[0],o=i[4],l=i[8],c=i[12],d=i[1],h=i[5],p=i[9],f=i[13],v=i[2],g=i[6],_=i[10],m=i[14],E=i[3],b=i[7],x=i[11],C=i[15],A=s[0],D=s[4],G=s[8],T=s[12],R=s[1],J=s[5],rt=s[9],mt=s[13],O=s[2],Z=s[6],tt=s[10],st=s[14],et=s[3],ot=s[7],ut=s[11],pt=s[15];return r[0]=a*A+o*R+l*O+c*et,r[4]=a*D+o*J+l*Z+c*ot,r[8]=a*G+o*rt+l*tt+c*ut,r[12]=a*T+o*mt+l*st+c*pt,r[1]=d*A+h*R+p*O+f*et,r[5]=d*D+h*J+p*Z+f*ot,r[9]=d*G+h*rt+p*tt+f*ut,r[13]=d*T+h*mt+p*st+f*pt,r[2]=v*A+g*R+_*O+m*et,r[6]=v*D+g*J+_*Z+m*ot,r[10]=v*G+g*rt+_*tt+m*ut,r[14]=v*T+g*mt+_*st+m*pt,r[3]=E*A+b*R+x*O+C*et,r[7]=E*D+b*J+x*Z+C*ot,r[11]=E*G+b*rt+x*tt+C*ut,r[15]=E*T+b*mt+x*st+C*pt,this}multiplyScalar(t){const e=this.elements;return e[0]*=t,e[4]*=t,e[8]*=t,e[12]*=t,e[1]*=t,e[5]*=t,e[9]*=t,e[13]*=t,e[2]*=t,e[6]*=t,e[10]*=t,e[14]*=t,e[3]*=t,e[7]*=t,e[11]*=t,e[15]*=t,this}determinant(){const t=this.elements,e=t[0],i=t[4],s=t[8],r=t[12],a=t[1],o=t[5],l=t[9],c=t[13],d=t[2],h=t[6],p=t[10],f=t[14],v=t[3],g=t[7],_=t[11],m=t[15];return v*(+r*l*h-s*c*h-r*o*p+i*c*p+s*o*f-i*l*f)+g*(+e*l*f-e*c*p+r*a*p-s*a*f+s*c*d-r*l*d)+_*(+e*c*h-e*o*f-r*a*h+i*a*f+r*o*d-i*c*d)+m*(-s*o*d-e*l*h+e*o*p+s*a*h-i*a*p+i*l*d)}transpose(){const t=this.elements;let e;return e=t[1],t[1]=t[4],t[4]=e,e=t[2],t[2]=t[8],t[8]=e,e=t[6],t[6]=t[9],t[9]=e,e=t[3],t[3]=t[12],t[12]=e,e=t[7],t[7]=t[13],t[13]=e,e=t[11],t[11]=t[14],t[14]=e,this}setPosition(t,e,i){const s=this.elements;return t.isVector3?(s[12]=t.x,s[13]=t.y,s[14]=t.z):(s[12]=t,s[13]=e,s[14]=i),this}invert(){const t=this.elements,e=t[0],i=t[1],s=t[2],r=t[3],a=t[4],o=t[5],l=t[6],c=t[7],d=t[8],h=t[9],p=t[10],f=t[11],v=t[12],g=t[13],_=t[14],m=t[15],E=h*_*c-g*p*c+g*l*f-o*_*f-h*l*m+o*p*m,b=v*p*c-d*_*c-v*l*f+a*_*f+d*l*m-a*p*m,x=d*g*c-v*h*c+v*o*f-a*g*f-d*o*m+a*h*m,C=v*h*l-d*g*l-v*o*p+a*g*p+d*o*_-a*h*_,A=e*E+i*b+s*x+r*C;if(A===0)return this.set(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);const D=1/A;return t[0]=E*D,t[1]=(g*p*r-h*_*r-g*s*f+i*_*f+h*s*m-i*p*m)*D,t[2]=(o*_*r-g*l*r+g*s*c-i*_*c-o*s*m+i*l*m)*D,t[3]=(h*l*r-o*p*r-h*s*c+i*p*c+o*s*f-i*l*f)*D,t[4]=b*D,t[5]=(d*_*r-v*p*r+v*s*f-e*_*f-d*s*m+e*p*m)*D,t[6]=(v*l*r-a*_*r-v*s*c+e*_*c+a*s*m-e*l*m)*D,t[7]=(a*p*r-d*l*r+d*s*c-e*p*c-a*s*f+e*l*f)*D,t[8]=x*D,t[9]=(v*h*r-d*g*r-v*i*f+e*g*f+d*i*m-e*h*m)*D,t[10]=(a*g*r-v*o*r+v*i*c-e*g*c-a*i*m+e*o*m)*D,t[11]=(d*o*r-a*h*r-d*i*c+e*h*c+a*i*f-e*o*f)*D,t[12]=C*D,t[13]=(d*g*s-v*h*s+v*i*p-e*g*p-d*i*_+e*h*_)*D,t[14]=(v*o*s-a*g*s-v*i*l+e*g*l+a*i*_-e*o*_)*D,t[15]=(a*h*s-d*o*s+d*i*l-e*h*l-a*i*p+e*o*p)*D,this}scale(t){const e=this.elements,i=t.x,s=t.y,r=t.z;return e[0]*=i,e[4]*=s,e[8]*=r,e[1]*=i,e[5]*=s,e[9]*=r,e[2]*=i,e[6]*=s,e[10]*=r,e[3]*=i,e[7]*=s,e[11]*=r,this}getMaxScaleOnAxis(){const t=this.elements,e=t[0]*t[0]+t[1]*t[1]+t[2]*t[2],i=t[4]*t[4]+t[5]*t[5]+t[6]*t[6],s=t[8]*t[8]+t[9]*t[9]+t[10]*t[10];return Math.sqrt(Math.max(e,i,s))}makeTranslation(t,e,i){return t.isVector3?this.set(1,0,0,t.x,0,1,0,t.y,0,0,1,t.z,0,0,0,1):this.set(1,0,0,t,0,1,0,e,0,0,1,i,0,0,0,1),this}makeRotationX(t){const e=Math.cos(t),i=Math.sin(t);return this.set(1,0,0,0,0,e,-i,0,0,i,e,0,0,0,0,1),this}makeRotationY(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,0,i,0,0,1,0,0,-i,0,e,0,0,0,0,1),this}makeRotationZ(t){const e=Math.cos(t),i=Math.sin(t);return this.set(e,-i,0,0,i,e,0,0,0,0,1,0,0,0,0,1),this}makeRotationAxis(t,e){const i=Math.cos(e),s=Math.sin(e),r=1-i,a=t.x,o=t.y,l=t.z,c=r*a,d=r*o;return this.set(c*a+i,c*o-s*l,c*l+s*o,0,c*o+s*l,d*o+i,d*l-s*a,0,c*l-s*o,d*l+s*a,r*l*l+i,0,0,0,0,1),this}makeScale(t,e,i){return this.set(t,0,0,0,0,e,0,0,0,0,i,0,0,0,0,1),this}makeShear(t,e,i,s,r,a){return this.set(1,i,r,0,t,1,a,0,e,s,1,0,0,0,0,1),this}compose(t,e,i){const s=this.elements,r=e._x,a=e._y,o=e._z,l=e._w,c=r+r,d=a+a,h=o+o,p=r*c,f=r*d,v=r*h,g=a*d,_=a*h,m=o*h,E=l*c,b=l*d,x=l*h,C=i.x,A=i.y,D=i.z;return s[0]=(1-(g+m))*C,s[1]=(f+x)*C,s[2]=(v-b)*C,s[3]=0,s[4]=(f-x)*A,s[5]=(1-(p+m))*A,s[6]=(_+E)*A,s[7]=0,s[8]=(v+b)*D,s[9]=(_-E)*D,s[10]=(1-(p+g))*D,s[11]=0,s[12]=t.x,s[13]=t.y,s[14]=t.z,s[15]=1,this}decompose(t,e,i){const s=this.elements;let r=Yi.set(s[0],s[1],s[2]).length();const a=Yi.set(s[4],s[5],s[6]).length(),o=Yi.set(s[8],s[9],s[10]).length();this.determinant()<0&&(r=-r),t.x=s[12],t.y=s[13],t.z=s[14],yn.copy(this);const c=1/r,d=1/a,h=1/o;return yn.elements[0]*=c,yn.elements[1]*=c,yn.elements[2]*=c,yn.elements[4]*=d,yn.elements[5]*=d,yn.elements[6]*=d,yn.elements[8]*=h,yn.elements[9]*=h,yn.elements[10]*=h,e.setFromRotationMatrix(yn),i.x=r,i.y=a,i.z=o,this}makePerspective(t,e,i,s,r,a,o=ti){const l=this.elements,c=2*r/(e-t),d=2*r/(i-s),h=(e+t)/(e-t),p=(i+s)/(i-s);let f,v;if(o===ti)f=-(a+r)/(a-r),v=-2*a*r/(a-r);else if(o===ro)f=-a/(a-r),v=-a*r/(a-r);else throw new Error("THREE.Matrix4.makePerspective(): Invalid coordinate system: "+o);return l[0]=c,l[4]=0,l[8]=h,l[12]=0,l[1]=0,l[5]=d,l[9]=p,l[13]=0,l[2]=0,l[6]=0,l[10]=f,l[14]=v,l[3]=0,l[7]=0,l[11]=-1,l[15]=0,this}makeOrthographic(t,e,i,s,r,a,o=ti){const l=this.elements,c=1/(e-t),d=1/(i-s),h=1/(a-r),p=(e+t)*c,f=(i+s)*d;let v,g;if(o===ti)v=(a+r)*h,g=-2*h;else if(o===ro)v=r*h,g=-1*h;else throw new Error("THREE.Matrix4.makeOrthographic(): Invalid coordinate system: "+o);return l[0]=2*c,l[4]=0,l[8]=0,l[12]=-p,l[1]=0,l[5]=2*d,l[9]=0,l[13]=-f,l[2]=0,l[6]=0,l[10]=g,l[14]=-v,l[3]=0,l[7]=0,l[11]=0,l[15]=1,this}equals(t){const e=this.elements,i=t.elements;for(let s=0;s<16;s++)if(e[s]!==i[s])return!1;return!0}fromArray(t,e=0){for(let i=0;i<16;i++)this.elements[i]=t[i+e];return this}toArray(t=[],e=0){const i=this.elements;return t[e]=i[0],t[e+1]=i[1],t[e+2]=i[2],t[e+3]=i[3],t[e+4]=i[4],t[e+5]=i[5],t[e+6]=i[6],t[e+7]=i[7],t[e+8]=i[8],t[e+9]=i[9],t[e+10]=i[10],t[e+11]=i[11],t[e+12]=i[12],t[e+13]=i[13],t[e+14]=i[14],t[e+15]=i[15],t}}const Yi=new z,yn=new Te,__=new z(0,0,0),g_=new z(1,1,1),ui=new z,Sr=new z,on=new z,Vc=new Te,Gc=new ar;class So{constructor(t=0,e=0,i=0,s=So.DEFAULT_ORDER){this.isEuler=!0,this._x=t,this._y=e,this._z=i,this._order=s}get x(){return this._x}set x(t){this._x=t,this._onChangeCallback()}get y(){return this._y}set y(t){this._y=t,this._onChangeCallback()}get z(){return this._z}set z(t){this._z=t,this._onChangeCallback()}get order(){return this._order}set order(t){this._order=t,this._onChangeCallback()}set(t,e,i,s=this._order){return this._x=t,this._y=e,this._z=i,this._order=s,this._onChangeCallback(),this}clone(){return new this.constructor(this._x,this._y,this._z,this._order)}copy(t){return this._x=t._x,this._y=t._y,this._z=t._z,this._order=t._order,this._onChangeCallback(),this}setFromRotationMatrix(t,e=this._order,i=!0){const s=t.elements,r=s[0],a=s[4],o=s[8],l=s[1],c=s[5],d=s[9],h=s[2],p=s[6],f=s[10];switch(e){case"XYZ":this._y=Math.asin(sn(o,-1,1)),Math.abs(o)<.9999999?(this._x=Math.atan2(-d,f),this._z=Math.atan2(-a,r)):(this._x=Math.atan2(p,c),this._z=0);break;case"YXZ":this._x=Math.asin(-sn(d,-1,1)),Math.abs(d)<.9999999?(this._y=Math.atan2(o,f),this._z=Math.atan2(l,c)):(this._y=Math.atan2(-h,r),this._z=0);break;case"ZXY":this._x=Math.asin(sn(p,-1,1)),Math.abs(p)<.9999999?(this._y=Math.atan2(-h,f),this._z=Math.atan2(-a,c)):(this._y=0,this._z=Math.atan2(l,r));break;case"ZYX":this._y=Math.asin(-sn(h,-1,1)),Math.abs(h)<.9999999?(this._x=Math.atan2(p,f),this._z=Math.atan2(l,r)):(this._x=0,this._z=Math.atan2(-a,c));break;case"YZX":this._z=Math.asin(sn(l,-1,1)),Math.abs(l)<.9999999?(this._x=Math.atan2(-d,c),this._y=Math.atan2(-h,r)):(this._x=0,this._y=Math.atan2(o,f));break;case"XZY":this._z=Math.asin(-sn(a,-1,1)),Math.abs(a)<.9999999?(this._x=Math.atan2(p,c),this._y=Math.atan2(o,r)):(this._x=Math.atan2(-d,f),this._y=0);break;default:console.warn("THREE.Euler: .setFromRotationMatrix() encountered an unknown order: "+e)}return this._order=e,i===!0&&this._onChangeCallback(),this}setFromQuaternion(t,e,i){return Vc.makeRotationFromQuaternion(t),this.setFromRotationMatrix(Vc,e,i)}setFromVector3(t,e=this._order){return this.set(t.x,t.y,t.z,e)}reorder(t){return Gc.setFromEuler(this),this.setFromQuaternion(Gc,t)}equals(t){return t._x===this._x&&t._y===this._y&&t._z===this._z&&t._order===this._order}fromArray(t){return this._x=t[0],this._y=t[1],this._z=t[2],t[3]!==void 0&&(this._order=t[3]),this._onChangeCallback(),this}toArray(t=[],e=0){return t[e]=this._x,t[e+1]=this._y,t[e+2]=this._z,t[e+3]=this._order,t}_onChange(t){return this._onChangeCallback=t,this}_onChangeCallback(){}*[Symbol.iterator](){yield this._x,yield this._y,yield this._z,yield this._order}}So.DEFAULT_ORDER="XYZ";class nh{constructor(){this.mask=1}set(t){this.mask=(1<<t|0)>>>0}enable(t){this.mask|=1<<t|0}enableAll(){this.mask=-1}toggle(t){this.mask^=1<<t|0}disable(t){this.mask&=~(1<<t|0)}disableAll(){this.mask=0}test(t){return(this.mask&t.mask)!==0}isEnabled(t){return(this.mask&(1<<t|0))!==0}}let v_=0;const Wc=new z,Ki=new ar,Gn=new Te,Er=new z,Is=new z,x_=new z,y_=new ar,$c=new z(1,0,0),Xc=new z(0,1,0),qc=new z(0,0,1),b_={type:"added"},M_={type:"removed"};class We extends Ts{constructor(){super(),this.isObject3D=!0,Object.defineProperty(this,"id",{value:v_++}),this.uuid=or(),this.name="",this.type="Object3D",this.parent=null,this.children=[],this.up=We.DEFAULT_UP.clone();const t=new z,e=new So,i=new ar,s=new z(1,1,1);function r(){i.setFromEuler(e,!1)}function a(){e.setFromQuaternion(i,void 0,!1)}e._onChange(r),i._onChange(a),Object.defineProperties(this,{position:{configurable:!0,enumerable:!0,value:t},rotation:{configurable:!0,enumerable:!0,value:e},quaternion:{configurable:!0,enumerable:!0,value:i},scale:{configurable:!0,enumerable:!0,value:s},modelViewMatrix:{value:new Te},normalMatrix:{value:new te}}),this.matrix=new Te,this.matrixWorld=new Te,this.matrixAutoUpdate=We.DEFAULT_MATRIX_AUTO_UPDATE,this.matrixWorldAutoUpdate=We.DEFAULT_MATRIX_WORLD_AUTO_UPDATE,this.matrixWorldNeedsUpdate=!1,this.layers=new nh,this.visible=!0,this.castShadow=!1,this.receiveShadow=!1,this.frustumCulled=!0,this.renderOrder=0,this.animations=[],this.userData={}}onBeforeShadow(){}onAfterShadow(){}onBeforeRender(){}onAfterRender(){}applyMatrix4(t){this.matrixAutoUpdate&&this.updateMatrix(),this.matrix.premultiply(t),this.matrix.decompose(this.position,this.quaternion,this.scale)}applyQuaternion(t){return this.quaternion.premultiply(t),this}setRotationFromAxisAngle(t,e){this.quaternion.setFromAxisAngle(t,e)}setRotationFromEuler(t){this.quaternion.setFromEuler(t,!0)}setRotationFromMatrix(t){this.quaternion.setFromRotationMatrix(t)}setRotationFromQuaternion(t){this.quaternion.copy(t)}rotateOnAxis(t,e){return Ki.setFromAxisAngle(t,e),this.quaternion.multiply(Ki),this}rotateOnWorldAxis(t,e){return Ki.setFromAxisAngle(t,e),this.quaternion.premultiply(Ki),this}rotateX(t){return this.rotateOnAxis($c,t)}rotateY(t){return this.rotateOnAxis(Xc,t)}rotateZ(t){return this.rotateOnAxis(qc,t)}translateOnAxis(t,e){return Wc.copy(t).applyQuaternion(this.quaternion),this.position.add(Wc.multiplyScalar(e)),this}translateX(t){return this.translateOnAxis($c,t)}translateY(t){return this.translateOnAxis(Xc,t)}translateZ(t){return this.translateOnAxis(qc,t)}localToWorld(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(this.matrixWorld)}worldToLocal(t){return this.updateWorldMatrix(!0,!1),t.applyMatrix4(Gn.copy(this.matrixWorld).invert())}lookAt(t,e,i){t.isVector3?Er.copy(t):Er.set(t,e,i);const s=this.parent;this.updateWorldMatrix(!0,!1),Is.setFromMatrixPosition(this.matrixWorld),this.isCamera||this.isLight?Gn.lookAt(Is,Er,this.up):Gn.lookAt(Er,Is,this.up),this.quaternion.setFromRotationMatrix(Gn),s&&(Gn.extractRotation(s.matrixWorld),Ki.setFromRotationMatrix(Gn),this.quaternion.premultiply(Ki.invert()))}add(t){if(arguments.length>1){for(let e=0;e<arguments.length;e++)this.add(arguments[e]);return this}return t===this?(console.error("THREE.Object3D.add: object can't be added as a child of itself.",t),this):(t&&t.isObject3D?(t.parent!==null&&t.parent.remove(t),t.parent=this,this.children.push(t),t.dispatchEvent(b_)):console.error("THREE.Object3D.add: object not an instance of THREE.Object3D.",t),this)}remove(t){if(arguments.length>1){for(let i=0;i<arguments.length;i++)this.remove(arguments[i]);return this}const e=this.children.indexOf(t);return e!==-1&&(t.parent=null,this.children.splice(e,1),t.dispatchEvent(M_)),this}removeFromParent(){const t=this.parent;return t!==null&&t.remove(this),this}clear(){return this.remove(...this.children)}attach(t){return this.updateWorldMatrix(!0,!1),Gn.copy(this.matrixWorld).invert(),t.parent!==null&&(t.parent.updateWorldMatrix(!0,!1),Gn.multiply(t.parent.matrixWorld)),t.applyMatrix4(Gn),this.add(t),t.updateWorldMatrix(!1,!0),this}getObjectById(t){return this.getObjectByProperty("id",t)}getObjectByName(t){return this.getObjectByProperty("name",t)}getObjectByProperty(t,e){if(this[t]===e)return this;for(let i=0,s=this.children.length;i<s;i++){const a=this.children[i].getObjectByProperty(t,e);if(a!==void 0)return a}}getObjectsByProperty(t,e,i=[]){this[t]===e&&i.push(this);const s=this.children;for(let r=0,a=s.length;r<a;r++)s[r].getObjectsByProperty(t,e,i);return i}getWorldPosition(t){return this.updateWorldMatrix(!0,!1),t.setFromMatrixPosition(this.matrixWorld)}getWorldQuaternion(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Is,t,x_),t}getWorldScale(t){return this.updateWorldMatrix(!0,!1),this.matrixWorld.decompose(Is,y_,t),t}getWorldDirection(t){this.updateWorldMatrix(!0,!1);const e=this.matrixWorld.elements;return t.set(e[8],e[9],e[10]).normalize()}raycast(){}traverse(t){t(this);const e=this.children;for(let i=0,s=e.length;i<s;i++)e[i].traverse(t)}traverseVisible(t){if(this.visible===!1)return;t(this);const e=this.children;for(let i=0,s=e.length;i<s;i++)e[i].traverseVisible(t)}traverseAncestors(t){const e=this.parent;e!==null&&(t(e),e.traverseAncestors(t))}updateMatrix(){this.matrix.compose(this.position,this.quaternion,this.scale),this.matrixWorldNeedsUpdate=!0}updateMatrixWorld(t){this.matrixAutoUpdate&&this.updateMatrix(),(this.matrixWorldNeedsUpdate||t)&&(this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix),this.matrixWorldNeedsUpdate=!1,t=!0);const e=this.children;for(let i=0,s=e.length;i<s;i++){const r=e[i];(r.matrixWorldAutoUpdate===!0||t===!0)&&r.updateMatrixWorld(t)}}updateWorldMatrix(t,e){const i=this.parent;if(t===!0&&i!==null&&i.matrixWorldAutoUpdate===!0&&i.updateWorldMatrix(!0,!1),this.matrixAutoUpdate&&this.updateMatrix(),this.parent===null?this.matrixWorld.copy(this.matrix):this.matrixWorld.multiplyMatrices(this.parent.matrixWorld,this.matrix),e===!0){const s=this.children;for(let r=0,a=s.length;r<a;r++){const o=s[r];o.matrixWorldAutoUpdate===!0&&o.updateWorldMatrix(!1,!0)}}}toJSON(t){const e=t===void 0||typeof t=="string",i={};e&&(t={geometries:{},materials:{},textures:{},images:{},shapes:{},skeletons:{},animations:{},nodes:{}},i.metadata={version:4.6,type:"Object",generator:"Object3D.toJSON"});const s={};s.uuid=this.uuid,s.type=this.type,this.name!==""&&(s.name=this.name),this.castShadow===!0&&(s.castShadow=!0),this.receiveShadow===!0&&(s.receiveShadow=!0),this.visible===!1&&(s.visible=!1),this.frustumCulled===!1&&(s.frustumCulled=!1),this.renderOrder!==0&&(s.renderOrder=this.renderOrder),Object.keys(this.userData).length>0&&(s.userData=this.userData),s.layers=this.layers.mask,s.matrix=this.matrix.toArray(),s.up=this.up.toArray(),this.matrixAutoUpdate===!1&&(s.matrixAutoUpdate=!1),this.isInstancedMesh&&(s.type="InstancedMesh",s.count=this.count,s.instanceMatrix=this.instanceMatrix.toJSON(),this.instanceColor!==null&&(s.instanceColor=this.instanceColor.toJSON())),this.isBatchedMesh&&(s.type="BatchedMesh",s.perObjectFrustumCulled=this.perObjectFrustumCulled,s.sortObjects=this.sortObjects,s.drawRanges=this._drawRanges,s.reservedRanges=this._reservedRanges,s.visibility=this._visibility,s.active=this._active,s.bounds=this._bounds.map(o=>({boxInitialized:o.boxInitialized,boxMin:o.box.min.toArray(),boxMax:o.box.max.toArray(),sphereInitialized:o.sphereInitialized,sphereRadius:o.sphere.radius,sphereCenter:o.sphere.center.toArray()})),s.maxGeometryCount=this._maxGeometryCount,s.maxVertexCount=this._maxVertexCount,s.maxIndexCount=this._maxIndexCount,s.geometryInitialized=this._geometryInitialized,s.geometryCount=this._geometryCount,s.matricesTexture=this._matricesTexture.toJSON(t),this.boundingSphere!==null&&(s.boundingSphere={center:s.boundingSphere.center.toArray(),radius:s.boundingSphere.radius}),this.boundingBox!==null&&(s.boundingBox={min:s.boundingBox.min.toArray(),max:s.boundingBox.max.toArray()}));function r(o,l){return o[l.uuid]===void 0&&(o[l.uuid]=l.toJSON(t)),l.uuid}if(this.isScene)this.background&&(this.background.isColor?s.background=this.background.toJSON():this.background.isTexture&&(s.background=this.background.toJSON(t).uuid)),this.environment&&this.environment.isTexture&&this.environment.isRenderTargetTexture!==!0&&(s.environment=this.environment.toJSON(t).uuid);else if(this.isMesh||this.isLine||this.isPoints){s.geometry=r(t.geometries,this.geometry);const o=this.geometry.parameters;if(o!==void 0&&o.shapes!==void 0){const l=o.shapes;if(Array.isArray(l))for(let c=0,d=l.length;c<d;c++){const h=l[c];r(t.shapes,h)}else r(t.shapes,l)}}if(this.isSkinnedMesh&&(s.bindMode=this.bindMode,s.bindMatrix=this.bindMatrix.toArray(),this.skeleton!==void 0&&(r(t.skeletons,this.skeleton),s.skeleton=this.skeleton.uuid)),this.material!==void 0)if(Array.isArray(this.material)){const o=[];for(let l=0,c=this.material.length;l<c;l++)o.push(r(t.materials,this.material[l]));s.material=o}else s.material=r(t.materials,this.material);if(this.children.length>0){s.children=[];for(let o=0;o<this.children.length;o++)s.children.push(this.children[o].toJSON(t).object)}if(this.animations.length>0){s.animations=[];for(let o=0;o<this.animations.length;o++){const l=this.animations[o];s.animations.push(r(t.animations,l))}}if(e){const o=a(t.geometries),l=a(t.materials),c=a(t.textures),d=a(t.images),h=a(t.shapes),p=a(t.skeletons),f=a(t.animations),v=a(t.nodes);o.length>0&&(i.geometries=o),l.length>0&&(i.materials=l),c.length>0&&(i.textures=c),d.length>0&&(i.images=d),h.length>0&&(i.shapes=h),p.length>0&&(i.skeletons=p),f.length>0&&(i.animations=f),v.length>0&&(i.nodes=v)}return i.object=s,i;function a(o){const l=[];for(const c in o){const d=o[c];delete d.metadata,l.push(d)}return l}}clone(t){return new this.constructor().copy(this,t)}copy(t,e=!0){if(this.name=t.name,this.up.copy(t.up),this.position.copy(t.position),this.rotation.order=t.rotation.order,this.quaternion.copy(t.quaternion),this.scale.copy(t.scale),this.matrix.copy(t.matrix),this.matrixWorld.copy(t.matrixWorld),this.matrixAutoUpdate=t.matrixAutoUpdate,this.matrixWorldAutoUpdate=t.matrixWorldAutoUpdate,this.matrixWorldNeedsUpdate=t.matrixWorldNeedsUpdate,this.layers.mask=t.layers.mask,this.visible=t.visible,this.castShadow=t.castShadow,this.receiveShadow=t.receiveShadow,this.frustumCulled=t.frustumCulled,this.renderOrder=t.renderOrder,this.animations=t.animations.slice(),this.userData=JSON.parse(JSON.stringify(t.userData)),e===!0)for(let i=0;i<t.children.length;i++){const s=t.children[i];this.add(s.clone())}return this}}We.DEFAULT_UP=new z(0,1,0);We.DEFAULT_MATRIX_AUTO_UPDATE=!0;We.DEFAULT_MATRIX_WORLD_AUTO_UPDATE=!0;const bn=new z,Wn=new z,ea=new z,$n=new z,Zi=new z,Ji=new z,jc=new z,na=new z,ia=new z,sa=new z;let Tr=!1;class Mn{constructor(t=new z,e=new z,i=new z){this.a=t,this.b=e,this.c=i}static getNormal(t,e,i,s){s.subVectors(i,e),bn.subVectors(t,e),s.cross(bn);const r=s.lengthSq();return r>0?s.multiplyScalar(1/Math.sqrt(r)):s.set(0,0,0)}static getBarycoord(t,e,i,s,r){bn.subVectors(s,e),Wn.subVectors(i,e),ea.subVectors(t,e);const a=bn.dot(bn),o=bn.dot(Wn),l=bn.dot(ea),c=Wn.dot(Wn),d=Wn.dot(ea),h=a*c-o*o;if(h===0)return r.set(0,0,0),null;const p=1/h,f=(c*l-o*d)*p,v=(a*d-o*l)*p;return r.set(1-f-v,v,f)}static containsPoint(t,e,i,s){return this.getBarycoord(t,e,i,s,$n)===null?!1:$n.x>=0&&$n.y>=0&&$n.x+$n.y<=1}static getUV(t,e,i,s,r,a,o,l){return Tr===!1&&(console.warn("THREE.Triangle.getUV() has been renamed to THREE.Triangle.getInterpolation()."),Tr=!0),this.getInterpolation(t,e,i,s,r,a,o,l)}static getInterpolation(t,e,i,s,r,a,o,l){return this.getBarycoord(t,e,i,s,$n)===null?(l.x=0,l.y=0,"z"in l&&(l.z=0),"w"in l&&(l.w=0),null):(l.setScalar(0),l.addScaledVector(r,$n.x),l.addScaledVector(a,$n.y),l.addScaledVector(o,$n.z),l)}static isFrontFacing(t,e,i,s){return bn.subVectors(i,e),Wn.subVectors(t,e),bn.cross(Wn).dot(s)<0}set(t,e,i){return this.a.copy(t),this.b.copy(e),this.c.copy(i),this}setFromPointsAndIndices(t,e,i,s){return this.a.copy(t[e]),this.b.copy(t[i]),this.c.copy(t[s]),this}setFromAttributeAndIndices(t,e,i,s){return this.a.fromBufferAttribute(t,e),this.b.fromBufferAttribute(t,i),this.c.fromBufferAttribute(t,s),this}clone(){return new this.constructor().copy(this)}copy(t){return this.a.copy(t.a),this.b.copy(t.b),this.c.copy(t.c),this}getArea(){return bn.subVectors(this.c,this.b),Wn.subVectors(this.a,this.b),bn.cross(Wn).length()*.5}getMidpoint(t){return t.addVectors(this.a,this.b).add(this.c).multiplyScalar(1/3)}getNormal(t){return Mn.getNormal(this.a,this.b,this.c,t)}getPlane(t){return t.setFromCoplanarPoints(this.a,this.b,this.c)}getBarycoord(t,e){return Mn.getBarycoord(t,this.a,this.b,this.c,e)}getUV(t,e,i,s,r){return Tr===!1&&(console.warn("THREE.Triangle.getUV() has been renamed to THREE.Triangle.getInterpolation()."),Tr=!0),Mn.getInterpolation(t,this.a,this.b,this.c,e,i,s,r)}getInterpolation(t,e,i,s,r){return Mn.getInterpolation(t,this.a,this.b,this.c,e,i,s,r)}containsPoint(t){return Mn.containsPoint(t,this.a,this.b,this.c)}isFrontFacing(t){return Mn.isFrontFacing(this.a,this.b,this.c,t)}intersectsBox(t){return t.intersectsTriangle(this)}closestPointToPoint(t,e){const i=this.a,s=this.b,r=this.c;let a,o;Zi.subVectors(s,i),Ji.subVectors(r,i),na.subVectors(t,i);const l=Zi.dot(na),c=Ji.dot(na);if(l<=0&&c<=0)return e.copy(i);ia.subVectors(t,s);const d=Zi.dot(ia),h=Ji.dot(ia);if(d>=0&&h<=d)return e.copy(s);const p=l*h-d*c;if(p<=0&&l>=0&&d<=0)return a=l/(l-d),e.copy(i).addScaledVector(Zi,a);sa.subVectors(t,r);const f=Zi.dot(sa),v=Ji.dot(sa);if(v>=0&&f<=v)return e.copy(r);const g=f*c-l*v;if(g<=0&&c>=0&&v<=0)return o=c/(c-v),e.copy(i).addScaledVector(Ji,o);const _=d*v-f*h;if(_<=0&&h-d>=0&&f-v>=0)return jc.subVectors(r,s),o=(h-d)/(h-d+(f-v)),e.copy(s).addScaledVector(jc,o);const m=1/(_+g+p);return a=g*m,o=p*m,e.copy(i).addScaledVector(Zi,a).addScaledVector(Ji,o)}equals(t){return t.a.equals(this.a)&&t.b.equals(this.b)&&t.c.equals(this.c)}}const ih={aliceblue:15792383,antiquewhite:16444375,aqua:65535,aquamarine:8388564,azure:15794175,beige:16119260,bisque:16770244,black:0,blanchedalmond:16772045,blue:255,blueviolet:9055202,brown:10824234,burlywood:14596231,cadetblue:6266528,chartreuse:8388352,chocolate:13789470,coral:16744272,cornflowerblue:6591981,cornsilk:16775388,crimson:14423100,cyan:65535,darkblue:139,darkcyan:35723,darkgoldenrod:12092939,darkgray:11119017,darkgreen:25600,darkgrey:11119017,darkkhaki:12433259,darkmagenta:9109643,darkolivegreen:5597999,darkorange:16747520,darkorchid:10040012,darkred:9109504,darksalmon:15308410,darkseagreen:9419919,darkslateblue:4734347,darkslategray:3100495,darkslategrey:3100495,darkturquoise:52945,darkviolet:9699539,deeppink:16716947,deepskyblue:49151,dimgray:6908265,dimgrey:6908265,dodgerblue:2003199,firebrick:11674146,floralwhite:16775920,forestgreen:2263842,fuchsia:16711935,gainsboro:14474460,ghostwhite:16316671,gold:16766720,goldenrod:14329120,gray:8421504,green:32768,greenyellow:11403055,grey:8421504,honeydew:15794160,hotpink:16738740,indianred:13458524,indigo:4915330,ivory:16777200,khaki:15787660,lavender:15132410,lavenderblush:16773365,lawngreen:8190976,lemonchiffon:16775885,lightblue:11393254,lightcoral:15761536,lightcyan:14745599,lightgoldenrodyellow:16448210,lightgray:13882323,lightgreen:9498256,lightgrey:13882323,lightpink:16758465,lightsalmon:16752762,lightseagreen:2142890,lightskyblue:8900346,lightslategray:7833753,lightslategrey:7833753,lightsteelblue:11584734,lightyellow:16777184,lime:65280,limegreen:3329330,linen:16445670,magenta:16711935,maroon:8388608,mediumaquamarine:6737322,mediumblue:205,mediumorchid:12211667,mediumpurple:9662683,mediumseagreen:3978097,mediumslateblue:8087790,mediumspringgreen:64154,mediumturquoise:4772300,mediumvioletred:13047173,midnightblue:1644912,mintcream:16121850,mistyrose:16770273,moccasin:16770229,navajowhite:16768685,navy:128,oldlace:16643558,olive:8421376,olivedrab:7048739,orange:16753920,orangered:16729344,orchid:14315734,palegoldenrod:15657130,palegreen:10025880,paleturquoise:11529966,palevioletred:14381203,papayawhip:16773077,peachpuff:16767673,peru:13468991,pink:16761035,plum:14524637,powderblue:11591910,purple:8388736,rebeccapurple:6697881,red:16711680,rosybrown:12357519,royalblue:4286945,saddlebrown:9127187,salmon:16416882,sandybrown:16032864,seagreen:3050327,seashell:16774638,sienna:10506797,silver:12632256,skyblue:8900331,slateblue:6970061,slategray:7372944,slategrey:7372944,snow:16775930,springgreen:65407,steelblue:4620980,tan:13808780,teal:32896,thistle:14204888,tomato:16737095,turquoise:4251856,violet:15631086,wheat:16113331,white:16777215,whitesmoke:16119285,yellow:16776960,yellowgreen:10145074},di={h:0,s:0,l:0},wr={h:0,s:0,l:0};function ra(n,t,e){return e<0&&(e+=1),e>1&&(e-=1),e<1/6?n+(t-n)*6*e:e<1/2?t:e<2/3?n+(t-n)*6*(2/3-e):n}class ne{constructor(t,e,i){return this.isColor=!0,this.r=1,this.g=1,this.b=1,this.set(t,e,i)}set(t,e,i){if(e===void 0&&i===void 0){const s=t;s&&s.isColor?this.copy(s):typeof s=="number"?this.setHex(s):typeof s=="string"&&this.setStyle(s)}else this.setRGB(t,e,i);return this}setScalar(t){return this.r=t,this.g=t,this.b=t,this}setHex(t,e=Ie){return t=Math.floor(t),this.r=(t>>16&255)/255,this.g=(t>>8&255)/255,this.b=(t&255)/255,ce.toWorkingColorSpace(this,e),this}setRGB(t,e,i,s=ce.workingColorSpace){return this.r=t,this.g=e,this.b=i,ce.toWorkingColorSpace(this,s),this}setHSL(t,e,i,s=ce.workingColorSpace){if(t=l_(t,1),e=sn(e,0,1),i=sn(i,0,1),e===0)this.r=this.g=this.b=i;else{const r=i<=.5?i*(1+e):i+e-i*e,a=2*i-r;this.r=ra(a,r,t+1/3),this.g=ra(a,r,t),this.b=ra(a,r,t-1/3)}return ce.toWorkingColorSpace(this,s),this}setStyle(t,e=Ie){function i(r){r!==void 0&&parseFloat(r)<1&&console.warn("THREE.Color: Alpha component of "+t+" will be ignored.")}let s;if(s=/^(\w+)\(([^\)]*)\)/.exec(t)){let r;const a=s[1],o=s[2];switch(a){case"rgb":case"rgba":if(r=/^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(255,parseInt(r[1],10))/255,Math.min(255,parseInt(r[2],10))/255,Math.min(255,parseInt(r[3],10))/255,e);if(r=/^\s*(\d+)\%\s*,\s*(\d+)\%\s*,\s*(\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setRGB(Math.min(100,parseInt(r[1],10))/100,Math.min(100,parseInt(r[2],10))/100,Math.min(100,parseInt(r[3],10))/100,e);break;case"hsl":case"hsla":if(r=/^\s*(\d*\.?\d+)\s*,\s*(\d*\.?\d+)\%\s*,\s*(\d*\.?\d+)\%\s*(?:,\s*(\d*\.?\d+)\s*)?$/.exec(o))return i(r[4]),this.setHSL(parseFloat(r[1])/360,parseFloat(r[2])/100,parseFloat(r[3])/100,e);break;default:console.warn("THREE.Color: Unknown color model "+t)}}else if(s=/^\#([A-Fa-f\d]+)$/.exec(t)){const r=s[1],a=r.length;if(a===3)return this.setRGB(parseInt(r.charAt(0),16)/15,parseInt(r.charAt(1),16)/15,parseInt(r.charAt(2),16)/15,e);if(a===6)return this.setHex(parseInt(r,16),e);console.warn("THREE.Color: Invalid hex color "+t)}else if(t&&t.length>0)return this.setColorName(t,e);return this}setColorName(t,e=Ie){const i=ih[t.toLowerCase()];return i!==void 0?this.setHex(i,e):console.warn("THREE.Color: Unknown color "+t),this}clone(){return new this.constructor(this.r,this.g,this.b)}copy(t){return this.r=t.r,this.g=t.g,this.b=t.b,this}copySRGBToLinear(t){return this.r=ms(t.r),this.g=ms(t.g),this.b=ms(t.b),this}copyLinearToSRGB(t){return this.r=qo(t.r),this.g=qo(t.g),this.b=qo(t.b),this}convertSRGBToLinear(){return this.copySRGBToLinear(this),this}convertLinearToSRGB(){return this.copyLinearToSRGB(this),this}getHex(t=Ie){return ce.fromWorkingColorSpace(Be.copy(this),t),Math.round(sn(Be.r*255,0,255))*65536+Math.round(sn(Be.g*255,0,255))*256+Math.round(sn(Be.b*255,0,255))}getHexString(t=Ie){return("000000"+this.getHex(t).toString(16)).slice(-6)}getHSL(t,e=ce.workingColorSpace){ce.fromWorkingColorSpace(Be.copy(this),e);const i=Be.r,s=Be.g,r=Be.b,a=Math.max(i,s,r),o=Math.min(i,s,r);let l,c;const d=(o+a)/2;if(o===a)l=0,c=0;else{const h=a-o;switch(c=d<=.5?h/(a+o):h/(2-a-o),a){case i:l=(s-r)/h+(s<r?6:0);break;case s:l=(r-i)/h+2;break;case r:l=(i-s)/h+4;break}l/=6}return t.h=l,t.s=c,t.l=d,t}getRGB(t,e=ce.workingColorSpace){return ce.fromWorkingColorSpace(Be.copy(this),e),t.r=Be.r,t.g=Be.g,t.b=Be.b,t}getStyle(t=Ie){ce.fromWorkingColorSpace(Be.copy(this),t);const e=Be.r,i=Be.g,s=Be.b;return t!==Ie?`color(${t} ${e.toFixed(3)} ${i.toFixed(3)} ${s.toFixed(3)})`:`rgb(${Math.round(e*255)},${Math.round(i*255)},${Math.round(s*255)})`}offsetHSL(t,e,i){return this.getHSL(di),this.setHSL(di.h+t,di.s+e,di.l+i)}add(t){return this.r+=t.r,this.g+=t.g,this.b+=t.b,this}addColors(t,e){return this.r=t.r+e.r,this.g=t.g+e.g,this.b=t.b+e.b,this}addScalar(t){return this.r+=t,this.g+=t,this.b+=t,this}sub(t){return this.r=Math.max(0,this.r-t.r),this.g=Math.max(0,this.g-t.g),this.b=Math.max(0,this.b-t.b),this}multiply(t){return this.r*=t.r,this.g*=t.g,this.b*=t.b,this}multiplyScalar(t){return this.r*=t,this.g*=t,this.b*=t,this}lerp(t,e){return this.r+=(t.r-this.r)*e,this.g+=(t.g-this.g)*e,this.b+=(t.b-this.b)*e,this}lerpColors(t,e,i){return this.r=t.r+(e.r-t.r)*i,this.g=t.g+(e.g-t.g)*i,this.b=t.b+(e.b-t.b)*i,this}lerpHSL(t,e){this.getHSL(di),t.getHSL(wr);const i=$o(di.h,wr.h,e),s=$o(di.s,wr.s,e),r=$o(di.l,wr.l,e);return this.setHSL(i,s,r),this}setFromVector3(t){return this.r=t.x,this.g=t.y,this.b=t.z,this}applyMatrix3(t){const e=this.r,i=this.g,s=this.b,r=t.elements;return this.r=r[0]*e+r[3]*i+r[6]*s,this.g=r[1]*e+r[4]*i+r[7]*s,this.b=r[2]*e+r[5]*i+r[8]*s,this}equals(t){return t.r===this.r&&t.g===this.g&&t.b===this.b}fromArray(t,e=0){return this.r=t[e],this.g=t[e+1],this.b=t[e+2],this}toArray(t=[],e=0){return t[e]=this.r,t[e+1]=this.g,t[e+2]=this.b,t}fromBufferAttribute(t,e){return this.r=t.getX(e),this.g=t.getY(e),this.b=t.getZ(e),this}toJSON(){return this.getHex()}*[Symbol.iterator](){yield this.r,yield this.g,yield this.b}}const Be=new ne;ne.NAMES=ih;let S_=0;class ws extends Ts{constructor(){super(),this.isMaterial=!0,Object.defineProperty(this,"id",{value:S_++}),this.uuid=or(),this.name="",this.type="Material",this.blending=ps,this.side=bi,this.vertexColors=!1,this.opacity=1,this.transparent=!1,this.alphaHash=!1,this.blendSrc=La,this.blendDst=Pa,this.blendEquation=Ii,this.blendSrcAlpha=null,this.blendDstAlpha=null,this.blendEquationAlpha=null,this.blendColor=new ne(0,0,0),this.blendAlpha=0,this.depthFunc=eo,this.depthTest=!0,this.depthWrite=!0,this.stencilWriteMask=255,this.stencilFunc=Ic,this.stencilRef=0,this.stencilFuncMask=255,this.stencilFail=Wi,this.stencilZFail=Wi,this.stencilZPass=Wi,this.stencilWrite=!1,this.clippingPlanes=null,this.clipIntersection=!1,this.clipShadows=!1,this.shadowSide=null,this.colorWrite=!0,this.precision=null,this.polygonOffset=!1,this.polygonOffsetFactor=0,this.polygonOffsetUnits=0,this.dithering=!1,this.alphaToCoverage=!1,this.premultipliedAlpha=!1,this.forceSinglePass=!1,this.visible=!0,this.toneMapped=!0,this.userData={},this.version=0,this._alphaTest=0}get alphaTest(){return this._alphaTest}set alphaTest(t){this._alphaTest>0!=t>0&&this.version++,this._alphaTest=t}onBuild(){}onBeforeRender(){}onBeforeCompile(){}customProgramCacheKey(){return this.onBeforeCompile.toString()}setValues(t){if(t!==void 0)for(const e in t){const i=t[e];if(i===void 0){console.warn(`THREE.Material: parameter '${e}' has value of undefined.`);continue}const s=this[e];if(s===void 0){console.warn(`THREE.Material: '${e}' is not a property of THREE.${this.type}.`);continue}s&&s.isColor?s.set(i):s&&s.isVector3&&i&&i.isVector3?s.copy(i):this[e]=i}}toJSON(t){const e=t===void 0||typeof t=="string";e&&(t={textures:{},images:{}});const i={metadata:{version:4.6,type:"Material",generator:"Material.toJSON"}};i.uuid=this.uuid,i.type=this.type,this.name!==""&&(i.name=this.name),this.color&&this.color.isColor&&(i.color=this.color.getHex()),this.roughness!==void 0&&(i.roughness=this.roughness),this.metalness!==void 0&&(i.metalness=this.metalness),this.sheen!==void 0&&(i.sheen=this.sheen),this.sheenColor&&this.sheenColor.isColor&&(i.sheenColor=this.sheenColor.getHex()),this.sheenRoughness!==void 0&&(i.sheenRoughness=this.sheenRoughness),this.emissive&&this.emissive.isColor&&(i.emissive=this.emissive.getHex()),this.emissiveIntensity&&this.emissiveIntensity!==1&&(i.emissiveIntensity=this.emissiveIntensity),this.specular&&this.specular.isColor&&(i.specular=this.specular.getHex()),this.specularIntensity!==void 0&&(i.specularIntensity=this.specularIntensity),this.specularColor&&this.specularColor.isColor&&(i.specularColor=this.specularColor.getHex()),this.shininess!==void 0&&(i.shininess=this.shininess),this.clearcoat!==void 0&&(i.clearcoat=this.clearcoat),this.clearcoatRoughness!==void 0&&(i.clearcoatRoughness=this.clearcoatRoughness),this.clearcoatMap&&this.clearcoatMap.isTexture&&(i.clearcoatMap=this.clearcoatMap.toJSON(t).uuid),this.clearcoatRoughnessMap&&this.clearcoatRoughnessMap.isTexture&&(i.clearcoatRoughnessMap=this.clearcoatRoughnessMap.toJSON(t).uuid),this.clearcoatNormalMap&&this.clearcoatNormalMap.isTexture&&(i.clearcoatNormalMap=this.clearcoatNormalMap.toJSON(t).uuid,i.clearcoatNormalScale=this.clearcoatNormalScale.toArray()),this.iridescence!==void 0&&(i.iridescence=this.iridescence),this.iridescenceIOR!==void 0&&(i.iridescenceIOR=this.iridescenceIOR),this.iridescenceThicknessRange!==void 0&&(i.iridescenceThicknessRange=this.iridescenceThicknessRange),this.iridescenceMap&&this.iridescenceMap.isTexture&&(i.iridescenceMap=this.iridescenceMap.toJSON(t).uuid),this.iridescenceThicknessMap&&this.iridescenceThicknessMap.isTexture&&(i.iridescenceThicknessMap=this.iridescenceThicknessMap.toJSON(t).uuid),this.anisotropy!==void 0&&(i.anisotropy=this.anisotropy),this.anisotropyRotation!==void 0&&(i.anisotropyRotation=this.anisotropyRotation),this.anisotropyMap&&this.anisotropyMap.isTexture&&(i.anisotropyMap=this.anisotropyMap.toJSON(t).uuid),this.map&&this.map.isTexture&&(i.map=this.map.toJSON(t).uuid),this.matcap&&this.matcap.isTexture&&(i.matcap=this.matcap.toJSON(t).uuid),this.alphaMap&&this.alphaMap.isTexture&&(i.alphaMap=this.alphaMap.toJSON(t).uuid),this.lightMap&&this.lightMap.isTexture&&(i.lightMap=this.lightMap.toJSON(t).uuid,i.lightMapIntensity=this.lightMapIntensity),this.aoMap&&this.aoMap.isTexture&&(i.aoMap=this.aoMap.toJSON(t).uuid,i.aoMapIntensity=this.aoMapIntensity),this.bumpMap&&this.bumpMap.isTexture&&(i.bumpMap=this.bumpMap.toJSON(t).uuid,i.bumpScale=this.bumpScale),this.normalMap&&this.normalMap.isTexture&&(i.normalMap=this.normalMap.toJSON(t).uuid,i.normalMapType=this.normalMapType,i.normalScale=this.normalScale.toArray()),this.displacementMap&&this.displacementMap.isTexture&&(i.displacementMap=this.displacementMap.toJSON(t).uuid,i.displacementScale=this.displacementScale,i.displacementBias=this.displacementBias),this.roughnessMap&&this.roughnessMap.isTexture&&(i.roughnessMap=this.roughnessMap.toJSON(t).uuid),this.metalnessMap&&this.metalnessMap.isTexture&&(i.metalnessMap=this.metalnessMap.toJSON(t).uuid),this.emissiveMap&&this.emissiveMap.isTexture&&(i.emissiveMap=this.emissiveMap.toJSON(t).uuid),this.specularMap&&this.specularMap.isTexture&&(i.specularMap=this.specularMap.toJSON(t).uuid),this.specularIntensityMap&&this.specularIntensityMap.isTexture&&(i.specularIntensityMap=this.specularIntensityMap.toJSON(t).uuid),this.specularColorMap&&this.specularColorMap.isTexture&&(i.specularColorMap=this.specularColorMap.toJSON(t).uuid),this.envMap&&this.envMap.isTexture&&(i.envMap=this.envMap.toJSON(t).uuid,this.combine!==void 0&&(i.combine=this.combine)),this.envMapIntensity!==void 0&&(i.envMapIntensity=this.envMapIntensity),this.reflectivity!==void 0&&(i.reflectivity=this.reflectivity),this.refractionRatio!==void 0&&(i.refractionRatio=this.refractionRatio),this.gradientMap&&this.gradientMap.isTexture&&(i.gradientMap=this.gradientMap.toJSON(t).uuid),this.transmission!==void 0&&(i.transmission=this.transmission),this.transmissionMap&&this.transmissionMap.isTexture&&(i.transmissionMap=this.transmissionMap.toJSON(t).uuid),this.thickness!==void 0&&(i.thickness=this.thickness),this.thicknessMap&&this.thicknessMap.isTexture&&(i.thicknessMap=this.thicknessMap.toJSON(t).uuid),this.attenuationDistance!==void 0&&this.attenuationDistance!==1/0&&(i.attenuationDistance=this.attenuationDistance),this.attenuationColor!==void 0&&(i.attenuationColor=this.attenuationColor.getHex()),this.size!==void 0&&(i.size=this.size),this.shadowSide!==null&&(i.shadowSide=this.shadowSide),this.sizeAttenuation!==void 0&&(i.sizeAttenuation=this.sizeAttenuation),this.blending!==ps&&(i.blending=this.blending),this.side!==bi&&(i.side=this.side),this.vertexColors===!0&&(i.vertexColors=!0),this.opacity<1&&(i.opacity=this.opacity),this.transparent===!0&&(i.transparent=!0),this.blendSrc!==La&&(i.blendSrc=this.blendSrc),this.blendDst!==Pa&&(i.blendDst=this.blendDst),this.blendEquation!==Ii&&(i.blendEquation=this.blendEquation),this.blendSrcAlpha!==null&&(i.blendSrcAlpha=this.blendSrcAlpha),this.blendDstAlpha!==null&&(i.blendDstAlpha=this.blendDstAlpha),this.blendEquationAlpha!==null&&(i.blendEquationAlpha=this.blendEquationAlpha),this.blendColor&&this.blendColor.isColor&&(i.blendColor=this.blendColor.getHex()),this.blendAlpha!==0&&(i.blendAlpha=this.blendAlpha),this.depthFunc!==eo&&(i.depthFunc=this.depthFunc),this.depthTest===!1&&(i.depthTest=this.depthTest),this.depthWrite===!1&&(i.depthWrite=this.depthWrite),this.colorWrite===!1&&(i.colorWrite=this.colorWrite),this.stencilWriteMask!==255&&(i.stencilWriteMask=this.stencilWriteMask),this.stencilFunc!==Ic&&(i.stencilFunc=this.stencilFunc),this.stencilRef!==0&&(i.stencilRef=this.stencilRef),this.stencilFuncMask!==255&&(i.stencilFuncMask=this.stencilFuncMask),this.stencilFail!==Wi&&(i.stencilFail=this.stencilFail),this.stencilZFail!==Wi&&(i.stencilZFail=this.stencilZFail),this.stencilZPass!==Wi&&(i.stencilZPass=this.stencilZPass),this.stencilWrite===!0&&(i.stencilWrite=this.stencilWrite),this.rotation!==void 0&&this.rotation!==0&&(i.rotation=this.rotation),this.polygonOffset===!0&&(i.polygonOffset=!0),this.polygonOffsetFactor!==0&&(i.polygonOffsetFactor=this.polygonOffsetFactor),this.polygonOffsetUnits!==0&&(i.polygonOffsetUnits=this.polygonOffsetUnits),this.linewidth!==void 0&&this.linewidth!==1&&(i.linewidth=this.linewidth),this.dashSize!==void 0&&(i.dashSize=this.dashSize),this.gapSize!==void 0&&(i.gapSize=this.gapSize),this.scale!==void 0&&(i.scale=this.scale),this.dithering===!0&&(i.dithering=!0),this.alphaTest>0&&(i.alphaTest=this.alphaTest),this.alphaHash===!0&&(i.alphaHash=!0),this.alphaToCoverage===!0&&(i.alphaToCoverage=!0),this.premultipliedAlpha===!0&&(i.premultipliedAlpha=!0),this.forceSinglePass===!0&&(i.forceSinglePass=!0),this.wireframe===!0&&(i.wireframe=!0),this.wireframeLinewidth>1&&(i.wireframeLinewidth=this.wireframeLinewidth),this.wireframeLinecap!=="round"&&(i.wireframeLinecap=this.wireframeLinecap),this.wireframeLinejoin!=="round"&&(i.wireframeLinejoin=this.wireframeLinejoin),this.flatShading===!0&&(i.flatShading=!0),this.visible===!1&&(i.visible=!1),this.toneMapped===!1&&(i.toneMapped=!1),this.fog===!1&&(i.fog=!1),Object.keys(this.userData).length>0&&(i.userData=this.userData);function s(r){const a=[];for(const o in r){const l=r[o];delete l.metadata,a.push(l)}return a}if(e){const r=s(t.textures),a=s(t.images);r.length>0&&(i.textures=r),a.length>0&&(i.images=a)}return i}clone(){return new this.constructor().copy(this)}copy(t){this.name=t.name,this.blending=t.blending,this.side=t.side,this.vertexColors=t.vertexColors,this.opacity=t.opacity,this.transparent=t.transparent,this.blendSrc=t.blendSrc,this.blendDst=t.blendDst,this.blendEquation=t.blendEquation,this.blendSrcAlpha=t.blendSrcAlpha,this.blendDstAlpha=t.blendDstAlpha,this.blendEquationAlpha=t.blendEquationAlpha,this.blendColor.copy(t.blendColor),this.blendAlpha=t.blendAlpha,this.depthFunc=t.depthFunc,this.depthTest=t.depthTest,this.depthWrite=t.depthWrite,this.stencilWriteMask=t.stencilWriteMask,this.stencilFunc=t.stencilFunc,this.stencilRef=t.stencilRef,this.stencilFuncMask=t.stencilFuncMask,this.stencilFail=t.stencilFail,this.stencilZFail=t.stencilZFail,this.stencilZPass=t.stencilZPass,this.stencilWrite=t.stencilWrite;const e=t.clippingPlanes;let i=null;if(e!==null){const s=e.length;i=new Array(s);for(let r=0;r!==s;++r)i[r]=e[r].clone()}return this.clippingPlanes=i,this.clipIntersection=t.clipIntersection,this.clipShadows=t.clipShadows,this.shadowSide=t.shadowSide,this.colorWrite=t.colorWrite,this.precision=t.precision,this.polygonOffset=t.polygonOffset,this.polygonOffsetFactor=t.polygonOffsetFactor,this.polygonOffsetUnits=t.polygonOffsetUnits,this.dithering=t.dithering,this.alphaTest=t.alphaTest,this.alphaHash=t.alphaHash,this.alphaToCoverage=t.alphaToCoverage,this.premultipliedAlpha=t.premultipliedAlpha,this.forceSinglePass=t.forceSinglePass,this.visible=t.visible,this.toneMapped=t.toneMapped,this.userData=JSON.parse(JSON.stringify(t.userData)),this}dispose(){this.dispatchEvent({type:"dispose"})}set needsUpdate(t){t===!0&&this.version++}}class as extends ws{constructor(t){super(),this.isMeshBasicMaterial=!0,this.type="MeshBasicMaterial",this.color=new ne(16777215),this.map=null,this.lightMap=null,this.lightMapIntensity=1,this.aoMap=null,this.aoMapIntensity=1,this.specularMap=null,this.alphaMap=null,this.envMap=null,this.combine=Hd,this.reflectivity=1,this.refractionRatio=.98,this.wireframe=!1,this.wireframeLinewidth=1,this.wireframeLinecap="round",this.wireframeLinejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.lightMap=t.lightMap,this.lightMapIntensity=t.lightMapIntensity,this.aoMap=t.aoMap,this.aoMapIntensity=t.aoMapIntensity,this.specularMap=t.specularMap,this.alphaMap=t.alphaMap,this.envMap=t.envMap,this.combine=t.combine,this.reflectivity=t.reflectivity,this.refractionRatio=t.refractionRatio,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.wireframeLinecap=t.wireframeLinecap,this.wireframeLinejoin=t.wireframeLinejoin,this.fog=t.fog,this}}const Ae=new z,Ar=new re;class Ne{constructor(t,e,i=!1){if(Array.isArray(t))throw new TypeError("THREE.BufferAttribute: array should be a Typed Array.");this.isBufferAttribute=!0,this.name="",this.array=t,this.itemSize=e,this.count=t!==void 0?t.length/e:0,this.normalized=i,this.usage=Nc,this._updateRange={offset:0,count:-1},this.updateRanges=[],this.gpuType=pi,this.version=0}onUploadCallback(){}set needsUpdate(t){t===!0&&this.version++}get updateRange(){return console.warn("THREE.BufferAttribute: updateRange() is deprecated and will be removed in r169. Use addUpdateRange() instead."),this._updateRange}setUsage(t){return this.usage=t,this}addUpdateRange(t,e){this.updateRanges.push({start:t,count:e})}clearUpdateRanges(){this.updateRanges.length=0}copy(t){return this.name=t.name,this.array=new t.array.constructor(t.array),this.itemSize=t.itemSize,this.count=t.count,this.normalized=t.normalized,this.usage=t.usage,this.gpuType=t.gpuType,this}copyAt(t,e,i){t*=this.itemSize,i*=e.itemSize;for(let s=0,r=this.itemSize;s<r;s++)this.array[t+s]=e.array[i+s];return this}copyArray(t){return this.array.set(t),this}applyMatrix3(t){if(this.itemSize===2)for(let e=0,i=this.count;e<i;e++)Ar.fromBufferAttribute(this,e),Ar.applyMatrix3(t),this.setXY(e,Ar.x,Ar.y);else if(this.itemSize===3)for(let e=0,i=this.count;e<i;e++)Ae.fromBufferAttribute(this,e),Ae.applyMatrix3(t),this.setXYZ(e,Ae.x,Ae.y,Ae.z);return this}applyMatrix4(t){for(let e=0,i=this.count;e<i;e++)Ae.fromBufferAttribute(this,e),Ae.applyMatrix4(t),this.setXYZ(e,Ae.x,Ae.y,Ae.z);return this}applyNormalMatrix(t){for(let e=0,i=this.count;e<i;e++)Ae.fromBufferAttribute(this,e),Ae.applyNormalMatrix(t),this.setXYZ(e,Ae.x,Ae.y,Ae.z);return this}transformDirection(t){for(let e=0,i=this.count;e<i;e++)Ae.fromBufferAttribute(this,e),Ae.transformDirection(t),this.setXYZ(e,Ae.x,Ae.y,Ae.z);return this}set(t,e=0){return this.array.set(t,e),this}getComponent(t,e){let i=this.array[t*this.itemSize+e];return this.normalized&&(i=Ps(i,this.array)),i}setComponent(t,e,i){return this.normalized&&(i=en(i,this.array)),this.array[t*this.itemSize+e]=i,this}getX(t){let e=this.array[t*this.itemSize];return this.normalized&&(e=Ps(e,this.array)),e}setX(t,e){return this.normalized&&(e=en(e,this.array)),this.array[t*this.itemSize]=e,this}getY(t){let e=this.array[t*this.itemSize+1];return this.normalized&&(e=Ps(e,this.array)),e}setY(t,e){return this.normalized&&(e=en(e,this.array)),this.array[t*this.itemSize+1]=e,this}getZ(t){let e=this.array[t*this.itemSize+2];return this.normalized&&(e=Ps(e,this.array)),e}setZ(t,e){return this.normalized&&(e=en(e,this.array)),this.array[t*this.itemSize+2]=e,this}getW(t){let e=this.array[t*this.itemSize+3];return this.normalized&&(e=Ps(e,this.array)),e}setW(t,e){return this.normalized&&(e=en(e,this.array)),this.array[t*this.itemSize+3]=e,this}setXY(t,e,i){return t*=this.itemSize,this.normalized&&(e=en(e,this.array),i=en(i,this.array)),this.array[t+0]=e,this.array[t+1]=i,this}setXYZ(t,e,i,s){return t*=this.itemSize,this.normalized&&(e=en(e,this.array),i=en(i,this.array),s=en(s,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=s,this}setXYZW(t,e,i,s,r){return t*=this.itemSize,this.normalized&&(e=en(e,this.array),i=en(i,this.array),s=en(s,this.array),r=en(r,this.array)),this.array[t+0]=e,this.array[t+1]=i,this.array[t+2]=s,this.array[t+3]=r,this}onUpload(t){return this.onUploadCallback=t,this}clone(){return new this.constructor(this.array,this.itemSize).copy(this)}toJSON(){const t={itemSize:this.itemSize,type:this.array.constructor.name,array:Array.from(this.array),normalized:this.normalized};return this.name!==""&&(t.name=this.name),this.usage!==Nc&&(t.usage=this.usage),t}}class sh extends Ne{constructor(t,e,i){super(new Uint16Array(t),e,i)}}class rh extends Ne{constructor(t,e,i){super(new Uint32Array(t),e,i)}}class ke extends Ne{constructor(t,e,i){super(new Float32Array(t),e,i)}}let E_=0;const fn=new Te,oa=new We,Qi=new z,an=new lr,Ns=new lr,De=new z;class Ve extends Ts{constructor(){super(),this.isBufferGeometry=!0,Object.defineProperty(this,"id",{value:E_++}),this.uuid=or(),this.name="",this.type="BufferGeometry",this.index=null,this.attributes={},this.morphAttributes={},this.morphTargetsRelative=!1,this.groups=[],this.boundingBox=null,this.boundingSphere=null,this.drawRange={start:0,count:1/0},this.userData={}}getIndex(){return this.index}setIndex(t){return Array.isArray(t)?this.index=new(Jd(t)?rh:sh)(t,1):this.index=t,this}getAttribute(t){return this.attributes[t]}setAttribute(t,e){return this.attributes[t]=e,this}deleteAttribute(t){return delete this.attributes[t],this}hasAttribute(t){return this.attributes[t]!==void 0}addGroup(t,e,i=0){this.groups.push({start:t,count:e,materialIndex:i})}clearGroups(){this.groups=[]}setDrawRange(t,e){this.drawRange.start=t,this.drawRange.count=e}applyMatrix4(t){const e=this.attributes.position;e!==void 0&&(e.applyMatrix4(t),e.needsUpdate=!0);const i=this.attributes.normal;if(i!==void 0){const r=new te().getNormalMatrix(t);i.applyNormalMatrix(r),i.needsUpdate=!0}const s=this.attributes.tangent;return s!==void 0&&(s.transformDirection(t),s.needsUpdate=!0),this.boundingBox!==null&&this.computeBoundingBox(),this.boundingSphere!==null&&this.computeBoundingSphere(),this}applyQuaternion(t){return fn.makeRotationFromQuaternion(t),this.applyMatrix4(fn),this}rotateX(t){return fn.makeRotationX(t),this.applyMatrix4(fn),this}rotateY(t){return fn.makeRotationY(t),this.applyMatrix4(fn),this}rotateZ(t){return fn.makeRotationZ(t),this.applyMatrix4(fn),this}translate(t,e,i){return fn.makeTranslation(t,e,i),this.applyMatrix4(fn),this}scale(t,e,i){return fn.makeScale(t,e,i),this.applyMatrix4(fn),this}lookAt(t){return oa.lookAt(t),oa.updateMatrix(),this.applyMatrix4(oa.matrix),this}center(){return this.computeBoundingBox(),this.boundingBox.getCenter(Qi).negate(),this.translate(Qi.x,Qi.y,Qi.z),this}setFromPoints(t){const e=[];for(let i=0,s=t.length;i<s;i++){const r=t[i];e.push(r.x,r.y,r.z||0)}return this.setAttribute("position",new ke(e,3)),this}computeBoundingBox(){this.boundingBox===null&&(this.boundingBox=new lr);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error('THREE.BufferGeometry.computeBoundingBox(): GLBufferAttribute requires a manual bounding box. Alternatively set "mesh.frustumCulled" to "false".',this),this.boundingBox.set(new z(-1/0,-1/0,-1/0),new z(1/0,1/0,1/0));return}if(t!==void 0){if(this.boundingBox.setFromBufferAttribute(t),e)for(let i=0,s=e.length;i<s;i++){const r=e[i];an.setFromBufferAttribute(r),this.morphTargetsRelative?(De.addVectors(this.boundingBox.min,an.min),this.boundingBox.expandByPoint(De),De.addVectors(this.boundingBox.max,an.max),this.boundingBox.expandByPoint(De)):(this.boundingBox.expandByPoint(an.min),this.boundingBox.expandByPoint(an.max))}}else this.boundingBox.makeEmpty();(isNaN(this.boundingBox.min.x)||isNaN(this.boundingBox.min.y)||isNaN(this.boundingBox.min.z))&&console.error('THREE.BufferGeometry.computeBoundingBox(): Computed min/max have NaN values. The "position" attribute is likely to have NaN values.',this)}computeBoundingSphere(){this.boundingSphere===null&&(this.boundingSphere=new cr);const t=this.attributes.position,e=this.morphAttributes.position;if(t&&t.isGLBufferAttribute){console.error('THREE.BufferGeometry.computeBoundingSphere(): GLBufferAttribute requires a manual bounding sphere. Alternatively set "mesh.frustumCulled" to "false".',this),this.boundingSphere.set(new z,1/0);return}if(t){const i=this.boundingSphere.center;if(an.setFromBufferAttribute(t),e)for(let r=0,a=e.length;r<a;r++){const o=e[r];Ns.setFromBufferAttribute(o),this.morphTargetsRelative?(De.addVectors(an.min,Ns.min),an.expandByPoint(De),De.addVectors(an.max,Ns.max),an.expandByPoint(De)):(an.expandByPoint(Ns.min),an.expandByPoint(Ns.max))}an.getCenter(i);let s=0;for(let r=0,a=t.count;r<a;r++)De.fromBufferAttribute(t,r),s=Math.max(s,i.distanceToSquared(De));if(e)for(let r=0,a=e.length;r<a;r++){const o=e[r],l=this.morphTargetsRelative;for(let c=0,d=o.count;c<d;c++)De.fromBufferAttribute(o,c),l&&(Qi.fromBufferAttribute(t,c),De.add(Qi)),s=Math.max(s,i.distanceToSquared(De))}this.boundingSphere.radius=Math.sqrt(s),isNaN(this.boundingSphere.radius)&&console.error('THREE.BufferGeometry.computeBoundingSphere(): Computed radius is NaN. The "position" attribute is likely to have NaN values.',this)}}computeTangents(){const t=this.index,e=this.attributes;if(t===null||e.position===void 0||e.normal===void 0||e.uv===void 0){console.error("THREE.BufferGeometry: .computeTangents() failed. Missing required attributes (index, position, normal or uv)");return}const i=t.array,s=e.position.array,r=e.normal.array,a=e.uv.array,o=s.length/3;this.hasAttribute("tangent")===!1&&this.setAttribute("tangent",new Ne(new Float32Array(4*o),4));const l=this.getAttribute("tangent").array,c=[],d=[];for(let R=0;R<o;R++)c[R]=new z,d[R]=new z;const h=new z,p=new z,f=new z,v=new re,g=new re,_=new re,m=new z,E=new z;function b(R,J,rt){h.fromArray(s,R*3),p.fromArray(s,J*3),f.fromArray(s,rt*3),v.fromArray(a,R*2),g.fromArray(a,J*2),_.fromArray(a,rt*2),p.sub(h),f.sub(h),g.sub(v),_.sub(v);const mt=1/(g.x*_.y-_.x*g.y);isFinite(mt)&&(m.copy(p).multiplyScalar(_.y).addScaledVector(f,-g.y).multiplyScalar(mt),E.copy(f).multiplyScalar(g.x).addScaledVector(p,-_.x).multiplyScalar(mt),c[R].add(m),c[J].add(m),c[rt].add(m),d[R].add(E),d[J].add(E),d[rt].add(E))}let x=this.groups;x.length===0&&(x=[{start:0,count:i.length}]);for(let R=0,J=x.length;R<J;++R){const rt=x[R],mt=rt.start,O=rt.count;for(let Z=mt,tt=mt+O;Z<tt;Z+=3)b(i[Z+0],i[Z+1],i[Z+2])}const C=new z,A=new z,D=new z,G=new z;function T(R){D.fromArray(r,R*3),G.copy(D);const J=c[R];C.copy(J),C.sub(D.multiplyScalar(D.dot(J))).normalize(),A.crossVectors(G,J);const mt=A.dot(d[R])<0?-1:1;l[R*4]=C.x,l[R*4+1]=C.y,l[R*4+2]=C.z,l[R*4+3]=mt}for(let R=0,J=x.length;R<J;++R){const rt=x[R],mt=rt.start,O=rt.count;for(let Z=mt,tt=mt+O;Z<tt;Z+=3)T(i[Z+0]),T(i[Z+1]),T(i[Z+2])}}computeVertexNormals(){const t=this.index,e=this.getAttribute("position");if(e!==void 0){let i=this.getAttribute("normal");if(i===void 0)i=new Ne(new Float32Array(e.count*3),3),this.setAttribute("normal",i);else for(let p=0,f=i.count;p<f;p++)i.setXYZ(p,0,0,0);const s=new z,r=new z,a=new z,o=new z,l=new z,c=new z,d=new z,h=new z;if(t)for(let p=0,f=t.count;p<f;p+=3){const v=t.getX(p+0),g=t.getX(p+1),_=t.getX(p+2);s.fromBufferAttribute(e,v),r.fromBufferAttribute(e,g),a.fromBufferAttribute(e,_),d.subVectors(a,r),h.subVectors(s,r),d.cross(h),o.fromBufferAttribute(i,v),l.fromBufferAttribute(i,g),c.fromBufferAttribute(i,_),o.add(d),l.add(d),c.add(d),i.setXYZ(v,o.x,o.y,o.z),i.setXYZ(g,l.x,l.y,l.z),i.setXYZ(_,c.x,c.y,c.z)}else for(let p=0,f=e.count;p<f;p+=3)s.fromBufferAttribute(e,p+0),r.fromBufferAttribute(e,p+1),a.fromBufferAttribute(e,p+2),d.subVectors(a,r),h.subVectors(s,r),d.cross(h),i.setXYZ(p+0,d.x,d.y,d.z),i.setXYZ(p+1,d.x,d.y,d.z),i.setXYZ(p+2,d.x,d.y,d.z);this.normalizeNormals(),i.needsUpdate=!0}}normalizeNormals(){const t=this.attributes.normal;for(let e=0,i=t.count;e<i;e++)De.fromBufferAttribute(t,e),De.normalize(),t.setXYZ(e,De.x,De.y,De.z)}toNonIndexed(){function t(o,l){const c=o.array,d=o.itemSize,h=o.normalized,p=new c.constructor(l.length*d);let f=0,v=0;for(let g=0,_=l.length;g<_;g++){o.isInterleavedBufferAttribute?f=l[g]*o.data.stride+o.offset:f=l[g]*d;for(let m=0;m<d;m++)p[v++]=c[f++]}return new Ne(p,d,h)}if(this.index===null)return console.warn("THREE.BufferGeometry.toNonIndexed(): BufferGeometry is already non-indexed."),this;const e=new Ve,i=this.index.array,s=this.attributes;for(const o in s){const l=s[o],c=t(l,i);e.setAttribute(o,c)}const r=this.morphAttributes;for(const o in r){const l=[],c=r[o];for(let d=0,h=c.length;d<h;d++){const p=c[d],f=t(p,i);l.push(f)}e.morphAttributes[o]=l}e.morphTargetsRelative=this.morphTargetsRelative;const a=this.groups;for(let o=0,l=a.length;o<l;o++){const c=a[o];e.addGroup(c.start,c.count,c.materialIndex)}return e}toJSON(){const t={metadata:{version:4.6,type:"BufferGeometry",generator:"BufferGeometry.toJSON"}};if(t.uuid=this.uuid,t.type=this.type,this.name!==""&&(t.name=this.name),Object.keys(this.userData).length>0&&(t.userData=this.userData),this.parameters!==void 0){const l=this.parameters;for(const c in l)l[c]!==void 0&&(t[c]=l[c]);return t}t.data={attributes:{}};const e=this.index;e!==null&&(t.data.index={type:e.array.constructor.name,array:Array.prototype.slice.call(e.array)});const i=this.attributes;for(const l in i){const c=i[l];t.data.attributes[l]=c.toJSON(t.data)}const s={};let r=!1;for(const l in this.morphAttributes){const c=this.morphAttributes[l],d=[];for(let h=0,p=c.length;h<p;h++){const f=c[h];d.push(f.toJSON(t.data))}d.length>0&&(s[l]=d,r=!0)}r&&(t.data.morphAttributes=s,t.data.morphTargetsRelative=this.morphTargetsRelative);const a=this.groups;a.length>0&&(t.data.groups=JSON.parse(JSON.stringify(a)));const o=this.boundingSphere;return o!==null&&(t.data.boundingSphere={center:o.center.toArray(),radius:o.radius}),t}clone(){return new this.constructor().copy(this)}copy(t){this.index=null,this.attributes={},this.morphAttributes={},this.groups=[],this.boundingBox=null,this.boundingSphere=null;const e={};this.name=t.name;const i=t.index;i!==null&&this.setIndex(i.clone(e));const s=t.attributes;for(const c in s){const d=s[c];this.setAttribute(c,d.clone(e))}const r=t.morphAttributes;for(const c in r){const d=[],h=r[c];for(let p=0,f=h.length;p<f;p++)d.push(h[p].clone(e));this.morphAttributes[c]=d}this.morphTargetsRelative=t.morphTargetsRelative;const a=t.groups;for(let c=0,d=a.length;c<d;c++){const h=a[c];this.addGroup(h.start,h.count,h.materialIndex)}const o=t.boundingBox;o!==null&&(this.boundingBox=o.clone());const l=t.boundingSphere;return l!==null&&(this.boundingSphere=l.clone()),this.drawRange.start=t.drawRange.start,this.drawRange.count=t.drawRange.count,this.userData=t.userData,this}dispose(){this.dispatchEvent({type:"dispose"})}}const Yc=new Te,Ri=new al,Cr=new cr,Kc=new z,ts=new z,es=new z,ns=new z,aa=new z,Rr=new z,Lr=new re,Pr=new re,Dr=new re,Zc=new z,Jc=new z,Qc=new z,Ur=new z,Ir=new z;class _n extends We{constructor(t=new Ve,e=new as){super(),this.isMesh=!0,this.type="Mesh",this.geometry=t,this.material=e,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),t.morphTargetInfluences!==void 0&&(this.morphTargetInfluences=t.morphTargetInfluences.slice()),t.morphTargetDictionary!==void 0&&(this.morphTargetDictionary=Object.assign({},t.morphTargetDictionary)),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}getVertexPosition(t,e){const i=this.geometry,s=i.attributes.position,r=i.morphAttributes.position,a=i.morphTargetsRelative;e.fromBufferAttribute(s,t);const o=this.morphTargetInfluences;if(r&&o){Rr.set(0,0,0);for(let l=0,c=r.length;l<c;l++){const d=o[l],h=r[l];d!==0&&(aa.fromBufferAttribute(h,t),a?Rr.addScaledVector(aa,d):Rr.addScaledVector(aa.sub(e),d))}e.add(Rr)}return e}raycast(t,e){const i=this.geometry,s=this.material,r=this.matrixWorld;s!==void 0&&(i.boundingSphere===null&&i.computeBoundingSphere(),Cr.copy(i.boundingSphere),Cr.applyMatrix4(r),Ri.copy(t.ray).recast(t.near),!(Cr.containsPoint(Ri.origin)===!1&&(Ri.intersectSphere(Cr,Kc)===null||Ri.origin.distanceToSquared(Kc)>(t.far-t.near)**2))&&(Yc.copy(r).invert(),Ri.copy(t.ray).applyMatrix4(Yc),!(i.boundingBox!==null&&Ri.intersectsBox(i.boundingBox)===!1)&&this._computeIntersections(t,e,Ri)))}_computeIntersections(t,e,i){let s;const r=this.geometry,a=this.material,o=r.index,l=r.attributes.position,c=r.attributes.uv,d=r.attributes.uv1,h=r.attributes.normal,p=r.groups,f=r.drawRange;if(o!==null)if(Array.isArray(a))for(let v=0,g=p.length;v<g;v++){const _=p[v],m=a[_.materialIndex],E=Math.max(_.start,f.start),b=Math.min(o.count,Math.min(_.start+_.count,f.start+f.count));for(let x=E,C=b;x<C;x+=3){const A=o.getX(x),D=o.getX(x+1),G=o.getX(x+2);s=Nr(this,m,t,i,c,d,h,A,D,G),s&&(s.faceIndex=Math.floor(x/3),s.face.materialIndex=_.materialIndex,e.push(s))}}else{const v=Math.max(0,f.start),g=Math.min(o.count,f.start+f.count);for(let _=v,m=g;_<m;_+=3){const E=o.getX(_),b=o.getX(_+1),x=o.getX(_+2);s=Nr(this,a,t,i,c,d,h,E,b,x),s&&(s.faceIndex=Math.floor(_/3),e.push(s))}}else if(l!==void 0)if(Array.isArray(a))for(let v=0,g=p.length;v<g;v++){const _=p[v],m=a[_.materialIndex],E=Math.max(_.start,f.start),b=Math.min(l.count,Math.min(_.start+_.count,f.start+f.count));for(let x=E,C=b;x<C;x+=3){const A=x,D=x+1,G=x+2;s=Nr(this,m,t,i,c,d,h,A,D,G),s&&(s.faceIndex=Math.floor(x/3),s.face.materialIndex=_.materialIndex,e.push(s))}}else{const v=Math.max(0,f.start),g=Math.min(l.count,f.start+f.count);for(let _=v,m=g;_<m;_+=3){const E=_,b=_+1,x=_+2;s=Nr(this,a,t,i,c,d,h,E,b,x),s&&(s.faceIndex=Math.floor(_/3),e.push(s))}}}}function T_(n,t,e,i,s,r,a,o){let l;if(t.side===tn?l=i.intersectTriangle(a,r,s,!0,o):l=i.intersectTriangle(s,r,a,t.side===bi,o),l===null)return null;Ir.copy(o),Ir.applyMatrix4(n.matrixWorld);const c=e.ray.origin.distanceTo(Ir);return c<e.near||c>e.far?null:{distance:c,point:Ir.clone(),object:n}}function Nr(n,t,e,i,s,r,a,o,l,c){n.getVertexPosition(o,ts),n.getVertexPosition(l,es),n.getVertexPosition(c,ns);const d=T_(n,t,e,i,ts,es,ns,Ur);if(d){s&&(Lr.fromBufferAttribute(s,o),Pr.fromBufferAttribute(s,l),Dr.fromBufferAttribute(s,c),d.uv=Mn.getInterpolation(Ur,ts,es,ns,Lr,Pr,Dr,new re)),r&&(Lr.fromBufferAttribute(r,o),Pr.fromBufferAttribute(r,l),Dr.fromBufferAttribute(r,c),d.uv1=Mn.getInterpolation(Ur,ts,es,ns,Lr,Pr,Dr,new re),d.uv2=d.uv1),a&&(Zc.fromBufferAttribute(a,o),Jc.fromBufferAttribute(a,l),Qc.fromBufferAttribute(a,c),d.normal=Mn.getInterpolation(Ur,ts,es,ns,Zc,Jc,Qc,new z),d.normal.dot(i.direction)>0&&d.normal.multiplyScalar(-1));const h={a:o,b:l,c,normal:new z,materialIndex:0};Mn.getNormal(ts,es,ns,h.normal),d.face=h}return d}class ur extends Ve{constructor(t=1,e=1,i=1,s=1,r=1,a=1){super(),this.type="BoxGeometry",this.parameters={width:t,height:e,depth:i,widthSegments:s,heightSegments:r,depthSegments:a};const o=this;s=Math.floor(s),r=Math.floor(r),a=Math.floor(a);const l=[],c=[],d=[],h=[];let p=0,f=0;v("z","y","x",-1,-1,i,e,t,a,r,0),v("z","y","x",1,-1,i,e,-t,a,r,1),v("x","z","y",1,1,t,i,e,s,a,2),v("x","z","y",1,-1,t,i,-e,s,a,3),v("x","y","z",1,-1,t,e,i,s,r,4),v("x","y","z",-1,-1,t,e,-i,s,r,5),this.setIndex(l),this.setAttribute("position",new ke(c,3)),this.setAttribute("normal",new ke(d,3)),this.setAttribute("uv",new ke(h,2));function v(g,_,m,E,b,x,C,A,D,G,T){const R=x/D,J=C/G,rt=x/2,mt=C/2,O=A/2,Z=D+1,tt=G+1;let st=0,et=0;const ot=new z;for(let ut=0;ut<tt;ut++){const pt=ut*J-mt;for(let ft=0;ft<Z;ft++){const it=ft*R-rt;ot[g]=it*E,ot[_]=pt*b,ot[m]=O,c.push(ot.x,ot.y,ot.z),ot[g]=0,ot[_]=0,ot[m]=A>0?1:-1,d.push(ot.x,ot.y,ot.z),h.push(ft/D),h.push(1-ut/G),st+=1}}for(let ut=0;ut<G;ut++)for(let pt=0;pt<D;pt++){const ft=p+pt+Z*ut,it=p+pt+Z*(ut+1),ht=p+(pt+1)+Z*(ut+1),bt=p+(pt+1)+Z*ut;l.push(ft,it,bt),l.push(it,ht,bt),et+=6}o.addGroup(f,et,T),f+=et,p+=st}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new ur(t.width,t.height,t.depth,t.widthSegments,t.heightSegments,t.depthSegments)}}function Ss(n){const t={};for(const e in n){t[e]={};for(const i in n[e]){const s=n[e][i];s&&(s.isColor||s.isMatrix3||s.isMatrix4||s.isVector2||s.isVector3||s.isVector4||s.isTexture||s.isQuaternion)?s.isRenderTargetTexture?(console.warn("UniformsUtils: Textures of render targets cannot be cloned via cloneUniforms() or mergeUniforms()."),t[e][i]=null):t[e][i]=s.clone():Array.isArray(s)?t[e][i]=s.slice():t[e][i]=s}}return t}function Ye(n){const t={};for(let e=0;e<n.length;e++){const i=Ss(n[e]);for(const s in i)t[s]=i[s]}return t}function w_(n){const t=[];for(let e=0;e<n.length;e++)t.push(n[e].clone());return t}function oh(n){return n.getRenderTarget()===null?n.outputColorSpace:ce.workingColorSpace}const A_={clone:Ss,merge:Ye};var C_=`void main() {
	gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );
}`,R_=`void main() {
	gl_FragColor = vec4( 1.0, 0.0, 0.0, 1.0 );
}`;class Vi extends ws{constructor(t){super(),this.isShaderMaterial=!0,this.type="ShaderMaterial",this.defines={},this.uniforms={},this.uniformsGroups=[],this.vertexShader=C_,this.fragmentShader=R_,this.linewidth=1,this.wireframe=!1,this.wireframeLinewidth=1,this.fog=!1,this.lights=!1,this.clipping=!1,this.forceSinglePass=!0,this.extensions={derivatives:!1,fragDepth:!1,drawBuffers:!1,shaderTextureLOD:!1,clipCullDistance:!1},this.defaultAttributeValues={color:[1,1,1],uv:[0,0],uv1:[0,0]},this.index0AttributeName=void 0,this.uniformsNeedUpdate=!1,this.glslVersion=null,t!==void 0&&this.setValues(t)}copy(t){return super.copy(t),this.fragmentShader=t.fragmentShader,this.vertexShader=t.vertexShader,this.uniforms=Ss(t.uniforms),this.uniformsGroups=w_(t.uniformsGroups),this.defines=Object.assign({},t.defines),this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this.fog=t.fog,this.lights=t.lights,this.clipping=t.clipping,this.extensions=Object.assign({},t.extensions),this.glslVersion=t.glslVersion,this}toJSON(t){const e=super.toJSON(t);e.glslVersion=this.glslVersion,e.uniforms={};for(const s in this.uniforms){const a=this.uniforms[s].value;a&&a.isTexture?e.uniforms[s]={type:"t",value:a.toJSON(t).uuid}:a&&a.isColor?e.uniforms[s]={type:"c",value:a.getHex()}:a&&a.isVector2?e.uniforms[s]={type:"v2",value:a.toArray()}:a&&a.isVector3?e.uniforms[s]={type:"v3",value:a.toArray()}:a&&a.isVector4?e.uniforms[s]={type:"v4",value:a.toArray()}:a&&a.isMatrix3?e.uniforms[s]={type:"m3",value:a.toArray()}:a&&a.isMatrix4?e.uniforms[s]={type:"m4",value:a.toArray()}:e.uniforms[s]={value:a}}Object.keys(this.defines).length>0&&(e.defines=this.defines),e.vertexShader=this.vertexShader,e.fragmentShader=this.fragmentShader,e.lights=this.lights,e.clipping=this.clipping;const i={};for(const s in this.extensions)this.extensions[s]===!0&&(i[s]=!0);return Object.keys(i).length>0&&(e.extensions=i),e}}class ah extends We{constructor(){super(),this.isCamera=!0,this.type="Camera",this.matrixWorldInverse=new Te,this.projectionMatrix=new Te,this.projectionMatrixInverse=new Te,this.coordinateSystem=ti}copy(t,e){return super.copy(t,e),this.matrixWorldInverse.copy(t.matrixWorldInverse),this.projectionMatrix.copy(t.projectionMatrix),this.projectionMatrixInverse.copy(t.projectionMatrixInverse),this.coordinateSystem=t.coordinateSystem,this}getWorldDirection(t){return super.getWorldDirection(t).negate()}updateMatrixWorld(t){super.updateMatrixWorld(t),this.matrixWorldInverse.copy(this.matrixWorld).invert()}updateWorldMatrix(t,e){super.updateWorldMatrix(t,e),this.matrixWorldInverse.copy(this.matrixWorld).invert()}clone(){return new this.constructor().copy(this)}}class ln extends ah{constructor(t=50,e=1,i=.1,s=2e3){super(),this.isPerspectiveCamera=!0,this.type="PerspectiveCamera",this.fov=t,this.zoom=1,this.near=i,this.far=s,this.focus=10,this.aspect=e,this.view=null,this.filmGauge=35,this.filmOffset=0,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.fov=t.fov,this.zoom=t.zoom,this.near=t.near,this.far=t.far,this.focus=t.focus,this.aspect=t.aspect,this.view=t.view===null?null:Object.assign({},t.view),this.filmGauge=t.filmGauge,this.filmOffset=t.filmOffset,this}setFocalLength(t){const e=.5*this.getFilmHeight()/t;this.fov=Fa*2*Math.atan(e),this.updateProjectionMatrix()}getFocalLength(){const t=Math.tan(Wo*.5*this.fov);return .5*this.getFilmHeight()/t}getEffectiveFOV(){return Fa*2*Math.atan(Math.tan(Wo*.5*this.fov)/this.zoom)}getFilmWidth(){return this.filmGauge*Math.min(this.aspect,1)}getFilmHeight(){return this.filmGauge/Math.max(this.aspect,1)}setViewOffset(t,e,i,s,r,a){this.aspect=t/e,this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=this.near;let e=t*Math.tan(Wo*.5*this.fov)/this.zoom,i=2*e,s=this.aspect*i,r=-.5*s;const a=this.view;if(this.view!==null&&this.view.enabled){const l=a.fullWidth,c=a.fullHeight;r+=a.offsetX*s/l,e-=a.offsetY*i/c,s*=a.width/l,i*=a.height/c}const o=this.filmOffset;o!==0&&(r+=t*o/this.getFilmWidth()),this.projectionMatrix.makePerspective(r,r+s,e,e-i,t,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.fov=this.fov,e.object.zoom=this.zoom,e.object.near=this.near,e.object.far=this.far,e.object.focus=this.focus,e.object.aspect=this.aspect,this.view!==null&&(e.object.view=Object.assign({},this.view)),e.object.filmGauge=this.filmGauge,e.object.filmOffset=this.filmOffset,e}}const is=-90,ss=1;class L_ extends We{constructor(t,e,i){super(),this.type="CubeCamera",this.renderTarget=i,this.coordinateSystem=null,this.activeMipmapLevel=0;const s=new ln(is,ss,t,e);s.layers=this.layers,this.add(s);const r=new ln(is,ss,t,e);r.layers=this.layers,this.add(r);const a=new ln(is,ss,t,e);a.layers=this.layers,this.add(a);const o=new ln(is,ss,t,e);o.layers=this.layers,this.add(o);const l=new ln(is,ss,t,e);l.layers=this.layers,this.add(l);const c=new ln(is,ss,t,e);c.layers=this.layers,this.add(c)}updateCoordinateSystem(){const t=this.coordinateSystem,e=this.children.concat(),[i,s,r,a,o,l]=e;for(const c of e)this.remove(c);if(t===ti)i.up.set(0,1,0),i.lookAt(1,0,0),s.up.set(0,1,0),s.lookAt(-1,0,0),r.up.set(0,0,-1),r.lookAt(0,1,0),a.up.set(0,0,1),a.lookAt(0,-1,0),o.up.set(0,1,0),o.lookAt(0,0,1),l.up.set(0,1,0),l.lookAt(0,0,-1);else if(t===ro)i.up.set(0,-1,0),i.lookAt(-1,0,0),s.up.set(0,-1,0),s.lookAt(1,0,0),r.up.set(0,0,1),r.lookAt(0,1,0),a.up.set(0,0,-1),a.lookAt(0,-1,0),o.up.set(0,-1,0),o.lookAt(0,0,1),l.up.set(0,-1,0),l.lookAt(0,0,-1);else throw new Error("THREE.CubeCamera.updateCoordinateSystem(): Invalid coordinate system: "+t);for(const c of e)this.add(c),c.updateMatrixWorld()}update(t,e){this.parent===null&&this.updateMatrixWorld();const{renderTarget:i,activeMipmapLevel:s}=this;this.coordinateSystem!==t.coordinateSystem&&(this.coordinateSystem=t.coordinateSystem,this.updateCoordinateSystem());const[r,a,o,l,c,d]=this.children,h=t.getRenderTarget(),p=t.getActiveCubeFace(),f=t.getActiveMipmapLevel(),v=t.xr.enabled;t.xr.enabled=!1;const g=i.texture.generateMipmaps;i.texture.generateMipmaps=!1,t.setRenderTarget(i,0,s),t.render(e,r),t.setRenderTarget(i,1,s),t.render(e,a),t.setRenderTarget(i,2,s),t.render(e,o),t.setRenderTarget(i,3,s),t.render(e,l),t.setRenderTarget(i,4,s),t.render(e,c),i.texture.generateMipmaps=g,t.setRenderTarget(i,5,s),t.render(e,d),t.setRenderTarget(h,p,f),t.xr.enabled=v,i.texture.needsPMREMUpdate=!0}}class lh extends dn{constructor(t,e,i,s,r,a,o,l,c,d){t=t!==void 0?t:[],e=e!==void 0?e:ys,super(t,e,i,s,r,a,o,l,c,d),this.isCubeTexture=!0,this.flipY=!1}get images(){return this.image}set images(t){this.image=t}}class P_ extends Hi{constructor(t=1,e={}){super(t,t,e),this.isWebGLCubeRenderTarget=!0;const i={width:t,height:t,depth:1},s=[i,i,i,i,i,i];e.encoding!==void 0&&(qs("THREE.WebGLCubeRenderTarget: option.encoding has been replaced by option.colorSpace."),e.colorSpace=e.encoding===zi?Ie:mn),this.texture=new lh(s,e.mapping,e.wrapS,e.wrapT,e.magFilter,e.minFilter,e.format,e.type,e.anisotropy,e.colorSpace),this.texture.isRenderTargetTexture=!0,this.texture.generateMipmaps=e.generateMipmaps!==void 0?e.generateMipmaps:!1,this.texture.minFilter=e.minFilter!==void 0?e.minFilter:pn}fromEquirectangularTexture(t,e){this.texture.type=e.type,this.texture.colorSpace=e.colorSpace,this.texture.generateMipmaps=e.generateMipmaps,this.texture.minFilter=e.minFilter,this.texture.magFilter=e.magFilter;const i={uniforms:{tEquirect:{value:null}},vertexShader:`

				varying vec3 vWorldDirection;

				vec3 transformDirection( in vec3 dir, in mat4 matrix ) {

					return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );

				}

				void main() {

					vWorldDirection = transformDirection( position, modelMatrix );

					#include <begin_vertex>
					#include <project_vertex>

				}
			`,fragmentShader:`

				uniform sampler2D tEquirect;

				varying vec3 vWorldDirection;

				#include <common>

				void main() {

					vec3 direction = normalize( vWorldDirection );

					vec2 sampleUV = equirectUv( direction );

					gl_FragColor = texture2D( tEquirect, sampleUV );

				}
			`},s=new ur(5,5,5),r=new Vi({name:"CubemapFromEquirect",uniforms:Ss(i.uniforms),vertexShader:i.vertexShader,fragmentShader:i.fragmentShader,side:tn,blending:mi});r.uniforms.tEquirect.value=e;const a=new _n(s,r),o=e.minFilter;return e.minFilter===tr&&(e.minFilter=pn),new L_(1,10,this).update(t,a),e.minFilter=o,a.geometry.dispose(),a.material.dispose(),this}clear(t,e,i,s){const r=t.getRenderTarget();for(let a=0;a<6;a++)t.setRenderTarget(this,a),t.clear(e,i,s);t.setRenderTarget(r)}}const la=new z,D_=new z,U_=new te;class Di{constructor(t=new z(1,0,0),e=0){this.isPlane=!0,this.normal=t,this.constant=e}set(t,e){return this.normal.copy(t),this.constant=e,this}setComponents(t,e,i,s){return this.normal.set(t,e,i),this.constant=s,this}setFromNormalAndCoplanarPoint(t,e){return this.normal.copy(t),this.constant=-e.dot(this.normal),this}setFromCoplanarPoints(t,e,i){const s=la.subVectors(i,e).cross(D_.subVectors(t,e)).normalize();return this.setFromNormalAndCoplanarPoint(s,t),this}copy(t){return this.normal.copy(t.normal),this.constant=t.constant,this}normalize(){const t=1/this.normal.length();return this.normal.multiplyScalar(t),this.constant*=t,this}negate(){return this.constant*=-1,this.normal.negate(),this}distanceToPoint(t){return this.normal.dot(t)+this.constant}distanceToSphere(t){return this.distanceToPoint(t.center)-t.radius}projectPoint(t,e){return e.copy(t).addScaledVector(this.normal,-this.distanceToPoint(t))}intersectLine(t,e){const i=t.delta(la),s=this.normal.dot(i);if(s===0)return this.distanceToPoint(t.start)===0?e.copy(t.start):null;const r=-(t.start.dot(this.normal)+this.constant)/s;return r<0||r>1?null:e.copy(t.start).addScaledVector(i,r)}intersectsLine(t){const e=this.distanceToPoint(t.start),i=this.distanceToPoint(t.end);return e<0&&i>0||i<0&&e>0}intersectsBox(t){return t.intersectsPlane(this)}intersectsSphere(t){return t.intersectsPlane(this)}coplanarPoint(t){return t.copy(this.normal).multiplyScalar(-this.constant)}applyMatrix4(t,e){const i=e||U_.getNormalMatrix(t),s=this.coplanarPoint(la).applyMatrix4(t),r=this.normal.applyMatrix3(i).normalize();return this.constant=-s.dot(r),this}translate(t){return this.constant-=t.dot(this.normal),this}equals(t){return t.normal.equals(this.normal)&&t.constant===this.constant}clone(){return new this.constructor().copy(this)}}const Li=new cr,kr=new z;class ll{constructor(t=new Di,e=new Di,i=new Di,s=new Di,r=new Di,a=new Di){this.planes=[t,e,i,s,r,a]}set(t,e,i,s,r,a){const o=this.planes;return o[0].copy(t),o[1].copy(e),o[2].copy(i),o[3].copy(s),o[4].copy(r),o[5].copy(a),this}copy(t){const e=this.planes;for(let i=0;i<6;i++)e[i].copy(t.planes[i]);return this}setFromProjectionMatrix(t,e=ti){const i=this.planes,s=t.elements,r=s[0],a=s[1],o=s[2],l=s[3],c=s[4],d=s[5],h=s[6],p=s[7],f=s[8],v=s[9],g=s[10],_=s[11],m=s[12],E=s[13],b=s[14],x=s[15];if(i[0].setComponents(l-r,p-c,_-f,x-m).normalize(),i[1].setComponents(l+r,p+c,_+f,x+m).normalize(),i[2].setComponents(l+a,p+d,_+v,x+E).normalize(),i[3].setComponents(l-a,p-d,_-v,x-E).normalize(),i[4].setComponents(l-o,p-h,_-g,x-b).normalize(),e===ti)i[5].setComponents(l+o,p+h,_+g,x+b).normalize();else if(e===ro)i[5].setComponents(o,h,g,b).normalize();else throw new Error("THREE.Frustum.setFromProjectionMatrix(): Invalid coordinate system: "+e);return this}intersectsObject(t){if(t.boundingSphere!==void 0)t.boundingSphere===null&&t.computeBoundingSphere(),Li.copy(t.boundingSphere).applyMatrix4(t.matrixWorld);else{const e=t.geometry;e.boundingSphere===null&&e.computeBoundingSphere(),Li.copy(e.boundingSphere).applyMatrix4(t.matrixWorld)}return this.intersectsSphere(Li)}intersectsSprite(t){return Li.center.set(0,0,0),Li.radius=.7071067811865476,Li.applyMatrix4(t.matrixWorld),this.intersectsSphere(Li)}intersectsSphere(t){const e=this.planes,i=t.center,s=-t.radius;for(let r=0;r<6;r++)if(e[r].distanceToPoint(i)<s)return!1;return!0}intersectsBox(t){const e=this.planes;for(let i=0;i<6;i++){const s=e[i];if(kr.x=s.normal.x>0?t.max.x:t.min.x,kr.y=s.normal.y>0?t.max.y:t.min.y,kr.z=s.normal.z>0?t.max.z:t.min.z,s.distanceToPoint(kr)<0)return!1}return!0}containsPoint(t){const e=this.planes;for(let i=0;i<6;i++)if(e[i].distanceToPoint(t)<0)return!1;return!0}clone(){return new this.constructor().copy(this)}}function ch(){let n=null,t=!1,e=null,i=null;function s(r,a){e(r,a),i=n.requestAnimationFrame(s)}return{start:function(){t!==!0&&e!==null&&(i=n.requestAnimationFrame(s),t=!0)},stop:function(){n.cancelAnimationFrame(i),t=!1},setAnimationLoop:function(r){e=r},setContext:function(r){n=r}}}function I_(n,t){const e=t.isWebGL2,i=new WeakMap;function s(c,d){const h=c.array,p=c.usage,f=h.byteLength,v=n.createBuffer();n.bindBuffer(d,v),n.bufferData(d,h,p),c.onUploadCallback();let g;if(h instanceof Float32Array)g=n.FLOAT;else if(h instanceof Uint16Array)if(c.isFloat16BufferAttribute)if(e)g=n.HALF_FLOAT;else throw new Error("THREE.WebGLAttributes: Usage of Float16BufferAttribute requires WebGL2.");else g=n.UNSIGNED_SHORT;else if(h instanceof Int16Array)g=n.SHORT;else if(h instanceof Uint32Array)g=n.UNSIGNED_INT;else if(h instanceof Int32Array)g=n.INT;else if(h instanceof Int8Array)g=n.BYTE;else if(h instanceof Uint8Array)g=n.UNSIGNED_BYTE;else if(h instanceof Uint8ClampedArray)g=n.UNSIGNED_BYTE;else throw new Error("THREE.WebGLAttributes: Unsupported buffer data format: "+h);return{buffer:v,type:g,bytesPerElement:h.BYTES_PER_ELEMENT,version:c.version,size:f}}function r(c,d,h){const p=d.array,f=d._updateRange,v=d.updateRanges;if(n.bindBuffer(h,c),f.count===-1&&v.length===0&&n.bufferSubData(h,0,p),v.length!==0){for(let g=0,_=v.length;g<_;g++){const m=v[g];e?n.bufferSubData(h,m.start*p.BYTES_PER_ELEMENT,p,m.start,m.count):n.bufferSubData(h,m.start*p.BYTES_PER_ELEMENT,p.subarray(m.start,m.start+m.count))}d.clearUpdateRanges()}f.count!==-1&&(e?n.bufferSubData(h,f.offset*p.BYTES_PER_ELEMENT,p,f.offset,f.count):n.bufferSubData(h,f.offset*p.BYTES_PER_ELEMENT,p.subarray(f.offset,f.offset+f.count)),f.count=-1),d.onUploadCallback()}function a(c){return c.isInterleavedBufferAttribute&&(c=c.data),i.get(c)}function o(c){c.isInterleavedBufferAttribute&&(c=c.data);const d=i.get(c);d&&(n.deleteBuffer(d.buffer),i.delete(c))}function l(c,d){if(c.isGLBufferAttribute){const p=i.get(c);(!p||p.version<c.version)&&i.set(c,{buffer:c.buffer,type:c.type,bytesPerElement:c.elementSize,version:c.version});return}c.isInterleavedBufferAttribute&&(c=c.data);const h=i.get(c);if(h===void 0)i.set(c,s(c,d));else if(h.version<c.version){if(h.size!==c.array.byteLength)throw new Error("THREE.WebGLAttributes: The size of the buffer attribute's array buffer does not match the original size. Resizing buffer attributes is not supported.");r(h.buffer,c,d),h.version=c.version}}return{get:a,remove:o,update:l}}class cl extends Ve{constructor(t=1,e=1,i=1,s=1){super(),this.type="PlaneGeometry",this.parameters={width:t,height:e,widthSegments:i,heightSegments:s};const r=t/2,a=e/2,o=Math.floor(i),l=Math.floor(s),c=o+1,d=l+1,h=t/o,p=e/l,f=[],v=[],g=[],_=[];for(let m=0;m<d;m++){const E=m*p-a;for(let b=0;b<c;b++){const x=b*h-r;v.push(x,-E,0),g.push(0,0,1),_.push(b/o),_.push(1-m/l)}}for(let m=0;m<l;m++)for(let E=0;E<o;E++){const b=E+c*m,x=E+c*(m+1),C=E+1+c*(m+1),A=E+1+c*m;f.push(b,x,A),f.push(x,C,A)}this.setIndex(f),this.setAttribute("position",new ke(v,3)),this.setAttribute("normal",new ke(g,3)),this.setAttribute("uv",new ke(_,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new cl(t.width,t.height,t.widthSegments,t.heightSegments)}}var N_=`#ifdef USE_ALPHAHASH
	if ( diffuseColor.a < getAlphaHashThreshold( vPosition ) ) discard;
#endif`,k_=`#ifdef USE_ALPHAHASH
	const float ALPHA_HASH_SCALE = 0.05;
	float hash2D( vec2 value ) {
		return fract( 1.0e4 * sin( 17.0 * value.x + 0.1 * value.y ) * ( 0.1 + abs( sin( 13.0 * value.y + value.x ) ) ) );
	}
	float hash3D( vec3 value ) {
		return hash2D( vec2( hash2D( value.xy ), value.z ) );
	}
	float getAlphaHashThreshold( vec3 position ) {
		float maxDeriv = max(
			length( dFdx( position.xyz ) ),
			length( dFdy( position.xyz ) )
		);
		float pixScale = 1.0 / ( ALPHA_HASH_SCALE * maxDeriv );
		vec2 pixScales = vec2(
			exp2( floor( log2( pixScale ) ) ),
			exp2( ceil( log2( pixScale ) ) )
		);
		vec2 alpha = vec2(
			hash3D( floor( pixScales.x * position.xyz ) ),
			hash3D( floor( pixScales.y * position.xyz ) )
		);
		float lerpFactor = fract( log2( pixScale ) );
		float x = ( 1.0 - lerpFactor ) * alpha.x + lerpFactor * alpha.y;
		float a = min( lerpFactor, 1.0 - lerpFactor );
		vec3 cases = vec3(
			x * x / ( 2.0 * a * ( 1.0 - a ) ),
			( x - 0.5 * a ) / ( 1.0 - a ),
			1.0 - ( ( 1.0 - x ) * ( 1.0 - x ) / ( 2.0 * a * ( 1.0 - a ) ) )
		);
		float threshold = ( x < ( 1.0 - a ) )
			? ( ( x < a ) ? cases.x : cases.y )
			: cases.z;
		return clamp( threshold , 1.0e-6, 1.0 );
	}
#endif`,F_=`#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, vAlphaMapUv ).g;
#endif`,O_=`#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,B_=`#ifdef USE_ALPHATEST
	if ( diffuseColor.a < alphaTest ) discard;
#endif`,z_=`#ifdef USE_ALPHATEST
	uniform float alphaTest;
#endif`,H_=`#ifdef USE_AOMAP
	float ambientOcclusion = ( texture2D( aoMap, vAoMapUv ).r - 1.0 ) * aoMapIntensity + 1.0;
	reflectedLight.indirectDiffuse *= ambientOcclusion;
	#if defined( USE_CLEARCOAT ) 
		clearcoatSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_SHEEN ) 
		sheenSpecularIndirect *= ambientOcclusion;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD )
		float dotNV = saturate( dot( geometryNormal, geometryViewDir ) );
		reflectedLight.indirectSpecular *= computeSpecularOcclusion( dotNV, ambientOcclusion, material.roughness );
	#endif
#endif`,V_=`#ifdef USE_AOMAP
	uniform sampler2D aoMap;
	uniform float aoMapIntensity;
#endif`,G_=`#ifdef USE_BATCHING
	attribute float batchId;
	uniform highp sampler2D batchingTexture;
	mat4 getBatchingMatrix( const in float i ) {
		int size = textureSize( batchingTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( batchingTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( batchingTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( batchingTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( batchingTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
#endif`,W_=`#ifdef USE_BATCHING
	mat4 batchingMatrix = getBatchingMatrix( batchId );
#endif`,$_=`vec3 transformed = vec3( position );
#ifdef USE_ALPHAHASH
	vPosition = vec3( position );
#endif`,X_=`vec3 objectNormal = vec3( normal );
#ifdef USE_TANGENT
	vec3 objectTangent = vec3( tangent.xyz );
#endif`,q_=`float G_BlinnPhong_Implicit( ) {
	return 0.25;
}
float D_BlinnPhong( const in float shininess, const in float dotNH ) {
	return RECIPROCAL_PI * ( shininess * 0.5 + 1.0 ) * pow( dotNH, shininess );
}
vec3 BRDF_BlinnPhong( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in vec3 specularColor, const in float shininess ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( specularColor, 1.0, dotVH );
	float G = G_BlinnPhong_Implicit( );
	float D = D_BlinnPhong( shininess, dotNH );
	return F * ( G * D );
} // validated`,j_=`#ifdef USE_IRIDESCENCE
	const mat3 XYZ_TO_REC709 = mat3(
		 3.2404542, -0.9692660,  0.0556434,
		-1.5371385,  1.8760108, -0.2040259,
		-0.4985314,  0.0415560,  1.0572252
	);
	vec3 Fresnel0ToIor( vec3 fresnel0 ) {
		vec3 sqrtF0 = sqrt( fresnel0 );
		return ( vec3( 1.0 ) + sqrtF0 ) / ( vec3( 1.0 ) - sqrtF0 );
	}
	vec3 IorToFresnel0( vec3 transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - vec3( incidentIor ) ) / ( transmittedIor + vec3( incidentIor ) ) );
	}
	float IorToFresnel0( float transmittedIor, float incidentIor ) {
		return pow2( ( transmittedIor - incidentIor ) / ( transmittedIor + incidentIor ));
	}
	vec3 evalSensitivity( float OPD, vec3 shift ) {
		float phase = 2.0 * PI * OPD * 1.0e-9;
		vec3 val = vec3( 5.4856e-13, 4.4201e-13, 5.2481e-13 );
		vec3 pos = vec3( 1.6810e+06, 1.7953e+06, 2.2084e+06 );
		vec3 var = vec3( 4.3278e+09, 9.3046e+09, 6.6121e+09 );
		vec3 xyz = val * sqrt( 2.0 * PI * var ) * cos( pos * phase + shift ) * exp( - pow2( phase ) * var );
		xyz.x += 9.7470e-14 * sqrt( 2.0 * PI * 4.5282e+09 ) * cos( 2.2399e+06 * phase + shift[ 0 ] ) * exp( - 4.5282e+09 * pow2( phase ) );
		xyz /= 1.0685e-7;
		vec3 rgb = XYZ_TO_REC709 * xyz;
		return rgb;
	}
	vec3 evalIridescence( float outsideIOR, float eta2, float cosTheta1, float thinFilmThickness, vec3 baseF0 ) {
		vec3 I;
		float iridescenceIOR = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
		float sinTheta2Sq = pow2( outsideIOR / iridescenceIOR ) * ( 1.0 - pow2( cosTheta1 ) );
		float cosTheta2Sq = 1.0 - sinTheta2Sq;
		if ( cosTheta2Sq < 0.0 ) {
			return vec3( 1.0 );
		}
		float cosTheta2 = sqrt( cosTheta2Sq );
		float R0 = IorToFresnel0( iridescenceIOR, outsideIOR );
		float R12 = F_Schlick( R0, 1.0, cosTheta1 );
		float T121 = 1.0 - R12;
		float phi12 = 0.0;
		if ( iridescenceIOR < outsideIOR ) phi12 = PI;
		float phi21 = PI - phi12;
		vec3 baseIOR = Fresnel0ToIor( clamp( baseF0, 0.0, 0.9999 ) );		vec3 R1 = IorToFresnel0( baseIOR, iridescenceIOR );
		vec3 R23 = F_Schlick( R1, 1.0, cosTheta2 );
		vec3 phi23 = vec3( 0.0 );
		if ( baseIOR[ 0 ] < iridescenceIOR ) phi23[ 0 ] = PI;
		if ( baseIOR[ 1 ] < iridescenceIOR ) phi23[ 1 ] = PI;
		if ( baseIOR[ 2 ] < iridescenceIOR ) phi23[ 2 ] = PI;
		float OPD = 2.0 * iridescenceIOR * thinFilmThickness * cosTheta2;
		vec3 phi = vec3( phi21 ) + phi23;
		vec3 R123 = clamp( R12 * R23, 1e-5, 0.9999 );
		vec3 r123 = sqrt( R123 );
		vec3 Rs = pow2( T121 ) * R23 / ( vec3( 1.0 ) - R123 );
		vec3 C0 = R12 + Rs;
		I = C0;
		vec3 Cm = Rs - T121;
		for ( int m = 1; m <= 2; ++ m ) {
			Cm *= r123;
			vec3 Sm = 2.0 * evalSensitivity( float( m ) * OPD, float( m ) * phi );
			I += Cm * Sm;
		}
		return max( I, vec3( 0.0 ) );
	}
#endif`,Y_=`#ifdef USE_BUMPMAP
	uniform sampler2D bumpMap;
	uniform float bumpScale;
	vec2 dHdxy_fwd() {
		vec2 dSTdx = dFdx( vBumpMapUv );
		vec2 dSTdy = dFdy( vBumpMapUv );
		float Hll = bumpScale * texture2D( bumpMap, vBumpMapUv ).x;
		float dBx = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdx ).x - Hll;
		float dBy = bumpScale * texture2D( bumpMap, vBumpMapUv + dSTdy ).x - Hll;
		return vec2( dBx, dBy );
	}
	vec3 perturbNormalArb( vec3 surf_pos, vec3 surf_norm, vec2 dHdxy, float faceDirection ) {
		vec3 vSigmaX = normalize( dFdx( surf_pos.xyz ) );
		vec3 vSigmaY = normalize( dFdy( surf_pos.xyz ) );
		vec3 vN = surf_norm;
		vec3 R1 = cross( vSigmaY, vN );
		vec3 R2 = cross( vN, vSigmaX );
		float fDet = dot( vSigmaX, R1 ) * faceDirection;
		vec3 vGrad = sign( fDet ) * ( dHdxy.x * R1 + dHdxy.y * R2 );
		return normalize( abs( fDet ) * surf_norm - vGrad );
	}
#endif`,K_=`#if NUM_CLIPPING_PLANES > 0
	vec4 plane;
	#pragma unroll_loop_start
	for ( int i = 0; i < UNION_CLIPPING_PLANES; i ++ ) {
		plane = clippingPlanes[ i ];
		if ( dot( vClipPosition, plane.xyz ) > plane.w ) discard;
	}
	#pragma unroll_loop_end
	#if UNION_CLIPPING_PLANES < NUM_CLIPPING_PLANES
		bool clipped = true;
		#pragma unroll_loop_start
		for ( int i = UNION_CLIPPING_PLANES; i < NUM_CLIPPING_PLANES; i ++ ) {
			plane = clippingPlanes[ i ];
			clipped = ( dot( vClipPosition, plane.xyz ) > plane.w ) && clipped;
		}
		#pragma unroll_loop_end
		if ( clipped ) discard;
	#endif
#endif`,Z_=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
	uniform vec4 clippingPlanes[ NUM_CLIPPING_PLANES ];
#endif`,J_=`#if NUM_CLIPPING_PLANES > 0
	varying vec3 vClipPosition;
#endif`,Q_=`#if NUM_CLIPPING_PLANES > 0
	vClipPosition = - mvPosition.xyz;
#endif`,tg=`#if defined( USE_COLOR_ALPHA )
	diffuseColor *= vColor;
#elif defined( USE_COLOR )
	diffuseColor.rgb *= vColor;
#endif`,eg=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR )
	varying vec3 vColor;
#endif`,ng=`#if defined( USE_COLOR_ALPHA )
	varying vec4 vColor;
#elif defined( USE_COLOR ) || defined( USE_INSTANCING_COLOR )
	varying vec3 vColor;
#endif`,ig=`#if defined( USE_COLOR_ALPHA )
	vColor = vec4( 1.0 );
#elif defined( USE_COLOR ) || defined( USE_INSTANCING_COLOR )
	vColor = vec3( 1.0 );
#endif
#ifdef USE_COLOR
	vColor *= color;
#endif
#ifdef USE_INSTANCING_COLOR
	vColor.xyz *= instanceColor.xyz;
#endif`,sg=`#define PI 3.141592653589793
#define PI2 6.283185307179586
#define PI_HALF 1.5707963267948966
#define RECIPROCAL_PI 0.3183098861837907
#define RECIPROCAL_PI2 0.15915494309189535
#define EPSILON 1e-6
#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
#define whiteComplement( a ) ( 1.0 - saturate( a ) )
float pow2( const in float x ) { return x*x; }
vec3 pow2( const in vec3 x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }
float max3( const in vec3 v ) { return max( max( v.x, v.y ), v.z ); }
float average( const in vec3 v ) { return dot( v, vec3( 0.3333333 ) ); }
highp float rand( const in vec2 uv ) {
	const highp float a = 12.9898, b = 78.233, c = 43758.5453;
	highp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );
	return fract( sin( sn ) * c );
}
#ifdef HIGH_PRECISION
	float precisionSafeLength( vec3 v ) { return length( v ); }
#else
	float precisionSafeLength( vec3 v ) {
		float maxComponent = max3( abs( v ) );
		return length( v / maxComponent ) * maxComponent;
	}
#endif
struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};
struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};
#ifdef USE_ALPHAHASH
	varying vec3 vPosition;
#endif
vec3 transformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );
}
vec3 inverseTransformDirection( in vec3 dir, in mat4 matrix ) {
	return normalize( ( vec4( dir, 0.0 ) * matrix ).xyz );
}
mat3 transposeMat3( const in mat3 m ) {
	mat3 tmp;
	tmp[ 0 ] = vec3( m[ 0 ].x, m[ 1 ].x, m[ 2 ].x );
	tmp[ 1 ] = vec3( m[ 0 ].y, m[ 1 ].y, m[ 2 ].y );
	tmp[ 2 ] = vec3( m[ 0 ].z, m[ 1 ].z, m[ 2 ].z );
	return tmp;
}
float luminance( const in vec3 rgb ) {
	const vec3 weights = vec3( 0.2126729, 0.7151522, 0.0721750 );
	return dot( weights, rgb );
}
bool isPerspectiveMatrix( mat4 m ) {
	return m[ 2 ][ 3 ] == - 1.0;
}
vec2 equirectUv( in vec3 dir ) {
	float u = atan( dir.z, dir.x ) * RECIPROCAL_PI2 + 0.5;
	float v = asin( clamp( dir.y, - 1.0, 1.0 ) ) * RECIPROCAL_PI + 0.5;
	return vec2( u, v );
}
vec3 BRDF_Lambert( const in vec3 diffuseColor ) {
	return RECIPROCAL_PI * diffuseColor;
}
vec3 F_Schlick( const in vec3 f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
}
float F_Schlick( const in float f0, const in float f90, const in float dotVH ) {
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );
	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );
} // validated`,rg=`#ifdef ENVMAP_TYPE_CUBE_UV
	#define cubeUV_minMipLevel 4.0
	#define cubeUV_minTileSize 16.0
	float getFace( vec3 direction ) {
		vec3 absDirection = abs( direction );
		float face = - 1.0;
		if ( absDirection.x > absDirection.z ) {
			if ( absDirection.x > absDirection.y )
				face = direction.x > 0.0 ? 0.0 : 3.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		} else {
			if ( absDirection.z > absDirection.y )
				face = direction.z > 0.0 ? 2.0 : 5.0;
			else
				face = direction.y > 0.0 ? 1.0 : 4.0;
		}
		return face;
	}
	vec2 getUV( vec3 direction, float face ) {
		vec2 uv;
		if ( face == 0.0 ) {
			uv = vec2( direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 1.0 ) {
			uv = vec2( - direction.x, - direction.z ) / abs( direction.y );
		} else if ( face == 2.0 ) {
			uv = vec2( - direction.x, direction.y ) / abs( direction.z );
		} else if ( face == 3.0 ) {
			uv = vec2( - direction.z, direction.y ) / abs( direction.x );
		} else if ( face == 4.0 ) {
			uv = vec2( - direction.x, direction.z ) / abs( direction.y );
		} else {
			uv = vec2( direction.x, direction.y ) / abs( direction.z );
		}
		return 0.5 * ( uv + 1.0 );
	}
	vec3 bilinearCubeUV( sampler2D envMap, vec3 direction, float mipInt ) {
		float face = getFace( direction );
		float filterInt = max( cubeUV_minMipLevel - mipInt, 0.0 );
		mipInt = max( mipInt, cubeUV_minMipLevel );
		float faceSize = exp2( mipInt );
		highp vec2 uv = getUV( direction, face ) * ( faceSize - 2.0 ) + 1.0;
		if ( face > 2.0 ) {
			uv.y += faceSize;
			face -= 3.0;
		}
		uv.x += face * faceSize;
		uv.x += filterInt * 3.0 * cubeUV_minTileSize;
		uv.y += 4.0 * ( exp2( CUBEUV_MAX_MIP ) - faceSize );
		uv.x *= CUBEUV_TEXEL_WIDTH;
		uv.y *= CUBEUV_TEXEL_HEIGHT;
		#ifdef texture2DGradEXT
			return texture2DGradEXT( envMap, uv, vec2( 0.0 ), vec2( 0.0 ) ).rgb;
		#else
			return texture2D( envMap, uv ).rgb;
		#endif
	}
	#define cubeUV_r0 1.0
	#define cubeUV_m0 - 2.0
	#define cubeUV_r1 0.8
	#define cubeUV_m1 - 1.0
	#define cubeUV_r4 0.4
	#define cubeUV_m4 2.0
	#define cubeUV_r5 0.305
	#define cubeUV_m5 3.0
	#define cubeUV_r6 0.21
	#define cubeUV_m6 4.0
	float roughnessToMip( float roughness ) {
		float mip = 0.0;
		if ( roughness >= cubeUV_r1 ) {
			mip = ( cubeUV_r0 - roughness ) * ( cubeUV_m1 - cubeUV_m0 ) / ( cubeUV_r0 - cubeUV_r1 ) + cubeUV_m0;
		} else if ( roughness >= cubeUV_r4 ) {
			mip = ( cubeUV_r1 - roughness ) * ( cubeUV_m4 - cubeUV_m1 ) / ( cubeUV_r1 - cubeUV_r4 ) + cubeUV_m1;
		} else if ( roughness >= cubeUV_r5 ) {
			mip = ( cubeUV_r4 - roughness ) * ( cubeUV_m5 - cubeUV_m4 ) / ( cubeUV_r4 - cubeUV_r5 ) + cubeUV_m4;
		} else if ( roughness >= cubeUV_r6 ) {
			mip = ( cubeUV_r5 - roughness ) * ( cubeUV_m6 - cubeUV_m5 ) / ( cubeUV_r5 - cubeUV_r6 ) + cubeUV_m5;
		} else {
			mip = - 2.0 * log2( 1.16 * roughness );		}
		return mip;
	}
	vec4 textureCubeUV( sampler2D envMap, vec3 sampleDir, float roughness ) {
		float mip = clamp( roughnessToMip( roughness ), cubeUV_m0, CUBEUV_MAX_MIP );
		float mipF = fract( mip );
		float mipInt = floor( mip );
		vec3 color0 = bilinearCubeUV( envMap, sampleDir, mipInt );
		if ( mipF == 0.0 ) {
			return vec4( color0, 1.0 );
		} else {
			vec3 color1 = bilinearCubeUV( envMap, sampleDir, mipInt + 1.0 );
			return vec4( mix( color0, color1, mipF ), 1.0 );
		}
	}
#endif`,og=`vec3 transformedNormal = objectNormal;
#ifdef USE_TANGENT
	vec3 transformedTangent = objectTangent;
#endif
#ifdef USE_BATCHING
	mat3 bm = mat3( batchingMatrix );
	transformedNormal /= vec3( dot( bm[ 0 ], bm[ 0 ] ), dot( bm[ 1 ], bm[ 1 ] ), dot( bm[ 2 ], bm[ 2 ] ) );
	transformedNormal = bm * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = bm * transformedTangent;
	#endif
#endif
#ifdef USE_INSTANCING
	mat3 im = mat3( instanceMatrix );
	transformedNormal /= vec3( dot( im[ 0 ], im[ 0 ] ), dot( im[ 1 ], im[ 1 ] ), dot( im[ 2 ], im[ 2 ] ) );
	transformedNormal = im * transformedNormal;
	#ifdef USE_TANGENT
		transformedTangent = im * transformedTangent;
	#endif
#endif
transformedNormal = normalMatrix * transformedNormal;
#ifdef FLIP_SIDED
	transformedNormal = - transformedNormal;
#endif
#ifdef USE_TANGENT
	transformedTangent = ( modelViewMatrix * vec4( transformedTangent, 0.0 ) ).xyz;
	#ifdef FLIP_SIDED
		transformedTangent = - transformedTangent;
	#endif
#endif`,ag=`#ifdef USE_DISPLACEMENTMAP
	uniform sampler2D displacementMap;
	uniform float displacementScale;
	uniform float displacementBias;
#endif`,lg=`#ifdef USE_DISPLACEMENTMAP
	transformed += normalize( objectNormal ) * ( texture2D( displacementMap, vDisplacementMapUv ).x * displacementScale + displacementBias );
#endif`,cg=`#ifdef USE_EMISSIVEMAP
	vec4 emissiveColor = texture2D( emissiveMap, vEmissiveMapUv );
	totalEmissiveRadiance *= emissiveColor.rgb;
#endif`,ug=`#ifdef USE_EMISSIVEMAP
	uniform sampler2D emissiveMap;
#endif`,dg="gl_FragColor = linearToOutputTexel( gl_FragColor );",hg=`
const mat3 LINEAR_SRGB_TO_LINEAR_DISPLAY_P3 = mat3(
	vec3( 0.8224621, 0.177538, 0.0 ),
	vec3( 0.0331941, 0.9668058, 0.0 ),
	vec3( 0.0170827, 0.0723974, 0.9105199 )
);
const mat3 LINEAR_DISPLAY_P3_TO_LINEAR_SRGB = mat3(
	vec3( 1.2249401, - 0.2249404, 0.0 ),
	vec3( - 0.0420569, 1.0420571, 0.0 ),
	vec3( - 0.0196376, - 0.0786361, 1.0982735 )
);
vec4 LinearSRGBToLinearDisplayP3( in vec4 value ) {
	return vec4( value.rgb * LINEAR_SRGB_TO_LINEAR_DISPLAY_P3, value.a );
}
vec4 LinearDisplayP3ToLinearSRGB( in vec4 value ) {
	return vec4( value.rgb * LINEAR_DISPLAY_P3_TO_LINEAR_SRGB, value.a );
}
vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}
vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}
vec4 LinearToLinear( in vec4 value ) {
	return value;
}
vec4 LinearTosRGB( in vec4 value ) {
	return sRGBTransferOETF( value );
}`,fg=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vec3 cameraToFrag;
		if ( isOrthographic ) {
			cameraToFrag = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToFrag = normalize( vWorldPosition - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vec3 reflectVec = reflect( cameraToFrag, worldNormal );
		#else
			vec3 reflectVec = refract( cameraToFrag, worldNormal, refractionRatio );
		#endif
	#else
		vec3 reflectVec = vReflect;
	#endif
	#ifdef ENVMAP_TYPE_CUBE
		vec4 envColor = textureCube( envMap, vec3( flipEnvMap * reflectVec.x, reflectVec.yz ) );
	#else
		vec4 envColor = vec4( 0.0 );
	#endif
	#ifdef ENVMAP_BLENDING_MULTIPLY
		outgoingLight = mix( outgoingLight, outgoingLight * envColor.xyz, specularStrength * reflectivity );
	#elif defined( ENVMAP_BLENDING_MIX )
		outgoingLight = mix( outgoingLight, envColor.xyz, specularStrength * reflectivity );
	#elif defined( ENVMAP_BLENDING_ADD )
		outgoingLight += envColor.xyz * specularStrength * reflectivity;
	#endif
#endif`,pg=`#ifdef USE_ENVMAP
	uniform float envMapIntensity;
	uniform float flipEnvMap;
	#ifdef ENVMAP_TYPE_CUBE
		uniform samplerCube envMap;
	#else
		uniform sampler2D envMap;
	#endif
	
#endif`,mg=`#ifdef USE_ENVMAP
	uniform float reflectivity;
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		varying vec3 vWorldPosition;
		uniform float refractionRatio;
	#else
		varying vec3 vReflect;
	#endif
#endif`,_g=`#ifdef USE_ENVMAP
	#if defined( USE_BUMPMAP ) || defined( USE_NORMALMAP ) || defined( PHONG ) || defined( LAMBERT )
		#define ENV_WORLDPOS
	#endif
	#ifdef ENV_WORLDPOS
		
		varying vec3 vWorldPosition;
	#else
		varying vec3 vReflect;
		uniform float refractionRatio;
	#endif
#endif`,gg=`#ifdef USE_ENVMAP
	#ifdef ENV_WORLDPOS
		vWorldPosition = worldPosition.xyz;
	#else
		vec3 cameraToVertex;
		if ( isOrthographic ) {
			cameraToVertex = normalize( vec3( - viewMatrix[ 0 ][ 2 ], - viewMatrix[ 1 ][ 2 ], - viewMatrix[ 2 ][ 2 ] ) );
		} else {
			cameraToVertex = normalize( worldPosition.xyz - cameraPosition );
		}
		vec3 worldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
		#ifdef ENVMAP_MODE_REFLECTION
			vReflect = reflect( cameraToVertex, worldNormal );
		#else
			vReflect = refract( cameraToVertex, worldNormal, refractionRatio );
		#endif
	#endif
#endif`,vg=`#ifdef USE_FOG
	vFogDepth = - mvPosition.z;
#endif`,xg=`#ifdef USE_FOG
	varying float vFogDepth;
#endif`,yg=`#ifdef USE_FOG
	#ifdef FOG_EXP2
		float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
	#else
		float fogFactor = smoothstep( fogNear, fogFar, vFogDepth );
	#endif
	gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
#endif`,bg=`#ifdef USE_FOG
	uniform vec3 fogColor;
	varying float vFogDepth;
	#ifdef FOG_EXP2
		uniform float fogDensity;
	#else
		uniform float fogNear;
		uniform float fogFar;
	#endif
#endif`,Mg=`#ifdef USE_GRADIENTMAP
	uniform sampler2D gradientMap;
#endif
vec3 getGradientIrradiance( vec3 normal, vec3 lightDirection ) {
	float dotNL = dot( normal, lightDirection );
	vec2 coord = vec2( dotNL * 0.5 + 0.5, 0.0 );
	#ifdef USE_GRADIENTMAP
		return vec3( texture2D( gradientMap, coord ).r );
	#else
		vec2 fw = fwidth( coord ) * 0.5;
		return mix( vec3( 0.7 ), vec3( 1.0 ), smoothstep( 0.7 - fw.x, 0.7 + fw.x, coord.x ) );
	#endif
}`,Sg=`#ifdef USE_LIGHTMAP
	vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
	vec3 lightMapIrradiance = lightMapTexel.rgb * lightMapIntensity;
	reflectedLight.indirectDiffuse += lightMapIrradiance;
#endif`,Eg=`#ifdef USE_LIGHTMAP
	uniform sampler2D lightMap;
	uniform float lightMapIntensity;
#endif`,Tg=`LambertMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularStrength = specularStrength;`,wg=`varying vec3 vViewPosition;
struct LambertMaterial {
	vec3 diffuseColor;
	float specularStrength;
};
void RE_Direct_Lambert( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Lambert( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in LambertMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Lambert
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Lambert`,Ag=`uniform bool receiveShadow;
uniform vec3 ambientLightColor;
#if defined( USE_LIGHT_PROBES )
	uniform vec3 lightProbe[ 9 ];
#endif
vec3 shGetIrradianceAt( in vec3 normal, in vec3 shCoefficients[ 9 ] ) {
	float x = normal.x, y = normal.y, z = normal.z;
	vec3 result = shCoefficients[ 0 ] * 0.886227;
	result += shCoefficients[ 1 ] * 2.0 * 0.511664 * y;
	result += shCoefficients[ 2 ] * 2.0 * 0.511664 * z;
	result += shCoefficients[ 3 ] * 2.0 * 0.511664 * x;
	result += shCoefficients[ 4 ] * 2.0 * 0.429043 * x * y;
	result += shCoefficients[ 5 ] * 2.0 * 0.429043 * y * z;
	result += shCoefficients[ 6 ] * ( 0.743125 * z * z - 0.247708 );
	result += shCoefficients[ 7 ] * 2.0 * 0.429043 * x * z;
	result += shCoefficients[ 8 ] * 0.429043 * ( x * x - y * y );
	return result;
}
vec3 getLightProbeIrradiance( const in vec3 lightProbe[ 9 ], const in vec3 normal ) {
	vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
	vec3 irradiance = shGetIrradianceAt( worldNormal, lightProbe );
	return irradiance;
}
vec3 getAmbientLightIrradiance( const in vec3 ambientLightColor ) {
	vec3 irradiance = ambientLightColor;
	return irradiance;
}
float getDistanceAttenuation( const in float lightDistance, const in float cutoffDistance, const in float decayExponent ) {
	#if defined ( LEGACY_LIGHTS )
		if ( cutoffDistance > 0.0 && decayExponent > 0.0 ) {
			return pow( saturate( - lightDistance / cutoffDistance + 1.0 ), decayExponent );
		}
		return 1.0;
	#else
		float distanceFalloff = 1.0 / max( pow( lightDistance, decayExponent ), 0.01 );
		if ( cutoffDistance > 0.0 ) {
			distanceFalloff *= pow2( saturate( 1.0 - pow4( lightDistance / cutoffDistance ) ) );
		}
		return distanceFalloff;
	#endif
}
float getSpotAttenuation( const in float coneCosine, const in float penumbraCosine, const in float angleCosine ) {
	return smoothstep( coneCosine, penumbraCosine, angleCosine );
}
#if NUM_DIR_LIGHTS > 0
	struct DirectionalLight {
		vec3 direction;
		vec3 color;
	};
	uniform DirectionalLight directionalLights[ NUM_DIR_LIGHTS ];
	void getDirectionalLightInfo( const in DirectionalLight directionalLight, out IncidentLight light ) {
		light.color = directionalLight.color;
		light.direction = directionalLight.direction;
		light.visible = true;
	}
#endif
#if NUM_POINT_LIGHTS > 0
	struct PointLight {
		vec3 position;
		vec3 color;
		float distance;
		float decay;
	};
	uniform PointLight pointLights[ NUM_POINT_LIGHTS ];
	void getPointLightInfo( const in PointLight pointLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = pointLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float lightDistance = length( lVector );
		light.color = pointLight.color;
		light.color *= getDistanceAttenuation( lightDistance, pointLight.distance, pointLight.decay );
		light.visible = ( light.color != vec3( 0.0 ) );
	}
#endif
#if NUM_SPOT_LIGHTS > 0
	struct SpotLight {
		vec3 position;
		vec3 direction;
		vec3 color;
		float distance;
		float decay;
		float coneCos;
		float penumbraCos;
	};
	uniform SpotLight spotLights[ NUM_SPOT_LIGHTS ];
	void getSpotLightInfo( const in SpotLight spotLight, const in vec3 geometryPosition, out IncidentLight light ) {
		vec3 lVector = spotLight.position - geometryPosition;
		light.direction = normalize( lVector );
		float angleCos = dot( light.direction, spotLight.direction );
		float spotAttenuation = getSpotAttenuation( spotLight.coneCos, spotLight.penumbraCos, angleCos );
		if ( spotAttenuation > 0.0 ) {
			float lightDistance = length( lVector );
			light.color = spotLight.color * spotAttenuation;
			light.color *= getDistanceAttenuation( lightDistance, spotLight.distance, spotLight.decay );
			light.visible = ( light.color != vec3( 0.0 ) );
		} else {
			light.color = vec3( 0.0 );
			light.visible = false;
		}
	}
#endif
#if NUM_RECT_AREA_LIGHTS > 0
	struct RectAreaLight {
		vec3 color;
		vec3 position;
		vec3 halfWidth;
		vec3 halfHeight;
	};
	uniform sampler2D ltc_1;	uniform sampler2D ltc_2;
	uniform RectAreaLight rectAreaLights[ NUM_RECT_AREA_LIGHTS ];
#endif
#if NUM_HEMI_LIGHTS > 0
	struct HemisphereLight {
		vec3 direction;
		vec3 skyColor;
		vec3 groundColor;
	};
	uniform HemisphereLight hemisphereLights[ NUM_HEMI_LIGHTS ];
	vec3 getHemisphereLightIrradiance( const in HemisphereLight hemiLight, const in vec3 normal ) {
		float dotNL = dot( normal, hemiLight.direction );
		float hemiDiffuseWeight = 0.5 * dotNL + 0.5;
		vec3 irradiance = mix( hemiLight.groundColor, hemiLight.skyColor, hemiDiffuseWeight );
		return irradiance;
	}
#endif`,Cg=`#ifdef USE_ENVMAP
	vec3 getIBLIrradiance( const in vec3 normal ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 worldNormal = inverseTransformDirection( normal, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, worldNormal, 1.0 );
			return PI * envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	vec3 getIBLRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness ) {
		#ifdef ENVMAP_TYPE_CUBE_UV
			vec3 reflectVec = reflect( - viewDir, normal );
			reflectVec = normalize( mix( reflectVec, normal, roughness * roughness) );
			reflectVec = inverseTransformDirection( reflectVec, viewMatrix );
			vec4 envMapColor = textureCubeUV( envMap, reflectVec, roughness );
			return envMapColor.rgb * envMapIntensity;
		#else
			return vec3( 0.0 );
		#endif
	}
	#ifdef USE_ANISOTROPY
		vec3 getIBLAnisotropyRadiance( const in vec3 viewDir, const in vec3 normal, const in float roughness, const in vec3 bitangent, const in float anisotropy ) {
			#ifdef ENVMAP_TYPE_CUBE_UV
				vec3 bentNormal = cross( bitangent, viewDir );
				bentNormal = normalize( cross( bentNormal, bitangent ) );
				bentNormal = normalize( mix( bentNormal, normal, pow2( pow2( 1.0 - anisotropy * ( 1.0 - roughness ) ) ) ) );
				return getIBLRadiance( viewDir, bentNormal, roughness );
			#else
				return vec3( 0.0 );
			#endif
		}
	#endif
#endif`,Rg=`ToonMaterial material;
material.diffuseColor = diffuseColor.rgb;`,Lg=`varying vec3 vViewPosition;
struct ToonMaterial {
	vec3 diffuseColor;
};
void RE_Direct_Toon( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	vec3 irradiance = getGradientIrradiance( geometryNormal, directLight.direction ) * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Toon( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in ToonMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_Toon
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Toon`,Pg=`BlinnPhongMaterial material;
material.diffuseColor = diffuseColor.rgb;
material.specularColor = specular;
material.specularShininess = shininess;
material.specularStrength = specularStrength;`,Dg=`varying vec3 vViewPosition;
struct BlinnPhongMaterial {
	vec3 diffuseColor;
	vec3 specularColor;
	float specularShininess;
	float specularStrength;
};
void RE_Direct_BlinnPhong( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
	reflectedLight.directSpecular += irradiance * BRDF_BlinnPhong( directLight.direction, geometryViewDir, geometryNormal, material.specularColor, material.specularShininess ) * material.specularStrength;
}
void RE_IndirectDiffuse_BlinnPhong( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in BlinnPhongMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
#define RE_Direct				RE_Direct_BlinnPhong
#define RE_IndirectDiffuse		RE_IndirectDiffuse_BlinnPhong`,Ug=`PhysicalMaterial material;
material.diffuseColor = diffuseColor.rgb * ( 1.0 - metalnessFactor );
vec3 dxy = max( abs( dFdx( nonPerturbedNormal ) ), abs( dFdy( nonPerturbedNormal ) ) );
float geometryRoughness = max( max( dxy.x, dxy.y ), dxy.z );
material.roughness = max( roughnessFactor, 0.0525 );material.roughness += geometryRoughness;
material.roughness = min( material.roughness, 1.0 );
#ifdef IOR
	material.ior = ior;
	#ifdef USE_SPECULAR
		float specularIntensityFactor = specularIntensity;
		vec3 specularColorFactor = specularColor;
		#ifdef USE_SPECULAR_COLORMAP
			specularColorFactor *= texture2D( specularColorMap, vSpecularColorMapUv ).rgb;
		#endif
		#ifdef USE_SPECULAR_INTENSITYMAP
			specularIntensityFactor *= texture2D( specularIntensityMap, vSpecularIntensityMapUv ).a;
		#endif
		material.specularF90 = mix( specularIntensityFactor, 1.0, metalnessFactor );
	#else
		float specularIntensityFactor = 1.0;
		vec3 specularColorFactor = vec3( 1.0 );
		material.specularF90 = 1.0;
	#endif
	material.specularColor = mix( min( pow2( ( material.ior - 1.0 ) / ( material.ior + 1.0 ) ) * specularColorFactor, vec3( 1.0 ) ) * specularIntensityFactor, diffuseColor.rgb, metalnessFactor );
#else
	material.specularColor = mix( vec3( 0.04 ), diffuseColor.rgb, metalnessFactor );
	material.specularF90 = 1.0;
#endif
#ifdef USE_CLEARCOAT
	material.clearcoat = clearcoat;
	material.clearcoatRoughness = clearcoatRoughness;
	material.clearcoatF0 = vec3( 0.04 );
	material.clearcoatF90 = 1.0;
	#ifdef USE_CLEARCOATMAP
		material.clearcoat *= texture2D( clearcoatMap, vClearcoatMapUv ).x;
	#endif
	#ifdef USE_CLEARCOAT_ROUGHNESSMAP
		material.clearcoatRoughness *= texture2D( clearcoatRoughnessMap, vClearcoatRoughnessMapUv ).y;
	#endif
	material.clearcoat = saturate( material.clearcoat );	material.clearcoatRoughness = max( material.clearcoatRoughness, 0.0525 );
	material.clearcoatRoughness += geometryRoughness;
	material.clearcoatRoughness = min( material.clearcoatRoughness, 1.0 );
#endif
#ifdef USE_IRIDESCENCE
	material.iridescence = iridescence;
	material.iridescenceIOR = iridescenceIOR;
	#ifdef USE_IRIDESCENCEMAP
		material.iridescence *= texture2D( iridescenceMap, vIridescenceMapUv ).r;
	#endif
	#ifdef USE_IRIDESCENCE_THICKNESSMAP
		material.iridescenceThickness = (iridescenceThicknessMaximum - iridescenceThicknessMinimum) * texture2D( iridescenceThicknessMap, vIridescenceThicknessMapUv ).g + iridescenceThicknessMinimum;
	#else
		material.iridescenceThickness = iridescenceThicknessMaximum;
	#endif
#endif
#ifdef USE_SHEEN
	material.sheenColor = sheenColor;
	#ifdef USE_SHEEN_COLORMAP
		material.sheenColor *= texture2D( sheenColorMap, vSheenColorMapUv ).rgb;
	#endif
	material.sheenRoughness = clamp( sheenRoughness, 0.07, 1.0 );
	#ifdef USE_SHEEN_ROUGHNESSMAP
		material.sheenRoughness *= texture2D( sheenRoughnessMap, vSheenRoughnessMapUv ).a;
	#endif
#endif
#ifdef USE_ANISOTROPY
	#ifdef USE_ANISOTROPYMAP
		mat2 anisotropyMat = mat2( anisotropyVector.x, anisotropyVector.y, - anisotropyVector.y, anisotropyVector.x );
		vec3 anisotropyPolar = texture2D( anisotropyMap, vAnisotropyMapUv ).rgb;
		vec2 anisotropyV = anisotropyMat * normalize( 2.0 * anisotropyPolar.rg - vec2( 1.0 ) ) * anisotropyPolar.b;
	#else
		vec2 anisotropyV = anisotropyVector;
	#endif
	material.anisotropy = length( anisotropyV );
	if( material.anisotropy == 0.0 ) {
		anisotropyV = vec2( 1.0, 0.0 );
	} else {
		anisotropyV /= material.anisotropy;
		material.anisotropy = saturate( material.anisotropy );
	}
	material.alphaT = mix( pow2( material.roughness ), 1.0, pow2( material.anisotropy ) );
	material.anisotropyT = tbn[ 0 ] * anisotropyV.x + tbn[ 1 ] * anisotropyV.y;
	material.anisotropyB = tbn[ 1 ] * anisotropyV.x - tbn[ 0 ] * anisotropyV.y;
#endif`,Ig=`struct PhysicalMaterial {
	vec3 diffuseColor;
	float roughness;
	vec3 specularColor;
	float specularF90;
	#ifdef USE_CLEARCOAT
		float clearcoat;
		float clearcoatRoughness;
		vec3 clearcoatF0;
		float clearcoatF90;
	#endif
	#ifdef USE_IRIDESCENCE
		float iridescence;
		float iridescenceIOR;
		float iridescenceThickness;
		vec3 iridescenceFresnel;
		vec3 iridescenceF0;
	#endif
	#ifdef USE_SHEEN
		vec3 sheenColor;
		float sheenRoughness;
	#endif
	#ifdef IOR
		float ior;
	#endif
	#ifdef USE_TRANSMISSION
		float transmission;
		float transmissionAlpha;
		float thickness;
		float attenuationDistance;
		vec3 attenuationColor;
	#endif
	#ifdef USE_ANISOTROPY
		float anisotropy;
		float alphaT;
		vec3 anisotropyT;
		vec3 anisotropyB;
	#endif
};
vec3 clearcoatSpecularDirect = vec3( 0.0 );
vec3 clearcoatSpecularIndirect = vec3( 0.0 );
vec3 sheenSpecularDirect = vec3( 0.0 );
vec3 sheenSpecularIndirect = vec3(0.0 );
vec3 Schlick_to_F0( const in vec3 f, const in float f90, const in float dotVH ) {
    float x = clamp( 1.0 - dotVH, 0.0, 1.0 );
    float x2 = x * x;
    float x5 = clamp( x * x2 * x2, 0.0, 0.9999 );
    return ( f - vec3( f90 ) * x5 ) / ( 1.0 - x5 );
}
float V_GGX_SmithCorrelated( const in float alpha, const in float dotNL, const in float dotNV ) {
	float a2 = pow2( alpha );
	float gv = dotNL * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNV ) );
	float gl = dotNV * sqrt( a2 + ( 1.0 - a2 ) * pow2( dotNL ) );
	return 0.5 / max( gv + gl, EPSILON );
}
float D_GGX( const in float alpha, const in float dotNH ) {
	float a2 = pow2( alpha );
	float denom = pow2( dotNH ) * ( a2 - 1.0 ) + 1.0;
	return RECIPROCAL_PI * a2 / pow2( denom );
}
#ifdef USE_ANISOTROPY
	float V_GGX_SmithCorrelated_Anisotropic( const in float alphaT, const in float alphaB, const in float dotTV, const in float dotBV, const in float dotTL, const in float dotBL, const in float dotNV, const in float dotNL ) {
		float gv = dotNL * length( vec3( alphaT * dotTV, alphaB * dotBV, dotNV ) );
		float gl = dotNV * length( vec3( alphaT * dotTL, alphaB * dotBL, dotNL ) );
		float v = 0.5 / ( gv + gl );
		return saturate(v);
	}
	float D_GGX_Anisotropic( const in float alphaT, const in float alphaB, const in float dotNH, const in float dotTH, const in float dotBH ) {
		float a2 = alphaT * alphaB;
		highp vec3 v = vec3( alphaB * dotTH, alphaT * dotBH, a2 * dotNH );
		highp float v2 = dot( v, v );
		float w2 = a2 / v2;
		return RECIPROCAL_PI * a2 * pow2 ( w2 );
	}
#endif
#ifdef USE_CLEARCOAT
	vec3 BRDF_GGX_Clearcoat( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material) {
		vec3 f0 = material.clearcoatF0;
		float f90 = material.clearcoatF90;
		float roughness = material.clearcoatRoughness;
		float alpha = pow2( roughness );
		vec3 halfDir = normalize( lightDir + viewDir );
		float dotNL = saturate( dot( normal, lightDir ) );
		float dotNV = saturate( dot( normal, viewDir ) );
		float dotNH = saturate( dot( normal, halfDir ) );
		float dotVH = saturate( dot( viewDir, halfDir ) );
		vec3 F = F_Schlick( f0, f90, dotVH );
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
		return F * ( V * D );
	}
#endif
vec3 BRDF_GGX( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, const in PhysicalMaterial material ) {
	vec3 f0 = material.specularColor;
	float f90 = material.specularF90;
	float roughness = material.roughness;
	float alpha = pow2( roughness );
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float dotVH = saturate( dot( viewDir, halfDir ) );
	vec3 F = F_Schlick( f0, f90, dotVH );
	#ifdef USE_IRIDESCENCE
		F = mix( F, material.iridescenceFresnel, material.iridescence );
	#endif
	#ifdef USE_ANISOTROPY
		float dotTL = dot( material.anisotropyT, lightDir );
		float dotTV = dot( material.anisotropyT, viewDir );
		float dotTH = dot( material.anisotropyT, halfDir );
		float dotBL = dot( material.anisotropyB, lightDir );
		float dotBV = dot( material.anisotropyB, viewDir );
		float dotBH = dot( material.anisotropyB, halfDir );
		float V = V_GGX_SmithCorrelated_Anisotropic( material.alphaT, alpha, dotTV, dotBV, dotTL, dotBL, dotNV, dotNL );
		float D = D_GGX_Anisotropic( material.alphaT, alpha, dotNH, dotTH, dotBH );
	#else
		float V = V_GGX_SmithCorrelated( alpha, dotNL, dotNV );
		float D = D_GGX( alpha, dotNH );
	#endif
	return F * ( V * D );
}
vec2 LTC_Uv( const in vec3 N, const in vec3 V, const in float roughness ) {
	const float LUT_SIZE = 64.0;
	const float LUT_SCALE = ( LUT_SIZE - 1.0 ) / LUT_SIZE;
	const float LUT_BIAS = 0.5 / LUT_SIZE;
	float dotNV = saturate( dot( N, V ) );
	vec2 uv = vec2( roughness, sqrt( 1.0 - dotNV ) );
	uv = uv * LUT_SCALE + LUT_BIAS;
	return uv;
}
float LTC_ClippedSphereFormFactor( const in vec3 f ) {
	float l = length( f );
	return max( ( l * l + f.z ) / ( l + 1.0 ), 0.0 );
}
vec3 LTC_EdgeVectorFormFactor( const in vec3 v1, const in vec3 v2 ) {
	float x = dot( v1, v2 );
	float y = abs( x );
	float a = 0.8543985 + ( 0.4965155 + 0.0145206 * y ) * y;
	float b = 3.4175940 + ( 4.1616724 + y ) * y;
	float v = a / b;
	float theta_sintheta = ( x > 0.0 ) ? v : 0.5 * inversesqrt( max( 1.0 - x * x, 1e-7 ) ) - v;
	return cross( v1, v2 ) * theta_sintheta;
}
vec3 LTC_Evaluate( const in vec3 N, const in vec3 V, const in vec3 P, const in mat3 mInv, const in vec3 rectCoords[ 4 ] ) {
	vec3 v1 = rectCoords[ 1 ] - rectCoords[ 0 ];
	vec3 v2 = rectCoords[ 3 ] - rectCoords[ 0 ];
	vec3 lightNormal = cross( v1, v2 );
	if( dot( lightNormal, P - rectCoords[ 0 ] ) < 0.0 ) return vec3( 0.0 );
	vec3 T1, T2;
	T1 = normalize( V - N * dot( V, N ) );
	T2 = - cross( N, T1 );
	mat3 mat = mInv * transposeMat3( mat3( T1, T2, N ) );
	vec3 coords[ 4 ];
	coords[ 0 ] = mat * ( rectCoords[ 0 ] - P );
	coords[ 1 ] = mat * ( rectCoords[ 1 ] - P );
	coords[ 2 ] = mat * ( rectCoords[ 2 ] - P );
	coords[ 3 ] = mat * ( rectCoords[ 3 ] - P );
	coords[ 0 ] = normalize( coords[ 0 ] );
	coords[ 1 ] = normalize( coords[ 1 ] );
	coords[ 2 ] = normalize( coords[ 2 ] );
	coords[ 3 ] = normalize( coords[ 3 ] );
	vec3 vectorFormFactor = vec3( 0.0 );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 0 ], coords[ 1 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 1 ], coords[ 2 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 2 ], coords[ 3 ] );
	vectorFormFactor += LTC_EdgeVectorFormFactor( coords[ 3 ], coords[ 0 ] );
	float result = LTC_ClippedSphereFormFactor( vectorFormFactor );
	return vec3( result );
}
#if defined( USE_SHEEN )
float D_Charlie( float roughness, float dotNH ) {
	float alpha = pow2( roughness );
	float invAlpha = 1.0 / alpha;
	float cos2h = dotNH * dotNH;
	float sin2h = max( 1.0 - cos2h, 0.0078125 );
	return ( 2.0 + invAlpha ) * pow( sin2h, invAlpha * 0.5 ) / ( 2.0 * PI );
}
float V_Neubelt( float dotNV, float dotNL ) {
	return saturate( 1.0 / ( 4.0 * ( dotNL + dotNV - dotNL * dotNV ) ) );
}
vec3 BRDF_Sheen( const in vec3 lightDir, const in vec3 viewDir, const in vec3 normal, vec3 sheenColor, const in float sheenRoughness ) {
	vec3 halfDir = normalize( lightDir + viewDir );
	float dotNL = saturate( dot( normal, lightDir ) );
	float dotNV = saturate( dot( normal, viewDir ) );
	float dotNH = saturate( dot( normal, halfDir ) );
	float D = D_Charlie( sheenRoughness, dotNH );
	float V = V_Neubelt( dotNV, dotNL );
	return sheenColor * ( D * V );
}
#endif
float IBLSheenBRDF( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	float r2 = roughness * roughness;
	float a = roughness < 0.25 ? -339.2 * r2 + 161.4 * roughness - 25.9 : -8.48 * r2 + 14.3 * roughness - 9.95;
	float b = roughness < 0.25 ? 44.0 * r2 - 23.7 * roughness + 3.26 : 1.97 * r2 - 3.27 * roughness + 0.72;
	float DG = exp( a * dotNV + b ) + ( roughness < 0.25 ? 0.0 : 0.1 * ( roughness - 0.25 ) );
	return saturate( DG * RECIPROCAL_PI );
}
vec2 DFGApprox( const in vec3 normal, const in vec3 viewDir, const in float roughness ) {
	float dotNV = saturate( dot( normal, viewDir ) );
	const vec4 c0 = vec4( - 1, - 0.0275, - 0.572, 0.022 );
	const vec4 c1 = vec4( 1, 0.0425, 1.04, - 0.04 );
	vec4 r = roughness * c0 + c1;
	float a004 = min( r.x * r.x, exp2( - 9.28 * dotNV ) ) * r.x + r.y;
	vec2 fab = vec2( - 1.04, 1.04 ) * a004 + r.zw;
	return fab;
}
vec3 EnvironmentBRDF( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness ) {
	vec2 fab = DFGApprox( normal, viewDir, roughness );
	return specularColor * fab.x + specularF90 * fab.y;
}
#ifdef USE_IRIDESCENCE
void computeMultiscatteringIridescence( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float iridescence, const in vec3 iridescenceF0, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#else
void computeMultiscattering( const in vec3 normal, const in vec3 viewDir, const in vec3 specularColor, const in float specularF90, const in float roughness, inout vec3 singleScatter, inout vec3 multiScatter ) {
#endif
	vec2 fab = DFGApprox( normal, viewDir, roughness );
	#ifdef USE_IRIDESCENCE
		vec3 Fr = mix( specularColor, iridescenceF0, iridescence );
	#else
		vec3 Fr = specularColor;
	#endif
	vec3 FssEss = Fr * fab.x + specularF90 * fab.y;
	float Ess = fab.x + fab.y;
	float Ems = 1.0 - Ess;
	vec3 Favg = Fr + ( 1.0 - Fr ) * 0.047619;	vec3 Fms = FssEss * Favg / ( 1.0 - Ems * Favg );
	singleScatter += FssEss;
	multiScatter += Fms * Ems;
}
#if NUM_RECT_AREA_LIGHTS > 0
	void RE_Direct_RectArea_Physical( const in RectAreaLight rectAreaLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
		vec3 normal = geometryNormal;
		vec3 viewDir = geometryViewDir;
		vec3 position = geometryPosition;
		vec3 lightPos = rectAreaLight.position;
		vec3 halfWidth = rectAreaLight.halfWidth;
		vec3 halfHeight = rectAreaLight.halfHeight;
		vec3 lightColor = rectAreaLight.color;
		float roughness = material.roughness;
		vec3 rectCoords[ 4 ];
		rectCoords[ 0 ] = lightPos + halfWidth - halfHeight;		rectCoords[ 1 ] = lightPos - halfWidth - halfHeight;
		rectCoords[ 2 ] = lightPos - halfWidth + halfHeight;
		rectCoords[ 3 ] = lightPos + halfWidth + halfHeight;
		vec2 uv = LTC_Uv( normal, viewDir, roughness );
		vec4 t1 = texture2D( ltc_1, uv );
		vec4 t2 = texture2D( ltc_2, uv );
		mat3 mInv = mat3(
			vec3( t1.x, 0, t1.y ),
			vec3(    0, 1,    0 ),
			vec3( t1.z, 0, t1.w )
		);
		vec3 fresnel = ( material.specularColor * t2.x + ( vec3( 1.0 ) - material.specularColor ) * t2.y );
		reflectedLight.directSpecular += lightColor * fresnel * LTC_Evaluate( normal, viewDir, position, mInv, rectCoords );
		reflectedLight.directDiffuse += lightColor * material.diffuseColor * LTC_Evaluate( normal, viewDir, position, mat3( 1.0 ), rectCoords );
	}
#endif
void RE_Direct_Physical( const in IncidentLight directLight, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	float dotNL = saturate( dot( geometryNormal, directLight.direction ) );
	vec3 irradiance = dotNL * directLight.color;
	#ifdef USE_CLEARCOAT
		float dotNLcc = saturate( dot( geometryClearcoatNormal, directLight.direction ) );
		vec3 ccIrradiance = dotNLcc * directLight.color;
		clearcoatSpecularDirect += ccIrradiance * BRDF_GGX_Clearcoat( directLight.direction, geometryViewDir, geometryClearcoatNormal, material );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularDirect += irradiance * BRDF_Sheen( directLight.direction, geometryViewDir, geometryNormal, material.sheenColor, material.sheenRoughness );
	#endif
	reflectedLight.directSpecular += irradiance * BRDF_GGX( directLight.direction, geometryViewDir, geometryNormal, material );
	reflectedLight.directDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectDiffuse_Physical( const in vec3 irradiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight ) {
	reflectedLight.indirectDiffuse += irradiance * BRDF_Lambert( material.diffuseColor );
}
void RE_IndirectSpecular_Physical( const in vec3 radiance, const in vec3 irradiance, const in vec3 clearcoatRadiance, const in vec3 geometryPosition, const in vec3 geometryNormal, const in vec3 geometryViewDir, const in vec3 geometryClearcoatNormal, const in PhysicalMaterial material, inout ReflectedLight reflectedLight) {
	#ifdef USE_CLEARCOAT
		clearcoatSpecularIndirect += clearcoatRadiance * EnvironmentBRDF( geometryClearcoatNormal, geometryViewDir, material.clearcoatF0, material.clearcoatF90, material.clearcoatRoughness );
	#endif
	#ifdef USE_SHEEN
		sheenSpecularIndirect += irradiance * material.sheenColor * IBLSheenBRDF( geometryNormal, geometryViewDir, material.sheenRoughness );
	#endif
	vec3 singleScattering = vec3( 0.0 );
	vec3 multiScattering = vec3( 0.0 );
	vec3 cosineWeightedIrradiance = irradiance * RECIPROCAL_PI;
	#ifdef USE_IRIDESCENCE
		computeMultiscatteringIridescence( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.iridescence, material.iridescenceFresnel, material.roughness, singleScattering, multiScattering );
	#else
		computeMultiscattering( geometryNormal, geometryViewDir, material.specularColor, material.specularF90, material.roughness, singleScattering, multiScattering );
	#endif
	vec3 totalScattering = singleScattering + multiScattering;
	vec3 diffuse = material.diffuseColor * ( 1.0 - max( max( totalScattering.r, totalScattering.g ), totalScattering.b ) );
	reflectedLight.indirectSpecular += radiance * singleScattering;
	reflectedLight.indirectSpecular += multiScattering * cosineWeightedIrradiance;
	reflectedLight.indirectDiffuse += diffuse * cosineWeightedIrradiance;
}
#define RE_Direct				RE_Direct_Physical
#define RE_Direct_RectArea		RE_Direct_RectArea_Physical
#define RE_IndirectDiffuse		RE_IndirectDiffuse_Physical
#define RE_IndirectSpecular		RE_IndirectSpecular_Physical
float computeSpecularOcclusion( const in float dotNV, const in float ambientOcclusion, const in float roughness ) {
	return saturate( pow( dotNV + ambientOcclusion, exp2( - 16.0 * roughness - 1.0 ) ) - 1.0 + ambientOcclusion );
}`,Ng=`
vec3 geometryPosition = - vViewPosition;
vec3 geometryNormal = normal;
vec3 geometryViewDir = ( isOrthographic ) ? vec3( 0, 0, 1 ) : normalize( vViewPosition );
vec3 geometryClearcoatNormal = vec3( 0.0 );
#ifdef USE_CLEARCOAT
	geometryClearcoatNormal = clearcoatNormal;
#endif
#ifdef USE_IRIDESCENCE
	float dotNVi = saturate( dot( normal, geometryViewDir ) );
	if ( material.iridescenceThickness == 0.0 ) {
		material.iridescence = 0.0;
	} else {
		material.iridescence = saturate( material.iridescence );
	}
	if ( material.iridescence > 0.0 ) {
		material.iridescenceFresnel = evalIridescence( 1.0, material.iridescenceIOR, dotNVi, material.iridescenceThickness, material.specularColor );
		material.iridescenceF0 = Schlick_to_F0( material.iridescenceFresnel, 1.0, dotNVi );
	}
#endif
IncidentLight directLight;
#if ( NUM_POINT_LIGHTS > 0 ) && defined( RE_Direct )
	PointLight pointLight;
	#if defined( USE_SHADOWMAP ) && NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHTS; i ++ ) {
		pointLight = pointLights[ i ];
		getPointLightInfo( pointLight, geometryPosition, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_POINT_LIGHT_SHADOWS )
		pointLightShadow = pointLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getPointShadow( pointShadowMap[ i ], pointLightShadow.shadowMapSize, pointLightShadow.shadowBias, pointLightShadow.shadowRadius, vPointShadowCoord[ i ], pointLightShadow.shadowCameraNear, pointLightShadow.shadowCameraFar ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_SPOT_LIGHTS > 0 ) && defined( RE_Direct )
	SpotLight spotLight;
	vec4 spotColor;
	vec3 spotLightCoord;
	bool inSpotLightMap;
	#if defined( USE_SHADOWMAP ) && NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHTS; i ++ ) {
		spotLight = spotLights[ i ];
		getSpotLightInfo( spotLight, geometryPosition, directLight );
		#if ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#define SPOT_LIGHT_MAP_INDEX UNROLLED_LOOP_INDEX
		#elif ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		#define SPOT_LIGHT_MAP_INDEX NUM_SPOT_LIGHT_MAPS
		#else
		#define SPOT_LIGHT_MAP_INDEX ( UNROLLED_LOOP_INDEX - NUM_SPOT_LIGHT_SHADOWS + NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS )
		#endif
		#if ( SPOT_LIGHT_MAP_INDEX < NUM_SPOT_LIGHT_MAPS )
			spotLightCoord = vSpotLightCoord[ i ].xyz / vSpotLightCoord[ i ].w;
			inSpotLightMap = all( lessThan( abs( spotLightCoord * 2. - 1. ), vec3( 1.0 ) ) );
			spotColor = texture2D( spotLightMap[ SPOT_LIGHT_MAP_INDEX ], spotLightCoord.xy );
			directLight.color = inSpotLightMap ? directLight.color * spotColor.rgb : directLight.color;
		#endif
		#undef SPOT_LIGHT_MAP_INDEX
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
		spotLightShadow = spotLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( spotShadowMap[ i ], spotLightShadow.shadowMapSize, spotLightShadow.shadowBias, spotLightShadow.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_DIR_LIGHTS > 0 ) && defined( RE_Direct )
	DirectionalLight directionalLight;
	#if defined( USE_SHADOWMAP ) && NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLightShadow;
	#endif
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHTS; i ++ ) {
		directionalLight = directionalLights[ i ];
		getDirectionalLightInfo( directionalLight, directLight );
		#if defined( USE_SHADOWMAP ) && ( UNROLLED_LOOP_INDEX < NUM_DIR_LIGHT_SHADOWS )
		directionalLightShadow = directionalLightShadows[ i ];
		directLight.color *= ( directLight.visible && receiveShadow ) ? getShadow( directionalShadowMap[ i ], directionalLightShadow.shadowMapSize, directionalLightShadow.shadowBias, directionalLightShadow.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
		#endif
		RE_Direct( directLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if ( NUM_RECT_AREA_LIGHTS > 0 ) && defined( RE_Direct_RectArea )
	RectAreaLight rectAreaLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_RECT_AREA_LIGHTS; i ++ ) {
		rectAreaLight = rectAreaLights[ i ];
		RE_Direct_RectArea( rectAreaLight, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
	}
	#pragma unroll_loop_end
#endif
#if defined( RE_IndirectDiffuse )
	vec3 iblIrradiance = vec3( 0.0 );
	vec3 irradiance = getAmbientLightIrradiance( ambientLightColor );
	#if defined( USE_LIGHT_PROBES )
		irradiance += getLightProbeIrradiance( lightProbe, geometryNormal );
	#endif
	#if ( NUM_HEMI_LIGHTS > 0 )
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_HEMI_LIGHTS; i ++ ) {
			irradiance += getHemisphereLightIrradiance( hemisphereLights[ i ], geometryNormal );
		}
		#pragma unroll_loop_end
	#endif
#endif
#if defined( RE_IndirectSpecular )
	vec3 radiance = vec3( 0.0 );
	vec3 clearcoatRadiance = vec3( 0.0 );
#endif`,kg=`#if defined( RE_IndirectDiffuse )
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		vec3 lightMapIrradiance = lightMapTexel.rgb * lightMapIntensity;
		irradiance += lightMapIrradiance;
	#endif
	#if defined( USE_ENVMAP ) && defined( STANDARD ) && defined( ENVMAP_TYPE_CUBE_UV )
		iblIrradiance += getIBLIrradiance( geometryNormal );
	#endif
#endif
#if defined( USE_ENVMAP ) && defined( RE_IndirectSpecular )
	#ifdef USE_ANISOTROPY
		radiance += getIBLAnisotropyRadiance( geometryViewDir, geometryNormal, material.roughness, material.anisotropyB, material.anisotropy );
	#else
		radiance += getIBLRadiance( geometryViewDir, geometryNormal, material.roughness );
	#endif
	#ifdef USE_CLEARCOAT
		clearcoatRadiance += getIBLRadiance( geometryViewDir, geometryClearcoatNormal, material.clearcoatRoughness );
	#endif
#endif`,Fg=`#if defined( RE_IndirectDiffuse )
	RE_IndirectDiffuse( irradiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif
#if defined( RE_IndirectSpecular )
	RE_IndirectSpecular( radiance, iblIrradiance, clearcoatRadiance, geometryPosition, geometryNormal, geometryViewDir, geometryClearcoatNormal, material, reflectedLight );
#endif`,Og=`#if defined( USE_LOGDEPTHBUF ) && defined( USE_LOGDEPTHBUF_EXT )
	gl_FragDepthEXT = vIsPerspective == 0.0 ? gl_FragCoord.z : log2( vFragDepth ) * logDepthBufFC * 0.5;
#endif`,Bg=`#if defined( USE_LOGDEPTHBUF ) && defined( USE_LOGDEPTHBUF_EXT )
	uniform float logDepthBufFC;
	varying float vFragDepth;
	varying float vIsPerspective;
#endif`,zg=`#ifdef USE_LOGDEPTHBUF
	#ifdef USE_LOGDEPTHBUF_EXT
		varying float vFragDepth;
		varying float vIsPerspective;
	#else
		uniform float logDepthBufFC;
	#endif
#endif`,Hg=`#ifdef USE_LOGDEPTHBUF
	#ifdef USE_LOGDEPTHBUF_EXT
		vFragDepth = 1.0 + gl_Position.w;
		vIsPerspective = float( isPerspectiveMatrix( projectionMatrix ) );
	#else
		if ( isPerspectiveMatrix( projectionMatrix ) ) {
			gl_Position.z = log2( max( EPSILON, gl_Position.w + 1.0 ) ) * logDepthBufFC - 1.0;
			gl_Position.z *= gl_Position.w;
		}
	#endif
#endif`,Vg=`#ifdef USE_MAP
	vec4 sampledDiffuseColor = texture2D( map, vMapUv );
	#ifdef DECODE_VIDEO_TEXTURE
		sampledDiffuseColor = vec4( mix( pow( sampledDiffuseColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), sampledDiffuseColor.rgb * 0.0773993808, vec3( lessThanEqual( sampledDiffuseColor.rgb, vec3( 0.04045 ) ) ) ), sampledDiffuseColor.w );
	
	#endif
	diffuseColor *= sampledDiffuseColor;
#endif`,Gg=`#ifdef USE_MAP
	uniform sampler2D map;
#endif`,Wg=`#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
	#if defined( USE_POINTS_UV )
		vec2 uv = vUv;
	#else
		vec2 uv = ( uvTransform * vec3( gl_PointCoord.x, 1.0 - gl_PointCoord.y, 1 ) ).xy;
	#endif
#endif
#ifdef USE_MAP
	diffuseColor *= texture2D( map, uv );
#endif
#ifdef USE_ALPHAMAP
	diffuseColor.a *= texture2D( alphaMap, uv ).g;
#endif`,$g=`#if defined( USE_POINTS_UV )
	varying vec2 vUv;
#else
	#if defined( USE_MAP ) || defined( USE_ALPHAMAP )
		uniform mat3 uvTransform;
	#endif
#endif
#ifdef USE_MAP
	uniform sampler2D map;
#endif
#ifdef USE_ALPHAMAP
	uniform sampler2D alphaMap;
#endif`,Xg=`float metalnessFactor = metalness;
#ifdef USE_METALNESSMAP
	vec4 texelMetalness = texture2D( metalnessMap, vMetalnessMapUv );
	metalnessFactor *= texelMetalness.b;
#endif`,qg=`#ifdef USE_METALNESSMAP
	uniform sampler2D metalnessMap;
#endif`,jg=`#if defined( USE_MORPHCOLORS ) && defined( MORPHTARGETS_TEXTURE )
	vColor *= morphTargetBaseInfluence;
	for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
		#if defined( USE_COLOR_ALPHA )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ) * morphTargetInfluences[ i ];
		#elif defined( USE_COLOR )
			if ( morphTargetInfluences[ i ] != 0.0 ) vColor += getMorph( gl_VertexID, i, 2 ).rgb * morphTargetInfluences[ i ];
		#endif
	}
#endif`,Yg=`#ifdef USE_MORPHNORMALS
	objectNormal *= morphTargetBaseInfluence;
	#ifdef MORPHTARGETS_TEXTURE
		for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
			if ( morphTargetInfluences[ i ] != 0.0 ) objectNormal += getMorph( gl_VertexID, i, 1 ).xyz * morphTargetInfluences[ i ];
		}
	#else
		objectNormal += morphNormal0 * morphTargetInfluences[ 0 ];
		objectNormal += morphNormal1 * morphTargetInfluences[ 1 ];
		objectNormal += morphNormal2 * morphTargetInfluences[ 2 ];
		objectNormal += morphNormal3 * morphTargetInfluences[ 3 ];
	#endif
#endif`,Kg=`#ifdef USE_MORPHTARGETS
	uniform float morphTargetBaseInfluence;
	#ifdef MORPHTARGETS_TEXTURE
		uniform float morphTargetInfluences[ MORPHTARGETS_COUNT ];
		uniform sampler2DArray morphTargetsTexture;
		uniform ivec2 morphTargetsTextureSize;
		vec4 getMorph( const in int vertexIndex, const in int morphTargetIndex, const in int offset ) {
			int texelIndex = vertexIndex * MORPHTARGETS_TEXTURE_STRIDE + offset;
			int y = texelIndex / morphTargetsTextureSize.x;
			int x = texelIndex - y * morphTargetsTextureSize.x;
			ivec3 morphUV = ivec3( x, y, morphTargetIndex );
			return texelFetch( morphTargetsTexture, morphUV, 0 );
		}
	#else
		#ifndef USE_MORPHNORMALS
			uniform float morphTargetInfluences[ 8 ];
		#else
			uniform float morphTargetInfluences[ 4 ];
		#endif
	#endif
#endif`,Zg=`#ifdef USE_MORPHTARGETS
	transformed *= morphTargetBaseInfluence;
	#ifdef MORPHTARGETS_TEXTURE
		for ( int i = 0; i < MORPHTARGETS_COUNT; i ++ ) {
			if ( morphTargetInfluences[ i ] != 0.0 ) transformed += getMorph( gl_VertexID, i, 0 ).xyz * morphTargetInfluences[ i ];
		}
	#else
		transformed += morphTarget0 * morphTargetInfluences[ 0 ];
		transformed += morphTarget1 * morphTargetInfluences[ 1 ];
		transformed += morphTarget2 * morphTargetInfluences[ 2 ];
		transformed += morphTarget3 * morphTargetInfluences[ 3 ];
		#ifndef USE_MORPHNORMALS
			transformed += morphTarget4 * morphTargetInfluences[ 4 ];
			transformed += morphTarget5 * morphTargetInfluences[ 5 ];
			transformed += morphTarget6 * morphTargetInfluences[ 6 ];
			transformed += morphTarget7 * morphTargetInfluences[ 7 ];
		#endif
	#endif
#endif`,Jg=`float faceDirection = gl_FrontFacing ? 1.0 : - 1.0;
#ifdef FLAT_SHADED
	vec3 fdx = dFdx( vViewPosition );
	vec3 fdy = dFdy( vViewPosition );
	vec3 normal = normalize( cross( fdx, fdy ) );
#else
	vec3 normal = normalize( vNormal );
	#ifdef DOUBLE_SIDED
		normal *= faceDirection;
	#endif
#endif
#if defined( USE_NORMALMAP_TANGENTSPACE ) || defined( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY )
	#ifdef USE_TANGENT
		mat3 tbn = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn = getTangentFrame( - vViewPosition, normal,
		#if defined( USE_NORMALMAP )
			vNormalMapUv
		#elif defined( USE_CLEARCOAT_NORMALMAP )
			vClearcoatNormalMapUv
		#else
			vUv
		#endif
		);
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn[0] *= faceDirection;
		tbn[1] *= faceDirection;
	#endif
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	#ifdef USE_TANGENT
		mat3 tbn2 = mat3( normalize( vTangent ), normalize( vBitangent ), normal );
	#else
		mat3 tbn2 = getTangentFrame( - vViewPosition, normal, vClearcoatNormalMapUv );
	#endif
	#if defined( DOUBLE_SIDED ) && ! defined( FLAT_SHADED )
		tbn2[0] *= faceDirection;
		tbn2[1] *= faceDirection;
	#endif
#endif
vec3 nonPerturbedNormal = normal;`,Qg=`#ifdef USE_NORMALMAP_OBJECTSPACE
	normal = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	#ifdef FLIP_SIDED
		normal = - normal;
	#endif
	#ifdef DOUBLE_SIDED
		normal = normal * faceDirection;
	#endif
	normal = normalize( normalMatrix * normal );
#elif defined( USE_NORMALMAP_TANGENTSPACE )
	vec3 mapN = texture2D( normalMap, vNormalMapUv ).xyz * 2.0 - 1.0;
	mapN.xy *= normalScale;
	normal = normalize( tbn * mapN );
#elif defined( USE_BUMPMAP )
	normal = perturbNormalArb( - vViewPosition, normal, dHdxy_fwd(), faceDirection );
#endif`,tv=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,ev=`#ifndef FLAT_SHADED
	varying vec3 vNormal;
	#ifdef USE_TANGENT
		varying vec3 vTangent;
		varying vec3 vBitangent;
	#endif
#endif`,nv=`#ifndef FLAT_SHADED
	vNormal = normalize( transformedNormal );
	#ifdef USE_TANGENT
		vTangent = normalize( transformedTangent );
		vBitangent = normalize( cross( vNormal, vTangent ) * tangent.w );
	#endif
#endif`,iv=`#ifdef USE_NORMALMAP
	uniform sampler2D normalMap;
	uniform vec2 normalScale;
#endif
#ifdef USE_NORMALMAP_OBJECTSPACE
	uniform mat3 normalMatrix;
#endif
#if ! defined ( USE_TANGENT ) && ( defined ( USE_NORMALMAP_TANGENTSPACE ) || defined ( USE_CLEARCOAT_NORMALMAP ) || defined( USE_ANISOTROPY ) )
	mat3 getTangentFrame( vec3 eye_pos, vec3 surf_norm, vec2 uv ) {
		vec3 q0 = dFdx( eye_pos.xyz );
		vec3 q1 = dFdy( eye_pos.xyz );
		vec2 st0 = dFdx( uv.st );
		vec2 st1 = dFdy( uv.st );
		vec3 N = surf_norm;
		vec3 q1perp = cross( q1, N );
		vec3 q0perp = cross( N, q0 );
		vec3 T = q1perp * st0.x + q0perp * st1.x;
		vec3 B = q1perp * st0.y + q0perp * st1.y;
		float det = max( dot( T, T ), dot( B, B ) );
		float scale = ( det == 0.0 ) ? 0.0 : inversesqrt( det );
		return mat3( T * scale, B * scale, N );
	}
#endif`,sv=`#ifdef USE_CLEARCOAT
	vec3 clearcoatNormal = nonPerturbedNormal;
#endif`,rv=`#ifdef USE_CLEARCOAT_NORMALMAP
	vec3 clearcoatMapN = texture2D( clearcoatNormalMap, vClearcoatNormalMapUv ).xyz * 2.0 - 1.0;
	clearcoatMapN.xy *= clearcoatNormalScale;
	clearcoatNormal = normalize( tbn2 * clearcoatMapN );
#endif`,ov=`#ifdef USE_CLEARCOATMAP
	uniform sampler2D clearcoatMap;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform sampler2D clearcoatNormalMap;
	uniform vec2 clearcoatNormalScale;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform sampler2D clearcoatRoughnessMap;
#endif`,av=`#ifdef USE_IRIDESCENCEMAP
	uniform sampler2D iridescenceMap;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform sampler2D iridescenceThicknessMap;
#endif`,lv=`#ifdef OPAQUE
diffuseColor.a = 1.0;
#endif
#ifdef USE_TRANSMISSION
diffuseColor.a *= material.transmissionAlpha;
#endif
gl_FragColor = vec4( outgoingLight, diffuseColor.a );`,cv=`vec3 packNormalToRGB( const in vec3 normal ) {
	return normalize( normal ) * 0.5 + 0.5;
}
vec3 unpackRGBToNormal( const in vec3 rgb ) {
	return 2.0 * rgb.xyz - 1.0;
}
const float PackUpscale = 256. / 255.;const float UnpackDownscale = 255. / 256.;
const vec3 PackFactors = vec3( 256. * 256. * 256., 256. * 256., 256. );
const vec4 UnpackFactors = UnpackDownscale / vec4( PackFactors, 1. );
const float ShiftRight8 = 1. / 256.;
vec4 packDepthToRGBA( const in float v ) {
	vec4 r = vec4( fract( v * PackFactors ), v );
	r.yzw -= r.xyz * ShiftRight8;	return r * PackUpscale;
}
float unpackRGBAToDepth( const in vec4 v ) {
	return dot( v, UnpackFactors );
}
vec2 packDepthToRG( in highp float v ) {
	return packDepthToRGBA( v ).yx;
}
float unpackRGToDepth( const in highp vec2 v ) {
	return unpackRGBAToDepth( vec4( v.xy, 0.0, 0.0 ) );
}
vec4 pack2HalfToRGBA( vec2 v ) {
	vec4 r = vec4( v.x, fract( v.x * 255.0 ), v.y, fract( v.y * 255.0 ) );
	return vec4( r.x - r.y / 255.0, r.y, r.z - r.w / 255.0, r.w );
}
vec2 unpackRGBATo2Half( vec4 v ) {
	return vec2( v.x + ( v.y / 255.0 ), v.z + ( v.w / 255.0 ) );
}
float viewZToOrthographicDepth( const in float viewZ, const in float near, const in float far ) {
	return ( viewZ + near ) / ( near - far );
}
float orthographicDepthToViewZ( const in float depth, const in float near, const in float far ) {
	return depth * ( near - far ) - near;
}
float viewZToPerspectiveDepth( const in float viewZ, const in float near, const in float far ) {
	return ( ( near + viewZ ) * far ) / ( ( far - near ) * viewZ );
}
float perspectiveDepthToViewZ( const in float depth, const in float near, const in float far ) {
	return ( near * far ) / ( ( far - near ) * depth - far );
}`,uv=`#ifdef PREMULTIPLIED_ALPHA
	gl_FragColor.rgb *= gl_FragColor.a;
#endif`,dv=`vec4 mvPosition = vec4( transformed, 1.0 );
#ifdef USE_BATCHING
	mvPosition = batchingMatrix * mvPosition;
#endif
#ifdef USE_INSTANCING
	mvPosition = instanceMatrix * mvPosition;
#endif
mvPosition = modelViewMatrix * mvPosition;
gl_Position = projectionMatrix * mvPosition;`,hv=`#ifdef DITHERING
	gl_FragColor.rgb = dithering( gl_FragColor.rgb );
#endif`,fv=`#ifdef DITHERING
	vec3 dithering( vec3 color ) {
		float grid_position = rand( gl_FragCoord.xy );
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );
		return color + dither_shift_RGB;
	}
#endif`,pv=`float roughnessFactor = roughness;
#ifdef USE_ROUGHNESSMAP
	vec4 texelRoughness = texture2D( roughnessMap, vRoughnessMapUv );
	roughnessFactor *= texelRoughness.g;
#endif`,mv=`#ifdef USE_ROUGHNESSMAP
	uniform sampler2D roughnessMap;
#endif`,_v=`#if NUM_SPOT_LIGHT_COORDS > 0
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#if NUM_SPOT_LIGHT_MAPS > 0
	uniform sampler2D spotLightMap[ NUM_SPOT_LIGHT_MAPS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform sampler2D directionalShadowMap[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		uniform sampler2D spotShadowMap[ NUM_SPOT_LIGHT_SHADOWS ];
		struct SpotLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform sampler2D pointShadowMap[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
	float texture2DCompare( sampler2D depths, vec2 uv, float compare ) {
		return step( compare, unpackRGBAToDepth( texture2D( depths, uv ) ) );
	}
	vec2 texture2DDistribution( sampler2D shadow, vec2 uv ) {
		return unpackRGBATo2Half( texture2D( shadow, uv ) );
	}
	float VSMShadow (sampler2D shadow, vec2 uv, float compare ){
		float occlusion = 1.0;
		vec2 distribution = texture2DDistribution( shadow, uv );
		float hard_shadow = step( compare , distribution.x );
		if (hard_shadow != 1.0 ) {
			float distance = compare - distribution.x ;
			float variance = max( 0.00000, distribution.y * distribution.y );
			float softness_probability = variance / (variance + distance * distance );			softness_probability = clamp( ( softness_probability - 0.3 ) / ( 0.95 - 0.3 ), 0.0, 1.0 );			occlusion = clamp( max( hard_shadow, softness_probability ), 0.0, 1.0 );
		}
		return occlusion;
	}
	float getShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowBias, float shadowRadius, vec4 shadowCoord ) {
		float shadow = 1.0;
		shadowCoord.xyz /= shadowCoord.w;
		shadowCoord.z += shadowBias;
		bool inFrustum = shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0 && shadowCoord.y >= 0.0 && shadowCoord.y <= 1.0;
		bool frustumTest = inFrustum && shadowCoord.z <= 1.0;
		if ( frustumTest ) {
		#if defined( SHADOWMAP_TYPE_PCF )
			vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
			float dx0 = - texelSize.x * shadowRadius;
			float dy0 = - texelSize.y * shadowRadius;
			float dx1 = + texelSize.x * shadowRadius;
			float dy1 = + texelSize.y * shadowRadius;
			float dx2 = dx0 / 2.0;
			float dy2 = dy0 / 2.0;
			float dx3 = dx1 / 2.0;
			float dy3 = dy1 / 2.0;
			shadow = (
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, dy0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, dy2 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy, shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx2, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx3, dy3 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx0, dy1 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( 0.0, dy1 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, shadowCoord.xy + vec2( dx1, dy1 ), shadowCoord.z )
			) * ( 1.0 / 17.0 );
		#elif defined( SHADOWMAP_TYPE_PCF_SOFT )
			vec2 texelSize = vec2( 1.0 ) / shadowMapSize;
			float dx = texelSize.x;
			float dy = texelSize.y;
			vec2 uv = shadowCoord.xy;
			vec2 f = fract( uv * shadowMapSize + 0.5 );
			uv -= f * texelSize;
			shadow = (
				texture2DCompare( shadowMap, uv, shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + vec2( dx, 0.0 ), shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + vec2( 0.0, dy ), shadowCoord.z ) +
				texture2DCompare( shadowMap, uv + texelSize, shadowCoord.z ) +
				mix( texture2DCompare( shadowMap, uv + vec2( -dx, 0.0 ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, 0.0 ), shadowCoord.z ),
					 f.x ) +
				mix( texture2DCompare( shadowMap, uv + vec2( -dx, dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, dy ), shadowCoord.z ),
					 f.x ) +
				mix( texture2DCompare( shadowMap, uv + vec2( 0.0, -dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( 0.0, 2.0 * dy ), shadowCoord.z ),
					 f.y ) +
				mix( texture2DCompare( shadowMap, uv + vec2( dx, -dy ), shadowCoord.z ),
					 texture2DCompare( shadowMap, uv + vec2( dx, 2.0 * dy ), shadowCoord.z ),
					 f.y ) +
				mix( mix( texture2DCompare( shadowMap, uv + vec2( -dx, -dy ), shadowCoord.z ),
						  texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, -dy ), shadowCoord.z ),
						  f.x ),
					 mix( texture2DCompare( shadowMap, uv + vec2( -dx, 2.0 * dy ), shadowCoord.z ),
						  texture2DCompare( shadowMap, uv + vec2( 2.0 * dx, 2.0 * dy ), shadowCoord.z ),
						  f.x ),
					 f.y )
			) * ( 1.0 / 9.0 );
		#elif defined( SHADOWMAP_TYPE_VSM )
			shadow = VSMShadow( shadowMap, shadowCoord.xy, shadowCoord.z );
		#else
			shadow = texture2DCompare( shadowMap, shadowCoord.xy, shadowCoord.z );
		#endif
		}
		return shadow;
	}
	vec2 cubeToUV( vec3 v, float texelSizeY ) {
		vec3 absV = abs( v );
		float scaleToCube = 1.0 / max( absV.x, max( absV.y, absV.z ) );
		absV *= scaleToCube;
		v *= scaleToCube * ( 1.0 - 2.0 * texelSizeY );
		vec2 planar = v.xy;
		float almostATexel = 1.5 * texelSizeY;
		float almostOne = 1.0 - almostATexel;
		if ( absV.z >= almostOne ) {
			if ( v.z > 0.0 )
				planar.x = 4.0 - v.x;
		} else if ( absV.x >= almostOne ) {
			float signX = sign( v.x );
			planar.x = v.z * signX + 2.0 * signX;
		} else if ( absV.y >= almostOne ) {
			float signY = sign( v.y );
			planar.x = v.x + 2.0 * signY + 2.0;
			planar.y = v.z * signY - 2.0;
		}
		return vec2( 0.125, 0.25 ) * planar + vec2( 0.375, 0.75 );
	}
	float getPointShadow( sampler2D shadowMap, vec2 shadowMapSize, float shadowBias, float shadowRadius, vec4 shadowCoord, float shadowCameraNear, float shadowCameraFar ) {
		vec2 texelSize = vec2( 1.0 ) / ( shadowMapSize * vec2( 4.0, 2.0 ) );
		vec3 lightToPosition = shadowCoord.xyz;
		float dp = ( length( lightToPosition ) - shadowCameraNear ) / ( shadowCameraFar - shadowCameraNear );		dp += shadowBias;
		vec3 bd3D = normalize( lightToPosition );
		#if defined( SHADOWMAP_TYPE_PCF ) || defined( SHADOWMAP_TYPE_PCF_SOFT ) || defined( SHADOWMAP_TYPE_VSM )
			vec2 offset = vec2( - 1, 1 ) * shadowRadius * texelSize.y;
			return (
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xyy, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yyy, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xyx, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yyx, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xxy, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yxy, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.xxx, texelSize.y ), dp ) +
				texture2DCompare( shadowMap, cubeToUV( bd3D + offset.yxx, texelSize.y ), dp )
			) * ( 1.0 / 9.0 );
		#else
			return texture2DCompare( shadowMap, cubeToUV( bd3D, texelSize.y ), dp );
		#endif
	}
#endif`,gv=`#if NUM_SPOT_LIGHT_COORDS > 0
	uniform mat4 spotLightMatrix[ NUM_SPOT_LIGHT_COORDS ];
	varying vec4 vSpotLightCoord[ NUM_SPOT_LIGHT_COORDS ];
#endif
#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
		uniform mat4 directionalShadowMatrix[ NUM_DIR_LIGHT_SHADOWS ];
		varying vec4 vDirectionalShadowCoord[ NUM_DIR_LIGHT_SHADOWS ];
		struct DirectionalLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform DirectionalLightShadow directionalLightShadows[ NUM_DIR_LIGHT_SHADOWS ];
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
		struct SpotLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
		};
		uniform SpotLightShadow spotLightShadows[ NUM_SPOT_LIGHT_SHADOWS ];
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		uniform mat4 pointShadowMatrix[ NUM_POINT_LIGHT_SHADOWS ];
		varying vec4 vPointShadowCoord[ NUM_POINT_LIGHT_SHADOWS ];
		struct PointLightShadow {
			float shadowBias;
			float shadowNormalBias;
			float shadowRadius;
			vec2 shadowMapSize;
			float shadowCameraNear;
			float shadowCameraFar;
		};
		uniform PointLightShadow pointLightShadows[ NUM_POINT_LIGHT_SHADOWS ];
	#endif
#endif`,vv=`#if ( defined( USE_SHADOWMAP ) && ( NUM_DIR_LIGHT_SHADOWS > 0 || NUM_POINT_LIGHT_SHADOWS > 0 ) ) || ( NUM_SPOT_LIGHT_COORDS > 0 )
	vec3 shadowWorldNormal = inverseTransformDirection( transformedNormal, viewMatrix );
	vec4 shadowWorldPosition;
#endif
#if defined( USE_SHADOWMAP )
	#if NUM_DIR_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * directionalLightShadows[ i ].shadowNormalBias, 0 );
			vDirectionalShadowCoord[ i ] = directionalShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
		#pragma unroll_loop_start
		for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
			shadowWorldPosition = worldPosition + vec4( shadowWorldNormal * pointLightShadows[ i ].shadowNormalBias, 0 );
			vPointShadowCoord[ i ] = pointShadowMatrix[ i ] * shadowWorldPosition;
		}
		#pragma unroll_loop_end
	#endif
#endif
#if NUM_SPOT_LIGHT_COORDS > 0
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_COORDS; i ++ ) {
		shadowWorldPosition = worldPosition;
		#if ( defined( USE_SHADOWMAP ) && UNROLLED_LOOP_INDEX < NUM_SPOT_LIGHT_SHADOWS )
			shadowWorldPosition.xyz += shadowWorldNormal * spotLightShadows[ i ].shadowNormalBias;
		#endif
		vSpotLightCoord[ i ] = spotLightMatrix[ i ] * shadowWorldPosition;
	}
	#pragma unroll_loop_end
#endif`,xv=`float getShadowMask() {
	float shadow = 1.0;
	#ifdef USE_SHADOWMAP
	#if NUM_DIR_LIGHT_SHADOWS > 0
	DirectionalLightShadow directionalLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_DIR_LIGHT_SHADOWS; i ++ ) {
		directionalLight = directionalLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( directionalShadowMap[ i ], directionalLight.shadowMapSize, directionalLight.shadowBias, directionalLight.shadowRadius, vDirectionalShadowCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_SPOT_LIGHT_SHADOWS > 0
	SpotLightShadow spotLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_SPOT_LIGHT_SHADOWS; i ++ ) {
		spotLight = spotLightShadows[ i ];
		shadow *= receiveShadow ? getShadow( spotShadowMap[ i ], spotLight.shadowMapSize, spotLight.shadowBias, spotLight.shadowRadius, vSpotLightCoord[ i ] ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#if NUM_POINT_LIGHT_SHADOWS > 0
	PointLightShadow pointLight;
	#pragma unroll_loop_start
	for ( int i = 0; i < NUM_POINT_LIGHT_SHADOWS; i ++ ) {
		pointLight = pointLightShadows[ i ];
		shadow *= receiveShadow ? getPointShadow( pointShadowMap[ i ], pointLight.shadowMapSize, pointLight.shadowBias, pointLight.shadowRadius, vPointShadowCoord[ i ], pointLight.shadowCameraNear, pointLight.shadowCameraFar ) : 1.0;
	}
	#pragma unroll_loop_end
	#endif
	#endif
	return shadow;
}`,yv=`#ifdef USE_SKINNING
	mat4 boneMatX = getBoneMatrix( skinIndex.x );
	mat4 boneMatY = getBoneMatrix( skinIndex.y );
	mat4 boneMatZ = getBoneMatrix( skinIndex.z );
	mat4 boneMatW = getBoneMatrix( skinIndex.w );
#endif`,bv=`#ifdef USE_SKINNING
	uniform mat4 bindMatrix;
	uniform mat4 bindMatrixInverse;
	uniform highp sampler2D boneTexture;
	mat4 getBoneMatrix( const in float i ) {
		int size = textureSize( boneTexture, 0 ).x;
		int j = int( i ) * 4;
		int x = j % size;
		int y = j / size;
		vec4 v1 = texelFetch( boneTexture, ivec2( x, y ), 0 );
		vec4 v2 = texelFetch( boneTexture, ivec2( x + 1, y ), 0 );
		vec4 v3 = texelFetch( boneTexture, ivec2( x + 2, y ), 0 );
		vec4 v4 = texelFetch( boneTexture, ivec2( x + 3, y ), 0 );
		return mat4( v1, v2, v3, v4 );
	}
#endif`,Mv=`#ifdef USE_SKINNING
	vec4 skinVertex = bindMatrix * vec4( transformed, 1.0 );
	vec4 skinned = vec4( 0.0 );
	skinned += boneMatX * skinVertex * skinWeight.x;
	skinned += boneMatY * skinVertex * skinWeight.y;
	skinned += boneMatZ * skinVertex * skinWeight.z;
	skinned += boneMatW * skinVertex * skinWeight.w;
	transformed = ( bindMatrixInverse * skinned ).xyz;
#endif`,Sv=`#ifdef USE_SKINNING
	mat4 skinMatrix = mat4( 0.0 );
	skinMatrix += skinWeight.x * boneMatX;
	skinMatrix += skinWeight.y * boneMatY;
	skinMatrix += skinWeight.z * boneMatZ;
	skinMatrix += skinWeight.w * boneMatW;
	skinMatrix = bindMatrixInverse * skinMatrix * bindMatrix;
	objectNormal = vec4( skinMatrix * vec4( objectNormal, 0.0 ) ).xyz;
	#ifdef USE_TANGENT
		objectTangent = vec4( skinMatrix * vec4( objectTangent, 0.0 ) ).xyz;
	#endif
#endif`,Ev=`float specularStrength;
#ifdef USE_SPECULARMAP
	vec4 texelSpecular = texture2D( specularMap, vSpecularMapUv );
	specularStrength = texelSpecular.r;
#else
	specularStrength = 1.0;
#endif`,Tv=`#ifdef USE_SPECULARMAP
	uniform sampler2D specularMap;
#endif`,wv=`#if defined( TONE_MAPPING )
	gl_FragColor.rgb = toneMapping( gl_FragColor.rgb );
#endif`,Av=`#ifndef saturate
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
uniform float toneMappingExposure;
vec3 LinearToneMapping( vec3 color ) {
	return saturate( toneMappingExposure * color );
}
vec3 ReinhardToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	return saturate( color / ( vec3( 1.0 ) + color ) );
}
vec3 OptimizedCineonToneMapping( vec3 color ) {
	color *= toneMappingExposure;
	color = max( vec3( 0.0 ), color - 0.004 );
	return pow( ( color * ( 6.2 * color + 0.5 ) ) / ( color * ( 6.2 * color + 1.7 ) + 0.06 ), vec3( 2.2 ) );
}
vec3 RRTAndODTFit( vec3 v ) {
	vec3 a = v * ( v + 0.0245786 ) - 0.000090537;
	vec3 b = v * ( 0.983729 * v + 0.4329510 ) + 0.238081;
	return a / b;
}
vec3 ACESFilmicToneMapping( vec3 color ) {
	const mat3 ACESInputMat = mat3(
		vec3( 0.59719, 0.07600, 0.02840 ),		vec3( 0.35458, 0.90834, 0.13383 ),
		vec3( 0.04823, 0.01566, 0.83777 )
	);
	const mat3 ACESOutputMat = mat3(
		vec3(  1.60475, -0.10208, -0.00327 ),		vec3( -0.53108,  1.10813, -0.07276 ),
		vec3( -0.07367, -0.00605,  1.07602 )
	);
	color *= toneMappingExposure / 0.6;
	color = ACESInputMat * color;
	color = RRTAndODTFit( color );
	color = ACESOutputMat * color;
	return saturate( color );
}
const mat3 LINEAR_REC2020_TO_LINEAR_SRGB = mat3(
	vec3( 1.6605, - 0.1246, - 0.0182 ),
	vec3( - 0.5876, 1.1329, - 0.1006 ),
	vec3( - 0.0728, - 0.0083, 1.1187 )
);
const mat3 LINEAR_SRGB_TO_LINEAR_REC2020 = mat3(
	vec3( 0.6274, 0.0691, 0.0164 ),
	vec3( 0.3293, 0.9195, 0.0880 ),
	vec3( 0.0433, 0.0113, 0.8956 )
);
vec3 agxDefaultContrastApprox( vec3 x ) {
	vec3 x2 = x * x;
	vec3 x4 = x2 * x2;
	return + 15.5 * x4 * x2
		- 40.14 * x4 * x
		+ 31.96 * x4
		- 6.868 * x2 * x
		+ 0.4298 * x2
		+ 0.1191 * x
		- 0.00232;
}
vec3 AgXToneMapping( vec3 color ) {
	const mat3 AgXInsetMatrix = mat3(
		vec3( 0.856627153315983, 0.137318972929847, 0.11189821299995 ),
		vec3( 0.0951212405381588, 0.761241990602591, 0.0767994186031903 ),
		vec3( 0.0482516061458583, 0.101439036467562, 0.811302368396859 )
	);
	const mat3 AgXOutsetMatrix = mat3(
		vec3( 1.1271005818144368, - 0.1413297634984383, - 0.14132976349843826 ),
		vec3( - 0.11060664309660323, 1.157823702216272, - 0.11060664309660294 ),
		vec3( - 0.016493938717834573, - 0.016493938717834257, 1.2519364065950405 )
	);
	const float AgxMinEv = - 12.47393;	const float AgxMaxEv = 4.026069;
	color = LINEAR_SRGB_TO_LINEAR_REC2020 * color;
	color *= toneMappingExposure;
	color = AgXInsetMatrix * color;
	color = max( color, 1e-10 );	color = log2( color );
	color = ( color - AgxMinEv ) / ( AgxMaxEv - AgxMinEv );
	color = clamp( color, 0.0, 1.0 );
	color = agxDefaultContrastApprox( color );
	color = AgXOutsetMatrix * color;
	color = pow( max( vec3( 0.0 ), color ), vec3( 2.2 ) );
	color = LINEAR_REC2020_TO_LINEAR_SRGB * color;
	return color;
}
vec3 CustomToneMapping( vec3 color ) { return color; }`,Cv=`#ifdef USE_TRANSMISSION
	material.transmission = transmission;
	material.transmissionAlpha = 1.0;
	material.thickness = thickness;
	material.attenuationDistance = attenuationDistance;
	material.attenuationColor = attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		material.transmission *= texture2D( transmissionMap, vTransmissionMapUv ).r;
	#endif
	#ifdef USE_THICKNESSMAP
		material.thickness *= texture2D( thicknessMap, vThicknessMapUv ).g;
	#endif
	vec3 pos = vWorldPosition;
	vec3 v = normalize( cameraPosition - pos );
	vec3 n = inverseTransformDirection( normal, viewMatrix );
	vec4 transmitted = getIBLVolumeRefraction(
		n, v, material.roughness, material.diffuseColor, material.specularColor, material.specularF90,
		pos, modelMatrix, viewMatrix, projectionMatrix, material.ior, material.thickness,
		material.attenuationColor, material.attenuationDistance );
	material.transmissionAlpha = mix( material.transmissionAlpha, transmitted.a, material.transmission );
	totalDiffuse = mix( totalDiffuse, transmitted.rgb, material.transmission );
#endif`,Rv=`#ifdef USE_TRANSMISSION
	uniform float transmission;
	uniform float thickness;
	uniform float attenuationDistance;
	uniform vec3 attenuationColor;
	#ifdef USE_TRANSMISSIONMAP
		uniform sampler2D transmissionMap;
	#endif
	#ifdef USE_THICKNESSMAP
		uniform sampler2D thicknessMap;
	#endif
	uniform vec2 transmissionSamplerSize;
	uniform sampler2D transmissionSamplerMap;
	uniform mat4 modelMatrix;
	uniform mat4 projectionMatrix;
	varying vec3 vWorldPosition;
	float w0( float a ) {
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - a + 3.0 ) - 3.0 ) + 1.0 );
	}
	float w1( float a ) {
		return ( 1.0 / 6.0 ) * ( a *  a * ( 3.0 * a - 6.0 ) + 4.0 );
	}
	float w2( float a ){
		return ( 1.0 / 6.0 ) * ( a * ( a * ( - 3.0 * a + 3.0 ) + 3.0 ) + 1.0 );
	}
	float w3( float a ) {
		return ( 1.0 / 6.0 ) * ( a * a * a );
	}
	float g0( float a ) {
		return w0( a ) + w1( a );
	}
	float g1( float a ) {
		return w2( a ) + w3( a );
	}
	float h0( float a ) {
		return - 1.0 + w1( a ) / ( w0( a ) + w1( a ) );
	}
	float h1( float a ) {
		return 1.0 + w3( a ) / ( w2( a ) + w3( a ) );
	}
	vec4 bicubic( sampler2D tex, vec2 uv, vec4 texelSize, float lod ) {
		uv = uv * texelSize.zw + 0.5;
		vec2 iuv = floor( uv );
		vec2 fuv = fract( uv );
		float g0x = g0( fuv.x );
		float g1x = g1( fuv.x );
		float h0x = h0( fuv.x );
		float h1x = h1( fuv.x );
		float h0y = h0( fuv.y );
		float h1y = h1( fuv.y );
		vec2 p0 = ( vec2( iuv.x + h0x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p1 = ( vec2( iuv.x + h1x, iuv.y + h0y ) - 0.5 ) * texelSize.xy;
		vec2 p2 = ( vec2( iuv.x + h0x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		vec2 p3 = ( vec2( iuv.x + h1x, iuv.y + h1y ) - 0.5 ) * texelSize.xy;
		return g0( fuv.y ) * ( g0x * textureLod( tex, p0, lod ) + g1x * textureLod( tex, p1, lod ) ) +
			g1( fuv.y ) * ( g0x * textureLod( tex, p2, lod ) + g1x * textureLod( tex, p3, lod ) );
	}
	vec4 textureBicubic( sampler2D sampler, vec2 uv, float lod ) {
		vec2 fLodSize = vec2( textureSize( sampler, int( lod ) ) );
		vec2 cLodSize = vec2( textureSize( sampler, int( lod + 1.0 ) ) );
		vec2 fLodSizeInv = 1.0 / fLodSize;
		vec2 cLodSizeInv = 1.0 / cLodSize;
		vec4 fSample = bicubic( sampler, uv, vec4( fLodSizeInv, fLodSize ), floor( lod ) );
		vec4 cSample = bicubic( sampler, uv, vec4( cLodSizeInv, cLodSize ), ceil( lod ) );
		return mix( fSample, cSample, fract( lod ) );
	}
	vec3 getVolumeTransmissionRay( const in vec3 n, const in vec3 v, const in float thickness, const in float ior, const in mat4 modelMatrix ) {
		vec3 refractionVector = refract( - v, normalize( n ), 1.0 / ior );
		vec3 modelScale;
		modelScale.x = length( vec3( modelMatrix[ 0 ].xyz ) );
		modelScale.y = length( vec3( modelMatrix[ 1 ].xyz ) );
		modelScale.z = length( vec3( modelMatrix[ 2 ].xyz ) );
		return normalize( refractionVector ) * thickness * modelScale;
	}
	float applyIorToRoughness( const in float roughness, const in float ior ) {
		return roughness * clamp( ior * 2.0 - 2.0, 0.0, 1.0 );
	}
	vec4 getTransmissionSample( const in vec2 fragCoord, const in float roughness, const in float ior ) {
		float lod = log2( transmissionSamplerSize.x ) * applyIorToRoughness( roughness, ior );
		return textureBicubic( transmissionSamplerMap, fragCoord.xy, lod );
	}
	vec3 volumeAttenuation( const in float transmissionDistance, const in vec3 attenuationColor, const in float attenuationDistance ) {
		if ( isinf( attenuationDistance ) ) {
			return vec3( 1.0 );
		} else {
			vec3 attenuationCoefficient = -log( attenuationColor ) / attenuationDistance;
			vec3 transmittance = exp( - attenuationCoefficient * transmissionDistance );			return transmittance;
		}
	}
	vec4 getIBLVolumeRefraction( const in vec3 n, const in vec3 v, const in float roughness, const in vec3 diffuseColor,
		const in vec3 specularColor, const in float specularF90, const in vec3 position, const in mat4 modelMatrix,
		const in mat4 viewMatrix, const in mat4 projMatrix, const in float ior, const in float thickness,
		const in vec3 attenuationColor, const in float attenuationDistance ) {
		vec3 transmissionRay = getVolumeTransmissionRay( n, v, thickness, ior, modelMatrix );
		vec3 refractedRayExit = position + transmissionRay;
		vec4 ndcPos = projMatrix * viewMatrix * vec4( refractedRayExit, 1.0 );
		vec2 refractionCoords = ndcPos.xy / ndcPos.w;
		refractionCoords += 1.0;
		refractionCoords /= 2.0;
		vec4 transmittedLight = getTransmissionSample( refractionCoords, roughness, ior );
		vec3 transmittance = diffuseColor * volumeAttenuation( length( transmissionRay ), attenuationColor, attenuationDistance );
		vec3 attenuatedColor = transmittance * transmittedLight.rgb;
		vec3 F = EnvironmentBRDF( n, v, specularColor, specularF90, roughness );
		float transmittanceFactor = ( transmittance.r + transmittance.g + transmittance.b ) / 3.0;
		return vec4( ( 1.0 - F ) * attenuatedColor, 1.0 - ( 1.0 - transmittedLight.a ) * transmittanceFactor );
	}
#endif`,Lv=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_SPECULARMAP
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,Pv=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	varying vec2 vUv;
#endif
#ifdef USE_MAP
	uniform mat3 mapTransform;
	varying vec2 vMapUv;
#endif
#ifdef USE_ALPHAMAP
	uniform mat3 alphaMapTransform;
	varying vec2 vAlphaMapUv;
#endif
#ifdef USE_LIGHTMAP
	uniform mat3 lightMapTransform;
	varying vec2 vLightMapUv;
#endif
#ifdef USE_AOMAP
	uniform mat3 aoMapTransform;
	varying vec2 vAoMapUv;
#endif
#ifdef USE_BUMPMAP
	uniform mat3 bumpMapTransform;
	varying vec2 vBumpMapUv;
#endif
#ifdef USE_NORMALMAP
	uniform mat3 normalMapTransform;
	varying vec2 vNormalMapUv;
#endif
#ifdef USE_DISPLACEMENTMAP
	uniform mat3 displacementMapTransform;
	varying vec2 vDisplacementMapUv;
#endif
#ifdef USE_EMISSIVEMAP
	uniform mat3 emissiveMapTransform;
	varying vec2 vEmissiveMapUv;
#endif
#ifdef USE_METALNESSMAP
	uniform mat3 metalnessMapTransform;
	varying vec2 vMetalnessMapUv;
#endif
#ifdef USE_ROUGHNESSMAP
	uniform mat3 roughnessMapTransform;
	varying vec2 vRoughnessMapUv;
#endif
#ifdef USE_ANISOTROPYMAP
	uniform mat3 anisotropyMapTransform;
	varying vec2 vAnisotropyMapUv;
#endif
#ifdef USE_CLEARCOATMAP
	uniform mat3 clearcoatMapTransform;
	varying vec2 vClearcoatMapUv;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	uniform mat3 clearcoatNormalMapTransform;
	varying vec2 vClearcoatNormalMapUv;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	uniform mat3 clearcoatRoughnessMapTransform;
	varying vec2 vClearcoatRoughnessMapUv;
#endif
#ifdef USE_SHEEN_COLORMAP
	uniform mat3 sheenColorMapTransform;
	varying vec2 vSheenColorMapUv;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	uniform mat3 sheenRoughnessMapTransform;
	varying vec2 vSheenRoughnessMapUv;
#endif
#ifdef USE_IRIDESCENCEMAP
	uniform mat3 iridescenceMapTransform;
	varying vec2 vIridescenceMapUv;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	uniform mat3 iridescenceThicknessMapTransform;
	varying vec2 vIridescenceThicknessMapUv;
#endif
#ifdef USE_SPECULARMAP
	uniform mat3 specularMapTransform;
	varying vec2 vSpecularMapUv;
#endif
#ifdef USE_SPECULAR_COLORMAP
	uniform mat3 specularColorMapTransform;
	varying vec2 vSpecularColorMapUv;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	uniform mat3 specularIntensityMapTransform;
	varying vec2 vSpecularIntensityMapUv;
#endif
#ifdef USE_TRANSMISSIONMAP
	uniform mat3 transmissionMapTransform;
	varying vec2 vTransmissionMapUv;
#endif
#ifdef USE_THICKNESSMAP
	uniform mat3 thicknessMapTransform;
	varying vec2 vThicknessMapUv;
#endif`,Dv=`#if defined( USE_UV ) || defined( USE_ANISOTROPY )
	vUv = vec3( uv, 1 ).xy;
#endif
#ifdef USE_MAP
	vMapUv = ( mapTransform * vec3( MAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ALPHAMAP
	vAlphaMapUv = ( alphaMapTransform * vec3( ALPHAMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_LIGHTMAP
	vLightMapUv = ( lightMapTransform * vec3( LIGHTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_AOMAP
	vAoMapUv = ( aoMapTransform * vec3( AOMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_BUMPMAP
	vBumpMapUv = ( bumpMapTransform * vec3( BUMPMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_NORMALMAP
	vNormalMapUv = ( normalMapTransform * vec3( NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_DISPLACEMENTMAP
	vDisplacementMapUv = ( displacementMapTransform * vec3( DISPLACEMENTMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_EMISSIVEMAP
	vEmissiveMapUv = ( emissiveMapTransform * vec3( EMISSIVEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_METALNESSMAP
	vMetalnessMapUv = ( metalnessMapTransform * vec3( METALNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ROUGHNESSMAP
	vRoughnessMapUv = ( roughnessMapTransform * vec3( ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_ANISOTROPYMAP
	vAnisotropyMapUv = ( anisotropyMapTransform * vec3( ANISOTROPYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOATMAP
	vClearcoatMapUv = ( clearcoatMapTransform * vec3( CLEARCOATMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_NORMALMAP
	vClearcoatNormalMapUv = ( clearcoatNormalMapTransform * vec3( CLEARCOAT_NORMALMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_CLEARCOAT_ROUGHNESSMAP
	vClearcoatRoughnessMapUv = ( clearcoatRoughnessMapTransform * vec3( CLEARCOAT_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCEMAP
	vIridescenceMapUv = ( iridescenceMapTransform * vec3( IRIDESCENCEMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_IRIDESCENCE_THICKNESSMAP
	vIridescenceThicknessMapUv = ( iridescenceThicknessMapTransform * vec3( IRIDESCENCE_THICKNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_COLORMAP
	vSheenColorMapUv = ( sheenColorMapTransform * vec3( SHEEN_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SHEEN_ROUGHNESSMAP
	vSheenRoughnessMapUv = ( sheenRoughnessMapTransform * vec3( SHEEN_ROUGHNESSMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULARMAP
	vSpecularMapUv = ( specularMapTransform * vec3( SPECULARMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_COLORMAP
	vSpecularColorMapUv = ( specularColorMapTransform * vec3( SPECULAR_COLORMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_SPECULAR_INTENSITYMAP
	vSpecularIntensityMapUv = ( specularIntensityMapTransform * vec3( SPECULAR_INTENSITYMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_TRANSMISSIONMAP
	vTransmissionMapUv = ( transmissionMapTransform * vec3( TRANSMISSIONMAP_UV, 1 ) ).xy;
#endif
#ifdef USE_THICKNESSMAP
	vThicknessMapUv = ( thicknessMapTransform * vec3( THICKNESSMAP_UV, 1 ) ).xy;
#endif`,Uv=`#if defined( USE_ENVMAP ) || defined( DISTANCE ) || defined ( USE_SHADOWMAP ) || defined ( USE_TRANSMISSION ) || NUM_SPOT_LIGHT_COORDS > 0
	vec4 worldPosition = vec4( transformed, 1.0 );
	#ifdef USE_BATCHING
		worldPosition = batchingMatrix * worldPosition;
	#endif
	#ifdef USE_INSTANCING
		worldPosition = instanceMatrix * worldPosition;
	#endif
	worldPosition = modelMatrix * worldPosition;
#endif`;const Iv=`varying vec2 vUv;
uniform mat3 uvTransform;
void main() {
	vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	gl_Position = vec4( position.xy, 1.0, 1.0 );
}`,Nv=`uniform sampler2D t2D;
uniform float backgroundIntensity;
varying vec2 vUv;
void main() {
	vec4 texColor = texture2D( t2D, vUv );
	#ifdef DECODE_VIDEO_TEXTURE
		texColor = vec4( mix( pow( texColor.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), texColor.rgb * 0.0773993808, vec3( lessThanEqual( texColor.rgb, vec3( 0.04045 ) ) ) ), texColor.w );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,kv=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Fv=`#ifdef ENVMAP_TYPE_CUBE
	uniform samplerCube envMap;
#elif defined( ENVMAP_TYPE_CUBE_UV )
	uniform sampler2D envMap;
#endif
uniform float flipEnvMap;
uniform float backgroundBlurriness;
uniform float backgroundIntensity;
varying vec3 vWorldDirection;
#include <cube_uv_reflection_fragment>
void main() {
	#ifdef ENVMAP_TYPE_CUBE
		vec4 texColor = textureCube( envMap, vec3( flipEnvMap * vWorldDirection.x, vWorldDirection.yz ) );
	#elif defined( ENVMAP_TYPE_CUBE_UV )
		vec4 texColor = textureCubeUV( envMap, vWorldDirection, backgroundBlurriness );
	#else
		vec4 texColor = vec4( 0.0, 0.0, 0.0, 1.0 );
	#endif
	texColor.rgb *= backgroundIntensity;
	gl_FragColor = texColor;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Ov=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
	gl_Position.z = gl_Position.w;
}`,Bv=`uniform samplerCube tCube;
uniform float tFlip;
uniform float opacity;
varying vec3 vWorldDirection;
void main() {
	vec4 texColor = textureCube( tCube, vec3( tFlip * vWorldDirection.x, vWorldDirection.yz ) );
	gl_FragColor = texColor;
	gl_FragColor.a *= opacity;
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,zv=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
varying vec2 vHighPrecisionZW;
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vHighPrecisionZW = gl_Position.zw;
}`,Hv=`#if DEPTH_PACKING == 3200
	uniform float opacity;
#endif
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
varying vec2 vHighPrecisionZW;
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( 1.0 );
	#if DEPTH_PACKING == 3200
		diffuseColor.a = opacity;
	#endif
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <logdepthbuf_fragment>
	float fragCoordZ = 0.5 * vHighPrecisionZW[0] / vHighPrecisionZW[1] + 0.5;
	#if DEPTH_PACKING == 3200
		gl_FragColor = vec4( vec3( 1.0 - fragCoordZ ), opacity );
	#elif DEPTH_PACKING == 3201
		gl_FragColor = packDepthToRGBA( fragCoordZ );
	#endif
}`,Vv=`#define DISTANCE
varying vec3 vWorldPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <skinbase_vertex>
	#ifdef USE_DISPLACEMENTMAP
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <worldpos_vertex>
	#include <clipping_planes_vertex>
	vWorldPosition = worldPosition.xyz;
}`,Gv=`#define DISTANCE
uniform vec3 referencePosition;
uniform float nearDistance;
uniform float farDistance;
varying vec3 vWorldPosition;
#include <common>
#include <packing>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <clipping_planes_pars_fragment>
void main () {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( 1.0 );
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	float dist = length( vWorldPosition - referencePosition );
	dist = ( dist - nearDistance ) / ( farDistance - nearDistance );
	dist = saturate( dist );
	gl_FragColor = packDepthToRGBA( dist );
}`,Wv=`varying vec3 vWorldDirection;
#include <common>
void main() {
	vWorldDirection = transformDirection( position, modelMatrix );
	#include <begin_vertex>
	#include <project_vertex>
}`,$v=`uniform sampler2D tEquirect;
varying vec3 vWorldDirection;
#include <common>
void main() {
	vec3 direction = normalize( vWorldDirection );
	vec2 sampleUV = equirectUv( direction );
	gl_FragColor = texture2D( tEquirect, sampleUV );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
}`,Xv=`uniform float scale;
attribute float lineDistance;
varying float vLineDistance;
#include <common>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	vLineDistance = scale * lineDistance;
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,qv=`uniform vec3 diffuse;
uniform float opacity;
uniform float dashSize;
uniform float totalSize;
varying float vLineDistance;
#include <common>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	if ( mod( vLineDistance, totalSize ) > dashSize ) {
		discard;
	}
	vec3 outgoingLight = vec3( 0.0 );
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,jv=`#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#if defined ( USE_ENVMAP ) || defined ( USE_SKINNING )
		#include <beginnormal_vertex>
		#include <morphnormal_vertex>
		#include <skinbase_vertex>
		#include <skinnormal_vertex>
		#include <defaultnormal_vertex>
	#endif
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <fog_vertex>
}`,Yv=`uniform vec3 diffuse;
uniform float opacity;
#ifndef FLAT_SHADED
	varying vec3 vNormal;
#endif
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	#ifdef USE_LIGHTMAP
		vec4 lightMapTexel = texture2D( lightMap, vLightMapUv );
		reflectedLight.indirectDiffuse += lightMapTexel.rgb * lightMapIntensity * RECIPROCAL_PI;
	#else
		reflectedLight.indirectDiffuse += vec3( 1.0 );
	#endif
	#include <aomap_fragment>
	reflectedLight.indirectDiffuse *= diffuseColor.rgb;
	vec3 outgoingLight = reflectedLight.indirectDiffuse;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Kv=`#define LAMBERT
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,Zv=`#define LAMBERT
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_lambert_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_lambert_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,Jv=`#define MATCAP
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <color_pars_vertex>
#include <displacementmap_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
	vViewPosition = - mvPosition.xyz;
}`,Qv=`#define MATCAP
uniform vec3 diffuse;
uniform float opacity;
uniform sampler2D matcap;
varying vec3 vViewPosition;
#include <common>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	vec3 viewDir = normalize( vViewPosition );
	vec3 x = normalize( vec3( viewDir.z, 0.0, - viewDir.x ) );
	vec3 y = cross( viewDir, x );
	vec2 uv = vec2( dot( x, normal ), dot( y, normal ) ) * 0.495 + 0.5;
	#ifdef USE_MATCAP
		vec4 matcapColor = texture2D( matcap, uv );
	#else
		vec4 matcapColor = vec4( vec3( mix( 0.2, 0.8, uv.y ) ), 1.0 );
	#endif
	vec3 outgoingLight = diffuseColor.rgb * matcapColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,t0=`#define NORMAL
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	vViewPosition = - mvPosition.xyz;
#endif
}`,e0=`#define NORMAL
uniform float opacity;
#if defined( FLAT_SHADED ) || defined( USE_BUMPMAP ) || defined( USE_NORMALMAP_TANGENTSPACE )
	varying vec3 vViewPosition;
#endif
#include <packing>
#include <uv_pars_fragment>
#include <normal_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	#include <logdepthbuf_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	gl_FragColor = vec4( packNormalToRGB( normal ), opacity );
	#ifdef OPAQUE
		gl_FragColor.a = 1.0;
	#endif
}`,n0=`#define PHONG
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <envmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <envmap_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,i0=`#define PHONG
uniform vec3 diffuse;
uniform vec3 emissive;
uniform vec3 specular;
uniform float shininess;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_phong_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <specularmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <specularmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_phong_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + reflectedLight.directSpecular + reflectedLight.indirectSpecular + totalEmissiveRadiance;
	#include <envmap_fragment>
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,s0=`#define STANDARD
varying vec3 vViewPosition;
#ifdef USE_TRANSMISSION
	varying vec3 vWorldPosition;
#endif
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
#ifdef USE_TRANSMISSION
	vWorldPosition = worldPosition.xyz;
#endif
}`,r0=`#define STANDARD
#ifdef PHYSICAL
	#define IOR
	#define USE_SPECULAR
#endif
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float roughness;
uniform float metalness;
uniform float opacity;
#ifdef IOR
	uniform float ior;
#endif
#ifdef USE_SPECULAR
	uniform float specularIntensity;
	uniform vec3 specularColor;
	#ifdef USE_SPECULAR_COLORMAP
		uniform sampler2D specularColorMap;
	#endif
	#ifdef USE_SPECULAR_INTENSITYMAP
		uniform sampler2D specularIntensityMap;
	#endif
#endif
#ifdef USE_CLEARCOAT
	uniform float clearcoat;
	uniform float clearcoatRoughness;
#endif
#ifdef USE_IRIDESCENCE
	uniform float iridescence;
	uniform float iridescenceIOR;
	uniform float iridescenceThicknessMinimum;
	uniform float iridescenceThicknessMaximum;
#endif
#ifdef USE_SHEEN
	uniform vec3 sheenColor;
	uniform float sheenRoughness;
	#ifdef USE_SHEEN_COLORMAP
		uniform sampler2D sheenColorMap;
	#endif
	#ifdef USE_SHEEN_ROUGHNESSMAP
		uniform sampler2D sheenRoughnessMap;
	#endif
#endif
#ifdef USE_ANISOTROPY
	uniform vec2 anisotropyVector;
	#ifdef USE_ANISOTROPYMAP
		uniform sampler2D anisotropyMap;
	#endif
#endif
varying vec3 vViewPosition;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <iridescence_fragment>
#include <cube_uv_reflection_fragment>
#include <envmap_common_pars_fragment>
#include <envmap_physical_pars_fragment>
#include <fog_pars_fragment>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_physical_pars_fragment>
#include <transmission_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <clearcoat_pars_fragment>
#include <iridescence_pars_fragment>
#include <roughnessmap_pars_fragment>
#include <metalnessmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <roughnessmap_fragment>
	#include <metalnessmap_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <clearcoat_normal_fragment_begin>
	#include <clearcoat_normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_physical_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 totalDiffuse = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse;
	vec3 totalSpecular = reflectedLight.directSpecular + reflectedLight.indirectSpecular;
	#include <transmission_fragment>
	vec3 outgoingLight = totalDiffuse + totalSpecular + totalEmissiveRadiance;
	#ifdef USE_SHEEN
		float sheenEnergyComp = 1.0 - 0.157 * max3( material.sheenColor );
		outgoingLight = outgoingLight * sheenEnergyComp + sheenSpecularDirect + sheenSpecularIndirect;
	#endif
	#ifdef USE_CLEARCOAT
		float dotNVcc = saturate( dot( geometryClearcoatNormal, geometryViewDir ) );
		vec3 Fcc = F_Schlick( material.clearcoatF0, material.clearcoatF90, dotNVcc );
		outgoingLight = outgoingLight * ( 1.0 - material.clearcoat * Fcc ) + ( clearcoatSpecularDirect + clearcoatSpecularIndirect ) * material.clearcoat;
	#endif
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,o0=`#define TOON
varying vec3 vViewPosition;
#include <common>
#include <batching_pars_vertex>
#include <uv_pars_vertex>
#include <displacementmap_pars_vertex>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <normal_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <shadowmap_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <normal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <displacementmap_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	vViewPosition = - mvPosition.xyz;
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,a0=`#define TOON
uniform vec3 diffuse;
uniform vec3 emissive;
uniform float opacity;
#include <common>
#include <packing>
#include <dithering_pars_fragment>
#include <color_pars_fragment>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <aomap_pars_fragment>
#include <lightmap_pars_fragment>
#include <emissivemap_pars_fragment>
#include <gradientmap_pars_fragment>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <normal_pars_fragment>
#include <lights_toon_pars_fragment>
#include <shadowmap_pars_fragment>
#include <bumpmap_pars_fragment>
#include <normalmap_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec4 diffuseColor = vec4( diffuse, opacity );
	ReflectedLight reflectedLight = ReflectedLight( vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ), vec3( 0.0 ) );
	vec3 totalEmissiveRadiance = emissive;
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <color_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	#include <normal_fragment_begin>
	#include <normal_fragment_maps>
	#include <emissivemap_fragment>
	#include <lights_toon_fragment>
	#include <lights_fragment_begin>
	#include <lights_fragment_maps>
	#include <lights_fragment_end>
	#include <aomap_fragment>
	vec3 outgoingLight = reflectedLight.directDiffuse + reflectedLight.indirectDiffuse + totalEmissiveRadiance;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
	#include <dithering_fragment>
}`,l0=`uniform float size;
uniform float scale;
#include <common>
#include <color_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
#ifdef USE_POINTS_UV
	varying vec2 vUv;
	uniform mat3 uvTransform;
#endif
void main() {
	#ifdef USE_POINTS_UV
		vUv = ( uvTransform * vec3( uv, 1 ) ).xy;
	#endif
	#include <color_vertex>
	#include <morphcolor_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <project_vertex>
	gl_PointSize = size;
	#ifdef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) gl_PointSize *= ( scale / - mvPosition.z );
	#endif
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <worldpos_vertex>
	#include <fog_vertex>
}`,c0=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <color_pars_fragment>
#include <map_particle_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <logdepthbuf_fragment>
	#include <map_particle_fragment>
	#include <color_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
	#include <premultiplied_alpha_fragment>
}`,u0=`#include <common>
#include <batching_pars_vertex>
#include <fog_pars_vertex>
#include <morphtarget_pars_vertex>
#include <skinning_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <shadowmap_pars_vertex>
void main() {
	#include <batching_vertex>
	#include <beginnormal_vertex>
	#include <morphnormal_vertex>
	#include <skinbase_vertex>
	#include <skinnormal_vertex>
	#include <defaultnormal_vertex>
	#include <begin_vertex>
	#include <morphtarget_vertex>
	#include <skinning_vertex>
	#include <project_vertex>
	#include <logdepthbuf_vertex>
	#include <worldpos_vertex>
	#include <shadowmap_vertex>
	#include <fog_vertex>
}`,d0=`uniform vec3 color;
uniform float opacity;
#include <common>
#include <packing>
#include <fog_pars_fragment>
#include <bsdfs>
#include <lights_pars_begin>
#include <logdepthbuf_pars_fragment>
#include <shadowmap_pars_fragment>
#include <shadowmask_pars_fragment>
void main() {
	#include <logdepthbuf_fragment>
	gl_FragColor = vec4( color, opacity * ( 1.0 - getShadowMask() ) );
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`,h0=`uniform float rotation;
uniform vec2 center;
#include <common>
#include <uv_pars_vertex>
#include <fog_pars_vertex>
#include <logdepthbuf_pars_vertex>
#include <clipping_planes_pars_vertex>
void main() {
	#include <uv_vertex>
	vec4 mvPosition = modelViewMatrix * vec4( 0.0, 0.0, 0.0, 1.0 );
	vec2 scale;
	scale.x = length( vec3( modelMatrix[ 0 ].x, modelMatrix[ 0 ].y, modelMatrix[ 0 ].z ) );
	scale.y = length( vec3( modelMatrix[ 1 ].x, modelMatrix[ 1 ].y, modelMatrix[ 1 ].z ) );
	#ifndef USE_SIZEATTENUATION
		bool isPerspective = isPerspectiveMatrix( projectionMatrix );
		if ( isPerspective ) scale *= - mvPosition.z;
	#endif
	vec2 alignedPosition = ( position.xy - ( center - vec2( 0.5 ) ) ) * scale;
	vec2 rotatedPosition;
	rotatedPosition.x = cos( rotation ) * alignedPosition.x - sin( rotation ) * alignedPosition.y;
	rotatedPosition.y = sin( rotation ) * alignedPosition.x + cos( rotation ) * alignedPosition.y;
	mvPosition.xy += rotatedPosition;
	gl_Position = projectionMatrix * mvPosition;
	#include <logdepthbuf_vertex>
	#include <clipping_planes_vertex>
	#include <fog_vertex>
}`,f0=`uniform vec3 diffuse;
uniform float opacity;
#include <common>
#include <uv_pars_fragment>
#include <map_pars_fragment>
#include <alphamap_pars_fragment>
#include <alphatest_pars_fragment>
#include <alphahash_pars_fragment>
#include <fog_pars_fragment>
#include <logdepthbuf_pars_fragment>
#include <clipping_planes_pars_fragment>
void main() {
	#include <clipping_planes_fragment>
	vec3 outgoingLight = vec3( 0.0 );
	vec4 diffuseColor = vec4( diffuse, opacity );
	#include <logdepthbuf_fragment>
	#include <map_fragment>
	#include <alphamap_fragment>
	#include <alphatest_fragment>
	#include <alphahash_fragment>
	outgoingLight = diffuseColor.rgb;
	#include <opaque_fragment>
	#include <tonemapping_fragment>
	#include <colorspace_fragment>
	#include <fog_fragment>
}`,jt={alphahash_fragment:N_,alphahash_pars_fragment:k_,alphamap_fragment:F_,alphamap_pars_fragment:O_,alphatest_fragment:B_,alphatest_pars_fragment:z_,aomap_fragment:H_,aomap_pars_fragment:V_,batching_pars_vertex:G_,batching_vertex:W_,begin_vertex:$_,beginnormal_vertex:X_,bsdfs:q_,iridescence_fragment:j_,bumpmap_pars_fragment:Y_,clipping_planes_fragment:K_,clipping_planes_pars_fragment:Z_,clipping_planes_pars_vertex:J_,clipping_planes_vertex:Q_,color_fragment:tg,color_pars_fragment:eg,color_pars_vertex:ng,color_vertex:ig,common:sg,cube_uv_reflection_fragment:rg,defaultnormal_vertex:og,displacementmap_pars_vertex:ag,displacementmap_vertex:lg,emissivemap_fragment:cg,emissivemap_pars_fragment:ug,colorspace_fragment:dg,colorspace_pars_fragment:hg,envmap_fragment:fg,envmap_common_pars_fragment:pg,envmap_pars_fragment:mg,envmap_pars_vertex:_g,envmap_physical_pars_fragment:Cg,envmap_vertex:gg,fog_vertex:vg,fog_pars_vertex:xg,fog_fragment:yg,fog_pars_fragment:bg,gradientmap_pars_fragment:Mg,lightmap_fragment:Sg,lightmap_pars_fragment:Eg,lights_lambert_fragment:Tg,lights_lambert_pars_fragment:wg,lights_pars_begin:Ag,lights_toon_fragment:Rg,lights_toon_pars_fragment:Lg,lights_phong_fragment:Pg,lights_phong_pars_fragment:Dg,lights_physical_fragment:Ug,lights_physical_pars_fragment:Ig,lights_fragment_begin:Ng,lights_fragment_maps:kg,lights_fragment_end:Fg,logdepthbuf_fragment:Og,logdepthbuf_pars_fragment:Bg,logdepthbuf_pars_vertex:zg,logdepthbuf_vertex:Hg,map_fragment:Vg,map_pars_fragment:Gg,map_particle_fragment:Wg,map_particle_pars_fragment:$g,metalnessmap_fragment:Xg,metalnessmap_pars_fragment:qg,morphcolor_vertex:jg,morphnormal_vertex:Yg,morphtarget_pars_vertex:Kg,morphtarget_vertex:Zg,normal_fragment_begin:Jg,normal_fragment_maps:Qg,normal_pars_fragment:tv,normal_pars_vertex:ev,normal_vertex:nv,normalmap_pars_fragment:iv,clearcoat_normal_fragment_begin:sv,clearcoat_normal_fragment_maps:rv,clearcoat_pars_fragment:ov,iridescence_pars_fragment:av,opaque_fragment:lv,packing:cv,premultiplied_alpha_fragment:uv,project_vertex:dv,dithering_fragment:hv,dithering_pars_fragment:fv,roughnessmap_fragment:pv,roughnessmap_pars_fragment:mv,shadowmap_pars_fragment:_v,shadowmap_pars_vertex:gv,shadowmap_vertex:vv,shadowmask_pars_fragment:xv,skinbase_vertex:yv,skinning_pars_vertex:bv,skinning_vertex:Mv,skinnormal_vertex:Sv,specularmap_fragment:Ev,specularmap_pars_fragment:Tv,tonemapping_fragment:wv,tonemapping_pars_fragment:Av,transmission_fragment:Cv,transmission_pars_fragment:Rv,uv_pars_fragment:Lv,uv_pars_vertex:Pv,uv_vertex:Dv,worldpos_vertex:Uv,background_vert:Iv,background_frag:Nv,backgroundCube_vert:kv,backgroundCube_frag:Fv,cube_vert:Ov,cube_frag:Bv,depth_vert:zv,depth_frag:Hv,distanceRGBA_vert:Vv,distanceRGBA_frag:Gv,equirect_vert:Wv,equirect_frag:$v,linedashed_vert:Xv,linedashed_frag:qv,meshbasic_vert:jv,meshbasic_frag:Yv,meshlambert_vert:Kv,meshlambert_frag:Zv,meshmatcap_vert:Jv,meshmatcap_frag:Qv,meshnormal_vert:t0,meshnormal_frag:e0,meshphong_vert:n0,meshphong_frag:i0,meshphysical_vert:s0,meshphysical_frag:r0,meshtoon_vert:o0,meshtoon_frag:a0,points_vert:l0,points_frag:c0,shadow_vert:u0,shadow_frag:d0,sprite_vert:h0,sprite_frag:f0},vt={common:{diffuse:{value:new ne(16777215)},opacity:{value:1},map:{value:null},mapTransform:{value:new te},alphaMap:{value:null},alphaMapTransform:{value:new te},alphaTest:{value:0}},specularmap:{specularMap:{value:null},specularMapTransform:{value:new te}},envmap:{envMap:{value:null},flipEnvMap:{value:-1},reflectivity:{value:1},ior:{value:1.5},refractionRatio:{value:.98}},aomap:{aoMap:{value:null},aoMapIntensity:{value:1},aoMapTransform:{value:new te}},lightmap:{lightMap:{value:null},lightMapIntensity:{value:1},lightMapTransform:{value:new te}},bumpmap:{bumpMap:{value:null},bumpMapTransform:{value:new te},bumpScale:{value:1}},normalmap:{normalMap:{value:null},normalMapTransform:{value:new te},normalScale:{value:new re(1,1)}},displacementmap:{displacementMap:{value:null},displacementMapTransform:{value:new te},displacementScale:{value:1},displacementBias:{value:0}},emissivemap:{emissiveMap:{value:null},emissiveMapTransform:{value:new te}},metalnessmap:{metalnessMap:{value:null},metalnessMapTransform:{value:new te}},roughnessmap:{roughnessMap:{value:null},roughnessMapTransform:{value:new te}},gradientmap:{gradientMap:{value:null}},fog:{fogDensity:{value:25e-5},fogNear:{value:1},fogFar:{value:2e3},fogColor:{value:new ne(16777215)}},lights:{ambientLightColor:{value:[]},lightProbe:{value:[]},directionalLights:{value:[],properties:{direction:{},color:{}}},directionalLightShadows:{value:[],properties:{shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},directionalShadowMap:{value:[]},directionalShadowMatrix:{value:[]},spotLights:{value:[],properties:{color:{},position:{},direction:{},distance:{},coneCos:{},penumbraCos:{},decay:{}}},spotLightShadows:{value:[],properties:{shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{}}},spotLightMap:{value:[]},spotShadowMap:{value:[]},spotLightMatrix:{value:[]},pointLights:{value:[],properties:{color:{},position:{},decay:{},distance:{}}},pointLightShadows:{value:[],properties:{shadowBias:{},shadowNormalBias:{},shadowRadius:{},shadowMapSize:{},shadowCameraNear:{},shadowCameraFar:{}}},pointShadowMap:{value:[]},pointShadowMatrix:{value:[]},hemisphereLights:{value:[],properties:{direction:{},skyColor:{},groundColor:{}}},rectAreaLights:{value:[],properties:{color:{},position:{},width:{},height:{}}},ltc_1:{value:null},ltc_2:{value:null}},points:{diffuse:{value:new ne(16777215)},opacity:{value:1},size:{value:1},scale:{value:1},map:{value:null},alphaMap:{value:null},alphaMapTransform:{value:new te},alphaTest:{value:0},uvTransform:{value:new te}},sprite:{diffuse:{value:new ne(16777215)},opacity:{value:1},center:{value:new re(.5,.5)},rotation:{value:0},map:{value:null},mapTransform:{value:new te},alphaMap:{value:null},alphaMapTransform:{value:new te},alphaTest:{value:0}}},Un={basic:{uniforms:Ye([vt.common,vt.specularmap,vt.envmap,vt.aomap,vt.lightmap,vt.fog]),vertexShader:jt.meshbasic_vert,fragmentShader:jt.meshbasic_frag},lambert:{uniforms:Ye([vt.common,vt.specularmap,vt.envmap,vt.aomap,vt.lightmap,vt.emissivemap,vt.bumpmap,vt.normalmap,vt.displacementmap,vt.fog,vt.lights,{emissive:{value:new ne(0)}}]),vertexShader:jt.meshlambert_vert,fragmentShader:jt.meshlambert_frag},phong:{uniforms:Ye([vt.common,vt.specularmap,vt.envmap,vt.aomap,vt.lightmap,vt.emissivemap,vt.bumpmap,vt.normalmap,vt.displacementmap,vt.fog,vt.lights,{emissive:{value:new ne(0)},specular:{value:new ne(1118481)},shininess:{value:30}}]),vertexShader:jt.meshphong_vert,fragmentShader:jt.meshphong_frag},standard:{uniforms:Ye([vt.common,vt.envmap,vt.aomap,vt.lightmap,vt.emissivemap,vt.bumpmap,vt.normalmap,vt.displacementmap,vt.roughnessmap,vt.metalnessmap,vt.fog,vt.lights,{emissive:{value:new ne(0)},roughness:{value:1},metalness:{value:0},envMapIntensity:{value:1}}]),vertexShader:jt.meshphysical_vert,fragmentShader:jt.meshphysical_frag},toon:{uniforms:Ye([vt.common,vt.aomap,vt.lightmap,vt.emissivemap,vt.bumpmap,vt.normalmap,vt.displacementmap,vt.gradientmap,vt.fog,vt.lights,{emissive:{value:new ne(0)}}]),vertexShader:jt.meshtoon_vert,fragmentShader:jt.meshtoon_frag},matcap:{uniforms:Ye([vt.common,vt.bumpmap,vt.normalmap,vt.displacementmap,vt.fog,{matcap:{value:null}}]),vertexShader:jt.meshmatcap_vert,fragmentShader:jt.meshmatcap_frag},points:{uniforms:Ye([vt.points,vt.fog]),vertexShader:jt.points_vert,fragmentShader:jt.points_frag},dashed:{uniforms:Ye([vt.common,vt.fog,{scale:{value:1},dashSize:{value:1},totalSize:{value:2}}]),vertexShader:jt.linedashed_vert,fragmentShader:jt.linedashed_frag},depth:{uniforms:Ye([vt.common,vt.displacementmap]),vertexShader:jt.depth_vert,fragmentShader:jt.depth_frag},normal:{uniforms:Ye([vt.common,vt.bumpmap,vt.normalmap,vt.displacementmap,{opacity:{value:1}}]),vertexShader:jt.meshnormal_vert,fragmentShader:jt.meshnormal_frag},sprite:{uniforms:Ye([vt.sprite,vt.fog]),vertexShader:jt.sprite_vert,fragmentShader:jt.sprite_frag},background:{uniforms:{uvTransform:{value:new te},t2D:{value:null},backgroundIntensity:{value:1}},vertexShader:jt.background_vert,fragmentShader:jt.background_frag},backgroundCube:{uniforms:{envMap:{value:null},flipEnvMap:{value:-1},backgroundBlurriness:{value:0},backgroundIntensity:{value:1}},vertexShader:jt.backgroundCube_vert,fragmentShader:jt.backgroundCube_frag},cube:{uniforms:{tCube:{value:null},tFlip:{value:-1},opacity:{value:1}},vertexShader:jt.cube_vert,fragmentShader:jt.cube_frag},equirect:{uniforms:{tEquirect:{value:null}},vertexShader:jt.equirect_vert,fragmentShader:jt.equirect_frag},distanceRGBA:{uniforms:Ye([vt.common,vt.displacementmap,{referencePosition:{value:new z},nearDistance:{value:1},farDistance:{value:1e3}}]),vertexShader:jt.distanceRGBA_vert,fragmentShader:jt.distanceRGBA_frag},shadow:{uniforms:Ye([vt.lights,vt.fog,{color:{value:new ne(0)},opacity:{value:1}}]),vertexShader:jt.shadow_vert,fragmentShader:jt.shadow_frag}};Un.physical={uniforms:Ye([Un.standard.uniforms,{clearcoat:{value:0},clearcoatMap:{value:null},clearcoatMapTransform:{value:new te},clearcoatNormalMap:{value:null},clearcoatNormalMapTransform:{value:new te},clearcoatNormalScale:{value:new re(1,1)},clearcoatRoughness:{value:0},clearcoatRoughnessMap:{value:null},clearcoatRoughnessMapTransform:{value:new te},iridescence:{value:0},iridescenceMap:{value:null},iridescenceMapTransform:{value:new te},iridescenceIOR:{value:1.3},iridescenceThicknessMinimum:{value:100},iridescenceThicknessMaximum:{value:400},iridescenceThicknessMap:{value:null},iridescenceThicknessMapTransform:{value:new te},sheen:{value:0},sheenColor:{value:new ne(0)},sheenColorMap:{value:null},sheenColorMapTransform:{value:new te},sheenRoughness:{value:1},sheenRoughnessMap:{value:null},sheenRoughnessMapTransform:{value:new te},transmission:{value:0},transmissionMap:{value:null},transmissionMapTransform:{value:new te},transmissionSamplerSize:{value:new re},transmissionSamplerMap:{value:null},thickness:{value:0},thicknessMap:{value:null},thicknessMapTransform:{value:new te},attenuationDistance:{value:0},attenuationColor:{value:new ne(0)},specularColor:{value:new ne(1,1,1)},specularColorMap:{value:null},specularColorMapTransform:{value:new te},specularIntensity:{value:1},specularIntensityMap:{value:null},specularIntensityMapTransform:{value:new te},anisotropyVector:{value:new re},anisotropyMap:{value:null},anisotropyMapTransform:{value:new te}}]),vertexShader:jt.meshphysical_vert,fragmentShader:jt.meshphysical_frag};const Fr={r:0,b:0,g:0};function p0(n,t,e,i,s,r,a){const o=new ne(0);let l=r===!0?0:1,c,d,h=null,p=0,f=null;function v(_,m){let E=!1,b=m.isScene===!0?m.background:null;b&&b.isTexture&&(b=(m.backgroundBlurriness>0?e:t).get(b)),b===null?g(o,l):b&&b.isColor&&(g(b,1),E=!0);const x=n.xr.getEnvironmentBlendMode();x==="additive"?i.buffers.color.setClear(0,0,0,1,a):x==="alpha-blend"&&i.buffers.color.setClear(0,0,0,0,a),(n.autoClear||E)&&n.clear(n.autoClearColor,n.autoClearDepth,n.autoClearStencil),b&&(b.isCubeTexture||b.mapping===bo)?(d===void 0&&(d=new _n(new ur(1,1,1),new Vi({name:"BackgroundCubeMaterial",uniforms:Ss(Un.backgroundCube.uniforms),vertexShader:Un.backgroundCube.vertexShader,fragmentShader:Un.backgroundCube.fragmentShader,side:tn,depthTest:!1,depthWrite:!1,fog:!1})),d.geometry.deleteAttribute("normal"),d.geometry.deleteAttribute("uv"),d.onBeforeRender=function(C,A,D){this.matrixWorld.copyPosition(D.matrixWorld)},Object.defineProperty(d.material,"envMap",{get:function(){return this.uniforms.envMap.value}}),s.update(d)),d.material.uniforms.envMap.value=b,d.material.uniforms.flipEnvMap.value=b.isCubeTexture&&b.isRenderTargetTexture===!1?-1:1,d.material.uniforms.backgroundBlurriness.value=m.backgroundBlurriness,d.material.uniforms.backgroundIntensity.value=m.backgroundIntensity,d.material.toneMapped=ce.getTransfer(b.colorSpace)!==ge,(h!==b||p!==b.version||f!==n.toneMapping)&&(d.material.needsUpdate=!0,h=b,p=b.version,f=n.toneMapping),d.layers.enableAll(),_.unshift(d,d.geometry,d.material,0,0,null)):b&&b.isTexture&&(c===void 0&&(c=new _n(new cl(2,2),new Vi({name:"BackgroundMaterial",uniforms:Ss(Un.background.uniforms),vertexShader:Un.background.vertexShader,fragmentShader:Un.background.fragmentShader,side:bi,depthTest:!1,depthWrite:!1,fog:!1})),c.geometry.deleteAttribute("normal"),Object.defineProperty(c.material,"map",{get:function(){return this.uniforms.t2D.value}}),s.update(c)),c.material.uniforms.t2D.value=b,c.material.uniforms.backgroundIntensity.value=m.backgroundIntensity,c.material.toneMapped=ce.getTransfer(b.colorSpace)!==ge,b.matrixAutoUpdate===!0&&b.updateMatrix(),c.material.uniforms.uvTransform.value.copy(b.matrix),(h!==b||p!==b.version||f!==n.toneMapping)&&(c.material.needsUpdate=!0,h=b,p=b.version,f=n.toneMapping),c.layers.enableAll(),_.unshift(c,c.geometry,c.material,0,0,null))}function g(_,m){_.getRGB(Fr,oh(n)),i.buffers.color.setClear(Fr.r,Fr.g,Fr.b,m,a)}return{getClearColor:function(){return o},setClearColor:function(_,m=1){o.set(_),l=m,g(o,l)},getClearAlpha:function(){return l},setClearAlpha:function(_){l=_,g(o,l)},render:v}}function m0(n,t,e,i){const s=n.getParameter(n.MAX_VERTEX_ATTRIBS),r=i.isWebGL2?null:t.get("OES_vertex_array_object"),a=i.isWebGL2||r!==null,o={},l=_(null);let c=l,d=!1;function h(O,Z,tt,st,et){let ot=!1;if(a){const ut=g(st,tt,Z);c!==ut&&(c=ut,f(c.object)),ot=m(O,st,tt,et),ot&&E(O,st,tt,et)}else{const ut=Z.wireframe===!0;(c.geometry!==st.id||c.program!==tt.id||c.wireframe!==ut)&&(c.geometry=st.id,c.program=tt.id,c.wireframe=ut,ot=!0)}et!==null&&e.update(et,n.ELEMENT_ARRAY_BUFFER),(ot||d)&&(d=!1,G(O,Z,tt,st),et!==null&&n.bindBuffer(n.ELEMENT_ARRAY_BUFFER,e.get(et).buffer))}function p(){return i.isWebGL2?n.createVertexArray():r.createVertexArrayOES()}function f(O){return i.isWebGL2?n.bindVertexArray(O):r.bindVertexArrayOES(O)}function v(O){return i.isWebGL2?n.deleteVertexArray(O):r.deleteVertexArrayOES(O)}function g(O,Z,tt){const st=tt.wireframe===!0;let et=o[O.id];et===void 0&&(et={},o[O.id]=et);let ot=et[Z.id];ot===void 0&&(ot={},et[Z.id]=ot);let ut=ot[st];return ut===void 0&&(ut=_(p()),ot[st]=ut),ut}function _(O){const Z=[],tt=[],st=[];for(let et=0;et<s;et++)Z[et]=0,tt[et]=0,st[et]=0;return{geometry:null,program:null,wireframe:!1,newAttributes:Z,enabledAttributes:tt,attributeDivisors:st,object:O,attributes:{},index:null}}function m(O,Z,tt,st){const et=c.attributes,ot=Z.attributes;let ut=0;const pt=tt.getAttributes();for(const ft in pt)if(pt[ft].location>=0){const ht=et[ft];let bt=ot[ft];if(bt===void 0&&(ft==="instanceMatrix"&&O.instanceMatrix&&(bt=O.instanceMatrix),ft==="instanceColor"&&O.instanceColor&&(bt=O.instanceColor)),ht===void 0||ht.attribute!==bt||bt&&ht.data!==bt.data)return!0;ut++}return c.attributesNum!==ut||c.index!==st}function E(O,Z,tt,st){const et={},ot=Z.attributes;let ut=0;const pt=tt.getAttributes();for(const ft in pt)if(pt[ft].location>=0){let ht=ot[ft];ht===void 0&&(ft==="instanceMatrix"&&O.instanceMatrix&&(ht=O.instanceMatrix),ft==="instanceColor"&&O.instanceColor&&(ht=O.instanceColor));const bt={};bt.attribute=ht,ht&&ht.data&&(bt.data=ht.data),et[ft]=bt,ut++}c.attributes=et,c.attributesNum=ut,c.index=st}function b(){const O=c.newAttributes;for(let Z=0,tt=O.length;Z<tt;Z++)O[Z]=0}function x(O){C(O,0)}function C(O,Z){const tt=c.newAttributes,st=c.enabledAttributes,et=c.attributeDivisors;tt[O]=1,st[O]===0&&(n.enableVertexAttribArray(O),st[O]=1),et[O]!==Z&&((i.isWebGL2?n:t.get("ANGLE_instanced_arrays"))[i.isWebGL2?"vertexAttribDivisor":"vertexAttribDivisorANGLE"](O,Z),et[O]=Z)}function A(){const O=c.newAttributes,Z=c.enabledAttributes;for(let tt=0,st=Z.length;tt<st;tt++)Z[tt]!==O[tt]&&(n.disableVertexAttribArray(tt),Z[tt]=0)}function D(O,Z,tt,st,et,ot,ut){ut===!0?n.vertexAttribIPointer(O,Z,tt,et,ot):n.vertexAttribPointer(O,Z,tt,st,et,ot)}function G(O,Z,tt,st){if(i.isWebGL2===!1&&(O.isInstancedMesh||st.isInstancedBufferGeometry)&&t.get("ANGLE_instanced_arrays")===null)return;b();const et=st.attributes,ot=tt.getAttributes(),ut=Z.defaultAttributeValues;for(const pt in ot){const ft=ot[pt];if(ft.location>=0){let it=et[pt];if(it===void 0&&(pt==="instanceMatrix"&&O.instanceMatrix&&(it=O.instanceMatrix),pt==="instanceColor"&&O.instanceColor&&(it=O.instanceColor)),it!==void 0){const ht=it.normalized,bt=it.itemSize,Lt=e.get(it);if(Lt===void 0)continue;const Pt=Lt.buffer,Ht=Lt.type,Gt=Lt.bytesPerElement,Nt=i.isWebGL2===!0&&(Ht===n.INT||Ht===n.UNSIGNED_INT||it.gpuType===Gd);if(it.isInterleavedBufferAttribute){const Zt=it.data,M=Zt.stride,k=it.offset;if(Zt.isInstancedInterleavedBuffer){for(let B=0;B<ft.locationSize;B++)C(ft.location+B,Zt.meshPerAttribute);O.isInstancedMesh!==!0&&st._maxInstanceCount===void 0&&(st._maxInstanceCount=Zt.meshPerAttribute*Zt.count)}else for(let B=0;B<ft.locationSize;B++)x(ft.location+B);n.bindBuffer(n.ARRAY_BUFFER,Pt);for(let B=0;B<ft.locationSize;B++)D(ft.location+B,bt/ft.locationSize,Ht,ht,M*Gt,(k+bt/ft.locationSize*B)*Gt,Nt)}else{if(it.isInstancedBufferAttribute){for(let Zt=0;Zt<ft.locationSize;Zt++)C(ft.location+Zt,it.meshPerAttribute);O.isInstancedMesh!==!0&&st._maxInstanceCount===void 0&&(st._maxInstanceCount=it.meshPerAttribute*it.count)}else for(let Zt=0;Zt<ft.locationSize;Zt++)x(ft.location+Zt);n.bindBuffer(n.ARRAY_BUFFER,Pt);for(let Zt=0;Zt<ft.locationSize;Zt++)D(ft.location+Zt,bt/ft.locationSize,Ht,ht,bt*Gt,bt/ft.locationSize*Zt*Gt,Nt)}}else if(ut!==void 0){const ht=ut[pt];if(ht!==void 0)switch(ht.length){case 2:n.vertexAttrib2fv(ft.location,ht);break;case 3:n.vertexAttrib3fv(ft.location,ht);break;case 4:n.vertexAttrib4fv(ft.location,ht);break;default:n.vertexAttrib1fv(ft.location,ht)}}}}A()}function T(){rt();for(const O in o){const Z=o[O];for(const tt in Z){const st=Z[tt];for(const et in st)v(st[et].object),delete st[et];delete Z[tt]}delete o[O]}}function R(O){if(o[O.id]===void 0)return;const Z=o[O.id];for(const tt in Z){const st=Z[tt];for(const et in st)v(st[et].object),delete st[et];delete Z[tt]}delete o[O.id]}function J(O){for(const Z in o){const tt=o[Z];if(tt[O.id]===void 0)continue;const st=tt[O.id];for(const et in st)v(st[et].object),delete st[et];delete tt[O.id]}}function rt(){mt(),d=!0,c!==l&&(c=l,f(c.object))}function mt(){l.geometry=null,l.program=null,l.wireframe=!1}return{setup:h,reset:rt,resetDefaultState:mt,dispose:T,releaseStatesOfGeometry:R,releaseStatesOfProgram:J,initAttributes:b,enableAttribute:x,disableUnusedAttributes:A}}function _0(n,t,e,i){const s=i.isWebGL2;let r;function a(d){r=d}function o(d,h){n.drawArrays(r,d,h),e.update(h,r,1)}function l(d,h,p){if(p===0)return;let f,v;if(s)f=n,v="drawArraysInstanced";else if(f=t.get("ANGLE_instanced_arrays"),v="drawArraysInstancedANGLE",f===null){console.error("THREE.WebGLBufferRenderer: using THREE.InstancedBufferGeometry but hardware does not support extension ANGLE_instanced_arrays.");return}f[v](r,d,h,p),e.update(h,r,p)}function c(d,h,p){if(p===0)return;const f=t.get("WEBGL_multi_draw");if(f===null)for(let v=0;v<p;v++)this.render(d[v],h[v]);else{f.multiDrawArraysWEBGL(r,d,0,h,0,p);let v=0;for(let g=0;g<p;g++)v+=h[g];e.update(v,r,1)}}this.setMode=a,this.render=o,this.renderInstances=l,this.renderMultiDraw=c}function g0(n,t,e){let i;function s(){if(i!==void 0)return i;if(t.has("EXT_texture_filter_anisotropic")===!0){const D=t.get("EXT_texture_filter_anisotropic");i=n.getParameter(D.MAX_TEXTURE_MAX_ANISOTROPY_EXT)}else i=0;return i}function r(D){if(D==="highp"){if(n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.HIGH_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.HIGH_FLOAT).precision>0)return"highp";D="mediump"}return D==="mediump"&&n.getShaderPrecisionFormat(n.VERTEX_SHADER,n.MEDIUM_FLOAT).precision>0&&n.getShaderPrecisionFormat(n.FRAGMENT_SHADER,n.MEDIUM_FLOAT).precision>0?"mediump":"lowp"}const a=typeof WebGL2RenderingContext<"u"&&n.constructor.name==="WebGL2RenderingContext";let o=e.precision!==void 0?e.precision:"highp";const l=r(o);l!==o&&(console.warn("THREE.WebGLRenderer:",o,"not supported, using",l,"instead."),o=l);const c=a||t.has("WEBGL_draw_buffers"),d=e.logarithmicDepthBuffer===!0,h=n.getParameter(n.MAX_TEXTURE_IMAGE_UNITS),p=n.getParameter(n.MAX_VERTEX_TEXTURE_IMAGE_UNITS),f=n.getParameter(n.MAX_TEXTURE_SIZE),v=n.getParameter(n.MAX_CUBE_MAP_TEXTURE_SIZE),g=n.getParameter(n.MAX_VERTEX_ATTRIBS),_=n.getParameter(n.MAX_VERTEX_UNIFORM_VECTORS),m=n.getParameter(n.MAX_VARYING_VECTORS),E=n.getParameter(n.MAX_FRAGMENT_UNIFORM_VECTORS),b=p>0,x=a||t.has("OES_texture_float"),C=b&&x,A=a?n.getParameter(n.MAX_SAMPLES):0;return{isWebGL2:a,drawBuffers:c,getMaxAnisotropy:s,getMaxPrecision:r,precision:o,logarithmicDepthBuffer:d,maxTextures:h,maxVertexTextures:p,maxTextureSize:f,maxCubemapSize:v,maxAttributes:g,maxVertexUniforms:_,maxVaryings:m,maxFragmentUniforms:E,vertexTextures:b,floatFragmentTextures:x,floatVertexTextures:C,maxSamples:A}}function v0(n){const t=this;let e=null,i=0,s=!1,r=!1;const a=new Di,o=new te,l={value:null,needsUpdate:!1};this.uniform=l,this.numPlanes=0,this.numIntersection=0,this.init=function(h,p){const f=h.length!==0||p||i!==0||s;return s=p,i=h.length,f},this.beginShadows=function(){r=!0,d(null)},this.endShadows=function(){r=!1},this.setGlobalState=function(h,p){e=d(h,p,0)},this.setState=function(h,p,f){const v=h.clippingPlanes,g=h.clipIntersection,_=h.clipShadows,m=n.get(h);if(!s||v===null||v.length===0||r&&!_)r?d(null):c();else{const E=r?0:i,b=E*4;let x=m.clippingState||null;l.value=x,x=d(v,p,b,f);for(let C=0;C!==b;++C)x[C]=e[C];m.clippingState=x,this.numIntersection=g?this.numPlanes:0,this.numPlanes+=E}};function c(){l.value!==e&&(l.value=e,l.needsUpdate=i>0),t.numPlanes=i,t.numIntersection=0}function d(h,p,f,v){const g=h!==null?h.length:0;let _=null;if(g!==0){if(_=l.value,v!==!0||_===null){const m=f+g*4,E=p.matrixWorldInverse;o.getNormalMatrix(E),(_===null||_.length<m)&&(_=new Float32Array(m));for(let b=0,x=f;b!==g;++b,x+=4)a.copy(h[b]).applyMatrix4(E,o),a.normal.toArray(_,x),_[x+3]=a.constant}l.value=_,l.needsUpdate=!0}return t.numPlanes=g,t.numIntersection=0,_}}function x0(n){let t=new WeakMap;function e(a,o){return o===Da?a.mapping=ys:o===Ua&&(a.mapping=bs),a}function i(a){if(a&&a.isTexture){const o=a.mapping;if(o===Da||o===Ua)if(t.has(a)){const l=t.get(a).texture;return e(l,a.mapping)}else{const l=a.image;if(l&&l.height>0){const c=new P_(l.height/2);return c.fromEquirectangularTexture(n,a),t.set(a,c),a.addEventListener("dispose",s),e(c.texture,a.mapping)}else return null}}return a}function s(a){const o=a.target;o.removeEventListener("dispose",s);const l=t.get(o);l!==void 0&&(t.delete(o),l.dispose())}function r(){t=new WeakMap}return{get:i,dispose:r}}class y0 extends ah{constructor(t=-1,e=1,i=1,s=-1,r=.1,a=2e3){super(),this.isOrthographicCamera=!0,this.type="OrthographicCamera",this.zoom=1,this.view=null,this.left=t,this.right=e,this.top=i,this.bottom=s,this.near=r,this.far=a,this.updateProjectionMatrix()}copy(t,e){return super.copy(t,e),this.left=t.left,this.right=t.right,this.top=t.top,this.bottom=t.bottom,this.near=t.near,this.far=t.far,this.zoom=t.zoom,this.view=t.view===null?null:Object.assign({},t.view),this}setViewOffset(t,e,i,s,r,a){this.view===null&&(this.view={enabled:!0,fullWidth:1,fullHeight:1,offsetX:0,offsetY:0,width:1,height:1}),this.view.enabled=!0,this.view.fullWidth=t,this.view.fullHeight=e,this.view.offsetX=i,this.view.offsetY=s,this.view.width=r,this.view.height=a,this.updateProjectionMatrix()}clearViewOffset(){this.view!==null&&(this.view.enabled=!1),this.updateProjectionMatrix()}updateProjectionMatrix(){const t=(this.right-this.left)/(2*this.zoom),e=(this.top-this.bottom)/(2*this.zoom),i=(this.right+this.left)/2,s=(this.top+this.bottom)/2;let r=i-t,a=i+t,o=s+e,l=s-e;if(this.view!==null&&this.view.enabled){const c=(this.right-this.left)/this.view.fullWidth/this.zoom,d=(this.top-this.bottom)/this.view.fullHeight/this.zoom;r+=c*this.view.offsetX,a=r+c*this.view.width,o-=d*this.view.offsetY,l=o-d*this.view.height}this.projectionMatrix.makeOrthographic(r,a,o,l,this.near,this.far,this.coordinateSystem),this.projectionMatrixInverse.copy(this.projectionMatrix).invert()}toJSON(t){const e=super.toJSON(t);return e.object.zoom=this.zoom,e.object.left=this.left,e.object.right=this.right,e.object.top=this.top,e.object.bottom=this.bottom,e.object.near=this.near,e.object.far=this.far,this.view!==null&&(e.object.view=Object.assign({},this.view)),e}}const ls=4,tu=[.125,.215,.35,.446,.526,.582],Ni=20,ca=new y0,eu=new ne;let ua=null,da=0,ha=0;const Ui=(1+Math.sqrt(5))/2,rs=1/Ui,nu=[new z(1,1,1),new z(-1,1,1),new z(1,1,-1),new z(-1,1,-1),new z(0,Ui,rs),new z(0,Ui,-rs),new z(rs,0,Ui),new z(-rs,0,Ui),new z(Ui,rs,0),new z(-Ui,rs,0)];class iu{constructor(t){this._renderer=t,this._pingPongRenderTarget=null,this._lodMax=0,this._cubeSize=0,this._lodPlanes=[],this._sizeLods=[],this._sigmas=[],this._blurMaterial=null,this._cubemapMaterial=null,this._equirectMaterial=null,this._compileMaterial(this._blurMaterial)}fromScene(t,e=0,i=.1,s=100){ua=this._renderer.getRenderTarget(),da=this._renderer.getActiveCubeFace(),ha=this._renderer.getActiveMipmapLevel(),this._setSize(256);const r=this._allocateTargets();return r.depthBuffer=!0,this._sceneToCubeUV(t,i,s,r),e>0&&this._blur(r,0,0,e),this._applyPMREM(r),this._cleanup(r),r}fromEquirectangular(t,e=null){return this._fromTexture(t,e)}fromCubemap(t,e=null){return this._fromTexture(t,e)}compileCubemapShader(){this._cubemapMaterial===null&&(this._cubemapMaterial=ou(),this._compileMaterial(this._cubemapMaterial))}compileEquirectangularShader(){this._equirectMaterial===null&&(this._equirectMaterial=ru(),this._compileMaterial(this._equirectMaterial))}dispose(){this._dispose(),this._cubemapMaterial!==null&&this._cubemapMaterial.dispose(),this._equirectMaterial!==null&&this._equirectMaterial.dispose()}_setSize(t){this._lodMax=Math.floor(Math.log2(t)),this._cubeSize=Math.pow(2,this._lodMax)}_dispose(){this._blurMaterial!==null&&this._blurMaterial.dispose(),this._pingPongRenderTarget!==null&&this._pingPongRenderTarget.dispose();for(let t=0;t<this._lodPlanes.length;t++)this._lodPlanes[t].dispose()}_cleanup(t){this._renderer.setRenderTarget(ua,da,ha),t.scissorTest=!1,Or(t,0,0,t.width,t.height)}_fromTexture(t,e){t.mapping===ys||t.mapping===bs?this._setSize(t.image.length===0?16:t.image[0].width||t.image[0].image.width):this._setSize(t.image.width/4),ua=this._renderer.getRenderTarget(),da=this._renderer.getActiveCubeFace(),ha=this._renderer.getActiveMipmapLevel();const i=e||this._allocateTargets();return this._textureToCubeUV(t,i),this._applyPMREM(i),this._cleanup(i),i}_allocateTargets(){const t=3*Math.max(this._cubeSize,112),e=4*this._cubeSize,i={magFilter:pn,minFilter:pn,generateMipmaps:!1,type:er,format:En,colorSpace:ri,depthBuffer:!1},s=su(t,e,i);if(this._pingPongRenderTarget===null||this._pingPongRenderTarget.width!==t||this._pingPongRenderTarget.height!==e){this._pingPongRenderTarget!==null&&this._dispose(),this._pingPongRenderTarget=su(t,e,i);const{_lodMax:r}=this;({sizeLods:this._sizeLods,lodPlanes:this._lodPlanes,sigmas:this._sigmas}=b0(r)),this._blurMaterial=M0(r,t,e)}return s}_compileMaterial(t){const e=new _n(this._lodPlanes[0],t);this._renderer.compile(e,ca)}_sceneToCubeUV(t,e,i,s){const o=new ln(90,1,e,i),l=[1,-1,1,1,1,1],c=[1,1,1,-1,-1,-1],d=this._renderer,h=d.autoClear,p=d.toneMapping;d.getClearColor(eu),d.toneMapping=_i,d.autoClear=!1;const f=new as({name:"PMREM.Background",side:tn,depthWrite:!1,depthTest:!1}),v=new _n(new ur,f);let g=!1;const _=t.background;_?_.isColor&&(f.color.copy(_),t.background=null,g=!0):(f.color.copy(eu),g=!0);for(let m=0;m<6;m++){const E=m%3;E===0?(o.up.set(0,l[m],0),o.lookAt(c[m],0,0)):E===1?(o.up.set(0,0,l[m]),o.lookAt(0,c[m],0)):(o.up.set(0,l[m],0),o.lookAt(0,0,c[m]));const b=this._cubeSize;Or(s,E*b,m>2?b:0,b,b),d.setRenderTarget(s),g&&d.render(v,o),d.render(t,o)}v.geometry.dispose(),v.material.dispose(),d.toneMapping=p,d.autoClear=h,t.background=_}_textureToCubeUV(t,e){const i=this._renderer,s=t.mapping===ys||t.mapping===bs;s?(this._cubemapMaterial===null&&(this._cubemapMaterial=ou()),this._cubemapMaterial.uniforms.flipEnvMap.value=t.isRenderTargetTexture===!1?-1:1):this._equirectMaterial===null&&(this._equirectMaterial=ru());const r=s?this._cubemapMaterial:this._equirectMaterial,a=new _n(this._lodPlanes[0],r),o=r.uniforms;o.envMap.value=t;const l=this._cubeSize;Or(e,0,0,3*l,2*l),i.setRenderTarget(e),i.render(a,ca)}_applyPMREM(t){const e=this._renderer,i=e.autoClear;e.autoClear=!1;for(let s=1;s<this._lodPlanes.length;s++){const r=Math.sqrt(this._sigmas[s]*this._sigmas[s]-this._sigmas[s-1]*this._sigmas[s-1]),a=nu[(s-1)%nu.length];this._blur(t,s-1,s,r,a)}e.autoClear=i}_blur(t,e,i,s,r){const a=this._pingPongRenderTarget;this._halfBlur(t,a,e,i,s,"latitudinal",r),this._halfBlur(a,t,i,i,s,"longitudinal",r)}_halfBlur(t,e,i,s,r,a,o){const l=this._renderer,c=this._blurMaterial;a!=="latitudinal"&&a!=="longitudinal"&&console.error("blur direction must be either latitudinal or longitudinal!");const d=3,h=new _n(this._lodPlanes[s],c),p=c.uniforms,f=this._sizeLods[i]-1,v=isFinite(r)?Math.PI/(2*f):2*Math.PI/(2*Ni-1),g=r/v,_=isFinite(r)?1+Math.floor(d*g):Ni;_>Ni&&console.warn(`sigmaRadians, ${r}, is too large and will clip, as it requested ${_} samples when the maximum is set to ${Ni}`);const m=[];let E=0;for(let D=0;D<Ni;++D){const G=D/g,T=Math.exp(-G*G/2);m.push(T),D===0?E+=T:D<_&&(E+=2*T)}for(let D=0;D<m.length;D++)m[D]=m[D]/E;p.envMap.value=t.texture,p.samples.value=_,p.weights.value=m,p.latitudinal.value=a==="latitudinal",o&&(p.poleAxis.value=o);const{_lodMax:b}=this;p.dTheta.value=v,p.mipInt.value=b-i;const x=this._sizeLods[s],C=3*x*(s>b-ls?s-b+ls:0),A=4*(this._cubeSize-x);Or(e,C,A,3*x,2*x),l.setRenderTarget(e),l.render(h,ca)}}function b0(n){const t=[],e=[],i=[];let s=n;const r=n-ls+1+tu.length;for(let a=0;a<r;a++){const o=Math.pow(2,s);e.push(o);let l=1/o;a>n-ls?l=tu[a-n+ls-1]:a===0&&(l=0),i.push(l);const c=1/(o-2),d=-c,h=1+c,p=[d,d,h,d,h,h,d,d,h,h,d,h],f=6,v=6,g=3,_=2,m=1,E=new Float32Array(g*v*f),b=new Float32Array(_*v*f),x=new Float32Array(m*v*f);for(let A=0;A<f;A++){const D=A%3*2/3-1,G=A>2?0:-1,T=[D,G,0,D+2/3,G,0,D+2/3,G+1,0,D,G,0,D+2/3,G+1,0,D,G+1,0];E.set(T,g*v*A),b.set(p,_*v*A);const R=[A,A,A,A,A,A];x.set(R,m*v*A)}const C=new Ve;C.setAttribute("position",new Ne(E,g)),C.setAttribute("uv",new Ne(b,_)),C.setAttribute("faceIndex",new Ne(x,m)),t.push(C),s>ls&&s--}return{lodPlanes:t,sizeLods:e,sigmas:i}}function su(n,t,e){const i=new Hi(n,t,e);return i.texture.mapping=bo,i.texture.name="PMREM.cubeUv",i.scissorTest=!0,i}function Or(n,t,e,i,s){n.viewport.set(t,e,i,s),n.scissor.set(t,e,i,s)}function M0(n,t,e){const i=new Float32Array(Ni),s=new z(0,1,0);return new Vi({name:"SphericalGaussianBlur",defines:{n:Ni,CUBEUV_TEXEL_WIDTH:1/t,CUBEUV_TEXEL_HEIGHT:1/e,CUBEUV_MAX_MIP:`${n}.0`},uniforms:{envMap:{value:null},samples:{value:1},weights:{value:i},latitudinal:{value:!1},dTheta:{value:0},mipInt:{value:0},poleAxis:{value:s}},vertexShader:ul(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;
			uniform int samples;
			uniform float weights[ n ];
			uniform bool latitudinal;
			uniform float dTheta;
			uniform float mipInt;
			uniform vec3 poleAxis;

			#define ENVMAP_TYPE_CUBE_UV
			#include <cube_uv_reflection_fragment>

			vec3 getSample( float theta, vec3 axis ) {

				float cosTheta = cos( theta );
				// Rodrigues' axis-angle rotation
				vec3 sampleDirection = vOutputDirection * cosTheta
					+ cross( axis, vOutputDirection ) * sin( theta )
					+ axis * dot( axis, vOutputDirection ) * ( 1.0 - cosTheta );

				return bilinearCubeUV( envMap, sampleDirection, mipInt );

			}

			void main() {

				vec3 axis = latitudinal ? poleAxis : cross( poleAxis, vOutputDirection );

				if ( all( equal( axis, vec3( 0.0 ) ) ) ) {

					axis = vec3( vOutputDirection.z, 0.0, - vOutputDirection.x );

				}

				axis = normalize( axis );

				gl_FragColor = vec4( 0.0, 0.0, 0.0, 1.0 );
				gl_FragColor.rgb += weights[ 0 ] * getSample( 0.0, axis );

				for ( int i = 1; i < n; i++ ) {

					if ( i >= samples ) {

						break;

					}

					float theta = dTheta * float( i );
					gl_FragColor.rgb += weights[ i ] * getSample( -1.0 * theta, axis );
					gl_FragColor.rgb += weights[ i ] * getSample( theta, axis );

				}

			}
		`,blending:mi,depthTest:!1,depthWrite:!1})}function ru(){return new Vi({name:"EquirectangularToCubeUV",uniforms:{envMap:{value:null}},vertexShader:ul(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			varying vec3 vOutputDirection;

			uniform sampler2D envMap;

			#include <common>

			void main() {

				vec3 outputDirection = normalize( vOutputDirection );
				vec2 uv = equirectUv( outputDirection );

				gl_FragColor = vec4( texture2D ( envMap, uv ).rgb, 1.0 );

			}
		`,blending:mi,depthTest:!1,depthWrite:!1})}function ou(){return new Vi({name:"CubemapToCubeUV",uniforms:{envMap:{value:null},flipEnvMap:{value:-1}},vertexShader:ul(),fragmentShader:`

			precision mediump float;
			precision mediump int;

			uniform float flipEnvMap;

			varying vec3 vOutputDirection;

			uniform samplerCube envMap;

			void main() {

				gl_FragColor = textureCube( envMap, vec3( flipEnvMap * vOutputDirection.x, vOutputDirection.yz ) );

			}
		`,blending:mi,depthTest:!1,depthWrite:!1})}function ul(){return`

		precision mediump float;
		precision mediump int;

		attribute float faceIndex;

		varying vec3 vOutputDirection;

		// RH coordinate system; PMREM face-indexing convention
		vec3 getDirection( vec2 uv, float face ) {

			uv = 2.0 * uv - 1.0;

			vec3 direction = vec3( uv, 1.0 );

			if ( face == 0.0 ) {

				direction = direction.zyx; // ( 1, v, u ) pos x

			} else if ( face == 1.0 ) {

				direction = direction.xzy;
				direction.xz *= -1.0; // ( -u, 1, -v ) pos y

			} else if ( face == 2.0 ) {

				direction.x *= -1.0; // ( -u, v, 1 ) pos z

			} else if ( face == 3.0 ) {

				direction = direction.zyx;
				direction.xz *= -1.0; // ( -1, v, -u ) neg x

			} else if ( face == 4.0 ) {

				direction = direction.xzy;
				direction.xy *= -1.0; // ( -u, -1, v ) neg y

			} else if ( face == 5.0 ) {

				direction.z *= -1.0; // ( u, v, -1 ) neg z

			}

			return direction;

		}

		void main() {

			vOutputDirection = getDirection( uv, faceIndex );
			gl_Position = vec4( position, 1.0 );

		}
	`}function S0(n){let t=new WeakMap,e=null;function i(o){if(o&&o.isTexture){const l=o.mapping,c=l===Da||l===Ua,d=l===ys||l===bs;if(c||d)if(o.isRenderTargetTexture&&o.needsPMREMUpdate===!0){o.needsPMREMUpdate=!1;let h=t.get(o);return e===null&&(e=new iu(n)),h=c?e.fromEquirectangular(o,h):e.fromCubemap(o,h),t.set(o,h),h.texture}else{if(t.has(o))return t.get(o).texture;{const h=o.image;if(c&&h&&h.height>0||d&&h&&s(h)){e===null&&(e=new iu(n));const p=c?e.fromEquirectangular(o):e.fromCubemap(o);return t.set(o,p),o.addEventListener("dispose",r),p.texture}else return null}}}return o}function s(o){let l=0;const c=6;for(let d=0;d<c;d++)o[d]!==void 0&&l++;return l===c}function r(o){const l=o.target;l.removeEventListener("dispose",r);const c=t.get(l);c!==void 0&&(t.delete(l),c.dispose())}function a(){t=new WeakMap,e!==null&&(e.dispose(),e=null)}return{get:i,dispose:a}}function E0(n){const t={};function e(i){if(t[i]!==void 0)return t[i];let s;switch(i){case"WEBGL_depth_texture":s=n.getExtension("WEBGL_depth_texture")||n.getExtension("MOZ_WEBGL_depth_texture")||n.getExtension("WEBKIT_WEBGL_depth_texture");break;case"EXT_texture_filter_anisotropic":s=n.getExtension("EXT_texture_filter_anisotropic")||n.getExtension("MOZ_EXT_texture_filter_anisotropic")||n.getExtension("WEBKIT_EXT_texture_filter_anisotropic");break;case"WEBGL_compressed_texture_s3tc":s=n.getExtension("WEBGL_compressed_texture_s3tc")||n.getExtension("MOZ_WEBGL_compressed_texture_s3tc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_s3tc");break;case"WEBGL_compressed_texture_pvrtc":s=n.getExtension("WEBGL_compressed_texture_pvrtc")||n.getExtension("WEBKIT_WEBGL_compressed_texture_pvrtc");break;default:s=n.getExtension(i)}return t[i]=s,s}return{has:function(i){return e(i)!==null},init:function(i){i.isWebGL2?(e("EXT_color_buffer_float"),e("WEBGL_clip_cull_distance")):(e("WEBGL_depth_texture"),e("OES_texture_float"),e("OES_texture_half_float"),e("OES_texture_half_float_linear"),e("OES_standard_derivatives"),e("OES_element_index_uint"),e("OES_vertex_array_object"),e("ANGLE_instanced_arrays")),e("OES_texture_float_linear"),e("EXT_color_buffer_half_float"),e("WEBGL_multisampled_render_to_texture")},get:function(i){const s=e(i);return s===null&&console.warn("THREE.WebGLRenderer: "+i+" extension not supported."),s}}}function T0(n,t,e,i){const s={},r=new WeakMap;function a(h){const p=h.target;p.index!==null&&t.remove(p.index);for(const v in p.attributes)t.remove(p.attributes[v]);for(const v in p.morphAttributes){const g=p.morphAttributes[v];for(let _=0,m=g.length;_<m;_++)t.remove(g[_])}p.removeEventListener("dispose",a),delete s[p.id];const f=r.get(p);f&&(t.remove(f),r.delete(p)),i.releaseStatesOfGeometry(p),p.isInstancedBufferGeometry===!0&&delete p._maxInstanceCount,e.memory.geometries--}function o(h,p){return s[p.id]===!0||(p.addEventListener("dispose",a),s[p.id]=!0,e.memory.geometries++),p}function l(h){const p=h.attributes;for(const v in p)t.update(p[v],n.ARRAY_BUFFER);const f=h.morphAttributes;for(const v in f){const g=f[v];for(let _=0,m=g.length;_<m;_++)t.update(g[_],n.ARRAY_BUFFER)}}function c(h){const p=[],f=h.index,v=h.attributes.position;let g=0;if(f!==null){const E=f.array;g=f.version;for(let b=0,x=E.length;b<x;b+=3){const C=E[b+0],A=E[b+1],D=E[b+2];p.push(C,A,A,D,D,C)}}else if(v!==void 0){const E=v.array;g=v.version;for(let b=0,x=E.length/3-1;b<x;b+=3){const C=b+0,A=b+1,D=b+2;p.push(C,A,A,D,D,C)}}else return;const _=new(Jd(p)?rh:sh)(p,1);_.version=g;const m=r.get(h);m&&t.remove(m),r.set(h,_)}function d(h){const p=r.get(h);if(p){const f=h.index;f!==null&&p.version<f.version&&c(h)}else c(h);return r.get(h)}return{get:o,update:l,getWireframeAttribute:d}}function w0(n,t,e,i){const s=i.isWebGL2;let r;function a(f){r=f}let o,l;function c(f){o=f.type,l=f.bytesPerElement}function d(f,v){n.drawElements(r,v,o,f*l),e.update(v,r,1)}function h(f,v,g){if(g===0)return;let _,m;if(s)_=n,m="drawElementsInstanced";else if(_=t.get("ANGLE_instanced_arrays"),m="drawElementsInstancedANGLE",_===null){console.error("THREE.WebGLIndexedBufferRenderer: using THREE.InstancedBufferGeometry but hardware does not support extension ANGLE_instanced_arrays.");return}_[m](r,v,o,f*l,g),e.update(v,r,g)}function p(f,v,g){if(g===0)return;const _=t.get("WEBGL_multi_draw");if(_===null)for(let m=0;m<g;m++)this.render(f[m]/l,v[m]);else{_.multiDrawElementsWEBGL(r,v,0,o,f,0,g);let m=0;for(let E=0;E<g;E++)m+=v[E];e.update(m,r,1)}}this.setMode=a,this.setIndex=c,this.render=d,this.renderInstances=h,this.renderMultiDraw=p}function A0(n){const t={geometries:0,textures:0},e={frame:0,calls:0,triangles:0,points:0,lines:0};function i(r,a,o){switch(e.calls++,a){case n.TRIANGLES:e.triangles+=o*(r/3);break;case n.LINES:e.lines+=o*(r/2);break;case n.LINE_STRIP:e.lines+=o*(r-1);break;case n.LINE_LOOP:e.lines+=o*r;break;case n.POINTS:e.points+=o*r;break;default:console.error("THREE.WebGLInfo: Unknown draw mode:",a);break}}function s(){e.calls=0,e.triangles=0,e.points=0,e.lines=0}return{memory:t,render:e,programs:null,autoReset:!0,reset:s,update:i}}function C0(n,t){return n[0]-t[0]}function R0(n,t){return Math.abs(t[1])-Math.abs(n[1])}function L0(n,t,e){const i={},s=new Float32Array(8),r=new WeakMap,a=new xe,o=[];for(let c=0;c<8;c++)o[c]=[c,0];function l(c,d,h){const p=c.morphTargetInfluences;if(t.isWebGL2===!0){const v=d.morphAttributes.position||d.morphAttributes.normal||d.morphAttributes.color,g=v!==void 0?v.length:0;let _=r.get(d);if(_===void 0||_.count!==g){let Z=function(){mt.dispose(),r.delete(d),d.removeEventListener("dispose",Z)};var f=Z;_!==void 0&&_.texture.dispose();const b=d.morphAttributes.position!==void 0,x=d.morphAttributes.normal!==void 0,C=d.morphAttributes.color!==void 0,A=d.morphAttributes.position||[],D=d.morphAttributes.normal||[],G=d.morphAttributes.color||[];let T=0;b===!0&&(T=1),x===!0&&(T=2),C===!0&&(T=3);let R=d.attributes.position.count*T,J=1;R>t.maxTextureSize&&(J=Math.ceil(R/t.maxTextureSize),R=t.maxTextureSize);const rt=new Float32Array(R*J*4*g),mt=new eh(rt,R,J,g);mt.type=pi,mt.needsUpdate=!0;const O=T*4;for(let tt=0;tt<g;tt++){const st=A[tt],et=D[tt],ot=G[tt],ut=R*J*4*tt;for(let pt=0;pt<st.count;pt++){const ft=pt*O;b===!0&&(a.fromBufferAttribute(st,pt),rt[ut+ft+0]=a.x,rt[ut+ft+1]=a.y,rt[ut+ft+2]=a.z,rt[ut+ft+3]=0),x===!0&&(a.fromBufferAttribute(et,pt),rt[ut+ft+4]=a.x,rt[ut+ft+5]=a.y,rt[ut+ft+6]=a.z,rt[ut+ft+7]=0),C===!0&&(a.fromBufferAttribute(ot,pt),rt[ut+ft+8]=a.x,rt[ut+ft+9]=a.y,rt[ut+ft+10]=a.z,rt[ut+ft+11]=ot.itemSize===4?a.w:1)}}_={count:g,texture:mt,size:new re(R,J)},r.set(d,_),d.addEventListener("dispose",Z)}let m=0;for(let b=0;b<p.length;b++)m+=p[b];const E=d.morphTargetsRelative?1:1-m;h.getUniforms().setValue(n,"morphTargetBaseInfluence",E),h.getUniforms().setValue(n,"morphTargetInfluences",p),h.getUniforms().setValue(n,"morphTargetsTexture",_.texture,e),h.getUniforms().setValue(n,"morphTargetsTextureSize",_.size)}else{const v=p===void 0?0:p.length;let g=i[d.id];if(g===void 0||g.length!==v){g=[];for(let x=0;x<v;x++)g[x]=[x,0];i[d.id]=g}for(let x=0;x<v;x++){const C=g[x];C[0]=x,C[1]=p[x]}g.sort(R0);for(let x=0;x<8;x++)x<v&&g[x][1]?(o[x][0]=g[x][0],o[x][1]=g[x][1]):(o[x][0]=Number.MAX_SAFE_INTEGER,o[x][1]=0);o.sort(C0);const _=d.morphAttributes.position,m=d.morphAttributes.normal;let E=0;for(let x=0;x<8;x++){const C=o[x],A=C[0],D=C[1];A!==Number.MAX_SAFE_INTEGER&&D?(_&&d.getAttribute("morphTarget"+x)!==_[A]&&d.setAttribute("morphTarget"+x,_[A]),m&&d.getAttribute("morphNormal"+x)!==m[A]&&d.setAttribute("morphNormal"+x,m[A]),s[x]=D,E+=D):(_&&d.hasAttribute("morphTarget"+x)===!0&&d.deleteAttribute("morphTarget"+x),m&&d.hasAttribute("morphNormal"+x)===!0&&d.deleteAttribute("morphNormal"+x),s[x]=0)}const b=d.morphTargetsRelative?1:1-E;h.getUniforms().setValue(n,"morphTargetBaseInfluence",b),h.getUniforms().setValue(n,"morphTargetInfluences",s)}}return{update:l}}function P0(n,t,e,i){let s=new WeakMap;function r(l){const c=i.render.frame,d=l.geometry,h=t.get(l,d);if(s.get(h)!==c&&(t.update(h),s.set(h,c)),l.isInstancedMesh&&(l.hasEventListener("dispose",o)===!1&&l.addEventListener("dispose",o),s.get(l)!==c&&(e.update(l.instanceMatrix,n.ARRAY_BUFFER),l.instanceColor!==null&&e.update(l.instanceColor,n.ARRAY_BUFFER),s.set(l,c))),l.isSkinnedMesh){const p=l.skeleton;s.get(p)!==c&&(p.update(),s.set(p,c))}return h}function a(){s=new WeakMap}function o(l){const c=l.target;c.removeEventListener("dispose",o),e.remove(c.instanceMatrix),c.instanceColor!==null&&e.remove(c.instanceColor)}return{update:r,dispose:a}}class uh extends dn{constructor(t,e,i,s,r,a,o,l,c,d){if(d=d!==void 0?d:Bi,d!==Bi&&d!==Ms)throw new Error("DepthTexture format must be either THREE.DepthFormat or THREE.DepthStencilFormat");i===void 0&&d===Bi&&(i=fi),i===void 0&&d===Ms&&(i=Oi),super(null,s,r,a,o,l,d,i,c),this.isDepthTexture=!0,this.image={width:t,height:e},this.magFilter=o!==void 0?o:Ke,this.minFilter=l!==void 0?l:Ke,this.flipY=!1,this.generateMipmaps=!1,this.compareFunction=null}copy(t){return super.copy(t),this.compareFunction=t.compareFunction,this}toJSON(t){const e=super.toJSON(t);return this.compareFunction!==null&&(e.compareFunction=this.compareFunction),e}}const dh=new dn,hh=new uh(1,1);hh.compareFunction=Zd;const fh=new eh,ph=new p_,mh=new lh,au=[],lu=[],cu=new Float32Array(16),uu=new Float32Array(9),du=new Float32Array(4);function As(n,t,e){const i=n[0];if(i<=0||i>0)return n;const s=t*e;let r=au[s];if(r===void 0&&(r=new Float32Array(s),au[s]=r),t!==0){i.toArray(r,0);for(let a=1,o=0;a!==t;++a)o+=e,n[a].toArray(r,o)}return r}function Ce(n,t){if(n.length!==t.length)return!1;for(let e=0,i=n.length;e<i;e++)if(n[e]!==t[e])return!1;return!0}function Re(n,t){for(let e=0,i=t.length;e<i;e++)n[e]=t[e]}function Eo(n,t){let e=lu[t];e===void 0&&(e=new Int32Array(t),lu[t]=e);for(let i=0;i!==t;++i)e[i]=n.allocateTextureUnit();return e}function D0(n,t){const e=this.cache;e[0]!==t&&(n.uniform1f(this.addr,t),e[0]=t)}function U0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2f(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Ce(e,t))return;n.uniform2fv(this.addr,t),Re(e,t)}}function I0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3f(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else if(t.r!==void 0)(e[0]!==t.r||e[1]!==t.g||e[2]!==t.b)&&(n.uniform3f(this.addr,t.r,t.g,t.b),e[0]=t.r,e[1]=t.g,e[2]=t.b);else{if(Ce(e,t))return;n.uniform3fv(this.addr,t),Re(e,t)}}function N0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4f(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Ce(e,t))return;n.uniform4fv(this.addr,t),Re(e,t)}}function k0(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Ce(e,t))return;n.uniformMatrix2fv(this.addr,!1,t),Re(e,t)}else{if(Ce(e,i))return;du.set(i),n.uniformMatrix2fv(this.addr,!1,du),Re(e,i)}}function F0(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Ce(e,t))return;n.uniformMatrix3fv(this.addr,!1,t),Re(e,t)}else{if(Ce(e,i))return;uu.set(i),n.uniformMatrix3fv(this.addr,!1,uu),Re(e,i)}}function O0(n,t){const e=this.cache,i=t.elements;if(i===void 0){if(Ce(e,t))return;n.uniformMatrix4fv(this.addr,!1,t),Re(e,t)}else{if(Ce(e,i))return;cu.set(i),n.uniformMatrix4fv(this.addr,!1,cu),Re(e,i)}}function B0(n,t){const e=this.cache;e[0]!==t&&(n.uniform1i(this.addr,t),e[0]=t)}function z0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2i(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Ce(e,t))return;n.uniform2iv(this.addr,t),Re(e,t)}}function H0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3i(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(Ce(e,t))return;n.uniform3iv(this.addr,t),Re(e,t)}}function V0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4i(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Ce(e,t))return;n.uniform4iv(this.addr,t),Re(e,t)}}function G0(n,t){const e=this.cache;e[0]!==t&&(n.uniform1ui(this.addr,t),e[0]=t)}function W0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y)&&(n.uniform2ui(this.addr,t.x,t.y),e[0]=t.x,e[1]=t.y);else{if(Ce(e,t))return;n.uniform2uiv(this.addr,t),Re(e,t)}}function $0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z)&&(n.uniform3ui(this.addr,t.x,t.y,t.z),e[0]=t.x,e[1]=t.y,e[2]=t.z);else{if(Ce(e,t))return;n.uniform3uiv(this.addr,t),Re(e,t)}}function X0(n,t){const e=this.cache;if(t.x!==void 0)(e[0]!==t.x||e[1]!==t.y||e[2]!==t.z||e[3]!==t.w)&&(n.uniform4ui(this.addr,t.x,t.y,t.z,t.w),e[0]=t.x,e[1]=t.y,e[2]=t.z,e[3]=t.w);else{if(Ce(e,t))return;n.uniform4uiv(this.addr,t),Re(e,t)}}function q0(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s);const r=this.type===n.SAMPLER_2D_SHADOW?hh:dh;e.setTexture2D(t||r,s)}function j0(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTexture3D(t||ph,s)}function Y0(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTextureCube(t||mh,s)}function K0(n,t,e){const i=this.cache,s=e.allocateTextureUnit();i[0]!==s&&(n.uniform1i(this.addr,s),i[0]=s),e.setTexture2DArray(t||fh,s)}function Z0(n){switch(n){case 5126:return D0;case 35664:return U0;case 35665:return I0;case 35666:return N0;case 35674:return k0;case 35675:return F0;case 35676:return O0;case 5124:case 35670:return B0;case 35667:case 35671:return z0;case 35668:case 35672:return H0;case 35669:case 35673:return V0;case 5125:return G0;case 36294:return W0;case 36295:return $0;case 36296:return X0;case 35678:case 36198:case 36298:case 36306:case 35682:return q0;case 35679:case 36299:case 36307:return j0;case 35680:case 36300:case 36308:case 36293:return Y0;case 36289:case 36303:case 36311:case 36292:return K0}}function J0(n,t){n.uniform1fv(this.addr,t)}function Q0(n,t){const e=As(t,this.size,2);n.uniform2fv(this.addr,e)}function tx(n,t){const e=As(t,this.size,3);n.uniform3fv(this.addr,e)}function ex(n,t){const e=As(t,this.size,4);n.uniform4fv(this.addr,e)}function nx(n,t){const e=As(t,this.size,4);n.uniformMatrix2fv(this.addr,!1,e)}function ix(n,t){const e=As(t,this.size,9);n.uniformMatrix3fv(this.addr,!1,e)}function sx(n,t){const e=As(t,this.size,16);n.uniformMatrix4fv(this.addr,!1,e)}function rx(n,t){n.uniform1iv(this.addr,t)}function ox(n,t){n.uniform2iv(this.addr,t)}function ax(n,t){n.uniform3iv(this.addr,t)}function lx(n,t){n.uniform4iv(this.addr,t)}function cx(n,t){n.uniform1uiv(this.addr,t)}function ux(n,t){n.uniform2uiv(this.addr,t)}function dx(n,t){n.uniform3uiv(this.addr,t)}function hx(n,t){n.uniform4uiv(this.addr,t)}function fx(n,t,e){const i=this.cache,s=t.length,r=Eo(e,s);Ce(i,r)||(n.uniform1iv(this.addr,r),Re(i,r));for(let a=0;a!==s;++a)e.setTexture2D(t[a]||dh,r[a])}function px(n,t,e){const i=this.cache,s=t.length,r=Eo(e,s);Ce(i,r)||(n.uniform1iv(this.addr,r),Re(i,r));for(let a=0;a!==s;++a)e.setTexture3D(t[a]||ph,r[a])}function mx(n,t,e){const i=this.cache,s=t.length,r=Eo(e,s);Ce(i,r)||(n.uniform1iv(this.addr,r),Re(i,r));for(let a=0;a!==s;++a)e.setTextureCube(t[a]||mh,r[a])}function _x(n,t,e){const i=this.cache,s=t.length,r=Eo(e,s);Ce(i,r)||(n.uniform1iv(this.addr,r),Re(i,r));for(let a=0;a!==s;++a)e.setTexture2DArray(t[a]||fh,r[a])}function gx(n){switch(n){case 5126:return J0;case 35664:return Q0;case 35665:return tx;case 35666:return ex;case 35674:return nx;case 35675:return ix;case 35676:return sx;case 5124:case 35670:return rx;case 35667:case 35671:return ox;case 35668:case 35672:return ax;case 35669:case 35673:return lx;case 5125:return cx;case 36294:return ux;case 36295:return dx;case 36296:return hx;case 35678:case 36198:case 36298:case 36306:case 35682:return fx;case 35679:case 36299:case 36307:return px;case 35680:case 36300:case 36308:case 36293:return mx;case 36289:case 36303:case 36311:case 36292:return _x}}class vx{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.setValue=Z0(e.type)}}class xx{constructor(t,e,i){this.id=t,this.addr=i,this.cache=[],this.type=e.type,this.size=e.size,this.setValue=gx(e.type)}}class yx{constructor(t){this.id=t,this.seq=[],this.map={}}setValue(t,e,i){const s=this.seq;for(let r=0,a=s.length;r!==a;++r){const o=s[r];o.setValue(t,e[o.id],i)}}}const fa=/(\w+)(\])?(\[|\.)?/g;function hu(n,t){n.seq.push(t),n.map[t.id]=t}function bx(n,t,e){const i=n.name,s=i.length;for(fa.lastIndex=0;;){const r=fa.exec(i),a=fa.lastIndex;let o=r[1];const l=r[2]==="]",c=r[3];if(l&&(o=o|0),c===void 0||c==="["&&a+2===s){hu(e,c===void 0?new vx(o,n,t):new xx(o,n,t));break}else{let h=e.map[o];h===void 0&&(h=new yx(o),hu(e,h)),e=h}}}class qr{constructor(t,e){this.seq=[],this.map={};const i=t.getProgramParameter(e,t.ACTIVE_UNIFORMS);for(let s=0;s<i;++s){const r=t.getActiveUniform(e,s),a=t.getUniformLocation(e,r.name);bx(r,a,this)}}setValue(t,e,i,s){const r=this.map[e];r!==void 0&&r.setValue(t,i,s)}setOptional(t,e,i){const s=e[i];s!==void 0&&this.setValue(t,i,s)}static upload(t,e,i,s){for(let r=0,a=e.length;r!==a;++r){const o=e[r],l=i[o.id];l.needsUpdate!==!1&&o.setValue(t,l.value,s)}}static seqWithValue(t,e){const i=[];for(let s=0,r=t.length;s!==r;++s){const a=t[s];a.id in e&&i.push(a)}return i}}function fu(n,t,e){const i=n.createShader(t);return n.shaderSource(i,e),n.compileShader(i),i}const Mx=37297;let Sx=0;function Ex(n,t){const e=n.split(`
`),i=[],s=Math.max(t-6,0),r=Math.min(t+6,e.length);for(let a=s;a<r;a++){const o=a+1;i.push(`${o===t?">":" "} ${o}: ${e[a]}`)}return i.join(`
`)}function Tx(n){const t=ce.getPrimaries(ce.workingColorSpace),e=ce.getPrimaries(n);let i;switch(t===e?i="":t===so&&e===io?i="LinearDisplayP3ToLinearSRGB":t===io&&e===so&&(i="LinearSRGBToLinearDisplayP3"),n){case ri:case Mo:return[i,"LinearTransferOETF"];case Ie:case ol:return[i,"sRGBTransferOETF"];default:return console.warn("THREE.WebGLProgram: Unsupported color space:",n),[i,"LinearTransferOETF"]}}function pu(n,t,e){const i=n.getShaderParameter(t,n.COMPILE_STATUS),s=n.getShaderInfoLog(t).trim();if(i&&s==="")return"";const r=/ERROR: 0:(\d+)/.exec(s);if(r){const a=parseInt(r[1]);return e.toUpperCase()+`

`+s+`

`+Ex(n.getShaderSource(t),a)}else return s}function wx(n,t){const e=Tx(t);return`vec4 ${n}( vec4 value ) { return ${e[0]}( ${e[1]}( value ) ); }`}function Ax(n,t){let e;switch(t){case km:e="Linear";break;case Fm:e="Reinhard";break;case Om:e="OptimizedCineon";break;case Bm:e="ACESFilmic";break;case Hm:e="AgX";break;case zm:e="Custom";break;default:console.warn("THREE.WebGLProgram: Unsupported toneMapping:",t),e="Linear"}return"vec3 "+n+"( vec3 color ) { return "+e+"ToneMapping( color ); }"}function Cx(n){return[n.extensionDerivatives||n.envMapCubeUVHeight||n.bumpMap||n.normalMapTangentSpace||n.clearcoatNormalMap||n.flatShading||n.shaderID==="physical"?"#extension GL_OES_standard_derivatives : enable":"",(n.extensionFragDepth||n.logarithmicDepthBuffer)&&n.rendererExtensionFragDepth?"#extension GL_EXT_frag_depth : enable":"",n.extensionDrawBuffers&&n.rendererExtensionDrawBuffers?"#extension GL_EXT_draw_buffers : require":"",(n.extensionShaderTextureLOD||n.envMap||n.transmission)&&n.rendererExtensionShaderTextureLod?"#extension GL_EXT_shader_texture_lod : enable":""].filter(cs).join(`
`)}function Rx(n){return[n.extensionClipCullDistance?"#extension GL_ANGLE_clip_cull_distance : require":""].filter(cs).join(`
`)}function Lx(n){const t=[];for(const e in n){const i=n[e];i!==!1&&t.push("#define "+e+" "+i)}return t.join(`
`)}function Px(n,t){const e={},i=n.getProgramParameter(t,n.ACTIVE_ATTRIBUTES);for(let s=0;s<i;s++){const r=n.getActiveAttrib(t,s),a=r.name;let o=1;r.type===n.FLOAT_MAT2&&(o=2),r.type===n.FLOAT_MAT3&&(o=3),r.type===n.FLOAT_MAT4&&(o=4),e[a]={type:r.type,location:n.getAttribLocation(t,a),locationSize:o}}return e}function cs(n){return n!==""}function mu(n,t){const e=t.numSpotLightShadows+t.numSpotLightMaps-t.numSpotLightShadowsWithMaps;return n.replace(/NUM_DIR_LIGHTS/g,t.numDirLights).replace(/NUM_SPOT_LIGHTS/g,t.numSpotLights).replace(/NUM_SPOT_LIGHT_MAPS/g,t.numSpotLightMaps).replace(/NUM_SPOT_LIGHT_COORDS/g,e).replace(/NUM_RECT_AREA_LIGHTS/g,t.numRectAreaLights).replace(/NUM_POINT_LIGHTS/g,t.numPointLights).replace(/NUM_HEMI_LIGHTS/g,t.numHemiLights).replace(/NUM_DIR_LIGHT_SHADOWS/g,t.numDirLightShadows).replace(/NUM_SPOT_LIGHT_SHADOWS_WITH_MAPS/g,t.numSpotLightShadowsWithMaps).replace(/NUM_SPOT_LIGHT_SHADOWS/g,t.numSpotLightShadows).replace(/NUM_POINT_LIGHT_SHADOWS/g,t.numPointLightShadows)}function _u(n,t){return n.replace(/NUM_CLIPPING_PLANES/g,t.numClippingPlanes).replace(/UNION_CLIPPING_PLANES/g,t.numClippingPlanes-t.numClipIntersection)}const Dx=/^[ \t]*#include +<([\w\d./]+)>/gm;function Ba(n){return n.replace(Dx,Ix)}const Ux=new Map([["encodings_fragment","colorspace_fragment"],["encodings_pars_fragment","colorspace_pars_fragment"],["output_fragment","opaque_fragment"]]);function Ix(n,t){let e=jt[t];if(e===void 0){const i=Ux.get(t);if(i!==void 0)e=jt[i],console.warn('THREE.WebGLRenderer: Shader chunk "%s" has been deprecated. Use "%s" instead.',t,i);else throw new Error("Can not resolve #include <"+t+">")}return Ba(e)}const Nx=/#pragma unroll_loop_start\s+for\s*\(\s*int\s+i\s*=\s*(\d+)\s*;\s*i\s*<\s*(\d+)\s*;\s*i\s*\+\+\s*\)\s*{([\s\S]+?)}\s+#pragma unroll_loop_end/g;function gu(n){return n.replace(Nx,kx)}function kx(n,t,e,i){let s="";for(let r=parseInt(t);r<parseInt(e);r++)s+=i.replace(/\[\s*i\s*\]/g,"[ "+r+" ]").replace(/UNROLLED_LOOP_INDEX/g,r);return s}function vu(n){let t="precision "+n.precision+` float;
precision `+n.precision+" int;";return n.precision==="highp"?t+=`
#define HIGH_PRECISION`:n.precision==="mediump"?t+=`
#define MEDIUM_PRECISION`:n.precision==="lowp"&&(t+=`
#define LOW_PRECISION`),t}function Fx(n){let t="SHADOWMAP_TYPE_BASIC";return n.shadowMapType===zd?t="SHADOWMAP_TYPE_PCF":n.shadowMapType===um?t="SHADOWMAP_TYPE_PCF_SOFT":n.shadowMapType===Xn&&(t="SHADOWMAP_TYPE_VSM"),t}function Ox(n){let t="ENVMAP_TYPE_CUBE";if(n.envMap)switch(n.envMapMode){case ys:case bs:t="ENVMAP_TYPE_CUBE";break;case bo:t="ENVMAP_TYPE_CUBE_UV";break}return t}function Bx(n){let t="ENVMAP_MODE_REFLECTION";if(n.envMap)switch(n.envMapMode){case bs:t="ENVMAP_MODE_REFRACTION";break}return t}function zx(n){let t="ENVMAP_BLENDING_NONE";if(n.envMap)switch(n.combine){case Hd:t="ENVMAP_BLENDING_MULTIPLY";break;case Im:t="ENVMAP_BLENDING_MIX";break;case Nm:t="ENVMAP_BLENDING_ADD";break}return t}function Hx(n){const t=n.envMapCubeUVHeight;if(t===null)return null;const e=Math.log2(t)-2,i=1/t;return{texelWidth:1/(3*Math.max(Math.pow(2,e),112)),texelHeight:i,maxMip:e}}function Vx(n,t,e,i){const s=n.getContext(),r=e.defines;let a=e.vertexShader,o=e.fragmentShader;const l=Fx(e),c=Ox(e),d=Bx(e),h=zx(e),p=Hx(e),f=e.isWebGL2?"":Cx(e),v=Rx(e),g=Lx(r),_=s.createProgram();let m,E,b=e.glslVersion?"#version "+e.glslVersion+`
`:"";e.isRawShaderMaterial?(m=["#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,g].filter(cs).join(`
`),m.length>0&&(m+=`
`),E=[f,"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,g].filter(cs).join(`
`),E.length>0&&(E+=`
`)):(m=[vu(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,g,e.extensionClipCullDistance?"#define USE_CLIP_DISTANCE":"",e.batching?"#define USE_BATCHING":"",e.instancing?"#define USE_INSTANCING":"",e.instancingColor?"#define USE_INSTANCING_COLOR":"",e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.map?"#define USE_MAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+d:"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.displacementMap?"#define USE_DISPLACEMENTMAP":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.mapUv?"#define MAP_UV "+e.mapUv:"",e.alphaMapUv?"#define ALPHAMAP_UV "+e.alphaMapUv:"",e.lightMapUv?"#define LIGHTMAP_UV "+e.lightMapUv:"",e.aoMapUv?"#define AOMAP_UV "+e.aoMapUv:"",e.emissiveMapUv?"#define EMISSIVEMAP_UV "+e.emissiveMapUv:"",e.bumpMapUv?"#define BUMPMAP_UV "+e.bumpMapUv:"",e.normalMapUv?"#define NORMALMAP_UV "+e.normalMapUv:"",e.displacementMapUv?"#define DISPLACEMENTMAP_UV "+e.displacementMapUv:"",e.metalnessMapUv?"#define METALNESSMAP_UV "+e.metalnessMapUv:"",e.roughnessMapUv?"#define ROUGHNESSMAP_UV "+e.roughnessMapUv:"",e.anisotropyMapUv?"#define ANISOTROPYMAP_UV "+e.anisotropyMapUv:"",e.clearcoatMapUv?"#define CLEARCOATMAP_UV "+e.clearcoatMapUv:"",e.clearcoatNormalMapUv?"#define CLEARCOAT_NORMALMAP_UV "+e.clearcoatNormalMapUv:"",e.clearcoatRoughnessMapUv?"#define CLEARCOAT_ROUGHNESSMAP_UV "+e.clearcoatRoughnessMapUv:"",e.iridescenceMapUv?"#define IRIDESCENCEMAP_UV "+e.iridescenceMapUv:"",e.iridescenceThicknessMapUv?"#define IRIDESCENCE_THICKNESSMAP_UV "+e.iridescenceThicknessMapUv:"",e.sheenColorMapUv?"#define SHEEN_COLORMAP_UV "+e.sheenColorMapUv:"",e.sheenRoughnessMapUv?"#define SHEEN_ROUGHNESSMAP_UV "+e.sheenRoughnessMapUv:"",e.specularMapUv?"#define SPECULARMAP_UV "+e.specularMapUv:"",e.specularColorMapUv?"#define SPECULAR_COLORMAP_UV "+e.specularColorMapUv:"",e.specularIntensityMapUv?"#define SPECULAR_INTENSITYMAP_UV "+e.specularIntensityMapUv:"",e.transmissionMapUv?"#define TRANSMISSIONMAP_UV "+e.transmissionMapUv:"",e.thicknessMapUv?"#define THICKNESSMAP_UV "+e.thicknessMapUv:"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.flatShading?"#define FLAT_SHADED":"",e.skinning?"#define USE_SKINNING":"",e.morphTargets?"#define USE_MORPHTARGETS":"",e.morphNormals&&e.flatShading===!1?"#define USE_MORPHNORMALS":"",e.morphColors&&e.isWebGL2?"#define USE_MORPHCOLORS":"",e.morphTargetsCount>0&&e.isWebGL2?"#define MORPHTARGETS_TEXTURE":"",e.morphTargetsCount>0&&e.isWebGL2?"#define MORPHTARGETS_TEXTURE_STRIDE "+e.morphTextureStride:"",e.morphTargetsCount>0&&e.isWebGL2?"#define MORPHTARGETS_COUNT "+e.morphTargetsCount:"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+l:"",e.sizeAttenuation?"#define USE_SIZEATTENUATION":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.useLegacyLights?"#define LEGACY_LIGHTS":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"",e.logarithmicDepthBuffer&&e.rendererExtensionFragDepth?"#define USE_LOGDEPTHBUF_EXT":"","uniform mat4 modelMatrix;","uniform mat4 modelViewMatrix;","uniform mat4 projectionMatrix;","uniform mat4 viewMatrix;","uniform mat3 normalMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;","#ifdef USE_INSTANCING","	attribute mat4 instanceMatrix;","#endif","#ifdef USE_INSTANCING_COLOR","	attribute vec3 instanceColor;","#endif","attribute vec3 position;","attribute vec3 normal;","attribute vec2 uv;","#ifdef USE_UV1","	attribute vec2 uv1;","#endif","#ifdef USE_UV2","	attribute vec2 uv2;","#endif","#ifdef USE_UV3","	attribute vec2 uv3;","#endif","#ifdef USE_TANGENT","	attribute vec4 tangent;","#endif","#if defined( USE_COLOR_ALPHA )","	attribute vec4 color;","#elif defined( USE_COLOR )","	attribute vec3 color;","#endif","#if ( defined( USE_MORPHTARGETS ) && ! defined( MORPHTARGETS_TEXTURE ) )","	attribute vec3 morphTarget0;","	attribute vec3 morphTarget1;","	attribute vec3 morphTarget2;","	attribute vec3 morphTarget3;","	#ifdef USE_MORPHNORMALS","		attribute vec3 morphNormal0;","		attribute vec3 morphNormal1;","		attribute vec3 morphNormal2;","		attribute vec3 morphNormal3;","	#else","		attribute vec3 morphTarget4;","		attribute vec3 morphTarget5;","		attribute vec3 morphTarget6;","		attribute vec3 morphTarget7;","	#endif","#endif","#ifdef USE_SKINNING","	attribute vec4 skinIndex;","	attribute vec4 skinWeight;","#endif",`
`].filter(cs).join(`
`),E=[f,vu(e),"#define SHADER_TYPE "+e.shaderType,"#define SHADER_NAME "+e.shaderName,g,e.useFog&&e.fog?"#define USE_FOG":"",e.useFog&&e.fogExp2?"#define FOG_EXP2":"",e.map?"#define USE_MAP":"",e.matcap?"#define USE_MATCAP":"",e.envMap?"#define USE_ENVMAP":"",e.envMap?"#define "+c:"",e.envMap?"#define "+d:"",e.envMap?"#define "+h:"",p?"#define CUBEUV_TEXEL_WIDTH "+p.texelWidth:"",p?"#define CUBEUV_TEXEL_HEIGHT "+p.texelHeight:"",p?"#define CUBEUV_MAX_MIP "+p.maxMip+".0":"",e.lightMap?"#define USE_LIGHTMAP":"",e.aoMap?"#define USE_AOMAP":"",e.bumpMap?"#define USE_BUMPMAP":"",e.normalMap?"#define USE_NORMALMAP":"",e.normalMapObjectSpace?"#define USE_NORMALMAP_OBJECTSPACE":"",e.normalMapTangentSpace?"#define USE_NORMALMAP_TANGENTSPACE":"",e.emissiveMap?"#define USE_EMISSIVEMAP":"",e.anisotropy?"#define USE_ANISOTROPY":"",e.anisotropyMap?"#define USE_ANISOTROPYMAP":"",e.clearcoat?"#define USE_CLEARCOAT":"",e.clearcoatMap?"#define USE_CLEARCOATMAP":"",e.clearcoatRoughnessMap?"#define USE_CLEARCOAT_ROUGHNESSMAP":"",e.clearcoatNormalMap?"#define USE_CLEARCOAT_NORMALMAP":"",e.iridescence?"#define USE_IRIDESCENCE":"",e.iridescenceMap?"#define USE_IRIDESCENCEMAP":"",e.iridescenceThicknessMap?"#define USE_IRIDESCENCE_THICKNESSMAP":"",e.specularMap?"#define USE_SPECULARMAP":"",e.specularColorMap?"#define USE_SPECULAR_COLORMAP":"",e.specularIntensityMap?"#define USE_SPECULAR_INTENSITYMAP":"",e.roughnessMap?"#define USE_ROUGHNESSMAP":"",e.metalnessMap?"#define USE_METALNESSMAP":"",e.alphaMap?"#define USE_ALPHAMAP":"",e.alphaTest?"#define USE_ALPHATEST":"",e.alphaHash?"#define USE_ALPHAHASH":"",e.sheen?"#define USE_SHEEN":"",e.sheenColorMap?"#define USE_SHEEN_COLORMAP":"",e.sheenRoughnessMap?"#define USE_SHEEN_ROUGHNESSMAP":"",e.transmission?"#define USE_TRANSMISSION":"",e.transmissionMap?"#define USE_TRANSMISSIONMAP":"",e.thicknessMap?"#define USE_THICKNESSMAP":"",e.vertexTangents&&e.flatShading===!1?"#define USE_TANGENT":"",e.vertexColors||e.instancingColor?"#define USE_COLOR":"",e.vertexAlphas?"#define USE_COLOR_ALPHA":"",e.vertexUv1s?"#define USE_UV1":"",e.vertexUv2s?"#define USE_UV2":"",e.vertexUv3s?"#define USE_UV3":"",e.pointsUvs?"#define USE_POINTS_UV":"",e.gradientMap?"#define USE_GRADIENTMAP":"",e.flatShading?"#define FLAT_SHADED":"",e.doubleSided?"#define DOUBLE_SIDED":"",e.flipSided?"#define FLIP_SIDED":"",e.shadowMapEnabled?"#define USE_SHADOWMAP":"",e.shadowMapEnabled?"#define "+l:"",e.premultipliedAlpha?"#define PREMULTIPLIED_ALPHA":"",e.numLightProbes>0?"#define USE_LIGHT_PROBES":"",e.useLegacyLights?"#define LEGACY_LIGHTS":"",e.decodeVideoTexture?"#define DECODE_VIDEO_TEXTURE":"",e.logarithmicDepthBuffer?"#define USE_LOGDEPTHBUF":"",e.logarithmicDepthBuffer&&e.rendererExtensionFragDepth?"#define USE_LOGDEPTHBUF_EXT":"","uniform mat4 viewMatrix;","uniform vec3 cameraPosition;","uniform bool isOrthographic;",e.toneMapping!==_i?"#define TONE_MAPPING":"",e.toneMapping!==_i?jt.tonemapping_pars_fragment:"",e.toneMapping!==_i?Ax("toneMapping",e.toneMapping):"",e.dithering?"#define DITHERING":"",e.opaque?"#define OPAQUE":"",jt.colorspace_pars_fragment,wx("linearToOutputTexel",e.outputColorSpace),e.useDepthPacking?"#define DEPTH_PACKING "+e.depthPacking:"",`
`].filter(cs).join(`
`)),a=Ba(a),a=mu(a,e),a=_u(a,e),o=Ba(o),o=mu(o,e),o=_u(o,e),a=gu(a),o=gu(o),e.isWebGL2&&e.isRawShaderMaterial!==!0&&(b=`#version 300 es
`,m=[v,"precision mediump sampler2DArray;","#define attribute in","#define varying out","#define texture2D texture"].join(`
`)+`
`+m,E=["precision mediump sampler2DArray;","#define varying in",e.glslVersion===kc?"":"layout(location = 0) out highp vec4 pc_fragColor;",e.glslVersion===kc?"":"#define gl_FragColor pc_fragColor","#define gl_FragDepthEXT gl_FragDepth","#define texture2D texture","#define textureCube texture","#define texture2DProj textureProj","#define texture2DLodEXT textureLod","#define texture2DProjLodEXT textureProjLod","#define textureCubeLodEXT textureLod","#define texture2DGradEXT textureGrad","#define texture2DProjGradEXT textureProjGrad","#define textureCubeGradEXT textureGrad"].join(`
`)+`
`+E);const x=b+m+a,C=b+E+o,A=fu(s,s.VERTEX_SHADER,x),D=fu(s,s.FRAGMENT_SHADER,C);s.attachShader(_,A),s.attachShader(_,D),e.index0AttributeName!==void 0?s.bindAttribLocation(_,0,e.index0AttributeName):e.morphTargets===!0&&s.bindAttribLocation(_,0,"position"),s.linkProgram(_);function G(rt){if(n.debug.checkShaderErrors){const mt=s.getProgramInfoLog(_).trim(),O=s.getShaderInfoLog(A).trim(),Z=s.getShaderInfoLog(D).trim();let tt=!0,st=!0;if(s.getProgramParameter(_,s.LINK_STATUS)===!1)if(tt=!1,typeof n.debug.onShaderError=="function")n.debug.onShaderError(s,_,A,D);else{const et=pu(s,A,"vertex"),ot=pu(s,D,"fragment");console.error("THREE.WebGLProgram: Shader Error "+s.getError()+" - VALIDATE_STATUS "+s.getProgramParameter(_,s.VALIDATE_STATUS)+`

Program Info Log: `+mt+`
`+et+`
`+ot)}else mt!==""?console.warn("THREE.WebGLProgram: Program Info Log:",mt):(O===""||Z==="")&&(st=!1);st&&(rt.diagnostics={runnable:tt,programLog:mt,vertexShader:{log:O,prefix:m},fragmentShader:{log:Z,prefix:E}})}s.deleteShader(A),s.deleteShader(D),T=new qr(s,_),R=Px(s,_)}let T;this.getUniforms=function(){return T===void 0&&G(this),T};let R;this.getAttributes=function(){return R===void 0&&G(this),R};let J=e.rendererExtensionParallelShaderCompile===!1;return this.isReady=function(){return J===!1&&(J=s.getProgramParameter(_,Mx)),J},this.destroy=function(){i.releaseStatesOfProgram(this),s.deleteProgram(_),this.program=void 0},this.type=e.shaderType,this.name=e.shaderName,this.id=Sx++,this.cacheKey=t,this.usedTimes=1,this.program=_,this.vertexShader=A,this.fragmentShader=D,this}let Gx=0;class Wx{constructor(){this.shaderCache=new Map,this.materialCache=new Map}update(t){const e=t.vertexShader,i=t.fragmentShader,s=this._getShaderStage(e),r=this._getShaderStage(i),a=this._getShaderCacheForMaterial(t);return a.has(s)===!1&&(a.add(s),s.usedTimes++),a.has(r)===!1&&(a.add(r),r.usedTimes++),this}remove(t){const e=this.materialCache.get(t);for(const i of e)i.usedTimes--,i.usedTimes===0&&this.shaderCache.delete(i.code);return this.materialCache.delete(t),this}getVertexShaderID(t){return this._getShaderStage(t.vertexShader).id}getFragmentShaderID(t){return this._getShaderStage(t.fragmentShader).id}dispose(){this.shaderCache.clear(),this.materialCache.clear()}_getShaderCacheForMaterial(t){const e=this.materialCache;let i=e.get(t);return i===void 0&&(i=new Set,e.set(t,i)),i}_getShaderStage(t){const e=this.shaderCache;let i=e.get(t);return i===void 0&&(i=new $x(t),e.set(t,i)),i}}class $x{constructor(t){this.id=Gx++,this.code=t,this.usedTimes=0}}function Xx(n,t,e,i,s,r,a){const o=new nh,l=new Wx,c=[],d=s.isWebGL2,h=s.logarithmicDepthBuffer,p=s.vertexTextures;let f=s.precision;const v={MeshDepthMaterial:"depth",MeshDistanceMaterial:"distanceRGBA",MeshNormalMaterial:"normal",MeshBasicMaterial:"basic",MeshLambertMaterial:"lambert",MeshPhongMaterial:"phong",MeshToonMaterial:"toon",MeshStandardMaterial:"physical",MeshPhysicalMaterial:"physical",MeshMatcapMaterial:"matcap",LineBasicMaterial:"basic",LineDashedMaterial:"dashed",PointsMaterial:"points",ShadowMaterial:"shadow",SpriteMaterial:"sprite"};function g(T){return T===0?"uv":`uv${T}`}function _(T,R,J,rt,mt){const O=rt.fog,Z=mt.geometry,tt=T.isMeshStandardMaterial?rt.environment:null,st=(T.isMeshStandardMaterial?e:t).get(T.envMap||tt),et=st&&st.mapping===bo?st.image.height:null,ot=v[T.type];T.precision!==null&&(f=s.getMaxPrecision(T.precision),f!==T.precision&&console.warn("THREE.WebGLProgram.getParameters:",T.precision,"not supported, using",f,"instead."));const ut=Z.morphAttributes.position||Z.morphAttributes.normal||Z.morphAttributes.color,pt=ut!==void 0?ut.length:0;let ft=0;Z.morphAttributes.position!==void 0&&(ft=1),Z.morphAttributes.normal!==void 0&&(ft=2),Z.morphAttributes.color!==void 0&&(ft=3);let it,ht,bt,Lt;if(ot){const Xe=Un[ot];it=Xe.vertexShader,ht=Xe.fragmentShader}else it=T.vertexShader,ht=T.fragmentShader,l.update(T),bt=l.getVertexShaderID(T),Lt=l.getFragmentShaderID(T);const Pt=n.getRenderTarget(),Ht=mt.isInstancedMesh===!0,Gt=mt.isBatchedMesh===!0,Nt=!!T.map,Zt=!!T.matcap,M=!!st,k=!!T.aoMap,B=!!T.lightMap,Y=!!T.bumpMap,N=!!T.normalMap,V=!!T.displacementMap,j=!!T.emissiveMap,S=!!T.metalnessMap,y=!!T.roughnessMap,U=T.anisotropy>0,q=T.clearcoat>0,H=T.iridescence>0,X=T.sheen>0,lt=T.transmission>0,at=U&&!!T.anisotropyMap,dt=q&&!!T.clearcoatMap,ct=q&&!!T.clearcoatNormalMap,_t=q&&!!T.clearcoatRoughnessMap,$=H&&!!T.iridescenceMap,Rt=H&&!!T.iridescenceThicknessMap,At=X&&!!T.sheenColorMap,Tt=X&&!!T.sheenRoughnessMap,Ct=!!T.specularMap,xt=!!T.specularColorMap,Dt=!!T.specularIntensityMap,Yt=lt&&!!T.transmissionMap,le=lt&&!!T.thicknessMap,Jt=!!T.gradientMap,gt=!!T.alphaMap,F=T.alphaTest>0,Mt=!!T.alphaHash,St=!!T.extensions,Vt=!!Z.attributes.uv1,Ft=!!Z.attributes.uv2,fe=!!Z.attributes.uv3;let pe=_i;return T.toneMapped&&(Pt===null||Pt.isXRRenderTarget===!0)&&(pe=n.toneMapping),{isWebGL2:d,shaderID:ot,shaderType:T.type,shaderName:T.name,vertexShader:it,fragmentShader:ht,defines:T.defines,customVertexShaderID:bt,customFragmentShaderID:Lt,isRawShaderMaterial:T.isRawShaderMaterial===!0,glslVersion:T.glslVersion,precision:f,batching:Gt,instancing:Ht,instancingColor:Ht&&mt.instanceColor!==null,supportsVertexTextures:p,outputColorSpace:Pt===null?n.outputColorSpace:Pt.isXRRenderTarget===!0?Pt.texture.colorSpace:ri,map:Nt,matcap:Zt,envMap:M,envMapMode:M&&st.mapping,envMapCubeUVHeight:et,aoMap:k,lightMap:B,bumpMap:Y,normalMap:N,displacementMap:p&&V,emissiveMap:j,normalMapObjectSpace:N&&T.normalMapType===t_,normalMapTangentSpace:N&&T.normalMapType===Qm,metalnessMap:S,roughnessMap:y,anisotropy:U,anisotropyMap:at,clearcoat:q,clearcoatMap:dt,clearcoatNormalMap:ct,clearcoatRoughnessMap:_t,iridescence:H,iridescenceMap:$,iridescenceThicknessMap:Rt,sheen:X,sheenColorMap:At,sheenRoughnessMap:Tt,specularMap:Ct,specularColorMap:xt,specularIntensityMap:Dt,transmission:lt,transmissionMap:Yt,thicknessMap:le,gradientMap:Jt,opaque:T.transparent===!1&&T.blending===ps,alphaMap:gt,alphaTest:F,alphaHash:Mt,combine:T.combine,mapUv:Nt&&g(T.map.channel),aoMapUv:k&&g(T.aoMap.channel),lightMapUv:B&&g(T.lightMap.channel),bumpMapUv:Y&&g(T.bumpMap.channel),normalMapUv:N&&g(T.normalMap.channel),displacementMapUv:V&&g(T.displacementMap.channel),emissiveMapUv:j&&g(T.emissiveMap.channel),metalnessMapUv:S&&g(T.metalnessMap.channel),roughnessMapUv:y&&g(T.roughnessMap.channel),anisotropyMapUv:at&&g(T.anisotropyMap.channel),clearcoatMapUv:dt&&g(T.clearcoatMap.channel),clearcoatNormalMapUv:ct&&g(T.clearcoatNormalMap.channel),clearcoatRoughnessMapUv:_t&&g(T.clearcoatRoughnessMap.channel),iridescenceMapUv:$&&g(T.iridescenceMap.channel),iridescenceThicknessMapUv:Rt&&g(T.iridescenceThicknessMap.channel),sheenColorMapUv:At&&g(T.sheenColorMap.channel),sheenRoughnessMapUv:Tt&&g(T.sheenRoughnessMap.channel),specularMapUv:Ct&&g(T.specularMap.channel),specularColorMapUv:xt&&g(T.specularColorMap.channel),specularIntensityMapUv:Dt&&g(T.specularIntensityMap.channel),transmissionMapUv:Yt&&g(T.transmissionMap.channel),thicknessMapUv:le&&g(T.thicknessMap.channel),alphaMapUv:gt&&g(T.alphaMap.channel),vertexTangents:!!Z.attributes.tangent&&(N||U),vertexColors:T.vertexColors,vertexAlphas:T.vertexColors===!0&&!!Z.attributes.color&&Z.attributes.color.itemSize===4,vertexUv1s:Vt,vertexUv2s:Ft,vertexUv3s:fe,pointsUvs:mt.isPoints===!0&&!!Z.attributes.uv&&(Nt||gt),fog:!!O,useFog:T.fog===!0,fogExp2:O&&O.isFogExp2,flatShading:T.flatShading===!0,sizeAttenuation:T.sizeAttenuation===!0,logarithmicDepthBuffer:h,skinning:mt.isSkinnedMesh===!0,morphTargets:Z.morphAttributes.position!==void 0,morphNormals:Z.morphAttributes.normal!==void 0,morphColors:Z.morphAttributes.color!==void 0,morphTargetsCount:pt,morphTextureStride:ft,numDirLights:R.directional.length,numPointLights:R.point.length,numSpotLights:R.spot.length,numSpotLightMaps:R.spotLightMap.length,numRectAreaLights:R.rectArea.length,numHemiLights:R.hemi.length,numDirLightShadows:R.directionalShadowMap.length,numPointLightShadows:R.pointShadowMap.length,numSpotLightShadows:R.spotShadowMap.length,numSpotLightShadowsWithMaps:R.numSpotLightShadowsWithMaps,numLightProbes:R.numLightProbes,numClippingPlanes:a.numPlanes,numClipIntersection:a.numIntersection,dithering:T.dithering,shadowMapEnabled:n.shadowMap.enabled&&J.length>0,shadowMapType:n.shadowMap.type,toneMapping:pe,useLegacyLights:n._useLegacyLights,decodeVideoTexture:Nt&&T.map.isVideoTexture===!0&&ce.getTransfer(T.map.colorSpace)===ge,premultipliedAlpha:T.premultipliedAlpha,doubleSided:T.side===Qn,flipSided:T.side===tn,useDepthPacking:T.depthPacking>=0,depthPacking:T.depthPacking||0,index0AttributeName:T.index0AttributeName,extensionDerivatives:St&&T.extensions.derivatives===!0,extensionFragDepth:St&&T.extensions.fragDepth===!0,extensionDrawBuffers:St&&T.extensions.drawBuffers===!0,extensionShaderTextureLOD:St&&T.extensions.shaderTextureLOD===!0,extensionClipCullDistance:St&&T.extensions.clipCullDistance&&i.has("WEBGL_clip_cull_distance"),rendererExtensionFragDepth:d||i.has("EXT_frag_depth"),rendererExtensionDrawBuffers:d||i.has("WEBGL_draw_buffers"),rendererExtensionShaderTextureLod:d||i.has("EXT_shader_texture_lod"),rendererExtensionParallelShaderCompile:i.has("KHR_parallel_shader_compile"),customProgramCacheKey:T.customProgramCacheKey()}}function m(T){const R=[];if(T.shaderID?R.push(T.shaderID):(R.push(T.customVertexShaderID),R.push(T.customFragmentShaderID)),T.defines!==void 0)for(const J in T.defines)R.push(J),R.push(T.defines[J]);return T.isRawShaderMaterial===!1&&(E(R,T),b(R,T),R.push(n.outputColorSpace)),R.push(T.customProgramCacheKey),R.join()}function E(T,R){T.push(R.precision),T.push(R.outputColorSpace),T.push(R.envMapMode),T.push(R.envMapCubeUVHeight),T.push(R.mapUv),T.push(R.alphaMapUv),T.push(R.lightMapUv),T.push(R.aoMapUv),T.push(R.bumpMapUv),T.push(R.normalMapUv),T.push(R.displacementMapUv),T.push(R.emissiveMapUv),T.push(R.metalnessMapUv),T.push(R.roughnessMapUv),T.push(R.anisotropyMapUv),T.push(R.clearcoatMapUv),T.push(R.clearcoatNormalMapUv),T.push(R.clearcoatRoughnessMapUv),T.push(R.iridescenceMapUv),T.push(R.iridescenceThicknessMapUv),T.push(R.sheenColorMapUv),T.push(R.sheenRoughnessMapUv),T.push(R.specularMapUv),T.push(R.specularColorMapUv),T.push(R.specularIntensityMapUv),T.push(R.transmissionMapUv),T.push(R.thicknessMapUv),T.push(R.combine),T.push(R.fogExp2),T.push(R.sizeAttenuation),T.push(R.morphTargetsCount),T.push(R.morphAttributeCount),T.push(R.numDirLights),T.push(R.numPointLights),T.push(R.numSpotLights),T.push(R.numSpotLightMaps),T.push(R.numHemiLights),T.push(R.numRectAreaLights),T.push(R.numDirLightShadows),T.push(R.numPointLightShadows),T.push(R.numSpotLightShadows),T.push(R.numSpotLightShadowsWithMaps),T.push(R.numLightProbes),T.push(R.shadowMapType),T.push(R.toneMapping),T.push(R.numClippingPlanes),T.push(R.numClipIntersection),T.push(R.depthPacking)}function b(T,R){o.disableAll(),R.isWebGL2&&o.enable(0),R.supportsVertexTextures&&o.enable(1),R.instancing&&o.enable(2),R.instancingColor&&o.enable(3),R.matcap&&o.enable(4),R.envMap&&o.enable(5),R.normalMapObjectSpace&&o.enable(6),R.normalMapTangentSpace&&o.enable(7),R.clearcoat&&o.enable(8),R.iridescence&&o.enable(9),R.alphaTest&&o.enable(10),R.vertexColors&&o.enable(11),R.vertexAlphas&&o.enable(12),R.vertexUv1s&&o.enable(13),R.vertexUv2s&&o.enable(14),R.vertexUv3s&&o.enable(15),R.vertexTangents&&o.enable(16),R.anisotropy&&o.enable(17),R.alphaHash&&o.enable(18),R.batching&&o.enable(19),T.push(o.mask),o.disableAll(),R.fog&&o.enable(0),R.useFog&&o.enable(1),R.flatShading&&o.enable(2),R.logarithmicDepthBuffer&&o.enable(3),R.skinning&&o.enable(4),R.morphTargets&&o.enable(5),R.morphNormals&&o.enable(6),R.morphColors&&o.enable(7),R.premultipliedAlpha&&o.enable(8),R.shadowMapEnabled&&o.enable(9),R.useLegacyLights&&o.enable(10),R.doubleSided&&o.enable(11),R.flipSided&&o.enable(12),R.useDepthPacking&&o.enable(13),R.dithering&&o.enable(14),R.transmission&&o.enable(15),R.sheen&&o.enable(16),R.opaque&&o.enable(17),R.pointsUvs&&o.enable(18),R.decodeVideoTexture&&o.enable(19),T.push(o.mask)}function x(T){const R=v[T.type];let J;if(R){const rt=Un[R];J=A_.clone(rt.uniforms)}else J=T.uniforms;return J}function C(T,R){let J;for(let rt=0,mt=c.length;rt<mt;rt++){const O=c[rt];if(O.cacheKey===R){J=O,++J.usedTimes;break}}return J===void 0&&(J=new Vx(n,R,T,r),c.push(J)),J}function A(T){if(--T.usedTimes===0){const R=c.indexOf(T);c[R]=c[c.length-1],c.pop(),T.destroy()}}function D(T){l.remove(T)}function G(){l.dispose()}return{getParameters:_,getProgramCacheKey:m,getUniforms:x,acquireProgram:C,releaseProgram:A,releaseShaderCache:D,programs:c,dispose:G}}function qx(){let n=new WeakMap;function t(r){let a=n.get(r);return a===void 0&&(a={},n.set(r,a)),a}function e(r){n.delete(r)}function i(r,a,o){n.get(r)[a]=o}function s(){n=new WeakMap}return{get:t,remove:e,update:i,dispose:s}}function jx(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.material.id!==t.material.id?n.material.id-t.material.id:n.z!==t.z?n.z-t.z:n.id-t.id}function xu(n,t){return n.groupOrder!==t.groupOrder?n.groupOrder-t.groupOrder:n.renderOrder!==t.renderOrder?n.renderOrder-t.renderOrder:n.z!==t.z?t.z-n.z:n.id-t.id}function yu(){const n=[];let t=0;const e=[],i=[],s=[];function r(){t=0,e.length=0,i.length=0,s.length=0}function a(h,p,f,v,g,_){let m=n[t];return m===void 0?(m={id:h.id,object:h,geometry:p,material:f,groupOrder:v,renderOrder:h.renderOrder,z:g,group:_},n[t]=m):(m.id=h.id,m.object=h,m.geometry=p,m.material=f,m.groupOrder=v,m.renderOrder=h.renderOrder,m.z=g,m.group=_),t++,m}function o(h,p,f,v,g,_){const m=a(h,p,f,v,g,_);f.transmission>0?i.push(m):f.transparent===!0?s.push(m):e.push(m)}function l(h,p,f,v,g,_){const m=a(h,p,f,v,g,_);f.transmission>0?i.unshift(m):f.transparent===!0?s.unshift(m):e.unshift(m)}function c(h,p){e.length>1&&e.sort(h||jx),i.length>1&&i.sort(p||xu),s.length>1&&s.sort(p||xu)}function d(){for(let h=t,p=n.length;h<p;h++){const f=n[h];if(f.id===null)break;f.id=null,f.object=null,f.geometry=null,f.material=null,f.group=null}}return{opaque:e,transmissive:i,transparent:s,init:r,push:o,unshift:l,finish:d,sort:c}}function Yx(){let n=new WeakMap;function t(i,s){const r=n.get(i);let a;return r===void 0?(a=new yu,n.set(i,[a])):s>=r.length?(a=new yu,r.push(a)):a=r[s],a}function e(){n=new WeakMap}return{get:t,dispose:e}}function Kx(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={direction:new z,color:new ne};break;case"SpotLight":e={position:new z,direction:new z,color:new ne,distance:0,coneCos:0,penumbraCos:0,decay:0};break;case"PointLight":e={position:new z,color:new ne,distance:0,decay:0};break;case"HemisphereLight":e={direction:new z,skyColor:new ne,groundColor:new ne};break;case"RectAreaLight":e={color:new ne,position:new z,halfWidth:new z,halfHeight:new z};break}return n[t.id]=e,e}}}function Zx(){const n={};return{get:function(t){if(n[t.id]!==void 0)return n[t.id];let e;switch(t.type){case"DirectionalLight":e={shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new re};break;case"SpotLight":e={shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new re};break;case"PointLight":e={shadowBias:0,shadowNormalBias:0,shadowRadius:1,shadowMapSize:new re,shadowCameraNear:1,shadowCameraFar:1e3};break}return n[t.id]=e,e}}}let Jx=0;function Qx(n,t){return(t.castShadow?2:0)-(n.castShadow?2:0)+(t.map?1:0)-(n.map?1:0)}function ty(n,t){const e=new Kx,i=Zx(),s={version:0,hash:{directionalLength:-1,pointLength:-1,spotLength:-1,rectAreaLength:-1,hemiLength:-1,numDirectionalShadows:-1,numPointShadows:-1,numSpotShadows:-1,numSpotMaps:-1,numLightProbes:-1},ambient:[0,0,0],probe:[],directional:[],directionalShadow:[],directionalShadowMap:[],directionalShadowMatrix:[],spot:[],spotLightMap:[],spotShadow:[],spotShadowMap:[],spotLightMatrix:[],rectArea:[],rectAreaLTC1:null,rectAreaLTC2:null,point:[],pointShadow:[],pointShadowMap:[],pointShadowMatrix:[],hemi:[],numSpotLightShadowsWithMaps:0,numLightProbes:0};for(let d=0;d<9;d++)s.probe.push(new z);const r=new z,a=new Te,o=new Te;function l(d,h){let p=0,f=0,v=0;for(let rt=0;rt<9;rt++)s.probe[rt].set(0,0,0);let g=0,_=0,m=0,E=0,b=0,x=0,C=0,A=0,D=0,G=0,T=0;d.sort(Qx);const R=h===!0?Math.PI:1;for(let rt=0,mt=d.length;rt<mt;rt++){const O=d[rt],Z=O.color,tt=O.intensity,st=O.distance,et=O.shadow&&O.shadow.map?O.shadow.map.texture:null;if(O.isAmbientLight)p+=Z.r*tt*R,f+=Z.g*tt*R,v+=Z.b*tt*R;else if(O.isLightProbe){for(let ot=0;ot<9;ot++)s.probe[ot].addScaledVector(O.sh.coefficients[ot],tt);T++}else if(O.isDirectionalLight){const ot=e.get(O);if(ot.color.copy(O.color).multiplyScalar(O.intensity*R),O.castShadow){const ut=O.shadow,pt=i.get(O);pt.shadowBias=ut.bias,pt.shadowNormalBias=ut.normalBias,pt.shadowRadius=ut.radius,pt.shadowMapSize=ut.mapSize,s.directionalShadow[g]=pt,s.directionalShadowMap[g]=et,s.directionalShadowMatrix[g]=O.shadow.matrix,x++}s.directional[g]=ot,g++}else if(O.isSpotLight){const ot=e.get(O);ot.position.setFromMatrixPosition(O.matrixWorld),ot.color.copy(Z).multiplyScalar(tt*R),ot.distance=st,ot.coneCos=Math.cos(O.angle),ot.penumbraCos=Math.cos(O.angle*(1-O.penumbra)),ot.decay=O.decay,s.spot[m]=ot;const ut=O.shadow;if(O.map&&(s.spotLightMap[D]=O.map,D++,ut.updateMatrices(O),O.castShadow&&G++),s.spotLightMatrix[m]=ut.matrix,O.castShadow){const pt=i.get(O);pt.shadowBias=ut.bias,pt.shadowNormalBias=ut.normalBias,pt.shadowRadius=ut.radius,pt.shadowMapSize=ut.mapSize,s.spotShadow[m]=pt,s.spotShadowMap[m]=et,A++}m++}else if(O.isRectAreaLight){const ot=e.get(O);ot.color.copy(Z).multiplyScalar(tt),ot.halfWidth.set(O.width*.5,0,0),ot.halfHeight.set(0,O.height*.5,0),s.rectArea[E]=ot,E++}else if(O.isPointLight){const ot=e.get(O);if(ot.color.copy(O.color).multiplyScalar(O.intensity*R),ot.distance=O.distance,ot.decay=O.decay,O.castShadow){const ut=O.shadow,pt=i.get(O);pt.shadowBias=ut.bias,pt.shadowNormalBias=ut.normalBias,pt.shadowRadius=ut.radius,pt.shadowMapSize=ut.mapSize,pt.shadowCameraNear=ut.camera.near,pt.shadowCameraFar=ut.camera.far,s.pointShadow[_]=pt,s.pointShadowMap[_]=et,s.pointShadowMatrix[_]=O.shadow.matrix,C++}s.point[_]=ot,_++}else if(O.isHemisphereLight){const ot=e.get(O);ot.skyColor.copy(O.color).multiplyScalar(tt*R),ot.groundColor.copy(O.groundColor).multiplyScalar(tt*R),s.hemi[b]=ot,b++}}E>0&&(t.isWebGL2?n.has("OES_texture_float_linear")===!0?(s.rectAreaLTC1=vt.LTC_FLOAT_1,s.rectAreaLTC2=vt.LTC_FLOAT_2):(s.rectAreaLTC1=vt.LTC_HALF_1,s.rectAreaLTC2=vt.LTC_HALF_2):n.has("OES_texture_float_linear")===!0?(s.rectAreaLTC1=vt.LTC_FLOAT_1,s.rectAreaLTC2=vt.LTC_FLOAT_2):n.has("OES_texture_half_float_linear")===!0?(s.rectAreaLTC1=vt.LTC_HALF_1,s.rectAreaLTC2=vt.LTC_HALF_2):console.error("THREE.WebGLRenderer: Unable to use RectAreaLight. Missing WebGL extensions.")),s.ambient[0]=p,s.ambient[1]=f,s.ambient[2]=v;const J=s.hash;(J.directionalLength!==g||J.pointLength!==_||J.spotLength!==m||J.rectAreaLength!==E||J.hemiLength!==b||J.numDirectionalShadows!==x||J.numPointShadows!==C||J.numSpotShadows!==A||J.numSpotMaps!==D||J.numLightProbes!==T)&&(s.directional.length=g,s.spot.length=m,s.rectArea.length=E,s.point.length=_,s.hemi.length=b,s.directionalShadow.length=x,s.directionalShadowMap.length=x,s.pointShadow.length=C,s.pointShadowMap.length=C,s.spotShadow.length=A,s.spotShadowMap.length=A,s.directionalShadowMatrix.length=x,s.pointShadowMatrix.length=C,s.spotLightMatrix.length=A+D-G,s.spotLightMap.length=D,s.numSpotLightShadowsWithMaps=G,s.numLightProbes=T,J.directionalLength=g,J.pointLength=_,J.spotLength=m,J.rectAreaLength=E,J.hemiLength=b,J.numDirectionalShadows=x,J.numPointShadows=C,J.numSpotShadows=A,J.numSpotMaps=D,J.numLightProbes=T,s.version=Jx++)}function c(d,h){let p=0,f=0,v=0,g=0,_=0;const m=h.matrixWorldInverse;for(let E=0,b=d.length;E<b;E++){const x=d[E];if(x.isDirectionalLight){const C=s.directional[p];C.direction.setFromMatrixPosition(x.matrixWorld),r.setFromMatrixPosition(x.target.matrixWorld),C.direction.sub(r),C.direction.transformDirection(m),p++}else if(x.isSpotLight){const C=s.spot[v];C.position.setFromMatrixPosition(x.matrixWorld),C.position.applyMatrix4(m),C.direction.setFromMatrixPosition(x.matrixWorld),r.setFromMatrixPosition(x.target.matrixWorld),C.direction.sub(r),C.direction.transformDirection(m),v++}else if(x.isRectAreaLight){const C=s.rectArea[g];C.position.setFromMatrixPosition(x.matrixWorld),C.position.applyMatrix4(m),o.identity(),a.copy(x.matrixWorld),a.premultiply(m),o.extractRotation(a),C.halfWidth.set(x.width*.5,0,0),C.halfHeight.set(0,x.height*.5,0),C.halfWidth.applyMatrix4(o),C.halfHeight.applyMatrix4(o),g++}else if(x.isPointLight){const C=s.point[f];C.position.setFromMatrixPosition(x.matrixWorld),C.position.applyMatrix4(m),f++}else if(x.isHemisphereLight){const C=s.hemi[_];C.direction.setFromMatrixPosition(x.matrixWorld),C.direction.transformDirection(m),_++}}}return{setup:l,setupView:c,state:s}}function bu(n,t){const e=new ty(n,t),i=[],s=[];function r(){i.length=0,s.length=0}function a(h){i.push(h)}function o(h){s.push(h)}function l(h){e.setup(i,h)}function c(h){e.setupView(i,h)}return{init:r,state:{lightsArray:i,shadowsArray:s,lights:e},setupLights:l,setupLightsView:c,pushLight:a,pushShadow:o}}function ey(n,t){let e=new WeakMap;function i(r,a=0){const o=e.get(r);let l;return o===void 0?(l=new bu(n,t),e.set(r,[l])):a>=o.length?(l=new bu(n,t),o.push(l)):l=o[a],l}function s(){e=new WeakMap}return{get:i,dispose:s}}class ny extends ws{constructor(t){super(),this.isMeshDepthMaterial=!0,this.type="MeshDepthMaterial",this.depthPacking=Zm,this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.wireframe=!1,this.wireframeLinewidth=1,this.setValues(t)}copy(t){return super.copy(t),this.depthPacking=t.depthPacking,this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this.wireframe=t.wireframe,this.wireframeLinewidth=t.wireframeLinewidth,this}}class iy extends ws{constructor(t){super(),this.isMeshDistanceMaterial=!0,this.type="MeshDistanceMaterial",this.map=null,this.alphaMap=null,this.displacementMap=null,this.displacementScale=1,this.displacementBias=0,this.setValues(t)}copy(t){return super.copy(t),this.map=t.map,this.alphaMap=t.alphaMap,this.displacementMap=t.displacementMap,this.displacementScale=t.displacementScale,this.displacementBias=t.displacementBias,this}}const sy=`void main() {
	gl_Position = vec4( position, 1.0 );
}`,ry=`uniform sampler2D shadow_pass;
uniform vec2 resolution;
uniform float radius;
#include <packing>
void main() {
	const float samples = float( VSM_SAMPLES );
	float mean = 0.0;
	float squared_mean = 0.0;
	float uvStride = samples <= 1.0 ? 0.0 : 2.0 / ( samples - 1.0 );
	float uvStart = samples <= 1.0 ? 0.0 : - 1.0;
	for ( float i = 0.0; i < samples; i ++ ) {
		float uvOffset = uvStart + i * uvStride;
		#ifdef HORIZONTAL_PASS
			vec2 distribution = unpackRGBATo2Half( texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( uvOffset, 0.0 ) * radius ) / resolution ) );
			mean += distribution.x;
			squared_mean += distribution.y * distribution.y + distribution.x * distribution.x;
		#else
			float depth = unpackRGBAToDepth( texture2D( shadow_pass, ( gl_FragCoord.xy + vec2( 0.0, uvOffset ) * radius ) / resolution ) );
			mean += depth;
			squared_mean += depth * depth;
		#endif
	}
	mean = mean / samples;
	squared_mean = squared_mean / samples;
	float std_dev = sqrt( squared_mean - mean * mean );
	gl_FragColor = pack2HalfToRGBA( vec2( mean, std_dev ) );
}`;function oy(n,t,e){let i=new ll;const s=new re,r=new re,a=new xe,o=new ny({depthPacking:Jm}),l=new iy,c={},d=e.maxTextureSize,h={[bi]:tn,[tn]:bi,[Qn]:Qn},p=new Vi({defines:{VSM_SAMPLES:8},uniforms:{shadow_pass:{value:null},resolution:{value:new re},radius:{value:4}},vertexShader:sy,fragmentShader:ry}),f=p.clone();f.defines.HORIZONTAL_PASS=1;const v=new Ve;v.setAttribute("position",new Ne(new Float32Array([-1,-1,.5,3,-1,.5,-1,3,.5]),3));const g=new _n(v,p),_=this;this.enabled=!1,this.autoUpdate=!0,this.needsUpdate=!1,this.type=zd;let m=this.type;this.render=function(A,D,G){if(_.enabled===!1||_.autoUpdate===!1&&_.needsUpdate===!1||A.length===0)return;const T=n.getRenderTarget(),R=n.getActiveCubeFace(),J=n.getActiveMipmapLevel(),rt=n.state;rt.setBlending(mi),rt.buffers.color.setClear(1,1,1,1),rt.buffers.depth.setTest(!0),rt.setScissorTest(!1);const mt=m!==Xn&&this.type===Xn,O=m===Xn&&this.type!==Xn;for(let Z=0,tt=A.length;Z<tt;Z++){const st=A[Z],et=st.shadow;if(et===void 0){console.warn("THREE.WebGLShadowMap:",st,"has no shadow.");continue}if(et.autoUpdate===!1&&et.needsUpdate===!1)continue;s.copy(et.mapSize);const ot=et.getFrameExtents();if(s.multiply(ot),r.copy(et.mapSize),(s.x>d||s.y>d)&&(s.x>d&&(r.x=Math.floor(d/ot.x),s.x=r.x*ot.x,et.mapSize.x=r.x),s.y>d&&(r.y=Math.floor(d/ot.y),s.y=r.y*ot.y,et.mapSize.y=r.y)),et.map===null||mt===!0||O===!0){const pt=this.type!==Xn?{minFilter:Ke,magFilter:Ke}:{};et.map!==null&&et.map.dispose(),et.map=new Hi(s.x,s.y,pt),et.map.texture.name=st.name+".shadowMap",et.camera.updateProjectionMatrix()}n.setRenderTarget(et.map),n.clear();const ut=et.getViewportCount();for(let pt=0;pt<ut;pt++){const ft=et.getViewport(pt);a.set(r.x*ft.x,r.y*ft.y,r.x*ft.z,r.y*ft.w),rt.viewport(a),et.updateMatrices(st,pt),i=et.getFrustum(),x(D,G,et.camera,st,this.type)}et.isPointLightShadow!==!0&&this.type===Xn&&E(et,G),et.needsUpdate=!1}m=this.type,_.needsUpdate=!1,n.setRenderTarget(T,R,J)};function E(A,D){const G=t.update(g);p.defines.VSM_SAMPLES!==A.blurSamples&&(p.defines.VSM_SAMPLES=A.blurSamples,f.defines.VSM_SAMPLES=A.blurSamples,p.needsUpdate=!0,f.needsUpdate=!0),A.mapPass===null&&(A.mapPass=new Hi(s.x,s.y)),p.uniforms.shadow_pass.value=A.map.texture,p.uniforms.resolution.value=A.mapSize,p.uniforms.radius.value=A.radius,n.setRenderTarget(A.mapPass),n.clear(),n.renderBufferDirect(D,null,G,p,g,null),f.uniforms.shadow_pass.value=A.mapPass.texture,f.uniforms.resolution.value=A.mapSize,f.uniforms.radius.value=A.radius,n.setRenderTarget(A.map),n.clear(),n.renderBufferDirect(D,null,G,f,g,null)}function b(A,D,G,T){let R=null;const J=G.isPointLight===!0?A.customDistanceMaterial:A.customDepthMaterial;if(J!==void 0)R=J;else if(R=G.isPointLight===!0?l:o,n.localClippingEnabled&&D.clipShadows===!0&&Array.isArray(D.clippingPlanes)&&D.clippingPlanes.length!==0||D.displacementMap&&D.displacementScale!==0||D.alphaMap&&D.alphaTest>0||D.map&&D.alphaTest>0){const rt=R.uuid,mt=D.uuid;let O=c[rt];O===void 0&&(O={},c[rt]=O);let Z=O[mt];Z===void 0&&(Z=R.clone(),O[mt]=Z,D.addEventListener("dispose",C)),R=Z}if(R.visible=D.visible,R.wireframe=D.wireframe,T===Xn?R.side=D.shadowSide!==null?D.shadowSide:D.side:R.side=D.shadowSide!==null?D.shadowSide:h[D.side],R.alphaMap=D.alphaMap,R.alphaTest=D.alphaTest,R.map=D.map,R.clipShadows=D.clipShadows,R.clippingPlanes=D.clippingPlanes,R.clipIntersection=D.clipIntersection,R.displacementMap=D.displacementMap,R.displacementScale=D.displacementScale,R.displacementBias=D.displacementBias,R.wireframeLinewidth=D.wireframeLinewidth,R.linewidth=D.linewidth,G.isPointLight===!0&&R.isMeshDistanceMaterial===!0){const rt=n.properties.get(R);rt.light=G}return R}function x(A,D,G,T,R){if(A.visible===!1)return;if(A.layers.test(D.layers)&&(A.isMesh||A.isLine||A.isPoints)&&(A.castShadow||A.receiveShadow&&R===Xn)&&(!A.frustumCulled||i.intersectsObject(A))){A.modelViewMatrix.multiplyMatrices(G.matrixWorldInverse,A.matrixWorld);const mt=t.update(A),O=A.material;if(Array.isArray(O)){const Z=mt.groups;for(let tt=0,st=Z.length;tt<st;tt++){const et=Z[tt],ot=O[et.materialIndex];if(ot&&ot.visible){const ut=b(A,ot,T,R);A.onBeforeShadow(n,A,D,G,mt,ut,et),n.renderBufferDirect(G,null,mt,ut,A,et),A.onAfterShadow(n,A,D,G,mt,ut,et)}}}else if(O.visible){const Z=b(A,O,T,R);A.onBeforeShadow(n,A,D,G,mt,Z,null),n.renderBufferDirect(G,null,mt,Z,A,null),A.onAfterShadow(n,A,D,G,mt,Z,null)}}const rt=A.children;for(let mt=0,O=rt.length;mt<O;mt++)x(rt[mt],D,G,T,R)}function C(A){A.target.removeEventListener("dispose",C);for(const G in c){const T=c[G],R=A.target.uuid;R in T&&(T[R].dispose(),delete T[R])}}}function ay(n,t,e){const i=e.isWebGL2;function s(){let F=!1;const Mt=new xe;let St=null;const Vt=new xe(0,0,0,0);return{setMask:function(Ft){St!==Ft&&!F&&(n.colorMask(Ft,Ft,Ft,Ft),St=Ft)},setLocked:function(Ft){F=Ft},setClear:function(Ft,fe,pe,Le,Xe){Xe===!0&&(Ft*=Le,fe*=Le,pe*=Le),Mt.set(Ft,fe,pe,Le),Vt.equals(Mt)===!1&&(n.clearColor(Ft,fe,pe,Le),Vt.copy(Mt))},reset:function(){F=!1,St=null,Vt.set(-1,0,0,0)}}}function r(){let F=!1,Mt=null,St=null,Vt=null;return{setTest:function(Ft){Ft?Gt(n.DEPTH_TEST):Nt(n.DEPTH_TEST)},setMask:function(Ft){Mt!==Ft&&!F&&(n.depthMask(Ft),Mt=Ft)},setFunc:function(Ft){if(St!==Ft){switch(Ft){case Am:n.depthFunc(n.NEVER);break;case Cm:n.depthFunc(n.ALWAYS);break;case Rm:n.depthFunc(n.LESS);break;case eo:n.depthFunc(n.LEQUAL);break;case Lm:n.depthFunc(n.EQUAL);break;case Pm:n.depthFunc(n.GEQUAL);break;case Dm:n.depthFunc(n.GREATER);break;case Um:n.depthFunc(n.NOTEQUAL);break;default:n.depthFunc(n.LEQUAL)}St=Ft}},setLocked:function(Ft){F=Ft},setClear:function(Ft){Vt!==Ft&&(n.clearDepth(Ft),Vt=Ft)},reset:function(){F=!1,Mt=null,St=null,Vt=null}}}function a(){let F=!1,Mt=null,St=null,Vt=null,Ft=null,fe=null,pe=null,Le=null,Xe=null;return{setTest:function(me){F||(me?Gt(n.STENCIL_TEST):Nt(n.STENCIL_TEST))},setMask:function(me){Mt!==me&&!F&&(n.stencilMask(me),Mt=me)},setFunc:function(me,qe,Cn){(St!==me||Vt!==qe||Ft!==Cn)&&(n.stencilFunc(me,qe,Cn),St=me,Vt=qe,Ft=Cn)},setOp:function(me,qe,Cn){(fe!==me||pe!==qe||Le!==Cn)&&(n.stencilOp(me,qe,Cn),fe=me,pe=qe,Le=Cn)},setLocked:function(me){F=me},setClear:function(me){Xe!==me&&(n.clearStencil(me),Xe=me)},reset:function(){F=!1,Mt=null,St=null,Vt=null,Ft=null,fe=null,pe=null,Le=null,Xe=null}}}const o=new s,l=new r,c=new a,d=new WeakMap,h=new WeakMap;let p={},f={},v=new WeakMap,g=[],_=null,m=!1,E=null,b=null,x=null,C=null,A=null,D=null,G=null,T=new ne(0,0,0),R=0,J=!1,rt=null,mt=null,O=null,Z=null,tt=null;const st=n.getParameter(n.MAX_COMBINED_TEXTURE_IMAGE_UNITS);let et=!1,ot=0;const ut=n.getParameter(n.VERSION);ut.indexOf("WebGL")!==-1?(ot=parseFloat(/^WebGL (\d)/.exec(ut)[1]),et=ot>=1):ut.indexOf("OpenGL ES")!==-1&&(ot=parseFloat(/^OpenGL ES (\d)/.exec(ut)[1]),et=ot>=2);let pt=null,ft={};const it=n.getParameter(n.SCISSOR_BOX),ht=n.getParameter(n.VIEWPORT),bt=new xe().fromArray(it),Lt=new xe().fromArray(ht);function Pt(F,Mt,St,Vt){const Ft=new Uint8Array(4),fe=n.createTexture();n.bindTexture(F,fe),n.texParameteri(F,n.TEXTURE_MIN_FILTER,n.NEAREST),n.texParameteri(F,n.TEXTURE_MAG_FILTER,n.NEAREST);for(let pe=0;pe<St;pe++)i&&(F===n.TEXTURE_3D||F===n.TEXTURE_2D_ARRAY)?n.texImage3D(Mt,0,n.RGBA,1,1,Vt,0,n.RGBA,n.UNSIGNED_BYTE,Ft):n.texImage2D(Mt+pe,0,n.RGBA,1,1,0,n.RGBA,n.UNSIGNED_BYTE,Ft);return fe}const Ht={};Ht[n.TEXTURE_2D]=Pt(n.TEXTURE_2D,n.TEXTURE_2D,1),Ht[n.TEXTURE_CUBE_MAP]=Pt(n.TEXTURE_CUBE_MAP,n.TEXTURE_CUBE_MAP_POSITIVE_X,6),i&&(Ht[n.TEXTURE_2D_ARRAY]=Pt(n.TEXTURE_2D_ARRAY,n.TEXTURE_2D_ARRAY,1,1),Ht[n.TEXTURE_3D]=Pt(n.TEXTURE_3D,n.TEXTURE_3D,1,1)),o.setClear(0,0,0,1),l.setClear(1),c.setClear(0),Gt(n.DEPTH_TEST),l.setFunc(eo),j(!1),S(ic),Gt(n.CULL_FACE),N(mi);function Gt(F){p[F]!==!0&&(n.enable(F),p[F]=!0)}function Nt(F){p[F]!==!1&&(n.disable(F),p[F]=!1)}function Zt(F,Mt){return f[F]!==Mt?(n.bindFramebuffer(F,Mt),f[F]=Mt,i&&(F===n.DRAW_FRAMEBUFFER&&(f[n.FRAMEBUFFER]=Mt),F===n.FRAMEBUFFER&&(f[n.DRAW_FRAMEBUFFER]=Mt)),!0):!1}function M(F,Mt){let St=g,Vt=!1;if(F)if(St=v.get(Mt),St===void 0&&(St=[],v.set(Mt,St)),F.isWebGLMultipleRenderTargets){const Ft=F.texture;if(St.length!==Ft.length||St[0]!==n.COLOR_ATTACHMENT0){for(let fe=0,pe=Ft.length;fe<pe;fe++)St[fe]=n.COLOR_ATTACHMENT0+fe;St.length=Ft.length,Vt=!0}}else St[0]!==n.COLOR_ATTACHMENT0&&(St[0]=n.COLOR_ATTACHMENT0,Vt=!0);else St[0]!==n.BACK&&(St[0]=n.BACK,Vt=!0);Vt&&(e.isWebGL2?n.drawBuffers(St):t.get("WEBGL_draw_buffers").drawBuffersWEBGL(St))}function k(F){return _!==F?(n.useProgram(F),_=F,!0):!1}const B={[Ii]:n.FUNC_ADD,[hm]:n.FUNC_SUBTRACT,[fm]:n.FUNC_REVERSE_SUBTRACT};if(i)B[oc]=n.MIN,B[ac]=n.MAX;else{const F=t.get("EXT_blend_minmax");F!==null&&(B[oc]=F.MIN_EXT,B[ac]=F.MAX_EXT)}const Y={[pm]:n.ZERO,[mm]:n.ONE,[_m]:n.SRC_COLOR,[La]:n.SRC_ALPHA,[Mm]:n.SRC_ALPHA_SATURATE,[ym]:n.DST_COLOR,[vm]:n.DST_ALPHA,[gm]:n.ONE_MINUS_SRC_COLOR,[Pa]:n.ONE_MINUS_SRC_ALPHA,[bm]:n.ONE_MINUS_DST_COLOR,[xm]:n.ONE_MINUS_DST_ALPHA,[Sm]:n.CONSTANT_COLOR,[Em]:n.ONE_MINUS_CONSTANT_COLOR,[Tm]:n.CONSTANT_ALPHA,[wm]:n.ONE_MINUS_CONSTANT_ALPHA};function N(F,Mt,St,Vt,Ft,fe,pe,Le,Xe,me){if(F===mi){m===!0&&(Nt(n.BLEND),m=!1);return}if(m===!1&&(Gt(n.BLEND),m=!0),F!==dm){if(F!==E||me!==J){if((b!==Ii||A!==Ii)&&(n.blendEquation(n.FUNC_ADD),b=Ii,A=Ii),me)switch(F){case ps:n.blendFuncSeparate(n.ONE,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case jn:n.blendFunc(n.ONE,n.ONE);break;case sc:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case rc:n.blendFuncSeparate(n.ZERO,n.SRC_COLOR,n.ZERO,n.SRC_ALPHA);break;default:console.error("THREE.WebGLState: Invalid blending: ",F);break}else switch(F){case ps:n.blendFuncSeparate(n.SRC_ALPHA,n.ONE_MINUS_SRC_ALPHA,n.ONE,n.ONE_MINUS_SRC_ALPHA);break;case jn:n.blendFunc(n.SRC_ALPHA,n.ONE);break;case sc:n.blendFuncSeparate(n.ZERO,n.ONE_MINUS_SRC_COLOR,n.ZERO,n.ONE);break;case rc:n.blendFunc(n.ZERO,n.SRC_COLOR);break;default:console.error("THREE.WebGLState: Invalid blending: ",F);break}x=null,C=null,D=null,G=null,T.set(0,0,0),R=0,E=F,J=me}return}Ft=Ft||Mt,fe=fe||St,pe=pe||Vt,(Mt!==b||Ft!==A)&&(n.blendEquationSeparate(B[Mt],B[Ft]),b=Mt,A=Ft),(St!==x||Vt!==C||fe!==D||pe!==G)&&(n.blendFuncSeparate(Y[St],Y[Vt],Y[fe],Y[pe]),x=St,C=Vt,D=fe,G=pe),(Le.equals(T)===!1||Xe!==R)&&(n.blendColor(Le.r,Le.g,Le.b,Xe),T.copy(Le),R=Xe),E=F,J=!1}function V(F,Mt){F.side===Qn?Nt(n.CULL_FACE):Gt(n.CULL_FACE);let St=F.side===tn;Mt&&(St=!St),j(St),F.blending===ps&&F.transparent===!1?N(mi):N(F.blending,F.blendEquation,F.blendSrc,F.blendDst,F.blendEquationAlpha,F.blendSrcAlpha,F.blendDstAlpha,F.blendColor,F.blendAlpha,F.premultipliedAlpha),l.setFunc(F.depthFunc),l.setTest(F.depthTest),l.setMask(F.depthWrite),o.setMask(F.colorWrite);const Vt=F.stencilWrite;c.setTest(Vt),Vt&&(c.setMask(F.stencilWriteMask),c.setFunc(F.stencilFunc,F.stencilRef,F.stencilFuncMask),c.setOp(F.stencilFail,F.stencilZFail,F.stencilZPass)),U(F.polygonOffset,F.polygonOffsetFactor,F.polygonOffsetUnits),F.alphaToCoverage===!0?Gt(n.SAMPLE_ALPHA_TO_COVERAGE):Nt(n.SAMPLE_ALPHA_TO_COVERAGE)}function j(F){rt!==F&&(F?n.frontFace(n.CW):n.frontFace(n.CCW),rt=F)}function S(F){F!==lm?(Gt(n.CULL_FACE),F!==mt&&(F===ic?n.cullFace(n.BACK):F===cm?n.cullFace(n.FRONT):n.cullFace(n.FRONT_AND_BACK))):Nt(n.CULL_FACE),mt=F}function y(F){F!==O&&(et&&n.lineWidth(F),O=F)}function U(F,Mt,St){F?(Gt(n.POLYGON_OFFSET_FILL),(Z!==Mt||tt!==St)&&(n.polygonOffset(Mt,St),Z=Mt,tt=St)):Nt(n.POLYGON_OFFSET_FILL)}function q(F){F?Gt(n.SCISSOR_TEST):Nt(n.SCISSOR_TEST)}function H(F){F===void 0&&(F=n.TEXTURE0+st-1),pt!==F&&(n.activeTexture(F),pt=F)}function X(F,Mt,St){St===void 0&&(pt===null?St=n.TEXTURE0+st-1:St=pt);let Vt=ft[St];Vt===void 0&&(Vt={type:void 0,texture:void 0},ft[St]=Vt),(Vt.type!==F||Vt.texture!==Mt)&&(pt!==St&&(n.activeTexture(St),pt=St),n.bindTexture(F,Mt||Ht[F]),Vt.type=F,Vt.texture=Mt)}function lt(){const F=ft[pt];F!==void 0&&F.type!==void 0&&(n.bindTexture(F.type,null),F.type=void 0,F.texture=void 0)}function at(){try{n.compressedTexImage2D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function dt(){try{n.compressedTexImage3D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function ct(){try{n.texSubImage2D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function _t(){try{n.texSubImage3D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function $(){try{n.compressedTexSubImage2D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function Rt(){try{n.compressedTexSubImage3D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function At(){try{n.texStorage2D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function Tt(){try{n.texStorage3D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function Ct(){try{n.texImage2D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function xt(){try{n.texImage3D.apply(n,arguments)}catch(F){console.error("THREE.WebGLState:",F)}}function Dt(F){bt.equals(F)===!1&&(n.scissor(F.x,F.y,F.z,F.w),bt.copy(F))}function Yt(F){Lt.equals(F)===!1&&(n.viewport(F.x,F.y,F.z,F.w),Lt.copy(F))}function le(F,Mt){let St=h.get(Mt);St===void 0&&(St=new WeakMap,h.set(Mt,St));let Vt=St.get(F);Vt===void 0&&(Vt=n.getUniformBlockIndex(Mt,F.name),St.set(F,Vt))}function Jt(F,Mt){const Vt=h.get(Mt).get(F);d.get(Mt)!==Vt&&(n.uniformBlockBinding(Mt,Vt,F.__bindingPointIndex),d.set(Mt,Vt))}function gt(){n.disable(n.BLEND),n.disable(n.CULL_FACE),n.disable(n.DEPTH_TEST),n.disable(n.POLYGON_OFFSET_FILL),n.disable(n.SCISSOR_TEST),n.disable(n.STENCIL_TEST),n.disable(n.SAMPLE_ALPHA_TO_COVERAGE),n.blendEquation(n.FUNC_ADD),n.blendFunc(n.ONE,n.ZERO),n.blendFuncSeparate(n.ONE,n.ZERO,n.ONE,n.ZERO),n.blendColor(0,0,0,0),n.colorMask(!0,!0,!0,!0),n.clearColor(0,0,0,0),n.depthMask(!0),n.depthFunc(n.LESS),n.clearDepth(1),n.stencilMask(4294967295),n.stencilFunc(n.ALWAYS,0,4294967295),n.stencilOp(n.KEEP,n.KEEP,n.KEEP),n.clearStencil(0),n.cullFace(n.BACK),n.frontFace(n.CCW),n.polygonOffset(0,0),n.activeTexture(n.TEXTURE0),n.bindFramebuffer(n.FRAMEBUFFER,null),i===!0&&(n.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),n.bindFramebuffer(n.READ_FRAMEBUFFER,null)),n.useProgram(null),n.lineWidth(1),n.scissor(0,0,n.canvas.width,n.canvas.height),n.viewport(0,0,n.canvas.width,n.canvas.height),p={},pt=null,ft={},f={},v=new WeakMap,g=[],_=null,m=!1,E=null,b=null,x=null,C=null,A=null,D=null,G=null,T=new ne(0,0,0),R=0,J=!1,rt=null,mt=null,O=null,Z=null,tt=null,bt.set(0,0,n.canvas.width,n.canvas.height),Lt.set(0,0,n.canvas.width,n.canvas.height),o.reset(),l.reset(),c.reset()}return{buffers:{color:o,depth:l,stencil:c},enable:Gt,disable:Nt,bindFramebuffer:Zt,drawBuffers:M,useProgram:k,setBlending:N,setMaterial:V,setFlipSided:j,setCullFace:S,setLineWidth:y,setPolygonOffset:U,setScissorTest:q,activeTexture:H,bindTexture:X,unbindTexture:lt,compressedTexImage2D:at,compressedTexImage3D:dt,texImage2D:Ct,texImage3D:xt,updateUBOMapping:le,uniformBlockBinding:Jt,texStorage2D:At,texStorage3D:Tt,texSubImage2D:ct,texSubImage3D:_t,compressedTexSubImage2D:$,compressedTexSubImage3D:Rt,scissor:Dt,viewport:Yt,reset:gt}}function ly(n,t,e,i,s,r,a){const o=s.isWebGL2,l=t.has("WEBGL_multisampled_render_to_texture")?t.get("WEBGL_multisampled_render_to_texture"):null,c=typeof navigator>"u"?!1:/OculusBrowser/g.test(navigator.userAgent),d=new WeakMap;let h;const p=new WeakMap;let f=!1;try{f=typeof OffscreenCanvas<"u"&&new OffscreenCanvas(1,1).getContext("2d")!==null}catch{}function v(S,y){return f?new OffscreenCanvas(S,y):oo("canvas")}function g(S,y,U,q){let H=1;if((S.width>q||S.height>q)&&(H=q/Math.max(S.width,S.height)),H<1||y===!0)if(typeof HTMLImageElement<"u"&&S instanceof HTMLImageElement||typeof HTMLCanvasElement<"u"&&S instanceof HTMLCanvasElement||typeof ImageBitmap<"u"&&S instanceof ImageBitmap){const X=y?Oa:Math.floor,lt=X(H*S.width),at=X(H*S.height);h===void 0&&(h=v(lt,at));const dt=U?v(lt,at):h;return dt.width=lt,dt.height=at,dt.getContext("2d").drawImage(S,0,0,lt,at),console.warn("THREE.WebGLRenderer: Texture has been resized from ("+S.width+"x"+S.height+") to ("+lt+"x"+at+")."),dt}else return"data"in S&&console.warn("THREE.WebGLRenderer: Image in DataTexture is too big ("+S.width+"x"+S.height+")."),S;return S}function _(S){return Fc(S.width)&&Fc(S.height)}function m(S){return o?!1:S.wrapS!==Sn||S.wrapT!==Sn||S.minFilter!==Ke&&S.minFilter!==pn}function E(S,y){return S.generateMipmaps&&y&&S.minFilter!==Ke&&S.minFilter!==pn}function b(S){n.generateMipmap(S)}function x(S,y,U,q,H=!1){if(o===!1)return y;if(S!==null){if(n[S]!==void 0)return n[S];console.warn("THREE.WebGLRenderer: Attempt to use non-existing WebGL internal format '"+S+"'")}let X=y;if(y===n.RED&&(U===n.FLOAT&&(X=n.R32F),U===n.HALF_FLOAT&&(X=n.R16F),U===n.UNSIGNED_BYTE&&(X=n.R8)),y===n.RED_INTEGER&&(U===n.UNSIGNED_BYTE&&(X=n.R8UI),U===n.UNSIGNED_SHORT&&(X=n.R16UI),U===n.UNSIGNED_INT&&(X=n.R32UI),U===n.BYTE&&(X=n.R8I),U===n.SHORT&&(X=n.R16I),U===n.INT&&(X=n.R32I)),y===n.RG&&(U===n.FLOAT&&(X=n.RG32F),U===n.HALF_FLOAT&&(X=n.RG16F),U===n.UNSIGNED_BYTE&&(X=n.RG8)),y===n.RGBA){const lt=H?no:ce.getTransfer(q);U===n.FLOAT&&(X=n.RGBA32F),U===n.HALF_FLOAT&&(X=n.RGBA16F),U===n.UNSIGNED_BYTE&&(X=lt===ge?n.SRGB8_ALPHA8:n.RGBA8),U===n.UNSIGNED_SHORT_4_4_4_4&&(X=n.RGBA4),U===n.UNSIGNED_SHORT_5_5_5_1&&(X=n.RGB5_A1)}return(X===n.R16F||X===n.R32F||X===n.RG16F||X===n.RG32F||X===n.RGBA16F||X===n.RGBA32F)&&t.get("EXT_color_buffer_float"),X}function C(S,y,U){return E(S,U)===!0||S.isFramebufferTexture&&S.minFilter!==Ke&&S.minFilter!==pn?Math.log2(Math.max(y.width,y.height))+1:S.mipmaps!==void 0&&S.mipmaps.length>0?S.mipmaps.length:S.isCompressedTexture&&Array.isArray(S.image)?y.mipmaps.length:1}function A(S){return S===Ke||S===lc||S===Oo?n.NEAREST:n.LINEAR}function D(S){const y=S.target;y.removeEventListener("dispose",D),T(y),y.isVideoTexture&&d.delete(y)}function G(S){const y=S.target;y.removeEventListener("dispose",G),J(y)}function T(S){const y=i.get(S);if(y.__webglInit===void 0)return;const U=S.source,q=p.get(U);if(q){const H=q[y.__cacheKey];H.usedTimes--,H.usedTimes===0&&R(S),Object.keys(q).length===0&&p.delete(U)}i.remove(S)}function R(S){const y=i.get(S);n.deleteTexture(y.__webglTexture);const U=S.source,q=p.get(U);delete q[y.__cacheKey],a.memory.textures--}function J(S){const y=S.texture,U=i.get(S),q=i.get(y);if(q.__webglTexture!==void 0&&(n.deleteTexture(q.__webglTexture),a.memory.textures--),S.depthTexture&&S.depthTexture.dispose(),S.isWebGLCubeRenderTarget)for(let H=0;H<6;H++){if(Array.isArray(U.__webglFramebuffer[H]))for(let X=0;X<U.__webglFramebuffer[H].length;X++)n.deleteFramebuffer(U.__webglFramebuffer[H][X]);else n.deleteFramebuffer(U.__webglFramebuffer[H]);U.__webglDepthbuffer&&n.deleteRenderbuffer(U.__webglDepthbuffer[H])}else{if(Array.isArray(U.__webglFramebuffer))for(let H=0;H<U.__webglFramebuffer.length;H++)n.deleteFramebuffer(U.__webglFramebuffer[H]);else n.deleteFramebuffer(U.__webglFramebuffer);if(U.__webglDepthbuffer&&n.deleteRenderbuffer(U.__webglDepthbuffer),U.__webglMultisampledFramebuffer&&n.deleteFramebuffer(U.__webglMultisampledFramebuffer),U.__webglColorRenderbuffer)for(let H=0;H<U.__webglColorRenderbuffer.length;H++)U.__webglColorRenderbuffer[H]&&n.deleteRenderbuffer(U.__webglColorRenderbuffer[H]);U.__webglDepthRenderbuffer&&n.deleteRenderbuffer(U.__webglDepthRenderbuffer)}if(S.isWebGLMultipleRenderTargets)for(let H=0,X=y.length;H<X;H++){const lt=i.get(y[H]);lt.__webglTexture&&(n.deleteTexture(lt.__webglTexture),a.memory.textures--),i.remove(y[H])}i.remove(y),i.remove(S)}let rt=0;function mt(){rt=0}function O(){const S=rt;return S>=s.maxTextures&&console.warn("THREE.WebGLTextures: Trying to use "+S+" texture units while this GPU supports only "+s.maxTextures),rt+=1,S}function Z(S){const y=[];return y.push(S.wrapS),y.push(S.wrapT),y.push(S.wrapR||0),y.push(S.magFilter),y.push(S.minFilter),y.push(S.anisotropy),y.push(S.internalFormat),y.push(S.format),y.push(S.type),y.push(S.generateMipmaps),y.push(S.premultiplyAlpha),y.push(S.flipY),y.push(S.unpackAlignment),y.push(S.colorSpace),y.join()}function tt(S,y){const U=i.get(S);if(S.isVideoTexture&&V(S),S.isRenderTargetTexture===!1&&S.version>0&&U.__version!==S.version){const q=S.image;if(q===null)console.warn("THREE.WebGLRenderer: Texture marked for update but no image data found.");else if(q.complete===!1)console.warn("THREE.WebGLRenderer: Texture marked for update but image is incomplete");else{bt(U,S,y);return}}e.bindTexture(n.TEXTURE_2D,U.__webglTexture,n.TEXTURE0+y)}function st(S,y){const U=i.get(S);if(S.version>0&&U.__version!==S.version){bt(U,S,y);return}e.bindTexture(n.TEXTURE_2D_ARRAY,U.__webglTexture,n.TEXTURE0+y)}function et(S,y){const U=i.get(S);if(S.version>0&&U.__version!==S.version){bt(U,S,y);return}e.bindTexture(n.TEXTURE_3D,U.__webglTexture,n.TEXTURE0+y)}function ot(S,y){const U=i.get(S);if(S.version>0&&U.__version!==S.version){Lt(U,S,y);return}e.bindTexture(n.TEXTURE_CUBE_MAP,U.__webglTexture,n.TEXTURE0+y)}const ut={[Ia]:n.REPEAT,[Sn]:n.CLAMP_TO_EDGE,[Na]:n.MIRRORED_REPEAT},pt={[Ke]:n.NEAREST,[lc]:n.NEAREST_MIPMAP_NEAREST,[Oo]:n.NEAREST_MIPMAP_LINEAR,[pn]:n.LINEAR,[Vm]:n.LINEAR_MIPMAP_NEAREST,[tr]:n.LINEAR_MIPMAP_LINEAR},ft={[e_]:n.NEVER,[a_]:n.ALWAYS,[n_]:n.LESS,[Zd]:n.LEQUAL,[i_]:n.EQUAL,[o_]:n.GEQUAL,[s_]:n.GREATER,[r_]:n.NOTEQUAL};function it(S,y,U){if(U?(n.texParameteri(S,n.TEXTURE_WRAP_S,ut[y.wrapS]),n.texParameteri(S,n.TEXTURE_WRAP_T,ut[y.wrapT]),(S===n.TEXTURE_3D||S===n.TEXTURE_2D_ARRAY)&&n.texParameteri(S,n.TEXTURE_WRAP_R,ut[y.wrapR]),n.texParameteri(S,n.TEXTURE_MAG_FILTER,pt[y.magFilter]),n.texParameteri(S,n.TEXTURE_MIN_FILTER,pt[y.minFilter])):(n.texParameteri(S,n.TEXTURE_WRAP_S,n.CLAMP_TO_EDGE),n.texParameteri(S,n.TEXTURE_WRAP_T,n.CLAMP_TO_EDGE),(S===n.TEXTURE_3D||S===n.TEXTURE_2D_ARRAY)&&n.texParameteri(S,n.TEXTURE_WRAP_R,n.CLAMP_TO_EDGE),(y.wrapS!==Sn||y.wrapT!==Sn)&&console.warn("THREE.WebGLRenderer: Texture is not power of two. Texture.wrapS and Texture.wrapT should be set to THREE.ClampToEdgeWrapping."),n.texParameteri(S,n.TEXTURE_MAG_FILTER,A(y.magFilter)),n.texParameteri(S,n.TEXTURE_MIN_FILTER,A(y.minFilter)),y.minFilter!==Ke&&y.minFilter!==pn&&console.warn("THREE.WebGLRenderer: Texture is not power of two. Texture.minFilter should be set to THREE.NearestFilter or THREE.LinearFilter.")),y.compareFunction&&(n.texParameteri(S,n.TEXTURE_COMPARE_MODE,n.COMPARE_REF_TO_TEXTURE),n.texParameteri(S,n.TEXTURE_COMPARE_FUNC,ft[y.compareFunction])),t.has("EXT_texture_filter_anisotropic")===!0){const q=t.get("EXT_texture_filter_anisotropic");if(y.magFilter===Ke||y.minFilter!==Oo&&y.minFilter!==tr||y.type===pi&&t.has("OES_texture_float_linear")===!1||o===!1&&y.type===er&&t.has("OES_texture_half_float_linear")===!1)return;(y.anisotropy>1||i.get(y).__currentAnisotropy)&&(n.texParameterf(S,q.TEXTURE_MAX_ANISOTROPY_EXT,Math.min(y.anisotropy,s.getMaxAnisotropy())),i.get(y).__currentAnisotropy=y.anisotropy)}}function ht(S,y){let U=!1;S.__webglInit===void 0&&(S.__webglInit=!0,y.addEventListener("dispose",D));const q=y.source;let H=p.get(q);H===void 0&&(H={},p.set(q,H));const X=Z(y);if(X!==S.__cacheKey){H[X]===void 0&&(H[X]={texture:n.createTexture(),usedTimes:0},a.memory.textures++,U=!0),H[X].usedTimes++;const lt=H[S.__cacheKey];lt!==void 0&&(H[S.__cacheKey].usedTimes--,lt.usedTimes===0&&R(y)),S.__cacheKey=X,S.__webglTexture=H[X].texture}return U}function bt(S,y,U){let q=n.TEXTURE_2D;(y.isDataArrayTexture||y.isCompressedArrayTexture)&&(q=n.TEXTURE_2D_ARRAY),y.isData3DTexture&&(q=n.TEXTURE_3D);const H=ht(S,y),X=y.source;e.bindTexture(q,S.__webglTexture,n.TEXTURE0+U);const lt=i.get(X);if(X.version!==lt.__version||H===!0){e.activeTexture(n.TEXTURE0+U);const at=ce.getPrimaries(ce.workingColorSpace),dt=y.colorSpace===mn?null:ce.getPrimaries(y.colorSpace),ct=y.colorSpace===mn||at===dt?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,y.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,y.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,y.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,ct);const _t=m(y)&&_(y.image)===!1;let $=g(y.image,_t,!1,s.maxTextureSize);$=j(y,$);const Rt=_($)||o,At=r.convert(y.format,y.colorSpace);let Tt=r.convert(y.type),Ct=x(y.internalFormat,At,Tt,y.colorSpace,y.isVideoTexture);it(q,y,Rt);let xt;const Dt=y.mipmaps,Yt=o&&y.isVideoTexture!==!0&&Ct!==Yd,le=lt.__version===void 0||H===!0,Jt=C(y,$,Rt);if(y.isDepthTexture)Ct=n.DEPTH_COMPONENT,o?y.type===pi?Ct=n.DEPTH_COMPONENT32F:y.type===fi?Ct=n.DEPTH_COMPONENT24:y.type===Oi?Ct=n.DEPTH24_STENCIL8:Ct=n.DEPTH_COMPONENT16:y.type===pi&&console.error("WebGLRenderer: Floating point depth texture requires WebGL2."),y.format===Bi&&Ct===n.DEPTH_COMPONENT&&y.type!==rl&&y.type!==fi&&(console.warn("THREE.WebGLRenderer: Use UnsignedShortType or UnsignedIntType for DepthFormat DepthTexture."),y.type=fi,Tt=r.convert(y.type)),y.format===Ms&&Ct===n.DEPTH_COMPONENT&&(Ct=n.DEPTH_STENCIL,y.type!==Oi&&(console.warn("THREE.WebGLRenderer: Use UnsignedInt248Type for DepthStencilFormat DepthTexture."),y.type=Oi,Tt=r.convert(y.type))),le&&(Yt?e.texStorage2D(n.TEXTURE_2D,1,Ct,$.width,$.height):e.texImage2D(n.TEXTURE_2D,0,Ct,$.width,$.height,0,At,Tt,null));else if(y.isDataTexture)if(Dt.length>0&&Rt){Yt&&le&&e.texStorage2D(n.TEXTURE_2D,Jt,Ct,Dt[0].width,Dt[0].height);for(let gt=0,F=Dt.length;gt<F;gt++)xt=Dt[gt],Yt?e.texSubImage2D(n.TEXTURE_2D,gt,0,0,xt.width,xt.height,At,Tt,xt.data):e.texImage2D(n.TEXTURE_2D,gt,Ct,xt.width,xt.height,0,At,Tt,xt.data);y.generateMipmaps=!1}else Yt?(le&&e.texStorage2D(n.TEXTURE_2D,Jt,Ct,$.width,$.height),e.texSubImage2D(n.TEXTURE_2D,0,0,0,$.width,$.height,At,Tt,$.data)):e.texImage2D(n.TEXTURE_2D,0,Ct,$.width,$.height,0,At,Tt,$.data);else if(y.isCompressedTexture)if(y.isCompressedArrayTexture){Yt&&le&&e.texStorage3D(n.TEXTURE_2D_ARRAY,Jt,Ct,Dt[0].width,Dt[0].height,$.depth);for(let gt=0,F=Dt.length;gt<F;gt++)xt=Dt[gt],y.format!==En?At!==null?Yt?e.compressedTexSubImage3D(n.TEXTURE_2D_ARRAY,gt,0,0,0,xt.width,xt.height,$.depth,At,xt.data,0,0):e.compressedTexImage3D(n.TEXTURE_2D_ARRAY,gt,Ct,xt.width,xt.height,$.depth,0,xt.data,0,0):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):Yt?e.texSubImage3D(n.TEXTURE_2D_ARRAY,gt,0,0,0,xt.width,xt.height,$.depth,At,Tt,xt.data):e.texImage3D(n.TEXTURE_2D_ARRAY,gt,Ct,xt.width,xt.height,$.depth,0,At,Tt,xt.data)}else{Yt&&le&&e.texStorage2D(n.TEXTURE_2D,Jt,Ct,Dt[0].width,Dt[0].height);for(let gt=0,F=Dt.length;gt<F;gt++)xt=Dt[gt],y.format!==En?At!==null?Yt?e.compressedTexSubImage2D(n.TEXTURE_2D,gt,0,0,xt.width,xt.height,At,xt.data):e.compressedTexImage2D(n.TEXTURE_2D,gt,Ct,xt.width,xt.height,0,xt.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .uploadTexture()"):Yt?e.texSubImage2D(n.TEXTURE_2D,gt,0,0,xt.width,xt.height,At,Tt,xt.data):e.texImage2D(n.TEXTURE_2D,gt,Ct,xt.width,xt.height,0,At,Tt,xt.data)}else if(y.isDataArrayTexture)Yt?(le&&e.texStorage3D(n.TEXTURE_2D_ARRAY,Jt,Ct,$.width,$.height,$.depth),e.texSubImage3D(n.TEXTURE_2D_ARRAY,0,0,0,0,$.width,$.height,$.depth,At,Tt,$.data)):e.texImage3D(n.TEXTURE_2D_ARRAY,0,Ct,$.width,$.height,$.depth,0,At,Tt,$.data);else if(y.isData3DTexture)Yt?(le&&e.texStorage3D(n.TEXTURE_3D,Jt,Ct,$.width,$.height,$.depth),e.texSubImage3D(n.TEXTURE_3D,0,0,0,0,$.width,$.height,$.depth,At,Tt,$.data)):e.texImage3D(n.TEXTURE_3D,0,Ct,$.width,$.height,$.depth,0,At,Tt,$.data);else if(y.isFramebufferTexture){if(le)if(Yt)e.texStorage2D(n.TEXTURE_2D,Jt,Ct,$.width,$.height);else{let gt=$.width,F=$.height;for(let Mt=0;Mt<Jt;Mt++)e.texImage2D(n.TEXTURE_2D,Mt,Ct,gt,F,0,At,Tt,null),gt>>=1,F>>=1}}else if(Dt.length>0&&Rt){Yt&&le&&e.texStorage2D(n.TEXTURE_2D,Jt,Ct,Dt[0].width,Dt[0].height);for(let gt=0,F=Dt.length;gt<F;gt++)xt=Dt[gt],Yt?e.texSubImage2D(n.TEXTURE_2D,gt,0,0,At,Tt,xt):e.texImage2D(n.TEXTURE_2D,gt,Ct,At,Tt,xt);y.generateMipmaps=!1}else Yt?(le&&e.texStorage2D(n.TEXTURE_2D,Jt,Ct,$.width,$.height),e.texSubImage2D(n.TEXTURE_2D,0,0,0,At,Tt,$)):e.texImage2D(n.TEXTURE_2D,0,Ct,At,Tt,$);E(y,Rt)&&b(q),lt.__version=X.version,y.onUpdate&&y.onUpdate(y)}S.__version=y.version}function Lt(S,y,U){if(y.image.length!==6)return;const q=ht(S,y),H=y.source;e.bindTexture(n.TEXTURE_CUBE_MAP,S.__webglTexture,n.TEXTURE0+U);const X=i.get(H);if(H.version!==X.__version||q===!0){e.activeTexture(n.TEXTURE0+U);const lt=ce.getPrimaries(ce.workingColorSpace),at=y.colorSpace===mn?null:ce.getPrimaries(y.colorSpace),dt=y.colorSpace===mn||lt===at?n.NONE:n.BROWSER_DEFAULT_WEBGL;n.pixelStorei(n.UNPACK_FLIP_Y_WEBGL,y.flipY),n.pixelStorei(n.UNPACK_PREMULTIPLY_ALPHA_WEBGL,y.premultiplyAlpha),n.pixelStorei(n.UNPACK_ALIGNMENT,y.unpackAlignment),n.pixelStorei(n.UNPACK_COLORSPACE_CONVERSION_WEBGL,dt);const ct=y.isCompressedTexture||y.image[0].isCompressedTexture,_t=y.image[0]&&y.image[0].isDataTexture,$=[];for(let gt=0;gt<6;gt++)!ct&&!_t?$[gt]=g(y.image[gt],!1,!0,s.maxCubemapSize):$[gt]=_t?y.image[gt].image:y.image[gt],$[gt]=j(y,$[gt]);const Rt=$[0],At=_(Rt)||o,Tt=r.convert(y.format,y.colorSpace),Ct=r.convert(y.type),xt=x(y.internalFormat,Tt,Ct,y.colorSpace),Dt=o&&y.isVideoTexture!==!0,Yt=X.__version===void 0||q===!0;let le=C(y,Rt,At);it(n.TEXTURE_CUBE_MAP,y,At);let Jt;if(ct){Dt&&Yt&&e.texStorage2D(n.TEXTURE_CUBE_MAP,le,xt,Rt.width,Rt.height);for(let gt=0;gt<6;gt++){Jt=$[gt].mipmaps;for(let F=0;F<Jt.length;F++){const Mt=Jt[F];y.format!==En?Tt!==null?Dt?e.compressedTexSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F,0,0,Mt.width,Mt.height,Tt,Mt.data):e.compressedTexImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F,xt,Mt.width,Mt.height,0,Mt.data):console.warn("THREE.WebGLRenderer: Attempt to load unsupported compressed texture format in .setTextureCube()"):Dt?e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F,0,0,Mt.width,Mt.height,Tt,Ct,Mt.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F,xt,Mt.width,Mt.height,0,Tt,Ct,Mt.data)}}}else{Jt=y.mipmaps,Dt&&Yt&&(Jt.length>0&&le++,e.texStorage2D(n.TEXTURE_CUBE_MAP,le,xt,$[0].width,$[0].height));for(let gt=0;gt<6;gt++)if(_t){Dt?e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,0,0,0,$[gt].width,$[gt].height,Tt,Ct,$[gt].data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,0,xt,$[gt].width,$[gt].height,0,Tt,Ct,$[gt].data);for(let F=0;F<Jt.length;F++){const St=Jt[F].image[gt].image;Dt?e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F+1,0,0,St.width,St.height,Tt,Ct,St.data):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F+1,xt,St.width,St.height,0,Tt,Ct,St.data)}}else{Dt?e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,0,0,0,Tt,Ct,$[gt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,0,xt,Tt,Ct,$[gt]);for(let F=0;F<Jt.length;F++){const Mt=Jt[F];Dt?e.texSubImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F+1,0,0,Tt,Ct,Mt.image[gt]):e.texImage2D(n.TEXTURE_CUBE_MAP_POSITIVE_X+gt,F+1,xt,Tt,Ct,Mt.image[gt])}}}E(y,At)&&b(n.TEXTURE_CUBE_MAP),X.__version=H.version,y.onUpdate&&y.onUpdate(y)}S.__version=y.version}function Pt(S,y,U,q,H,X){const lt=r.convert(U.format,U.colorSpace),at=r.convert(U.type),dt=x(U.internalFormat,lt,at,U.colorSpace);if(!i.get(y).__hasExternalTextures){const _t=Math.max(1,y.width>>X),$=Math.max(1,y.height>>X);H===n.TEXTURE_3D||H===n.TEXTURE_2D_ARRAY?e.texImage3D(H,X,dt,_t,$,y.depth,0,lt,at,null):e.texImage2D(H,X,dt,_t,$,0,lt,at,null)}e.bindFramebuffer(n.FRAMEBUFFER,S),N(y)?l.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,q,H,i.get(U).__webglTexture,0,Y(y)):(H===n.TEXTURE_2D||H>=n.TEXTURE_CUBE_MAP_POSITIVE_X&&H<=n.TEXTURE_CUBE_MAP_NEGATIVE_Z)&&n.framebufferTexture2D(n.FRAMEBUFFER,q,H,i.get(U).__webglTexture,X),e.bindFramebuffer(n.FRAMEBUFFER,null)}function Ht(S,y,U){if(n.bindRenderbuffer(n.RENDERBUFFER,S),y.depthBuffer&&!y.stencilBuffer){let q=o===!0?n.DEPTH_COMPONENT24:n.DEPTH_COMPONENT16;if(U||N(y)){const H=y.depthTexture;H&&H.isDepthTexture&&(H.type===pi?q=n.DEPTH_COMPONENT32F:H.type===fi&&(q=n.DEPTH_COMPONENT24));const X=Y(y);N(y)?l.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,X,q,y.width,y.height):n.renderbufferStorageMultisample(n.RENDERBUFFER,X,q,y.width,y.height)}else n.renderbufferStorage(n.RENDERBUFFER,q,y.width,y.height);n.framebufferRenderbuffer(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.RENDERBUFFER,S)}else if(y.depthBuffer&&y.stencilBuffer){const q=Y(y);U&&N(y)===!1?n.renderbufferStorageMultisample(n.RENDERBUFFER,q,n.DEPTH24_STENCIL8,y.width,y.height):N(y)?l.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,q,n.DEPTH24_STENCIL8,y.width,y.height):n.renderbufferStorage(n.RENDERBUFFER,n.DEPTH_STENCIL,y.width,y.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.RENDERBUFFER,S)}else{const q=y.isWebGLMultipleRenderTargets===!0?y.texture:[y.texture];for(let H=0;H<q.length;H++){const X=q[H],lt=r.convert(X.format,X.colorSpace),at=r.convert(X.type),dt=x(X.internalFormat,lt,at,X.colorSpace),ct=Y(y);U&&N(y)===!1?n.renderbufferStorageMultisample(n.RENDERBUFFER,ct,dt,y.width,y.height):N(y)?l.renderbufferStorageMultisampleEXT(n.RENDERBUFFER,ct,dt,y.width,y.height):n.renderbufferStorage(n.RENDERBUFFER,dt,y.width,y.height)}}n.bindRenderbuffer(n.RENDERBUFFER,null)}function Gt(S,y){if(y&&y.isWebGLCubeRenderTarget)throw new Error("Depth Texture with cube render targets is not supported");if(e.bindFramebuffer(n.FRAMEBUFFER,S),!(y.depthTexture&&y.depthTexture.isDepthTexture))throw new Error("renderTarget.depthTexture must be an instance of THREE.DepthTexture");(!i.get(y.depthTexture).__webglTexture||y.depthTexture.image.width!==y.width||y.depthTexture.image.height!==y.height)&&(y.depthTexture.image.width=y.width,y.depthTexture.image.height=y.height,y.depthTexture.needsUpdate=!0),tt(y.depthTexture,0);const q=i.get(y.depthTexture).__webglTexture,H=Y(y);if(y.depthTexture.format===Bi)N(y)?l.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,q,0,H):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_ATTACHMENT,n.TEXTURE_2D,q,0);else if(y.depthTexture.format===Ms)N(y)?l.framebufferTexture2DMultisampleEXT(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,q,0,H):n.framebufferTexture2D(n.FRAMEBUFFER,n.DEPTH_STENCIL_ATTACHMENT,n.TEXTURE_2D,q,0);else throw new Error("Unknown depthTexture format")}function Nt(S){const y=i.get(S),U=S.isWebGLCubeRenderTarget===!0;if(S.depthTexture&&!y.__autoAllocateDepthBuffer){if(U)throw new Error("target.depthTexture not supported in Cube render targets");Gt(y.__webglFramebuffer,S)}else if(U){y.__webglDepthbuffer=[];for(let q=0;q<6;q++)e.bindFramebuffer(n.FRAMEBUFFER,y.__webglFramebuffer[q]),y.__webglDepthbuffer[q]=n.createRenderbuffer(),Ht(y.__webglDepthbuffer[q],S,!1)}else e.bindFramebuffer(n.FRAMEBUFFER,y.__webglFramebuffer),y.__webglDepthbuffer=n.createRenderbuffer(),Ht(y.__webglDepthbuffer,S,!1);e.bindFramebuffer(n.FRAMEBUFFER,null)}function Zt(S,y,U){const q=i.get(S);y!==void 0&&Pt(q.__webglFramebuffer,S,S.texture,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,0),U!==void 0&&Nt(S)}function M(S){const y=S.texture,U=i.get(S),q=i.get(y);S.addEventListener("dispose",G),S.isWebGLMultipleRenderTargets!==!0&&(q.__webglTexture===void 0&&(q.__webglTexture=n.createTexture()),q.__version=y.version,a.memory.textures++);const H=S.isWebGLCubeRenderTarget===!0,X=S.isWebGLMultipleRenderTargets===!0,lt=_(S)||o;if(H){U.__webglFramebuffer=[];for(let at=0;at<6;at++)if(o&&y.mipmaps&&y.mipmaps.length>0){U.__webglFramebuffer[at]=[];for(let dt=0;dt<y.mipmaps.length;dt++)U.__webglFramebuffer[at][dt]=n.createFramebuffer()}else U.__webglFramebuffer[at]=n.createFramebuffer()}else{if(o&&y.mipmaps&&y.mipmaps.length>0){U.__webglFramebuffer=[];for(let at=0;at<y.mipmaps.length;at++)U.__webglFramebuffer[at]=n.createFramebuffer()}else U.__webglFramebuffer=n.createFramebuffer();if(X)if(s.drawBuffers){const at=S.texture;for(let dt=0,ct=at.length;dt<ct;dt++){const _t=i.get(at[dt]);_t.__webglTexture===void 0&&(_t.__webglTexture=n.createTexture(),a.memory.textures++)}}else console.warn("THREE.WebGLRenderer: WebGLMultipleRenderTargets can only be used with WebGL2 or WEBGL_draw_buffers extension.");if(o&&S.samples>0&&N(S)===!1){const at=X?y:[y];U.__webglMultisampledFramebuffer=n.createFramebuffer(),U.__webglColorRenderbuffer=[],e.bindFramebuffer(n.FRAMEBUFFER,U.__webglMultisampledFramebuffer);for(let dt=0;dt<at.length;dt++){const ct=at[dt];U.__webglColorRenderbuffer[dt]=n.createRenderbuffer(),n.bindRenderbuffer(n.RENDERBUFFER,U.__webglColorRenderbuffer[dt]);const _t=r.convert(ct.format,ct.colorSpace),$=r.convert(ct.type),Rt=x(ct.internalFormat,_t,$,ct.colorSpace,S.isXRRenderTarget===!0),At=Y(S);n.renderbufferStorageMultisample(n.RENDERBUFFER,At,Rt,S.width,S.height),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+dt,n.RENDERBUFFER,U.__webglColorRenderbuffer[dt])}n.bindRenderbuffer(n.RENDERBUFFER,null),S.depthBuffer&&(U.__webglDepthRenderbuffer=n.createRenderbuffer(),Ht(U.__webglDepthRenderbuffer,S,!0)),e.bindFramebuffer(n.FRAMEBUFFER,null)}}if(H){e.bindTexture(n.TEXTURE_CUBE_MAP,q.__webglTexture),it(n.TEXTURE_CUBE_MAP,y,lt);for(let at=0;at<6;at++)if(o&&y.mipmaps&&y.mipmaps.length>0)for(let dt=0;dt<y.mipmaps.length;dt++)Pt(U.__webglFramebuffer[at][dt],S,y,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+at,dt);else Pt(U.__webglFramebuffer[at],S,y,n.COLOR_ATTACHMENT0,n.TEXTURE_CUBE_MAP_POSITIVE_X+at,0);E(y,lt)&&b(n.TEXTURE_CUBE_MAP),e.unbindTexture()}else if(X){const at=S.texture;for(let dt=0,ct=at.length;dt<ct;dt++){const _t=at[dt],$=i.get(_t);e.bindTexture(n.TEXTURE_2D,$.__webglTexture),it(n.TEXTURE_2D,_t,lt),Pt(U.__webglFramebuffer,S,_t,n.COLOR_ATTACHMENT0+dt,n.TEXTURE_2D,0),E(_t,lt)&&b(n.TEXTURE_2D)}e.unbindTexture()}else{let at=n.TEXTURE_2D;if((S.isWebGL3DRenderTarget||S.isWebGLArrayRenderTarget)&&(o?at=S.isWebGL3DRenderTarget?n.TEXTURE_3D:n.TEXTURE_2D_ARRAY:console.error("THREE.WebGLTextures: THREE.Data3DTexture and THREE.DataArrayTexture only supported with WebGL2.")),e.bindTexture(at,q.__webglTexture),it(at,y,lt),o&&y.mipmaps&&y.mipmaps.length>0)for(let dt=0;dt<y.mipmaps.length;dt++)Pt(U.__webglFramebuffer[dt],S,y,n.COLOR_ATTACHMENT0,at,dt);else Pt(U.__webglFramebuffer,S,y,n.COLOR_ATTACHMENT0,at,0);E(y,lt)&&b(at),e.unbindTexture()}S.depthBuffer&&Nt(S)}function k(S){const y=_(S)||o,U=S.isWebGLMultipleRenderTargets===!0?S.texture:[S.texture];for(let q=0,H=U.length;q<H;q++){const X=U[q];if(E(X,y)){const lt=S.isWebGLCubeRenderTarget?n.TEXTURE_CUBE_MAP:n.TEXTURE_2D,at=i.get(X).__webglTexture;e.bindTexture(lt,at),b(lt),e.unbindTexture()}}}function B(S){if(o&&S.samples>0&&N(S)===!1){const y=S.isWebGLMultipleRenderTargets?S.texture:[S.texture],U=S.width,q=S.height;let H=n.COLOR_BUFFER_BIT;const X=[],lt=S.stencilBuffer?n.DEPTH_STENCIL_ATTACHMENT:n.DEPTH_ATTACHMENT,at=i.get(S),dt=S.isWebGLMultipleRenderTargets===!0;if(dt)for(let ct=0;ct<y.length;ct++)e.bindFramebuffer(n.FRAMEBUFFER,at.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+ct,n.RENDERBUFFER,null),e.bindFramebuffer(n.FRAMEBUFFER,at.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+ct,n.TEXTURE_2D,null,0);e.bindFramebuffer(n.READ_FRAMEBUFFER,at.__webglMultisampledFramebuffer),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,at.__webglFramebuffer);for(let ct=0;ct<y.length;ct++){X.push(n.COLOR_ATTACHMENT0+ct),S.depthBuffer&&X.push(lt);const _t=at.__ignoreDepthValues!==void 0?at.__ignoreDepthValues:!1;if(_t===!1&&(S.depthBuffer&&(H|=n.DEPTH_BUFFER_BIT),S.stencilBuffer&&(H|=n.STENCIL_BUFFER_BIT)),dt&&n.framebufferRenderbuffer(n.READ_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.RENDERBUFFER,at.__webglColorRenderbuffer[ct]),_t===!0&&(n.invalidateFramebuffer(n.READ_FRAMEBUFFER,[lt]),n.invalidateFramebuffer(n.DRAW_FRAMEBUFFER,[lt])),dt){const $=i.get(y[ct]).__webglTexture;n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0,n.TEXTURE_2D,$,0)}n.blitFramebuffer(0,0,U,q,0,0,U,q,H,n.NEAREST),c&&n.invalidateFramebuffer(n.READ_FRAMEBUFFER,X)}if(e.bindFramebuffer(n.READ_FRAMEBUFFER,null),e.bindFramebuffer(n.DRAW_FRAMEBUFFER,null),dt)for(let ct=0;ct<y.length;ct++){e.bindFramebuffer(n.FRAMEBUFFER,at.__webglMultisampledFramebuffer),n.framebufferRenderbuffer(n.FRAMEBUFFER,n.COLOR_ATTACHMENT0+ct,n.RENDERBUFFER,at.__webglColorRenderbuffer[ct]);const _t=i.get(y[ct]).__webglTexture;e.bindFramebuffer(n.FRAMEBUFFER,at.__webglFramebuffer),n.framebufferTexture2D(n.DRAW_FRAMEBUFFER,n.COLOR_ATTACHMENT0+ct,n.TEXTURE_2D,_t,0)}e.bindFramebuffer(n.DRAW_FRAMEBUFFER,at.__webglMultisampledFramebuffer)}}function Y(S){return Math.min(s.maxSamples,S.samples)}function N(S){const y=i.get(S);return o&&S.samples>0&&t.has("WEBGL_multisampled_render_to_texture")===!0&&y.__useRenderToTexture!==!1}function V(S){const y=a.render.frame;d.get(S)!==y&&(d.set(S,y),S.update())}function j(S,y){const U=S.colorSpace,q=S.format,H=S.type;return S.isCompressedTexture===!0||S.isVideoTexture===!0||S.format===ka||U!==ri&&U!==mn&&(ce.getTransfer(U)===ge?o===!1?t.has("EXT_sRGB")===!0&&q===En?(S.format=ka,S.minFilter=pn,S.generateMipmaps=!1):y=Qd.sRGBToLinear(y):(q!==En||H!==gi)&&console.warn("THREE.WebGLTextures: sRGB encoded textures have to use RGBAFormat and UnsignedByteType."):console.error("THREE.WebGLTextures: Unsupported texture color space:",U)),y}this.allocateTextureUnit=O,this.resetTextureUnits=mt,this.setTexture2D=tt,this.setTexture2DArray=st,this.setTexture3D=et,this.setTextureCube=ot,this.rebindTextures=Zt,this.setupRenderTarget=M,this.updateRenderTargetMipmap=k,this.updateMultisampleRenderTarget=B,this.setupDepthRenderbuffer=Nt,this.setupFrameBufferTexture=Pt,this.useMultisampledRTT=N}function cy(n,t,e){const i=e.isWebGL2;function s(r,a=mn){let o;const l=ce.getTransfer(a);if(r===gi)return n.UNSIGNED_BYTE;if(r===Wd)return n.UNSIGNED_SHORT_4_4_4_4;if(r===$d)return n.UNSIGNED_SHORT_5_5_5_1;if(r===Gm)return n.BYTE;if(r===Wm)return n.SHORT;if(r===rl)return n.UNSIGNED_SHORT;if(r===Gd)return n.INT;if(r===fi)return n.UNSIGNED_INT;if(r===pi)return n.FLOAT;if(r===er)return i?n.HALF_FLOAT:(o=t.get("OES_texture_half_float"),o!==null?o.HALF_FLOAT_OES:null);if(r===$m)return n.ALPHA;if(r===En)return n.RGBA;if(r===Xm)return n.LUMINANCE;if(r===qm)return n.LUMINANCE_ALPHA;if(r===Bi)return n.DEPTH_COMPONENT;if(r===Ms)return n.DEPTH_STENCIL;if(r===ka)return o=t.get("EXT_sRGB"),o!==null?o.SRGB_ALPHA_EXT:null;if(r===jm)return n.RED;if(r===Xd)return n.RED_INTEGER;if(r===Ym)return n.RG;if(r===qd)return n.RG_INTEGER;if(r===jd)return n.RGBA_INTEGER;if(r===Bo||r===zo||r===Ho||r===Vo)if(l===ge)if(o=t.get("WEBGL_compressed_texture_s3tc_srgb"),o!==null){if(r===Bo)return o.COMPRESSED_SRGB_S3TC_DXT1_EXT;if(r===zo)return o.COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT;if(r===Ho)return o.COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT;if(r===Vo)return o.COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT}else return null;else if(o=t.get("WEBGL_compressed_texture_s3tc"),o!==null){if(r===Bo)return o.COMPRESSED_RGB_S3TC_DXT1_EXT;if(r===zo)return o.COMPRESSED_RGBA_S3TC_DXT1_EXT;if(r===Ho)return o.COMPRESSED_RGBA_S3TC_DXT3_EXT;if(r===Vo)return o.COMPRESSED_RGBA_S3TC_DXT5_EXT}else return null;if(r===cc||r===uc||r===dc||r===hc)if(o=t.get("WEBGL_compressed_texture_pvrtc"),o!==null){if(r===cc)return o.COMPRESSED_RGB_PVRTC_4BPPV1_IMG;if(r===uc)return o.COMPRESSED_RGB_PVRTC_2BPPV1_IMG;if(r===dc)return o.COMPRESSED_RGBA_PVRTC_4BPPV1_IMG;if(r===hc)return o.COMPRESSED_RGBA_PVRTC_2BPPV1_IMG}else return null;if(r===Yd)return o=t.get("WEBGL_compressed_texture_etc1"),o!==null?o.COMPRESSED_RGB_ETC1_WEBGL:null;if(r===fc||r===pc)if(o=t.get("WEBGL_compressed_texture_etc"),o!==null){if(r===fc)return l===ge?o.COMPRESSED_SRGB8_ETC2:o.COMPRESSED_RGB8_ETC2;if(r===pc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ETC2_EAC:o.COMPRESSED_RGBA8_ETC2_EAC}else return null;if(r===mc||r===_c||r===gc||r===vc||r===xc||r===yc||r===bc||r===Mc||r===Sc||r===Ec||r===Tc||r===wc||r===Ac||r===Cc)if(o=t.get("WEBGL_compressed_texture_astc"),o!==null){if(r===mc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR:o.COMPRESSED_RGBA_ASTC_4x4_KHR;if(r===_c)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR:o.COMPRESSED_RGBA_ASTC_5x4_KHR;if(r===gc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR:o.COMPRESSED_RGBA_ASTC_5x5_KHR;if(r===vc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR:o.COMPRESSED_RGBA_ASTC_6x5_KHR;if(r===xc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR:o.COMPRESSED_RGBA_ASTC_6x6_KHR;if(r===yc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR:o.COMPRESSED_RGBA_ASTC_8x5_KHR;if(r===bc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR:o.COMPRESSED_RGBA_ASTC_8x6_KHR;if(r===Mc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR:o.COMPRESSED_RGBA_ASTC_8x8_KHR;if(r===Sc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR:o.COMPRESSED_RGBA_ASTC_10x5_KHR;if(r===Ec)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR:o.COMPRESSED_RGBA_ASTC_10x6_KHR;if(r===Tc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR:o.COMPRESSED_RGBA_ASTC_10x8_KHR;if(r===wc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR:o.COMPRESSED_RGBA_ASTC_10x10_KHR;if(r===Ac)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR:o.COMPRESSED_RGBA_ASTC_12x10_KHR;if(r===Cc)return l===ge?o.COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR:o.COMPRESSED_RGBA_ASTC_12x12_KHR}else return null;if(r===Go||r===Rc||r===Lc)if(o=t.get("EXT_texture_compression_bptc"),o!==null){if(r===Go)return l===ge?o.COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT:o.COMPRESSED_RGBA_BPTC_UNORM_EXT;if(r===Rc)return o.COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT;if(r===Lc)return o.COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT}else return null;if(r===Km||r===Pc||r===Dc||r===Uc)if(o=t.get("EXT_texture_compression_rgtc"),o!==null){if(r===Go)return o.COMPRESSED_RED_RGTC1_EXT;if(r===Pc)return o.COMPRESSED_SIGNED_RED_RGTC1_EXT;if(r===Dc)return o.COMPRESSED_RED_GREEN_RGTC2_EXT;if(r===Uc)return o.COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT}else return null;return r===Oi?i?n.UNSIGNED_INT_24_8:(o=t.get("WEBGL_depth_texture"),o!==null?o.UNSIGNED_INT_24_8_WEBGL:null):n[r]!==void 0?n[r]:null}return{convert:s}}class uy extends ln{constructor(t=[]){super(),this.isArrayCamera=!0,this.cameras=t}}class Bs extends We{constructor(){super(),this.isGroup=!0,this.type="Group"}}const dy={type:"move"};class pa{constructor(){this._targetRay=null,this._grip=null,this._hand=null}getHandSpace(){return this._hand===null&&(this._hand=new Bs,this._hand.matrixAutoUpdate=!1,this._hand.visible=!1,this._hand.joints={},this._hand.inputState={pinching:!1}),this._hand}getTargetRaySpace(){return this._targetRay===null&&(this._targetRay=new Bs,this._targetRay.matrixAutoUpdate=!1,this._targetRay.visible=!1,this._targetRay.hasLinearVelocity=!1,this._targetRay.linearVelocity=new z,this._targetRay.hasAngularVelocity=!1,this._targetRay.angularVelocity=new z),this._targetRay}getGripSpace(){return this._grip===null&&(this._grip=new Bs,this._grip.matrixAutoUpdate=!1,this._grip.visible=!1,this._grip.hasLinearVelocity=!1,this._grip.linearVelocity=new z,this._grip.hasAngularVelocity=!1,this._grip.angularVelocity=new z),this._grip}dispatchEvent(t){return this._targetRay!==null&&this._targetRay.dispatchEvent(t),this._grip!==null&&this._grip.dispatchEvent(t),this._hand!==null&&this._hand.dispatchEvent(t),this}connect(t){if(t&&t.hand){const e=this._hand;if(e)for(const i of t.hand.values())this._getHandJoint(e,i)}return this.dispatchEvent({type:"connected",data:t}),this}disconnect(t){return this.dispatchEvent({type:"disconnected",data:t}),this._targetRay!==null&&(this._targetRay.visible=!1),this._grip!==null&&(this._grip.visible=!1),this._hand!==null&&(this._hand.visible=!1),this}update(t,e,i){let s=null,r=null,a=null;const o=this._targetRay,l=this._grip,c=this._hand;if(t&&e.session.visibilityState!=="visible-blurred"){if(c&&t.hand){a=!0;for(const g of t.hand.values()){const _=e.getJointPose(g,i),m=this._getHandJoint(c,g);_!==null&&(m.matrix.fromArray(_.transform.matrix),m.matrix.decompose(m.position,m.rotation,m.scale),m.matrixWorldNeedsUpdate=!0,m.jointRadius=_.radius),m.visible=_!==null}const d=c.joints["index-finger-tip"],h=c.joints["thumb-tip"],p=d.position.distanceTo(h.position),f=.02,v=.005;c.inputState.pinching&&p>f+v?(c.inputState.pinching=!1,this.dispatchEvent({type:"pinchend",handedness:t.handedness,target:this})):!c.inputState.pinching&&p<=f-v&&(c.inputState.pinching=!0,this.dispatchEvent({type:"pinchstart",handedness:t.handedness,target:this}))}else l!==null&&t.gripSpace&&(r=e.getPose(t.gripSpace,i),r!==null&&(l.matrix.fromArray(r.transform.matrix),l.matrix.decompose(l.position,l.rotation,l.scale),l.matrixWorldNeedsUpdate=!0,r.linearVelocity?(l.hasLinearVelocity=!0,l.linearVelocity.copy(r.linearVelocity)):l.hasLinearVelocity=!1,r.angularVelocity?(l.hasAngularVelocity=!0,l.angularVelocity.copy(r.angularVelocity)):l.hasAngularVelocity=!1));o!==null&&(s=e.getPose(t.targetRaySpace,i),s===null&&r!==null&&(s=r),s!==null&&(o.matrix.fromArray(s.transform.matrix),o.matrix.decompose(o.position,o.rotation,o.scale),o.matrixWorldNeedsUpdate=!0,s.linearVelocity?(o.hasLinearVelocity=!0,o.linearVelocity.copy(s.linearVelocity)):o.hasLinearVelocity=!1,s.angularVelocity?(o.hasAngularVelocity=!0,o.angularVelocity.copy(s.angularVelocity)):o.hasAngularVelocity=!1,this.dispatchEvent(dy)))}return o!==null&&(o.visible=s!==null),l!==null&&(l.visible=r!==null),c!==null&&(c.visible=a!==null),this}_getHandJoint(t,e){if(t.joints[e.jointName]===void 0){const i=new Bs;i.matrixAutoUpdate=!1,i.visible=!1,t.joints[e.jointName]=i,t.add(i)}return t.joints[e.jointName]}}class hy extends Ts{constructor(t,e){super();const i=this;let s=null,r=1,a=null,o="local-floor",l=1,c=null,d=null,h=null,p=null,f=null,v=null;const g=e.getContextAttributes();let _=null,m=null;const E=[],b=[],x=new re;let C=null;const A=new ln;A.layers.enable(1),A.viewport=new xe;const D=new ln;D.layers.enable(2),D.viewport=new xe;const G=[A,D],T=new uy;T.layers.enable(1),T.layers.enable(2);let R=null,J=null;this.cameraAutoUpdate=!0,this.enabled=!1,this.isPresenting=!1,this.getController=function(it){let ht=E[it];return ht===void 0&&(ht=new pa,E[it]=ht),ht.getTargetRaySpace()},this.getControllerGrip=function(it){let ht=E[it];return ht===void 0&&(ht=new pa,E[it]=ht),ht.getGripSpace()},this.getHand=function(it){let ht=E[it];return ht===void 0&&(ht=new pa,E[it]=ht),ht.getHandSpace()};function rt(it){const ht=b.indexOf(it.inputSource);if(ht===-1)return;const bt=E[ht];bt!==void 0&&(bt.update(it.inputSource,it.frame,c||a),bt.dispatchEvent({type:it.type,data:it.inputSource}))}function mt(){s.removeEventListener("select",rt),s.removeEventListener("selectstart",rt),s.removeEventListener("selectend",rt),s.removeEventListener("squeeze",rt),s.removeEventListener("squeezestart",rt),s.removeEventListener("squeezeend",rt),s.removeEventListener("end",mt),s.removeEventListener("inputsourceschange",O);for(let it=0;it<E.length;it++){const ht=b[it];ht!==null&&(b[it]=null,E[it].disconnect(ht))}R=null,J=null,t.setRenderTarget(_),f=null,p=null,h=null,s=null,m=null,ft.stop(),i.isPresenting=!1,t.setPixelRatio(C),t.setSize(x.width,x.height,!1),i.dispatchEvent({type:"sessionend"})}this.setFramebufferScaleFactor=function(it){r=it,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change framebuffer scale while presenting.")},this.setReferenceSpaceType=function(it){o=it,i.isPresenting===!0&&console.warn("THREE.WebXRManager: Cannot change reference space type while presenting.")},this.getReferenceSpace=function(){return c||a},this.setReferenceSpace=function(it){c=it},this.getBaseLayer=function(){return p!==null?p:f},this.getBinding=function(){return h},this.getFrame=function(){return v},this.getSession=function(){return s},this.setSession=async function(it){if(s=it,s!==null){if(_=t.getRenderTarget(),s.addEventListener("select",rt),s.addEventListener("selectstart",rt),s.addEventListener("selectend",rt),s.addEventListener("squeeze",rt),s.addEventListener("squeezestart",rt),s.addEventListener("squeezeend",rt),s.addEventListener("end",mt),s.addEventListener("inputsourceschange",O),g.xrCompatible!==!0&&await e.makeXRCompatible(),C=t.getPixelRatio(),t.getSize(x),s.renderState.layers===void 0||t.capabilities.isWebGL2===!1){const ht={antialias:s.renderState.layers===void 0?g.antialias:!0,alpha:!0,depth:g.depth,stencil:g.stencil,framebufferScaleFactor:r};f=new XRWebGLLayer(s,e,ht),s.updateRenderState({baseLayer:f}),t.setPixelRatio(1),t.setSize(f.framebufferWidth,f.framebufferHeight,!1),m=new Hi(f.framebufferWidth,f.framebufferHeight,{format:En,type:gi,colorSpace:t.outputColorSpace,stencilBuffer:g.stencil})}else{let ht=null,bt=null,Lt=null;g.depth&&(Lt=g.stencil?e.DEPTH24_STENCIL8:e.DEPTH_COMPONENT24,ht=g.stencil?Ms:Bi,bt=g.stencil?Oi:fi);const Pt={colorFormat:e.RGBA8,depthFormat:Lt,scaleFactor:r};h=new XRWebGLBinding(s,e),p=h.createProjectionLayer(Pt),s.updateRenderState({layers:[p]}),t.setPixelRatio(1),t.setSize(p.textureWidth,p.textureHeight,!1),m=new Hi(p.textureWidth,p.textureHeight,{format:En,type:gi,depthTexture:new uh(p.textureWidth,p.textureHeight,bt,void 0,void 0,void 0,void 0,void 0,void 0,ht),stencilBuffer:g.stencil,colorSpace:t.outputColorSpace,samples:g.antialias?4:0});const Ht=t.properties.get(m);Ht.__ignoreDepthValues=p.ignoreDepthValues}m.isXRRenderTarget=!0,this.setFoveation(l),c=null,a=await s.requestReferenceSpace(o),ft.setContext(s),ft.start(),i.isPresenting=!0,i.dispatchEvent({type:"sessionstart"})}},this.getEnvironmentBlendMode=function(){if(s!==null)return s.environmentBlendMode};function O(it){for(let ht=0;ht<it.removed.length;ht++){const bt=it.removed[ht],Lt=b.indexOf(bt);Lt>=0&&(b[Lt]=null,E[Lt].disconnect(bt))}for(let ht=0;ht<it.added.length;ht++){const bt=it.added[ht];let Lt=b.indexOf(bt);if(Lt===-1){for(let Ht=0;Ht<E.length;Ht++)if(Ht>=b.length){b.push(bt),Lt=Ht;break}else if(b[Ht]===null){b[Ht]=bt,Lt=Ht;break}if(Lt===-1)break}const Pt=E[Lt];Pt&&Pt.connect(bt)}}const Z=new z,tt=new z;function st(it,ht,bt){Z.setFromMatrixPosition(ht.matrixWorld),tt.setFromMatrixPosition(bt.matrixWorld);const Lt=Z.distanceTo(tt),Pt=ht.projectionMatrix.elements,Ht=bt.projectionMatrix.elements,Gt=Pt[14]/(Pt[10]-1),Nt=Pt[14]/(Pt[10]+1),Zt=(Pt[9]+1)/Pt[5],M=(Pt[9]-1)/Pt[5],k=(Pt[8]-1)/Pt[0],B=(Ht[8]+1)/Ht[0],Y=Gt*k,N=Gt*B,V=Lt/(-k+B),j=V*-k;ht.matrixWorld.decompose(it.position,it.quaternion,it.scale),it.translateX(j),it.translateZ(V),it.matrixWorld.compose(it.position,it.quaternion,it.scale),it.matrixWorldInverse.copy(it.matrixWorld).invert();const S=Gt+V,y=Nt+V,U=Y-j,q=N+(Lt-j),H=Zt*Nt/y*S,X=M*Nt/y*S;it.projectionMatrix.makePerspective(U,q,H,X,S,y),it.projectionMatrixInverse.copy(it.projectionMatrix).invert()}function et(it,ht){ht===null?it.matrixWorld.copy(it.matrix):it.matrixWorld.multiplyMatrices(ht.matrixWorld,it.matrix),it.matrixWorldInverse.copy(it.matrixWorld).invert()}this.updateCamera=function(it){if(s===null)return;T.near=D.near=A.near=it.near,T.far=D.far=A.far=it.far,(R!==T.near||J!==T.far)&&(s.updateRenderState({depthNear:T.near,depthFar:T.far}),R=T.near,J=T.far);const ht=it.parent,bt=T.cameras;et(T,ht);for(let Lt=0;Lt<bt.length;Lt++)et(bt[Lt],ht);bt.length===2?st(T,A,D):T.projectionMatrix.copy(A.projectionMatrix),ot(it,T,ht)};function ot(it,ht,bt){bt===null?it.matrix.copy(ht.matrixWorld):(it.matrix.copy(bt.matrixWorld),it.matrix.invert(),it.matrix.multiply(ht.matrixWorld)),it.matrix.decompose(it.position,it.quaternion,it.scale),it.updateMatrixWorld(!0),it.projectionMatrix.copy(ht.projectionMatrix),it.projectionMatrixInverse.copy(ht.projectionMatrixInverse),it.isPerspectiveCamera&&(it.fov=Fa*2*Math.atan(1/it.projectionMatrix.elements[5]),it.zoom=1)}this.getCamera=function(){return T},this.getFoveation=function(){if(!(p===null&&f===null))return l},this.setFoveation=function(it){l=it,p!==null&&(p.fixedFoveation=it),f!==null&&f.fixedFoveation!==void 0&&(f.fixedFoveation=it)};let ut=null;function pt(it,ht){if(d=ht.getViewerPose(c||a),v=ht,d!==null){const bt=d.views;f!==null&&(t.setRenderTargetFramebuffer(m,f.framebuffer),t.setRenderTarget(m));let Lt=!1;bt.length!==T.cameras.length&&(T.cameras.length=0,Lt=!0);for(let Pt=0;Pt<bt.length;Pt++){const Ht=bt[Pt];let Gt=null;if(f!==null)Gt=f.getViewport(Ht);else{const Zt=h.getViewSubImage(p,Ht);Gt=Zt.viewport,Pt===0&&(t.setRenderTargetTextures(m,Zt.colorTexture,p.ignoreDepthValues?void 0:Zt.depthStencilTexture),t.setRenderTarget(m))}let Nt=G[Pt];Nt===void 0&&(Nt=new ln,Nt.layers.enable(Pt),Nt.viewport=new xe,G[Pt]=Nt),Nt.matrix.fromArray(Ht.transform.matrix),Nt.matrix.decompose(Nt.position,Nt.quaternion,Nt.scale),Nt.projectionMatrix.fromArray(Ht.projectionMatrix),Nt.projectionMatrixInverse.copy(Nt.projectionMatrix).invert(),Nt.viewport.set(Gt.x,Gt.y,Gt.width,Gt.height),Pt===0&&(T.matrix.copy(Nt.matrix),T.matrix.decompose(T.position,T.quaternion,T.scale)),Lt===!0&&T.cameras.push(Nt)}}for(let bt=0;bt<E.length;bt++){const Lt=b[bt],Pt=E[bt];Lt!==null&&Pt!==void 0&&Pt.update(Lt,ht,c||a)}ut&&ut(it,ht),ht.detectedPlanes&&i.dispatchEvent({type:"planesdetected",data:ht}),v=null}const ft=new ch;ft.setAnimationLoop(pt),this.setAnimationLoop=function(it){ut=it},this.dispose=function(){}}}function fy(n,t){function e(_,m){_.matrixAutoUpdate===!0&&_.updateMatrix(),m.value.copy(_.matrix)}function i(_,m){m.color.getRGB(_.fogColor.value,oh(n)),m.isFog?(_.fogNear.value=m.near,_.fogFar.value=m.far):m.isFogExp2&&(_.fogDensity.value=m.density)}function s(_,m,E,b,x){m.isMeshBasicMaterial||m.isMeshLambertMaterial?r(_,m):m.isMeshToonMaterial?(r(_,m),h(_,m)):m.isMeshPhongMaterial?(r(_,m),d(_,m)):m.isMeshStandardMaterial?(r(_,m),p(_,m),m.isMeshPhysicalMaterial&&f(_,m,x)):m.isMeshMatcapMaterial?(r(_,m),v(_,m)):m.isMeshDepthMaterial?r(_,m):m.isMeshDistanceMaterial?(r(_,m),g(_,m)):m.isMeshNormalMaterial?r(_,m):m.isLineBasicMaterial?(a(_,m),m.isLineDashedMaterial&&o(_,m)):m.isPointsMaterial?l(_,m,E,b):m.isSpriteMaterial?c(_,m):m.isShadowMaterial?(_.color.value.copy(m.color),_.opacity.value=m.opacity):m.isShaderMaterial&&(m.uniformsNeedUpdate=!1)}function r(_,m){_.opacity.value=m.opacity,m.color&&_.diffuse.value.copy(m.color),m.emissive&&_.emissive.value.copy(m.emissive).multiplyScalar(m.emissiveIntensity),m.map&&(_.map.value=m.map,e(m.map,_.mapTransform)),m.alphaMap&&(_.alphaMap.value=m.alphaMap,e(m.alphaMap,_.alphaMapTransform)),m.bumpMap&&(_.bumpMap.value=m.bumpMap,e(m.bumpMap,_.bumpMapTransform),_.bumpScale.value=m.bumpScale,m.side===tn&&(_.bumpScale.value*=-1)),m.normalMap&&(_.normalMap.value=m.normalMap,e(m.normalMap,_.normalMapTransform),_.normalScale.value.copy(m.normalScale),m.side===tn&&_.normalScale.value.negate()),m.displacementMap&&(_.displacementMap.value=m.displacementMap,e(m.displacementMap,_.displacementMapTransform),_.displacementScale.value=m.displacementScale,_.displacementBias.value=m.displacementBias),m.emissiveMap&&(_.emissiveMap.value=m.emissiveMap,e(m.emissiveMap,_.emissiveMapTransform)),m.specularMap&&(_.specularMap.value=m.specularMap,e(m.specularMap,_.specularMapTransform)),m.alphaTest>0&&(_.alphaTest.value=m.alphaTest);const E=t.get(m).envMap;if(E&&(_.envMap.value=E,_.flipEnvMap.value=E.isCubeTexture&&E.isRenderTargetTexture===!1?-1:1,_.reflectivity.value=m.reflectivity,_.ior.value=m.ior,_.refractionRatio.value=m.refractionRatio),m.lightMap){_.lightMap.value=m.lightMap;const b=n._useLegacyLights===!0?Math.PI:1;_.lightMapIntensity.value=m.lightMapIntensity*b,e(m.lightMap,_.lightMapTransform)}m.aoMap&&(_.aoMap.value=m.aoMap,_.aoMapIntensity.value=m.aoMapIntensity,e(m.aoMap,_.aoMapTransform))}function a(_,m){_.diffuse.value.copy(m.color),_.opacity.value=m.opacity,m.map&&(_.map.value=m.map,e(m.map,_.mapTransform))}function o(_,m){_.dashSize.value=m.dashSize,_.totalSize.value=m.dashSize+m.gapSize,_.scale.value=m.scale}function l(_,m,E,b){_.diffuse.value.copy(m.color),_.opacity.value=m.opacity,_.size.value=m.size*E,_.scale.value=b*.5,m.map&&(_.map.value=m.map,e(m.map,_.uvTransform)),m.alphaMap&&(_.alphaMap.value=m.alphaMap,e(m.alphaMap,_.alphaMapTransform)),m.alphaTest>0&&(_.alphaTest.value=m.alphaTest)}function c(_,m){_.diffuse.value.copy(m.color),_.opacity.value=m.opacity,_.rotation.value=m.rotation,m.map&&(_.map.value=m.map,e(m.map,_.mapTransform)),m.alphaMap&&(_.alphaMap.value=m.alphaMap,e(m.alphaMap,_.alphaMapTransform)),m.alphaTest>0&&(_.alphaTest.value=m.alphaTest)}function d(_,m){_.specular.value.copy(m.specular),_.shininess.value=Math.max(m.shininess,1e-4)}function h(_,m){m.gradientMap&&(_.gradientMap.value=m.gradientMap)}function p(_,m){_.metalness.value=m.metalness,m.metalnessMap&&(_.metalnessMap.value=m.metalnessMap,e(m.metalnessMap,_.metalnessMapTransform)),_.roughness.value=m.roughness,m.roughnessMap&&(_.roughnessMap.value=m.roughnessMap,e(m.roughnessMap,_.roughnessMapTransform)),t.get(m).envMap&&(_.envMapIntensity.value=m.envMapIntensity)}function f(_,m,E){_.ior.value=m.ior,m.sheen>0&&(_.sheenColor.value.copy(m.sheenColor).multiplyScalar(m.sheen),_.sheenRoughness.value=m.sheenRoughness,m.sheenColorMap&&(_.sheenColorMap.value=m.sheenColorMap,e(m.sheenColorMap,_.sheenColorMapTransform)),m.sheenRoughnessMap&&(_.sheenRoughnessMap.value=m.sheenRoughnessMap,e(m.sheenRoughnessMap,_.sheenRoughnessMapTransform))),m.clearcoat>0&&(_.clearcoat.value=m.clearcoat,_.clearcoatRoughness.value=m.clearcoatRoughness,m.clearcoatMap&&(_.clearcoatMap.value=m.clearcoatMap,e(m.clearcoatMap,_.clearcoatMapTransform)),m.clearcoatRoughnessMap&&(_.clearcoatRoughnessMap.value=m.clearcoatRoughnessMap,e(m.clearcoatRoughnessMap,_.clearcoatRoughnessMapTransform)),m.clearcoatNormalMap&&(_.clearcoatNormalMap.value=m.clearcoatNormalMap,e(m.clearcoatNormalMap,_.clearcoatNormalMapTransform),_.clearcoatNormalScale.value.copy(m.clearcoatNormalScale),m.side===tn&&_.clearcoatNormalScale.value.negate())),m.iridescence>0&&(_.iridescence.value=m.iridescence,_.iridescenceIOR.value=m.iridescenceIOR,_.iridescenceThicknessMinimum.value=m.iridescenceThicknessRange[0],_.iridescenceThicknessMaximum.value=m.iridescenceThicknessRange[1],m.iridescenceMap&&(_.iridescenceMap.value=m.iridescenceMap,e(m.iridescenceMap,_.iridescenceMapTransform)),m.iridescenceThicknessMap&&(_.iridescenceThicknessMap.value=m.iridescenceThicknessMap,e(m.iridescenceThicknessMap,_.iridescenceThicknessMapTransform))),m.transmission>0&&(_.transmission.value=m.transmission,_.transmissionSamplerMap.value=E.texture,_.transmissionSamplerSize.value.set(E.width,E.height),m.transmissionMap&&(_.transmissionMap.value=m.transmissionMap,e(m.transmissionMap,_.transmissionMapTransform)),_.thickness.value=m.thickness,m.thicknessMap&&(_.thicknessMap.value=m.thicknessMap,e(m.thicknessMap,_.thicknessMapTransform)),_.attenuationDistance.value=m.attenuationDistance,_.attenuationColor.value.copy(m.attenuationColor)),m.anisotropy>0&&(_.anisotropyVector.value.set(m.anisotropy*Math.cos(m.anisotropyRotation),m.anisotropy*Math.sin(m.anisotropyRotation)),m.anisotropyMap&&(_.anisotropyMap.value=m.anisotropyMap,e(m.anisotropyMap,_.anisotropyMapTransform))),_.specularIntensity.value=m.specularIntensity,_.specularColor.value.copy(m.specularColor),m.specularColorMap&&(_.specularColorMap.value=m.specularColorMap,e(m.specularColorMap,_.specularColorMapTransform)),m.specularIntensityMap&&(_.specularIntensityMap.value=m.specularIntensityMap,e(m.specularIntensityMap,_.specularIntensityMapTransform))}function v(_,m){m.matcap&&(_.matcap.value=m.matcap)}function g(_,m){const E=t.get(m).light;_.referencePosition.value.setFromMatrixPosition(E.matrixWorld),_.nearDistance.value=E.shadow.camera.near,_.farDistance.value=E.shadow.camera.far}return{refreshFogUniforms:i,refreshMaterialUniforms:s}}function py(n,t,e,i){let s={},r={},a=[];const o=e.isWebGL2?n.getParameter(n.MAX_UNIFORM_BUFFER_BINDINGS):0;function l(E,b){const x=b.program;i.uniformBlockBinding(E,x)}function c(E,b){let x=s[E.id];x===void 0&&(v(E),x=d(E),s[E.id]=x,E.addEventListener("dispose",_));const C=b.program;i.updateUBOMapping(E,C);const A=t.render.frame;r[E.id]!==A&&(p(E),r[E.id]=A)}function d(E){const b=h();E.__bindingPointIndex=b;const x=n.createBuffer(),C=E.__size,A=E.usage;return n.bindBuffer(n.UNIFORM_BUFFER,x),n.bufferData(n.UNIFORM_BUFFER,C,A),n.bindBuffer(n.UNIFORM_BUFFER,null),n.bindBufferBase(n.UNIFORM_BUFFER,b,x),x}function h(){for(let E=0;E<o;E++)if(a.indexOf(E)===-1)return a.push(E),E;return console.error("THREE.WebGLRenderer: Maximum number of simultaneously usable uniforms groups reached."),0}function p(E){const b=s[E.id],x=E.uniforms,C=E.__cache;n.bindBuffer(n.UNIFORM_BUFFER,b);for(let A=0,D=x.length;A<D;A++){const G=Array.isArray(x[A])?x[A]:[x[A]];for(let T=0,R=G.length;T<R;T++){const J=G[T];if(f(J,A,T,C)===!0){const rt=J.__offset,mt=Array.isArray(J.value)?J.value:[J.value];let O=0;for(let Z=0;Z<mt.length;Z++){const tt=mt[Z],st=g(tt);typeof tt=="number"||typeof tt=="boolean"?(J.__data[0]=tt,n.bufferSubData(n.UNIFORM_BUFFER,rt+O,J.__data)):tt.isMatrix3?(J.__data[0]=tt.elements[0],J.__data[1]=tt.elements[1],J.__data[2]=tt.elements[2],J.__data[3]=0,J.__data[4]=tt.elements[3],J.__data[5]=tt.elements[4],J.__data[6]=tt.elements[5],J.__data[7]=0,J.__data[8]=tt.elements[6],J.__data[9]=tt.elements[7],J.__data[10]=tt.elements[8],J.__data[11]=0):(tt.toArray(J.__data,O),O+=st.storage/Float32Array.BYTES_PER_ELEMENT)}n.bufferSubData(n.UNIFORM_BUFFER,rt,J.__data)}}}n.bindBuffer(n.UNIFORM_BUFFER,null)}function f(E,b,x,C){const A=E.value,D=b+"_"+x;if(C[D]===void 0)return typeof A=="number"||typeof A=="boolean"?C[D]=A:C[D]=A.clone(),!0;{const G=C[D];if(typeof A=="number"||typeof A=="boolean"){if(G!==A)return C[D]=A,!0}else if(G.equals(A)===!1)return G.copy(A),!0}return!1}function v(E){const b=E.uniforms;let x=0;const C=16;for(let D=0,G=b.length;D<G;D++){const T=Array.isArray(b[D])?b[D]:[b[D]];for(let R=0,J=T.length;R<J;R++){const rt=T[R],mt=Array.isArray(rt.value)?rt.value:[rt.value];for(let O=0,Z=mt.length;O<Z;O++){const tt=mt[O],st=g(tt),et=x%C;et!==0&&C-et<st.boundary&&(x+=C-et),rt.__data=new Float32Array(st.storage/Float32Array.BYTES_PER_ELEMENT),rt.__offset=x,x+=st.storage}}}const A=x%C;return A>0&&(x+=C-A),E.__size=x,E.__cache={},this}function g(E){const b={boundary:0,storage:0};return typeof E=="number"||typeof E=="boolean"?(b.boundary=4,b.storage=4):E.isVector2?(b.boundary=8,b.storage=8):E.isVector3||E.isColor?(b.boundary=16,b.storage=12):E.isVector4?(b.boundary=16,b.storage=16):E.isMatrix3?(b.boundary=48,b.storage=48):E.isMatrix4?(b.boundary=64,b.storage=64):E.isTexture?console.warn("THREE.WebGLRenderer: Texture samplers can not be part of an uniforms group."):console.warn("THREE.WebGLRenderer: Unsupported uniform value type.",E),b}function _(E){const b=E.target;b.removeEventListener("dispose",_);const x=a.indexOf(b.__bindingPointIndex);a.splice(x,1),n.deleteBuffer(s[b.id]),delete s[b.id],delete r[b.id]}function m(){for(const E in s)n.deleteBuffer(s[E]);a=[],s={},r={}}return{bind:l,update:c,dispose:m}}class _h{constructor(t={}){const{canvas:e=c_(),context:i=null,depth:s=!0,stencil:r=!0,alpha:a=!1,antialias:o=!1,premultipliedAlpha:l=!0,preserveDrawingBuffer:c=!1,powerPreference:d="default",failIfMajorPerformanceCaveat:h=!1}=t;this.isWebGLRenderer=!0;let p;i!==null?p=i.getContextAttributes().alpha:p=a;const f=new Uint32Array(4),v=new Int32Array(4);let g=null,_=null;const m=[],E=[];this.domElement=e,this.debug={checkShaderErrors:!0,onShaderError:null},this.autoClear=!0,this.autoClearColor=!0,this.autoClearDepth=!0,this.autoClearStencil=!0,this.sortObjects=!0,this.clippingPlanes=[],this.localClippingEnabled=!1,this._outputColorSpace=Ie,this._useLegacyLights=!1,this.toneMapping=_i,this.toneMappingExposure=1;const b=this;let x=!1,C=0,A=0,D=null,G=-1,T=null;const R=new xe,J=new xe;let rt=null;const mt=new ne(0);let O=0,Z=e.width,tt=e.height,st=1,et=null,ot=null;const ut=new xe(0,0,Z,tt),pt=new xe(0,0,Z,tt);let ft=!1;const it=new ll;let ht=!1,bt=!1,Lt=null;const Pt=new Te,Ht=new re,Gt=new z,Nt={background:null,fog:null,environment:null,overrideMaterial:null,isScene:!0};function Zt(){return D===null?st:1}let M=i;function k(w,W){for(let Q=0;Q<w.length;Q++){const nt=w[Q],K=e.getContext(nt,W);if(K!==null)return K}return null}try{const w={alpha:!0,depth:s,stencil:r,antialias:o,premultipliedAlpha:l,preserveDrawingBuffer:c,powerPreference:d,failIfMajorPerformanceCaveat:h};if("setAttribute"in e&&e.setAttribute("data-engine",`three.js r${sl}`),e.addEventListener("webglcontextlost",gt,!1),e.addEventListener("webglcontextrestored",F,!1),e.addEventListener("webglcontextcreationerror",Mt,!1),M===null){const W=["webgl2","webgl","experimental-webgl"];if(b.isWebGL1Renderer===!0&&W.shift(),M=k(W,w),M===null)throw k(W)?new Error("Error creating WebGL context with your selected attributes."):new Error("Error creating WebGL context.")}typeof WebGLRenderingContext<"u"&&M instanceof WebGLRenderingContext&&console.warn("THREE.WebGLRenderer: WebGL 1 support was deprecated in r153 and will be removed in r163."),M.getShaderPrecisionFormat===void 0&&(M.getShaderPrecisionFormat=function(){return{rangeMin:1,rangeMax:1,precision:1}})}catch(w){throw console.error("THREE.WebGLRenderer: "+w.message),w}let B,Y,N,V,j,S,y,U,q,H,X,lt,at,dt,ct,_t,$,Rt,At,Tt,Ct,xt,Dt,Yt;function le(){B=new E0(M),Y=new g0(M,B,t),B.init(Y),xt=new cy(M,B,Y),N=new ay(M,B,Y),V=new A0(M),j=new qx,S=new ly(M,B,N,j,Y,xt,V),y=new x0(b),U=new S0(b),q=new I_(M,Y),Dt=new m0(M,B,q,Y),H=new T0(M,q,V,Dt),X=new P0(M,H,q,V),At=new L0(M,Y,S),_t=new v0(j),lt=new Xx(b,y,U,B,Y,Dt,_t),at=new fy(b,j),dt=new Yx,ct=new ey(B,Y),Rt=new p0(b,y,U,N,X,p,l),$=new oy(b,X,Y),Yt=new py(M,V,Y,N),Tt=new _0(M,B,V,Y),Ct=new w0(M,B,V,Y),V.programs=lt.programs,b.capabilities=Y,b.extensions=B,b.properties=j,b.renderLists=dt,b.shadowMap=$,b.state=N,b.info=V}le();const Jt=new hy(b,M);this.xr=Jt,this.getContext=function(){return M},this.getContextAttributes=function(){return M.getContextAttributes()},this.forceContextLoss=function(){const w=B.get("WEBGL_lose_context");w&&w.loseContext()},this.forceContextRestore=function(){const w=B.get("WEBGL_lose_context");w&&w.restoreContext()},this.getPixelRatio=function(){return st},this.setPixelRatio=function(w){w!==void 0&&(st=w,this.setSize(Z,tt,!1))},this.getSize=function(w){return w.set(Z,tt)},this.setSize=function(w,W,Q=!0){if(Jt.isPresenting){console.warn("THREE.WebGLRenderer: Can't change size while VR device is presenting.");return}Z=w,tt=W,e.width=Math.floor(w*st),e.height=Math.floor(W*st),Q===!0&&(e.style.width=w+"px",e.style.height=W+"px"),this.setViewport(0,0,w,W)},this.getDrawingBufferSize=function(w){return w.set(Z*st,tt*st).floor()},this.setDrawingBufferSize=function(w,W,Q){Z=w,tt=W,st=Q,e.width=Math.floor(w*Q),e.height=Math.floor(W*Q),this.setViewport(0,0,w,W)},this.getCurrentViewport=function(w){return w.copy(R)},this.getViewport=function(w){return w.copy(ut)},this.setViewport=function(w,W,Q,nt){w.isVector4?ut.set(w.x,w.y,w.z,w.w):ut.set(w,W,Q,nt),N.viewport(R.copy(ut).multiplyScalar(st).floor())},this.getScissor=function(w){return w.copy(pt)},this.setScissor=function(w,W,Q,nt){w.isVector4?pt.set(w.x,w.y,w.z,w.w):pt.set(w,W,Q,nt),N.scissor(J.copy(pt).multiplyScalar(st).floor())},this.getScissorTest=function(){return ft},this.setScissorTest=function(w){N.setScissorTest(ft=w)},this.setOpaqueSort=function(w){et=w},this.setTransparentSort=function(w){ot=w},this.getClearColor=function(w){return w.copy(Rt.getClearColor())},this.setClearColor=function(){Rt.setClearColor.apply(Rt,arguments)},this.getClearAlpha=function(){return Rt.getClearAlpha()},this.setClearAlpha=function(){Rt.setClearAlpha.apply(Rt,arguments)},this.clear=function(w=!0,W=!0,Q=!0){let nt=0;if(w){let K=!1;if(D!==null){const wt=D.texture.format;K=wt===jd||wt===qd||wt===Xd}if(K){const wt=D.texture.type,kt=wt===gi||wt===fi||wt===rl||wt===Oi||wt===Wd||wt===$d,Bt=Rt.getClearColor(),Wt=Rt.getClearAlpha(),Kt=Bt.r,$t=Bt.g,qt=Bt.b;kt?(f[0]=Kt,f[1]=$t,f[2]=qt,f[3]=Wt,M.clearBufferuiv(M.COLOR,0,f)):(v[0]=Kt,v[1]=$t,v[2]=qt,v[3]=Wt,M.clearBufferiv(M.COLOR,0,v))}else nt|=M.COLOR_BUFFER_BIT}W&&(nt|=M.DEPTH_BUFFER_BIT),Q&&(nt|=M.STENCIL_BUFFER_BIT,this.state.buffers.stencil.setMask(4294967295)),M.clear(nt)},this.clearColor=function(){this.clear(!0,!1,!1)},this.clearDepth=function(){this.clear(!1,!0,!1)},this.clearStencil=function(){this.clear(!1,!1,!0)},this.dispose=function(){e.removeEventListener("webglcontextlost",gt,!1),e.removeEventListener("webglcontextrestored",F,!1),e.removeEventListener("webglcontextcreationerror",Mt,!1),dt.dispose(),ct.dispose(),j.dispose(),y.dispose(),U.dispose(),X.dispose(),Dt.dispose(),Yt.dispose(),lt.dispose(),Jt.dispose(),Jt.removeEventListener("sessionstart",Xe),Jt.removeEventListener("sessionend",me),Lt&&(Lt.dispose(),Lt=null),qe.stop()};function gt(w){w.preventDefault(),console.log("THREE.WebGLRenderer: Context Lost."),x=!0}function F(){console.log("THREE.WebGLRenderer: Context Restored."),x=!1;const w=V.autoReset,W=$.enabled,Q=$.autoUpdate,nt=$.needsUpdate,K=$.type;le(),V.autoReset=w,$.enabled=W,$.autoUpdate=Q,$.needsUpdate=nt,$.type=K}function Mt(w){console.error("THREE.WebGLRenderer: A WebGL context could not be created. Reason: ",w.statusMessage)}function St(w){const W=w.target;W.removeEventListener("dispose",St),Vt(W)}function Vt(w){Ft(w),j.remove(w)}function Ft(w){const W=j.get(w).programs;W!==void 0&&(W.forEach(function(Q){lt.releaseProgram(Q)}),w.isShaderMaterial&&lt.releaseShaderCache(w))}this.renderBufferDirect=function(w,W,Q,nt,K,wt){W===null&&(W=Nt);const kt=K.isMesh&&K.matrixWorld.determinant()<0,Bt=yh(w,W,Q,nt,K);N.setMaterial(nt,kt);let Wt=Q.index,Kt=1;if(nt.wireframe===!0){if(Wt=H.getWireframeAttribute(Q),Wt===void 0)return;Kt=2}const $t=Q.drawRange,qt=Q.attributes.position;let we=$t.start*Kt,rn=($t.start+$t.count)*Kt;wt!==null&&(we=Math.max(we,wt.start*Kt),rn=Math.min(rn,(wt.start+wt.count)*Kt)),Wt!==null?(we=Math.max(we,0),rn=Math.min(rn,Wt.count)):qt!=null&&(we=Math.max(we,0),rn=Math.min(rn,qt.count));const Pe=rn-we;if(Pe<0||Pe===1/0)return;Dt.setup(K,nt,Bt,Q,Wt);let Bn,ye=Tt;if(Wt!==null&&(Bn=q.get(Wt),ye=Ct,ye.setIndex(Bn)),K.isMesh)nt.wireframe===!0?(N.setLineWidth(nt.wireframeLinewidth*Zt()),ye.setMode(M.LINES)):ye.setMode(M.TRIANGLES);else if(K.isLine){let Qt=nt.linewidth;Qt===void 0&&(Qt=1),N.setLineWidth(Qt*Zt()),K.isLineSegments?ye.setMode(M.LINES):K.isLineLoop?ye.setMode(M.LINE_LOOP):ye.setMode(M.LINE_STRIP)}else K.isPoints?ye.setMode(M.POINTS):K.isSprite&&ye.setMode(M.TRIANGLES);if(K.isBatchedMesh)ye.renderMultiDraw(K._multiDrawStarts,K._multiDrawCounts,K._multiDrawCount);else if(K.isInstancedMesh)ye.renderInstances(we,Pe,K.count);else if(Q.isInstancedBufferGeometry){const Qt=Q._maxInstanceCount!==void 0?Q._maxInstanceCount:1/0,To=Math.min(Q.instanceCount,Qt);ye.renderInstances(we,Pe,To)}else ye.render(we,Pe)};function fe(w,W,Q){w.transparent===!0&&w.side===Qn&&w.forceSinglePass===!1?(w.side=tn,w.needsUpdate=!0,hr(w,W,Q),w.side=bi,w.needsUpdate=!0,hr(w,W,Q),w.side=Qn):hr(w,W,Q)}this.compile=function(w,W,Q=null){Q===null&&(Q=w),_=ct.get(Q),_.init(),E.push(_),Q.traverseVisible(function(K){K.isLight&&K.layers.test(W.layers)&&(_.pushLight(K),K.castShadow&&_.pushShadow(K))}),w!==Q&&w.traverseVisible(function(K){K.isLight&&K.layers.test(W.layers)&&(_.pushLight(K),K.castShadow&&_.pushShadow(K))}),_.setupLights(b._useLegacyLights);const nt=new Set;return w.traverse(function(K){const wt=K.material;if(wt)if(Array.isArray(wt))for(let kt=0;kt<wt.length;kt++){const Bt=wt[kt];fe(Bt,Q,K),nt.add(Bt)}else fe(wt,Q,K),nt.add(wt)}),E.pop(),_=null,nt},this.compileAsync=function(w,W,Q=null){const nt=this.compile(w,W,Q);return new Promise(K=>{function wt(){if(nt.forEach(function(kt){j.get(kt).currentProgram.isReady()&&nt.delete(kt)}),nt.size===0){K(w);return}setTimeout(wt,10)}B.get("KHR_parallel_shader_compile")!==null?wt():setTimeout(wt,10)})};let pe=null;function Le(w){pe&&pe(w)}function Xe(){qe.stop()}function me(){qe.start()}const qe=new ch;qe.setAnimationLoop(Le),typeof self<"u"&&qe.setContext(self),this.setAnimationLoop=function(w){pe=w,Jt.setAnimationLoop(w),w===null?qe.stop():qe.start()},Jt.addEventListener("sessionstart",Xe),Jt.addEventListener("sessionend",me),this.render=function(w,W){if(W!==void 0&&W.isCamera!==!0){console.error("THREE.WebGLRenderer.render: camera is not an instance of THREE.Camera.");return}if(x===!0)return;w.matrixWorldAutoUpdate===!0&&w.updateMatrixWorld(),W.parent===null&&W.matrixWorldAutoUpdate===!0&&W.updateMatrixWorld(),Jt.enabled===!0&&Jt.isPresenting===!0&&(Jt.cameraAutoUpdate===!0&&Jt.updateCamera(W),W=Jt.getCamera()),w.isScene===!0&&w.onBeforeRender(b,w,W,D),_=ct.get(w,E.length),_.init(),E.push(_),Pt.multiplyMatrices(W.projectionMatrix,W.matrixWorldInverse),it.setFromProjectionMatrix(Pt),bt=this.localClippingEnabled,ht=_t.init(this.clippingPlanes,bt),g=dt.get(w,m.length),g.init(),m.push(g),Cn(w,W,0,b.sortObjects),g.finish(),b.sortObjects===!0&&g.sort(et,ot),this.info.render.frame++,ht===!0&&_t.beginShadows();const Q=_.state.shadowsArray;if($.render(Q,w,W),ht===!0&&_t.endShadows(),this.info.autoReset===!0&&this.info.reset(),Rt.render(g,w),_.setupLights(b._useLegacyLights),W.isArrayCamera){const nt=W.cameras;for(let K=0,wt=nt.length;K<wt;K++){const kt=nt[K];ml(g,w,kt,kt.viewport)}}else ml(g,w,W);D!==null&&(S.updateMultisampleRenderTarget(D),S.updateRenderTargetMipmap(D)),w.isScene===!0&&w.onAfterRender(b,w,W),Dt.resetDefaultState(),G=-1,T=null,E.pop(),E.length>0?_=E[E.length-1]:_=null,m.pop(),m.length>0?g=m[m.length-1]:g=null};function Cn(w,W,Q,nt){if(w.visible===!1)return;if(w.layers.test(W.layers)){if(w.isGroup)Q=w.renderOrder;else if(w.isLOD)w.autoUpdate===!0&&w.update(W);else if(w.isLight)_.pushLight(w),w.castShadow&&_.pushShadow(w);else if(w.isSprite){if(!w.frustumCulled||it.intersectsSprite(w)){nt&&Gt.setFromMatrixPosition(w.matrixWorld).applyMatrix4(Pt);const kt=X.update(w),Bt=w.material;Bt.visible&&g.push(w,kt,Bt,Q,Gt.z,null)}}else if((w.isMesh||w.isLine||w.isPoints)&&(!w.frustumCulled||it.intersectsObject(w))){const kt=X.update(w),Bt=w.material;if(nt&&(w.boundingSphere!==void 0?(w.boundingSphere===null&&w.computeBoundingSphere(),Gt.copy(w.boundingSphere.center)):(kt.boundingSphere===null&&kt.computeBoundingSphere(),Gt.copy(kt.boundingSphere.center)),Gt.applyMatrix4(w.matrixWorld).applyMatrix4(Pt)),Array.isArray(Bt)){const Wt=kt.groups;for(let Kt=0,$t=Wt.length;Kt<$t;Kt++){const qt=Wt[Kt],we=Bt[qt.materialIndex];we&&we.visible&&g.push(w,kt,we,Q,Gt.z,qt)}}else Bt.visible&&g.push(w,kt,Bt,Q,Gt.z,null)}}const wt=w.children;for(let kt=0,Bt=wt.length;kt<Bt;kt++)Cn(wt[kt],W,Q,nt)}function ml(w,W,Q,nt){const K=w.opaque,wt=w.transmissive,kt=w.transparent;_.setupLightsView(Q),ht===!0&&_t.setGlobalState(b.clippingPlanes,Q),wt.length>0&&xh(K,wt,W,Q),nt&&N.viewport(R.copy(nt)),K.length>0&&dr(K,W,Q),wt.length>0&&dr(wt,W,Q),kt.length>0&&dr(kt,W,Q),N.buffers.depth.setTest(!0),N.buffers.depth.setMask(!0),N.buffers.color.setMask(!0),N.setPolygonOffset(!1)}function xh(w,W,Q,nt){if((Q.isScene===!0?Q.overrideMaterial:null)!==null)return;const wt=Y.isWebGL2;Lt===null&&(Lt=new Hi(1,1,{generateMipmaps:!0,type:B.has("EXT_color_buffer_half_float")?er:gi,minFilter:tr,samples:wt?4:0})),b.getDrawingBufferSize(Ht),wt?Lt.setSize(Ht.x,Ht.y):Lt.setSize(Oa(Ht.x),Oa(Ht.y));const kt=b.getRenderTarget();b.setRenderTarget(Lt),b.getClearColor(mt),O=b.getClearAlpha(),O<1&&b.setClearColor(16777215,.5),b.clear();const Bt=b.toneMapping;b.toneMapping=_i,dr(w,Q,nt),S.updateMultisampleRenderTarget(Lt),S.updateRenderTargetMipmap(Lt);let Wt=!1;for(let Kt=0,$t=W.length;Kt<$t;Kt++){const qt=W[Kt],we=qt.object,rn=qt.geometry,Pe=qt.material,Bn=qt.group;if(Pe.side===Qn&&we.layers.test(nt.layers)){const ye=Pe.side;Pe.side=tn,Pe.needsUpdate=!0,_l(we,Q,nt,rn,Pe,Bn),Pe.side=ye,Pe.needsUpdate=!0,Wt=!0}}Wt===!0&&(S.updateMultisampleRenderTarget(Lt),S.updateRenderTargetMipmap(Lt)),b.setRenderTarget(kt),b.setClearColor(mt,O),b.toneMapping=Bt}function dr(w,W,Q){const nt=W.isScene===!0?W.overrideMaterial:null;for(let K=0,wt=w.length;K<wt;K++){const kt=w[K],Bt=kt.object,Wt=kt.geometry,Kt=nt===null?kt.material:nt,$t=kt.group;Bt.layers.test(Q.layers)&&_l(Bt,W,Q,Wt,Kt,$t)}}function _l(w,W,Q,nt,K,wt){w.onBeforeRender(b,W,Q,nt,K,wt),w.modelViewMatrix.multiplyMatrices(Q.matrixWorldInverse,w.matrixWorld),w.normalMatrix.getNormalMatrix(w.modelViewMatrix),K.onBeforeRender(b,W,Q,nt,w,wt),K.transparent===!0&&K.side===Qn&&K.forceSinglePass===!1?(K.side=tn,K.needsUpdate=!0,b.renderBufferDirect(Q,W,nt,K,w,wt),K.side=bi,K.needsUpdate=!0,b.renderBufferDirect(Q,W,nt,K,w,wt),K.side=Qn):b.renderBufferDirect(Q,W,nt,K,w,wt),w.onAfterRender(b,W,Q,nt,K,wt)}function hr(w,W,Q){W.isScene!==!0&&(W=Nt);const nt=j.get(w),K=_.state.lights,wt=_.state.shadowsArray,kt=K.state.version,Bt=lt.getParameters(w,K.state,wt,W,Q),Wt=lt.getProgramCacheKey(Bt);let Kt=nt.programs;nt.environment=w.isMeshStandardMaterial?W.environment:null,nt.fog=W.fog,nt.envMap=(w.isMeshStandardMaterial?U:y).get(w.envMap||nt.environment),Kt===void 0&&(w.addEventListener("dispose",St),Kt=new Map,nt.programs=Kt);let $t=Kt.get(Wt);if($t!==void 0){if(nt.currentProgram===$t&&nt.lightsStateVersion===kt)return vl(w,Bt),$t}else Bt.uniforms=lt.getUniforms(w),w.onBuild(Q,Bt,b),w.onBeforeCompile(Bt,b),$t=lt.acquireProgram(Bt,Wt),Kt.set(Wt,$t),nt.uniforms=Bt.uniforms;const qt=nt.uniforms;return(!w.isShaderMaterial&&!w.isRawShaderMaterial||w.clipping===!0)&&(qt.clippingPlanes=_t.uniform),vl(w,Bt),nt.needsLights=Mh(w),nt.lightsStateVersion=kt,nt.needsLights&&(qt.ambientLightColor.value=K.state.ambient,qt.lightProbe.value=K.state.probe,qt.directionalLights.value=K.state.directional,qt.directionalLightShadows.value=K.state.directionalShadow,qt.spotLights.value=K.state.spot,qt.spotLightShadows.value=K.state.spotShadow,qt.rectAreaLights.value=K.state.rectArea,qt.ltc_1.value=K.state.rectAreaLTC1,qt.ltc_2.value=K.state.rectAreaLTC2,qt.pointLights.value=K.state.point,qt.pointLightShadows.value=K.state.pointShadow,qt.hemisphereLights.value=K.state.hemi,qt.directionalShadowMap.value=K.state.directionalShadowMap,qt.directionalShadowMatrix.value=K.state.directionalShadowMatrix,qt.spotShadowMap.value=K.state.spotShadowMap,qt.spotLightMatrix.value=K.state.spotLightMatrix,qt.spotLightMap.value=K.state.spotLightMap,qt.pointShadowMap.value=K.state.pointShadowMap,qt.pointShadowMatrix.value=K.state.pointShadowMatrix),nt.currentProgram=$t,nt.uniformsList=null,$t}function gl(w){if(w.uniformsList===null){const W=w.currentProgram.getUniforms();w.uniformsList=qr.seqWithValue(W.seq,w.uniforms)}return w.uniformsList}function vl(w,W){const Q=j.get(w);Q.outputColorSpace=W.outputColorSpace,Q.batching=W.batching,Q.instancing=W.instancing,Q.instancingColor=W.instancingColor,Q.skinning=W.skinning,Q.morphTargets=W.morphTargets,Q.morphNormals=W.morphNormals,Q.morphColors=W.morphColors,Q.morphTargetsCount=W.morphTargetsCount,Q.numClippingPlanes=W.numClippingPlanes,Q.numIntersection=W.numClipIntersection,Q.vertexAlphas=W.vertexAlphas,Q.vertexTangents=W.vertexTangents,Q.toneMapping=W.toneMapping}function yh(w,W,Q,nt,K){W.isScene!==!0&&(W=Nt),S.resetTextureUnits();const wt=W.fog,kt=nt.isMeshStandardMaterial?W.environment:null,Bt=D===null?b.outputColorSpace:D.isXRRenderTarget===!0?D.texture.colorSpace:ri,Wt=(nt.isMeshStandardMaterial?U:y).get(nt.envMap||kt),Kt=nt.vertexColors===!0&&!!Q.attributes.color&&Q.attributes.color.itemSize===4,$t=!!Q.attributes.tangent&&(!!nt.normalMap||nt.anisotropy>0),qt=!!Q.morphAttributes.position,we=!!Q.morphAttributes.normal,rn=!!Q.morphAttributes.color;let Pe=_i;nt.toneMapped&&(D===null||D.isXRRenderTarget===!0)&&(Pe=b.toneMapping);const Bn=Q.morphAttributes.position||Q.morphAttributes.normal||Q.morphAttributes.color,ye=Bn!==void 0?Bn.length:0,Qt=j.get(nt),To=_.state.lights;if(ht===!0&&(bt===!0||w!==T)){const hn=w===T&&nt.id===G;_t.setState(nt,w,hn)}let Se=!1;nt.version===Qt.__version?(Qt.needsLights&&Qt.lightsStateVersion!==To.state.version||Qt.outputColorSpace!==Bt||K.isBatchedMesh&&Qt.batching===!1||!K.isBatchedMesh&&Qt.batching===!0||K.isInstancedMesh&&Qt.instancing===!1||!K.isInstancedMesh&&Qt.instancing===!0||K.isSkinnedMesh&&Qt.skinning===!1||!K.isSkinnedMesh&&Qt.skinning===!0||K.isInstancedMesh&&Qt.instancingColor===!0&&K.instanceColor===null||K.isInstancedMesh&&Qt.instancingColor===!1&&K.instanceColor!==null||Qt.envMap!==Wt||nt.fog===!0&&Qt.fog!==wt||Qt.numClippingPlanes!==void 0&&(Qt.numClippingPlanes!==_t.numPlanes||Qt.numIntersection!==_t.numIntersection)||Qt.vertexAlphas!==Kt||Qt.vertexTangents!==$t||Qt.morphTargets!==qt||Qt.morphNormals!==we||Qt.morphColors!==rn||Qt.toneMapping!==Pe||Y.isWebGL2===!0&&Qt.morphTargetsCount!==ye)&&(Se=!0):(Se=!0,Qt.__version=nt.version);let Si=Qt.currentProgram;Se===!0&&(Si=hr(nt,W,K));let xl=!1,Cs=!1,wo=!1;const Fe=Si.getUniforms(),Ei=Qt.uniforms;if(N.useProgram(Si.program)&&(xl=!0,Cs=!0,wo=!0),nt.id!==G&&(G=nt.id,Cs=!0),xl||T!==w){Fe.setValue(M,"projectionMatrix",w.projectionMatrix),Fe.setValue(M,"viewMatrix",w.matrixWorldInverse);const hn=Fe.map.cameraPosition;hn!==void 0&&hn.setValue(M,Gt.setFromMatrixPosition(w.matrixWorld)),Y.logarithmicDepthBuffer&&Fe.setValue(M,"logDepthBufFC",2/(Math.log(w.far+1)/Math.LN2)),(nt.isMeshPhongMaterial||nt.isMeshToonMaterial||nt.isMeshLambertMaterial||nt.isMeshBasicMaterial||nt.isMeshStandardMaterial||nt.isShaderMaterial)&&Fe.setValue(M,"isOrthographic",w.isOrthographicCamera===!0),T!==w&&(T=w,Cs=!0,wo=!0)}if(K.isSkinnedMesh){Fe.setOptional(M,K,"bindMatrix"),Fe.setOptional(M,K,"bindMatrixInverse");const hn=K.skeleton;hn&&(Y.floatVertexTextures?(hn.boneTexture===null&&hn.computeBoneTexture(),Fe.setValue(M,"boneTexture",hn.boneTexture,S)):console.warn("THREE.WebGLRenderer: SkinnedMesh can only be used with WebGL 2. With WebGL 1 OES_texture_float and vertex textures support is required."))}K.isBatchedMesh&&(Fe.setOptional(M,K,"batchingTexture"),Fe.setValue(M,"batchingTexture",K._matricesTexture,S));const Ao=Q.morphAttributes;if((Ao.position!==void 0||Ao.normal!==void 0||Ao.color!==void 0&&Y.isWebGL2===!0)&&At.update(K,Q,Si),(Cs||Qt.receiveShadow!==K.receiveShadow)&&(Qt.receiveShadow=K.receiveShadow,Fe.setValue(M,"receiveShadow",K.receiveShadow)),nt.isMeshGouraudMaterial&&nt.envMap!==null&&(Ei.envMap.value=Wt,Ei.flipEnvMap.value=Wt.isCubeTexture&&Wt.isRenderTargetTexture===!1?-1:1),Cs&&(Fe.setValue(M,"toneMappingExposure",b.toneMappingExposure),Qt.needsLights&&bh(Ei,wo),wt&&nt.fog===!0&&at.refreshFogUniforms(Ei,wt),at.refreshMaterialUniforms(Ei,nt,st,tt,Lt),qr.upload(M,gl(Qt),Ei,S)),nt.isShaderMaterial&&nt.uniformsNeedUpdate===!0&&(qr.upload(M,gl(Qt),Ei,S),nt.uniformsNeedUpdate=!1),nt.isSpriteMaterial&&Fe.setValue(M,"center",K.center),Fe.setValue(M,"modelViewMatrix",K.modelViewMatrix),Fe.setValue(M,"normalMatrix",K.normalMatrix),Fe.setValue(M,"modelMatrix",K.matrixWorld),nt.isShaderMaterial||nt.isRawShaderMaterial){const hn=nt.uniformsGroups;for(let Co=0,Sh=hn.length;Co<Sh;Co++)if(Y.isWebGL2){const yl=hn[Co];Yt.update(yl,Si),Yt.bind(yl,Si)}else console.warn("THREE.WebGLRenderer: Uniform Buffer Objects can only be used with WebGL 2.")}return Si}function bh(w,W){w.ambientLightColor.needsUpdate=W,w.lightProbe.needsUpdate=W,w.directionalLights.needsUpdate=W,w.directionalLightShadows.needsUpdate=W,w.pointLights.needsUpdate=W,w.pointLightShadows.needsUpdate=W,w.spotLights.needsUpdate=W,w.spotLightShadows.needsUpdate=W,w.rectAreaLights.needsUpdate=W,w.hemisphereLights.needsUpdate=W}function Mh(w){return w.isMeshLambertMaterial||w.isMeshToonMaterial||w.isMeshPhongMaterial||w.isMeshStandardMaterial||w.isShadowMaterial||w.isShaderMaterial&&w.lights===!0}this.getActiveCubeFace=function(){return C},this.getActiveMipmapLevel=function(){return A},this.getRenderTarget=function(){return D},this.setRenderTargetTextures=function(w,W,Q){j.get(w.texture).__webglTexture=W,j.get(w.depthTexture).__webglTexture=Q;const nt=j.get(w);nt.__hasExternalTextures=!0,nt.__hasExternalTextures&&(nt.__autoAllocateDepthBuffer=Q===void 0,nt.__autoAllocateDepthBuffer||B.has("WEBGL_multisampled_render_to_texture")===!0&&(console.warn("THREE.WebGLRenderer: Render-to-texture extension was disabled because an external texture was provided"),nt.__useRenderToTexture=!1))},this.setRenderTargetFramebuffer=function(w,W){const Q=j.get(w);Q.__webglFramebuffer=W,Q.__useDefaultFramebuffer=W===void 0},this.setRenderTarget=function(w,W=0,Q=0){D=w,C=W,A=Q;let nt=!0,K=null,wt=!1,kt=!1;if(w){const Wt=j.get(w);Wt.__useDefaultFramebuffer!==void 0?(N.bindFramebuffer(M.FRAMEBUFFER,null),nt=!1):Wt.__webglFramebuffer===void 0?S.setupRenderTarget(w):Wt.__hasExternalTextures&&S.rebindTextures(w,j.get(w.texture).__webglTexture,j.get(w.depthTexture).__webglTexture);const Kt=w.texture;(Kt.isData3DTexture||Kt.isDataArrayTexture||Kt.isCompressedArrayTexture)&&(kt=!0);const $t=j.get(w).__webglFramebuffer;w.isWebGLCubeRenderTarget?(Array.isArray($t[W])?K=$t[W][Q]:K=$t[W],wt=!0):Y.isWebGL2&&w.samples>0&&S.useMultisampledRTT(w)===!1?K=j.get(w).__webglMultisampledFramebuffer:Array.isArray($t)?K=$t[Q]:K=$t,R.copy(w.viewport),J.copy(w.scissor),rt=w.scissorTest}else R.copy(ut).multiplyScalar(st).floor(),J.copy(pt).multiplyScalar(st).floor(),rt=ft;if(N.bindFramebuffer(M.FRAMEBUFFER,K)&&Y.drawBuffers&&nt&&N.drawBuffers(w,K),N.viewport(R),N.scissor(J),N.setScissorTest(rt),wt){const Wt=j.get(w.texture);M.framebufferTexture2D(M.FRAMEBUFFER,M.COLOR_ATTACHMENT0,M.TEXTURE_CUBE_MAP_POSITIVE_X+W,Wt.__webglTexture,Q)}else if(kt){const Wt=j.get(w.texture),Kt=W||0;M.framebufferTextureLayer(M.FRAMEBUFFER,M.COLOR_ATTACHMENT0,Wt.__webglTexture,Q||0,Kt)}G=-1},this.readRenderTargetPixels=function(w,W,Q,nt,K,wt,kt){if(!(w&&w.isWebGLRenderTarget)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not THREE.WebGLRenderTarget.");return}let Bt=j.get(w).__webglFramebuffer;if(w.isWebGLCubeRenderTarget&&kt!==void 0&&(Bt=Bt[kt]),Bt){N.bindFramebuffer(M.FRAMEBUFFER,Bt);try{const Wt=w.texture,Kt=Wt.format,$t=Wt.type;if(Kt!==En&&xt.convert(Kt)!==M.getParameter(M.IMPLEMENTATION_COLOR_READ_FORMAT)){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in RGBA or implementation defined format.");return}const qt=$t===er&&(B.has("EXT_color_buffer_half_float")||Y.isWebGL2&&B.has("EXT_color_buffer_float"));if($t!==gi&&xt.convert($t)!==M.getParameter(M.IMPLEMENTATION_COLOR_READ_TYPE)&&!($t===pi&&(Y.isWebGL2||B.has("OES_texture_float")||B.has("WEBGL_color_buffer_float")))&&!qt){console.error("THREE.WebGLRenderer.readRenderTargetPixels: renderTarget is not in UnsignedByteType or implementation defined type.");return}W>=0&&W<=w.width-nt&&Q>=0&&Q<=w.height-K&&M.readPixels(W,Q,nt,K,xt.convert(Kt),xt.convert($t),wt)}finally{const Wt=D!==null?j.get(D).__webglFramebuffer:null;N.bindFramebuffer(M.FRAMEBUFFER,Wt)}}},this.copyFramebufferToTexture=function(w,W,Q=0){const nt=Math.pow(2,-Q),K=Math.floor(W.image.width*nt),wt=Math.floor(W.image.height*nt);S.setTexture2D(W,0),M.copyTexSubImage2D(M.TEXTURE_2D,Q,0,0,w.x,w.y,K,wt),N.unbindTexture()},this.copyTextureToTexture=function(w,W,Q,nt=0){const K=W.image.width,wt=W.image.height,kt=xt.convert(Q.format),Bt=xt.convert(Q.type);S.setTexture2D(Q,0),M.pixelStorei(M.UNPACK_FLIP_Y_WEBGL,Q.flipY),M.pixelStorei(M.UNPACK_PREMULTIPLY_ALPHA_WEBGL,Q.premultiplyAlpha),M.pixelStorei(M.UNPACK_ALIGNMENT,Q.unpackAlignment),W.isDataTexture?M.texSubImage2D(M.TEXTURE_2D,nt,w.x,w.y,K,wt,kt,Bt,W.image.data):W.isCompressedTexture?M.compressedTexSubImage2D(M.TEXTURE_2D,nt,w.x,w.y,W.mipmaps[0].width,W.mipmaps[0].height,kt,W.mipmaps[0].data):M.texSubImage2D(M.TEXTURE_2D,nt,w.x,w.y,kt,Bt,W.image),nt===0&&Q.generateMipmaps&&M.generateMipmap(M.TEXTURE_2D),N.unbindTexture()},this.copyTextureToTexture3D=function(w,W,Q,nt,K=0){if(b.isWebGL1Renderer){console.warn("THREE.WebGLRenderer.copyTextureToTexture3D: can only be used with WebGL2.");return}const wt=w.max.x-w.min.x+1,kt=w.max.y-w.min.y+1,Bt=w.max.z-w.min.z+1,Wt=xt.convert(nt.format),Kt=xt.convert(nt.type);let $t;if(nt.isData3DTexture)S.setTexture3D(nt,0),$t=M.TEXTURE_3D;else if(nt.isDataArrayTexture||nt.isCompressedArrayTexture)S.setTexture2DArray(nt,0),$t=M.TEXTURE_2D_ARRAY;else{console.warn("THREE.WebGLRenderer.copyTextureToTexture3D: only supports THREE.DataTexture3D and THREE.DataTexture2DArray.");return}M.pixelStorei(M.UNPACK_FLIP_Y_WEBGL,nt.flipY),M.pixelStorei(M.UNPACK_PREMULTIPLY_ALPHA_WEBGL,nt.premultiplyAlpha),M.pixelStorei(M.UNPACK_ALIGNMENT,nt.unpackAlignment);const qt=M.getParameter(M.UNPACK_ROW_LENGTH),we=M.getParameter(M.UNPACK_IMAGE_HEIGHT),rn=M.getParameter(M.UNPACK_SKIP_PIXELS),Pe=M.getParameter(M.UNPACK_SKIP_ROWS),Bn=M.getParameter(M.UNPACK_SKIP_IMAGES),ye=Q.isCompressedTexture?Q.mipmaps[K]:Q.image;M.pixelStorei(M.UNPACK_ROW_LENGTH,ye.width),M.pixelStorei(M.UNPACK_IMAGE_HEIGHT,ye.height),M.pixelStorei(M.UNPACK_SKIP_PIXELS,w.min.x),M.pixelStorei(M.UNPACK_SKIP_ROWS,w.min.y),M.pixelStorei(M.UNPACK_SKIP_IMAGES,w.min.z),Q.isDataTexture||Q.isData3DTexture?M.texSubImage3D($t,K,W.x,W.y,W.z,wt,kt,Bt,Wt,Kt,ye.data):Q.isCompressedArrayTexture?(console.warn("THREE.WebGLRenderer.copyTextureToTexture3D: untested support for compressed srcTexture."),M.compressedTexSubImage3D($t,K,W.x,W.y,W.z,wt,kt,Bt,Wt,ye.data)):M.texSubImage3D($t,K,W.x,W.y,W.z,wt,kt,Bt,Wt,Kt,ye),M.pixelStorei(M.UNPACK_ROW_LENGTH,qt),M.pixelStorei(M.UNPACK_IMAGE_HEIGHT,we),M.pixelStorei(M.UNPACK_SKIP_PIXELS,rn),M.pixelStorei(M.UNPACK_SKIP_ROWS,Pe),M.pixelStorei(M.UNPACK_SKIP_IMAGES,Bn),K===0&&nt.generateMipmaps&&M.generateMipmap($t),N.unbindTexture()},this.initTexture=function(w){w.isCubeTexture?S.setTextureCube(w,0):w.isData3DTexture?S.setTexture3D(w,0):w.isDataArrayTexture||w.isCompressedArrayTexture?S.setTexture2DArray(w,0):S.setTexture2D(w,0),N.unbindTexture()},this.resetState=function(){C=0,A=0,D=null,N.reset(),Dt.reset()},typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}get coordinateSystem(){return ti}get outputColorSpace(){return this._outputColorSpace}set outputColorSpace(t){this._outputColorSpace=t;const e=this.getContext();e.drawingBufferColorSpace=t===ol?"display-p3":"srgb",e.unpackColorSpace=ce.workingColorSpace===Mo?"display-p3":"srgb"}get outputEncoding(){return console.warn("THREE.WebGLRenderer: Property .outputEncoding has been removed. Use .outputColorSpace instead."),this.outputColorSpace===Ie?zi:Kd}set outputEncoding(t){console.warn("THREE.WebGLRenderer: Property .outputEncoding has been removed. Use .outputColorSpace instead."),this.outputColorSpace=t===zi?Ie:ri}get useLegacyLights(){return console.warn("THREE.WebGLRenderer: The property .useLegacyLights has been deprecated. Migrate your lighting according to the following guide: https://discourse.threejs.org/t/updates-to-lighting-in-three-js-r155/53733."),this._useLegacyLights}set useLegacyLights(t){console.warn("THREE.WebGLRenderer: The property .useLegacyLights has been deprecated. Migrate your lighting according to the following guide: https://discourse.threejs.org/t/updates-to-lighting-in-three-js-r155/53733."),this._useLegacyLights=t}}class my extends _h{}my.prototype.isWebGL1Renderer=!0;class dl{constructor(t,e=25e-5){this.isFogExp2=!0,this.name="",this.color=new ne(t),this.density=e}clone(){return new dl(this.color,this.density)}toJSON(){return{type:"FogExp2",name:this.name,color:this.color.getHex(),density:this.density}}}class _y extends We{constructor(){super(),this.isScene=!0,this.type="Scene",this.background=null,this.environment=null,this.fog=null,this.backgroundBlurriness=0,this.backgroundIntensity=1,this.overrideMaterial=null,typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("observe",{detail:this}))}copy(t,e){return super.copy(t,e),t.background!==null&&(this.background=t.background.clone()),t.environment!==null&&(this.environment=t.environment.clone()),t.fog!==null&&(this.fog=t.fog.clone()),this.backgroundBlurriness=t.backgroundBlurriness,this.backgroundIntensity=t.backgroundIntensity,t.overrideMaterial!==null&&(this.overrideMaterial=t.overrideMaterial.clone()),this.matrixAutoUpdate=t.matrixAutoUpdate,this}toJSON(t){const e=super.toJSON(t);return this.fog!==null&&(e.object.fog=this.fog.toJSON()),this.backgroundBlurriness>0&&(e.object.backgroundBlurriness=this.backgroundBlurriness),this.backgroundIntensity!==1&&(e.object.backgroundIntensity=this.backgroundIntensity),e}}class hl extends ws{constructor(t){super(),this.isLineBasicMaterial=!0,this.type="LineBasicMaterial",this.color=new ne(16777215),this.map=null,this.linewidth=1,this.linecap="round",this.linejoin="round",this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.linewidth=t.linewidth,this.linecap=t.linecap,this.linejoin=t.linejoin,this.fog=t.fog,this}}const Mu=new z,Su=new z,Eu=new Te,ma=new al,Br=new cr;class gh extends We{constructor(t=new Ve,e=new hl){super(),this.isLine=!0,this.type="Line",this.geometry=t,this.material=e,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[0];for(let s=1,r=e.count;s<r;s++)Mu.fromBufferAttribute(e,s-1),Su.fromBufferAttribute(e,s),i[s]=i[s-1],i[s]+=Mu.distanceTo(Su);t.setAttribute("lineDistance",new ke(i,1))}else console.warn("THREE.Line.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}raycast(t,e){const i=this.geometry,s=this.matrixWorld,r=t.params.Line.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),Br.copy(i.boundingSphere),Br.applyMatrix4(s),Br.radius+=r,t.ray.intersectsSphere(Br)===!1)return;Eu.copy(s).invert(),ma.copy(t.ray).applyMatrix4(Eu);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),l=o*o,c=new z,d=new z,h=new z,p=new z,f=this.isLineSegments?2:1,v=i.index,_=i.attributes.position;if(v!==null){const m=Math.max(0,a.start),E=Math.min(v.count,a.start+a.count);for(let b=m,x=E-1;b<x;b+=f){const C=v.getX(b),A=v.getX(b+1);if(c.fromBufferAttribute(_,C),d.fromBufferAttribute(_,A),ma.distanceSqToSegment(c,d,p,h)>l)continue;p.applyMatrix4(this.matrixWorld);const G=t.ray.origin.distanceTo(p);G<t.near||G>t.far||e.push({distance:G,point:h.clone().applyMatrix4(this.matrixWorld),index:b,face:null,faceIndex:null,object:this})}}else{const m=Math.max(0,a.start),E=Math.min(_.count,a.start+a.count);for(let b=m,x=E-1;b<x;b+=f){if(c.fromBufferAttribute(_,b),d.fromBufferAttribute(_,b+1),ma.distanceSqToSegment(c,d,p,h)>l)continue;p.applyMatrix4(this.matrixWorld);const A=t.ray.origin.distanceTo(p);A<t.near||A>t.far||e.push({distance:A,point:h.clone().applyMatrix4(this.matrixWorld),index:b,face:null,faceIndex:null,object:this})}}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}const Tu=new z,wu=new z;class gy extends gh{constructor(t,e){super(t,e),this.isLineSegments=!0,this.type="LineSegments"}computeLineDistances(){const t=this.geometry;if(t.index===null){const e=t.attributes.position,i=[];for(let s=0,r=e.count;s<r;s+=2)Tu.fromBufferAttribute(e,s),wu.fromBufferAttribute(e,s+1),i[s]=s===0?0:i[s-1],i[s+1]=i[s]+Tu.distanceTo(wu);t.setAttribute("lineDistance",new ke(i,1))}else console.warn("THREE.LineSegments.computeLineDistances(): Computation only possible with non-indexed BufferGeometry.");return this}}class jr extends ws{constructor(t){super(),this.isPointsMaterial=!0,this.type="PointsMaterial",this.color=new ne(16777215),this.map=null,this.alphaMap=null,this.size=1,this.sizeAttenuation=!0,this.fog=!0,this.setValues(t)}copy(t){return super.copy(t),this.color.copy(t.color),this.map=t.map,this.alphaMap=t.alphaMap,this.size=t.size,this.sizeAttenuation=t.sizeAttenuation,this.fog=t.fog,this}}const Au=new Te,za=new al,zr=new cr,Hr=new z;class _a extends We{constructor(t=new Ve,e=new jr){super(),this.isPoints=!0,this.type="Points",this.geometry=t,this.material=e,this.updateMorphTargets()}copy(t,e){return super.copy(t,e),this.material=Array.isArray(t.material)?t.material.slice():t.material,this.geometry=t.geometry,this}raycast(t,e){const i=this.geometry,s=this.matrixWorld,r=t.params.Points.threshold,a=i.drawRange;if(i.boundingSphere===null&&i.computeBoundingSphere(),zr.copy(i.boundingSphere),zr.applyMatrix4(s),zr.radius+=r,t.ray.intersectsSphere(zr)===!1)return;Au.copy(s).invert(),za.copy(t.ray).applyMatrix4(Au);const o=r/((this.scale.x+this.scale.y+this.scale.z)/3),l=o*o,c=i.index,h=i.attributes.position;if(c!==null){const p=Math.max(0,a.start),f=Math.min(c.count,a.start+a.count);for(let v=p,g=f;v<g;v++){const _=c.getX(v);Hr.fromBufferAttribute(h,_),Cu(Hr,_,l,s,t,e,this)}}else{const p=Math.max(0,a.start),f=Math.min(h.count,a.start+a.count);for(let v=p,g=f;v<g;v++)Hr.fromBufferAttribute(h,v),Cu(Hr,v,l,s,t,e,this)}}updateMorphTargets(){const e=this.geometry.morphAttributes,i=Object.keys(e);if(i.length>0){const s=e[i[0]];if(s!==void 0){this.morphTargetInfluences=[],this.morphTargetDictionary={};for(let r=0,a=s.length;r<a;r++){const o=s[r].name||String(r);this.morphTargetInfluences.push(0),this.morphTargetDictionary[o]=r}}}}}function Cu(n,t,e,i,s,r,a){const o=za.distanceSqToPoint(n);if(o<e){const l=new z;za.closestPointToPoint(n,l),l.applyMatrix4(i);const c=s.ray.origin.distanceTo(l);if(c<s.near||c>s.far)return;r.push({distance:c,distanceToRay:Math.sqrt(o),point:l,index:t,face:null,object:a})}}class fl extends Ve{constructor(t=[],e=[],i=1,s=0){super(),this.type="PolyhedronGeometry",this.parameters={vertices:t,indices:e,radius:i,detail:s};const r=[],a=[];o(s),c(i),d(),this.setAttribute("position",new ke(r,3)),this.setAttribute("normal",new ke(r.slice(),3)),this.setAttribute("uv",new ke(a,2)),s===0?this.computeVertexNormals():this.normalizeNormals();function o(E){const b=new z,x=new z,C=new z;for(let A=0;A<e.length;A+=3)f(e[A+0],b),f(e[A+1],x),f(e[A+2],C),l(b,x,C,E)}function l(E,b,x,C){const A=C+1,D=[];for(let G=0;G<=A;G++){D[G]=[];const T=E.clone().lerp(x,G/A),R=b.clone().lerp(x,G/A),J=A-G;for(let rt=0;rt<=J;rt++)rt===0&&G===A?D[G][rt]=T:D[G][rt]=T.clone().lerp(R,rt/J)}for(let G=0;G<A;G++)for(let T=0;T<2*(A-G)-1;T++){const R=Math.floor(T/2);T%2===0?(p(D[G][R+1]),p(D[G+1][R]),p(D[G][R])):(p(D[G][R+1]),p(D[G+1][R+1]),p(D[G+1][R]))}}function c(E){const b=new z;for(let x=0;x<r.length;x+=3)b.x=r[x+0],b.y=r[x+1],b.z=r[x+2],b.normalize().multiplyScalar(E),r[x+0]=b.x,r[x+1]=b.y,r[x+2]=b.z}function d(){const E=new z;for(let b=0;b<r.length;b+=3){E.x=r[b+0],E.y=r[b+1],E.z=r[b+2];const x=_(E)/2/Math.PI+.5,C=m(E)/Math.PI+.5;a.push(x,1-C)}v(),h()}function h(){for(let E=0;E<a.length;E+=6){const b=a[E+0],x=a[E+2],C=a[E+4],A=Math.max(b,x,C),D=Math.min(b,x,C);A>.9&&D<.1&&(b<.2&&(a[E+0]+=1),x<.2&&(a[E+2]+=1),C<.2&&(a[E+4]+=1))}}function p(E){r.push(E.x,E.y,E.z)}function f(E,b){const x=E*3;b.x=t[x+0],b.y=t[x+1],b.z=t[x+2]}function v(){const E=new z,b=new z,x=new z,C=new z,A=new re,D=new re,G=new re;for(let T=0,R=0;T<r.length;T+=9,R+=6){E.set(r[T+0],r[T+1],r[T+2]),b.set(r[T+3],r[T+4],r[T+5]),x.set(r[T+6],r[T+7],r[T+8]),A.set(a[R+0],a[R+1]),D.set(a[R+2],a[R+3]),G.set(a[R+4],a[R+5]),C.copy(E).add(b).add(x).divideScalar(3);const J=_(C);g(A,R+0,E,J),g(D,R+2,b,J),g(G,R+4,x,J)}}function g(E,b,x,C){C<0&&E.x===1&&(a[b]=E.x-1),x.x===0&&x.z===0&&(a[b]=C/2/Math.PI+.5)}function _(E){return Math.atan2(E.z,-E.x)}function m(E){return Math.atan2(-E.y,Math.sqrt(E.x*E.x+E.z*E.z))}}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new fl(t.vertices,t.indices,t.radius,t.details)}}class pl extends fl{constructor(t=1,e=0){const i=(1+Math.sqrt(5))/2,s=[-1,i,0,1,i,0,-1,-i,0,1,-i,0,0,-1,i,0,1,i,0,-1,-i,0,1,-i,i,0,-1,i,0,1,-i,0,-1,-i,0,1],r=[0,11,5,0,5,1,0,1,7,0,7,10,0,10,11,1,5,9,5,11,4,11,10,2,10,7,6,7,1,8,3,9,4,3,4,2,3,2,6,3,6,8,3,8,9,4,9,5,2,4,11,6,2,10,8,6,7,9,8,1];super(s,r,t,e),this.type="IcosahedronGeometry",this.parameters={radius:t,detail:e}}static fromJSON(t){return new pl(t.radius,t.detail)}}class js extends Ve{constructor(t=1,e=32,i=16,s=0,r=Math.PI*2,a=0,o=Math.PI){super(),this.type="SphereGeometry",this.parameters={radius:t,widthSegments:e,heightSegments:i,phiStart:s,phiLength:r,thetaStart:a,thetaLength:o},e=Math.max(3,Math.floor(e)),i=Math.max(2,Math.floor(i));const l=Math.min(a+o,Math.PI);let c=0;const d=[],h=new z,p=new z,f=[],v=[],g=[],_=[];for(let m=0;m<=i;m++){const E=[],b=m/i;let x=0;m===0&&a===0?x=.5/e:m===i&&l===Math.PI&&(x=-.5/e);for(let C=0;C<=e;C++){const A=C/e;h.x=-t*Math.cos(s+A*r)*Math.sin(a+b*o),h.y=t*Math.cos(a+b*o),h.z=t*Math.sin(s+A*r)*Math.sin(a+b*o),v.push(h.x,h.y,h.z),p.copy(h).normalize(),g.push(p.x,p.y,p.z),_.push(A+x,1-b),E.push(c++)}d.push(E)}for(let m=0;m<i;m++)for(let E=0;E<e;E++){const b=d[m][E+1],x=d[m][E],C=d[m+1][E],A=d[m+1][E+1];(m!==0||a>0)&&f.push(b,x,A),(m!==i-1||l<Math.PI)&&f.push(x,C,A)}this.setIndex(f),this.setAttribute("position",new ke(v,3)),this.setAttribute("normal",new ke(g,3)),this.setAttribute("uv",new ke(_,2))}copy(t){return super.copy(t),this.parameters=Object.assign({},t.parameters),this}static fromJSON(t){return new js(t.radius,t.widthSegments,t.heightSegments,t.phiStart,t.phiLength,t.thetaStart,t.thetaLength)}}class vy extends We{constructor(t,e=1){super(),this.isLight=!0,this.type="Light",this.color=new ne(t),this.intensity=e}dispose(){}copy(t,e){return super.copy(t,e),this.color.copy(t.color),this.intensity=t.intensity,this}toJSON(t){const e=super.toJSON(t);return e.object.color=this.color.getHex(),e.object.intensity=this.intensity,this.groundColor!==void 0&&(e.object.groundColor=this.groundColor.getHex()),this.distance!==void 0&&(e.object.distance=this.distance),this.angle!==void 0&&(e.object.angle=this.angle),this.decay!==void 0&&(e.object.decay=this.decay),this.penumbra!==void 0&&(e.object.penumbra=this.penumbra),this.shadow!==void 0&&(e.object.shadow=this.shadow.toJSON()),e}}const ga=new Te,Ru=new z,Lu=new z;class xy{constructor(t){this.camera=t,this.bias=0,this.normalBias=0,this.radius=1,this.blurSamples=8,this.mapSize=new re(512,512),this.map=null,this.mapPass=null,this.matrix=new Te,this.autoUpdate=!0,this.needsUpdate=!1,this._frustum=new ll,this._frameExtents=new re(1,1),this._viewportCount=1,this._viewports=[new xe(0,0,1,1)]}getViewportCount(){return this._viewportCount}getFrustum(){return this._frustum}updateMatrices(t){const e=this.camera,i=this.matrix;Ru.setFromMatrixPosition(t.matrixWorld),e.position.copy(Ru),Lu.setFromMatrixPosition(t.target.matrixWorld),e.lookAt(Lu),e.updateMatrixWorld(),ga.multiplyMatrices(e.projectionMatrix,e.matrixWorldInverse),this._frustum.setFromProjectionMatrix(ga),i.set(.5,0,0,.5,0,.5,0,.5,0,0,.5,.5,0,0,0,1),i.multiply(ga)}getViewport(t){return this._viewports[t]}getFrameExtents(){return this._frameExtents}dispose(){this.map&&this.map.dispose(),this.mapPass&&this.mapPass.dispose()}copy(t){return this.camera=t.camera.clone(),this.bias=t.bias,this.radius=t.radius,this.mapSize.copy(t.mapSize),this}clone(){return new this.constructor().copy(this)}toJSON(){const t={};return this.bias!==0&&(t.bias=this.bias),this.normalBias!==0&&(t.normalBias=this.normalBias),this.radius!==1&&(t.radius=this.radius),(this.mapSize.x!==512||this.mapSize.y!==512)&&(t.mapSize=this.mapSize.toArray()),t.camera=this.camera.toJSON(!1).object,delete t.camera.matrix,t}}const Pu=new Te,ks=new z,va=new z;class yy extends xy{constructor(){super(new ln(90,1,.5,500)),this.isPointLightShadow=!0,this._frameExtents=new re(4,2),this._viewportCount=6,this._viewports=[new xe(2,1,1,1),new xe(0,1,1,1),new xe(3,1,1,1),new xe(1,1,1,1),new xe(3,0,1,1),new xe(1,0,1,1)],this._cubeDirections=[new z(1,0,0),new z(-1,0,0),new z(0,0,1),new z(0,0,-1),new z(0,1,0),new z(0,-1,0)],this._cubeUps=[new z(0,1,0),new z(0,1,0),new z(0,1,0),new z(0,1,0),new z(0,0,1),new z(0,0,-1)]}updateMatrices(t,e=0){const i=this.camera,s=this.matrix,r=t.distance||i.far;r!==i.far&&(i.far=r,i.updateProjectionMatrix()),ks.setFromMatrixPosition(t.matrixWorld),i.position.copy(ks),va.copy(i.position),va.add(this._cubeDirections[e]),i.up.copy(this._cubeUps[e]),i.lookAt(va),i.updateMatrixWorld(),s.makeTranslation(-ks.x,-ks.y,-ks.z),Pu.multiplyMatrices(i.projectionMatrix,i.matrixWorldInverse),this._frustum.setFromProjectionMatrix(Pu)}}class by extends vy{constructor(t,e,i=0,s=2){super(t,e),this.isPointLight=!0,this.type="PointLight",this.distance=i,this.decay=s,this.shadow=new yy}get power(){return this.intensity*4*Math.PI}set power(t){this.intensity=t/(4*Math.PI)}dispose(){this.shadow.dispose()}copy(t,e){return super.copy(t,e),this.distance=t.distance,this.decay=t.decay,this.shadow=t.shadow.clone(),this}}class My{constructor(t=!0){this.autoStart=t,this.startTime=0,this.oldTime=0,this.elapsedTime=0,this.running=!1}start(){this.startTime=Du(),this.oldTime=this.startTime,this.elapsedTime=0,this.running=!0}stop(){this.getElapsedTime(),this.running=!1,this.autoStart=!1}getElapsedTime(){return this.getDelta(),this.elapsedTime}getDelta(){let t=0;if(this.autoStart&&!this.running)return this.start(),0;if(this.running){const e=Du();t=(e-this.oldTime)/1e3,this.oldTime=e,this.elapsedTime+=t}return t}}function Du(){return(typeof performance>"u"?Date:performance).now()}class Sy extends gy{constructor(t=10,e=10,i=4473924,s=8947848){i=new ne(i),s=new ne(s);const r=e/2,a=t/e,o=t/2,l=[],c=[];for(let p=0,f=0,v=-o;p<=e;p++,v+=a){l.push(-o,0,v,o,0,v),l.push(v,0,-o,v,0,o);const g=p===r?i:s;g.toArray(c,f),f+=3,g.toArray(c,f),f+=3,g.toArray(c,f),f+=3,g.toArray(c,f),f+=3}const d=new Ve;d.setAttribute("position",new ke(l,3)),d.setAttribute("color",new ke(c,3));const h=new hl({vertexColors:!0,toneMapped:!1});super(d,h),this.type="GridHelper"}dispose(){this.geometry.dispose(),this.material.dispose()}}typeof __THREE_DEVTOOLS__<"u"&&__THREE_DEVTOOLS__.dispatchEvent(new CustomEvent("register",{detail:{revision:sl}}));typeof window<"u"&&(window.__THREE__?console.warn("WARNING: Multiple instances of Three.js being imported."):window.__THREE__=sl);const Me=(n,t)=>{const e=n.__vccOpts||n;for(const[i,s]of t)e[i]=s;return e},Ey={class:"ov-layout"},Ty={class:"core-stage-overlay"},wy={class:"core-readout"},Ay={class:"core-readout-item"},Cy={class:"val"},Ry={class:"core-readout-item"},Ly={class:"val"},Py={class:"core-readout-item"},Dy={class:"val"},Uy={class:"core-name"},Iy={class:"cn-sub"},Ny={class:"ov-right"},ky={class:"glass ov-panel"},Fy={class:"emo-head"},Oy={class:"emo-name"},By={class:"emo-val"},zy={class:"emo-track"},Hy={class:"glass ov-panel events-panel"},Vy={class:"event-stream"},Gy={class:"event-time"},Wy={class:"event-text"},$y={class:"hl"},Xy={key:0,class:"event-empty"},qy={__name:"CoreView",setup(n){const t=[{key:"joy",name:"愉悦度",color:"#F5A623"},{key:"curious",name:"好奇心",color:"#FFC85C"},{key:"empathy",name:"共情强度",color:"#B87A12"},{key:"stress",name:"压力水平",color:"#E5484D"}],e={Neutral:"CALM · 平静",Happy:"BRIGHT · 开心",Excited:"CHARGED · 兴奋",Sad:"HEAVY · 低落",Thinking:"DRIFTING · 出神",Surprised:"ALERT · 惊讶",Angry:"STORMY · 烦躁",Shy:"SOFT · 害羞",Worried:"UNEASY · 不踏实",Tired:"DIM · 疲惫",Like:"RESONANT · 心动"},i={sensation:"感官",inner:"内心",acted:"行动",digested:"沉淀",recall:"联想"},s={joy:.6,curious:.45,empathy:.5,stress:.2,target:{joy:.6,curious:.45,empathy:.5,stress:.2}},r=ir({joy:.6,curious:.45,empathy:.5,stress:.2,coherence:0,resonance:0,entropy:0}),a=yt("CALM · 平静"),o=yt([]),l=yt(null),c=N=>Math.round((N||0)*100)+"%",d=N=>N==null?"--":Number(N).toFixed(2);async function h(){try{const N=await Ot("/api/emotion/core");s.target.joy=f(N.joy,s.target.joy),s.target.curious=f(N.curiosity,s.target.curious),s.target.empathy=f(N.empathy,s.target.empathy),s.target.stress=f(N.stress,s.target.stress),r.coherence=d(N.coherence),r.resonance=d(N.resonance),r.entropy=d(N.entropy),a.value=e[N.state]||String(N.state||"CALM").toUpperCase()}catch{}}async function p(){try{const N=await Ot("/api/mind/now");o.value=(N.recent_stream||[]).slice(0,12).map(V=>({time:v(V.time),kind:V.recall?"联想":i[V.kind]||V.kind,content:V.content}))}catch{}}function f(N,V){const j=Number(N);return Number.isFinite(j)?Math.min(Math.max(j,0),1):V}function v(N){return N?new Date(N*1e3).toLocaleTimeString("zh-CN",{hour12:!1,hour:"2-digit",minute:"2-digit"}):"--:--"}let g,_,m,E,b,x=0,C=0,A=0,D=0,G=null,T,R,J,rt,mt,O,Z,tt=null,st=[],et=null,ot=[],ut=null,pt=0,ft=1.437,it=!1,ht=0,bt=0,Lt=!1;const Pt=(N,V,j)=>Math.min(Math.max(N,V),j);function Ht(){return document.documentElement.getAttribute("data-theme")==="light"}function Gt(){E=document.createElement("canvas"),E.id="core-webgl",document.body.appendChild(E),g=new _h({canvas:E,antialias:!0,alpha:!0}),g.setPixelRatio(Math.min(window.devicePixelRatio,2)),g.setSize(window.innerWidth,window.innerHeight),g.setClearColor(0,0),_=new _y,Nt(),m=new ln(55,window.innerWidth/window.innerHeight,.1,200),b=new My,T=new Bs,_.add(T),R=new _n(new js(1,64,64),new as({color:16098851,transparent:!0,opacity:.15,blending:jn})),T.add(R),J=new _n(new js(1.35,48,48),new as({color:16098851,transparent:!0,opacity:.06,blending:jn,side:tn})),T.add(J);const N=new pl(1.7,4);rt=new _n(N,new as({color:16098851,wireframe:!0,transparent:!0,opacity:.35,blending:jn})),T.add(rt),tt=N.attributes.position.array.slice();const V=400,j=new Float32Array(V*3),S=new Float32Array(V*3);for(let $=0;$<V;$++){const Rt=Math.random()*1.6,At=Math.random()*Math.PI*2,Tt=Math.acos(Math.random()*2-1);j[$*3]=Rt*Math.sin(Tt)*Math.cos(At),j[$*3+1]=Rt*Math.sin(Tt)*Math.sin(At),j[$*3+2]=Rt*Math.cos(Tt);const Ct=Math.random();S[$*3]=.96,S[$*3+1]=.65+Ct*.2,S[$*3+2]=.15+Ct*.2}const y=new Ve;y.setAttribute("position",new Ne(j,3)),y.setAttribute("color",new Ne(S,3)),mt=new _a(y,new jr({size:.03,vertexColors:!0,transparent:!0,opacity:.9,blending:jn,depthWrite:!1})),T.add(mt),O=new _n(new js(.15,16,16),new as({color:16769184,transparent:!0,opacity:.9})),T.add(O),Z=new by(16098851,2,20,2),T.add(Z),st=[],t.forEach(($,Rt)=>{const At=2.8+Rt*.55,Tt=128,Ct=new Float32Array(Tt*3);for(let Yt=0;Yt<Tt;Yt++){const le=Yt/Tt*Math.PI*2;Ct[Yt*3]=Math.cos(le)*At,Ct[Yt*3+1]=0,Ct[Yt*3+2]=Math.sin(le)*At}const xt=new Ve;xt.setAttribute("position",new Ne(Ct,3));const Dt=new gh(xt,new hl({color:new ne($.color),transparent:!0,opacity:.5,blending:jn}));Dt.rotation.x=Rt*.35-.5,Dt.rotation.y=Rt*.5,Dt.rotation.z=Rt*.2,_.add(Dt),st.push({mesh:Dt,baseRotation:{x:Dt.rotation.x,y:Dt.rotation.y,z:Dt.rotation.z},emoKey:$.key,phase:Rt*1.2,speed:.3+Rt*.15})});const U=800,q=new Float32Array(U*3),H=new Float32Array(U*3);ot=[];for(let $=0;$<U;$++){const Rt=Math.floor(Math.random()*4);ot.push({emoIdx:Rt,startAngle:Math.random()*Math.PI*2,radius:2.8+Rt*.55,t:Math.random(),speed:.15+Math.random()*.2,offset:Math.random()*Math.PI*2});const At=new ne(t[Rt].color);H[$*3]=At.r,H[$*3+1]=At.g,H[$*3+2]=At.b}const X=new Ve;X.setAttribute("position",new Ne(q,3)),X.setAttribute("color",new Ne(H,3)),et=new _a(X,new jr({size:.045,vertexColors:!0,transparent:!0,opacity:.75,blending:jn,depthWrite:!1,sizeAttenuation:!0})),_.add(et);const lt=1200,at=new Float32Array(lt*3),dt=new Float32Array(lt*3);for(let $=0;$<lt;$++){const Rt=30+Math.random()*50,At=Math.random()*Math.PI*2,Tt=Math.acos(Math.random()*2-1);at[$*3]=Rt*Math.sin(Tt)*Math.cos(At),at[$*3+1]=Rt*Math.sin(Tt)*Math.sin(At)*.6,at[$*3+2]=Rt*Math.cos(Tt)-20;const Ct=Math.random();Ct<.7?(dt[$*3]=.96,dt[$*3+1]=.65,dt[$*3+2]=.15):Ct<.9?(dt[$*3]=.6,dt[$*3+1]=.7,dt[$*3+2]=.9):(dt[$*3]=.9,dt[$*3+1]=.9,dt[$*3+2]=.95)}const ct=new Ve;ct.setAttribute("position",new Ne(at,3)),ct.setAttribute("color",new Ne(dt,3)),ut=new _a(ct,new jr({size:.15,vertexColors:!0,transparent:!0,opacity:.7,blending:jn,depthWrite:!1,sizeAttenuation:!0})),_.add(ut);const _t=new Sy(80,40,16098851,1710624);_t.position.y=-4,_t.material.opacity=.08,_t.material.transparent=!0,_.add(_t)}function Nt(){_&&(_.fog=new dl(Ht()?16183522:657933,.025))}function Zt(){!g||!m||(m.aspect=window.innerWidth/window.innerHeight,m.updateProjectionMatrix(),g.setSize(window.innerWidth,window.innerHeight),g.setPixelRatio(Math.min(window.devicePixelRatio,2)))}function M(){if(Lt)return;x=requestAnimationFrame(M);const N=Math.min(b.getDelta(),.1),V=b.elapsedTime;for(const ct of Object.keys(s.target))s[ct]+=(s.target[ct]-s[ct])*.02;const j=(s.joy+s.curious+s.empathy)/3-s.stress*.3,S=1+Math.sin(V*(1.5+j*2))*.04*Math.max(j,.1);T.scale.setScalar(S),T.rotation.y+=N*.15,T.rotation.x+=N*.05;const y=rt.geometry.attributes.position,U=y.array,q=.08*(.5+s.stress);for(let ct=0;ct<U.length;ct+=3){const _t=tt[ct],$=tt[ct+1],Rt=tt[ct+2],Tt=1+Math.sin(_t*3+V*2)*Math.cos($*3+V*1.7)*Math.sin(Rt*3+V*1.3)*q;U[ct]=_t*Tt,U[ct+1]=$*Tt,U[ct+2]=Rt*Tt}y.needsUpdate=!0;const H=new ne(12089874).lerp(new ne(16762972),s.joy);R.material.color.copy(H),J.material.color.copy(H),rt.material.color.copy(H),Z.color.copy(H),Z.intensity=1.5+s.joy*1.5,mt.material.opacity=.5+(1-s.stress)*.5,mt.rotation.y+=.001,O.scale.setScalar(1+Math.sin(V*(2+j*3))*.2);for(const ct of st){const _t=s[ct.emoKey]??.5;ct.mesh.rotation.y=ct.baseRotation.y+V*ct.speed*(.3+_t),ct.mesh.rotation.x=ct.baseRotation.x+Math.sin(V*.4+ct.phase)*.15;const $=.9+_t*.25;ct.mesh.scale.set($,$,$),ct.mesh.material.opacity=.15+_t*.6}const X=et.geometry.attributes.position.array;for(let ct=0;ct<ot.length;ct++){const _t=ot[ct];_t.t+=N*_t.speed,_t.t>1&&(_t.t-=1);const $=s[_t.emoKey??t[_t.emoIdx].key]??.5,Rt=_t.t*(.5+$*.8),At=1-Math.pow(1-Rt,2),Tt=_t.startAngle+V*.3;X[ct*3]=Math.cos(Tt)*_t.radius*At,X[ct*3+1]=Math.sin(Rt*Math.PI)*.5*(1+Math.sin(V+_t.offset)*.3),X[ct*3+2]=Math.sin(Tt)*_t.radius*At}et.geometry.attributes.position.needsUpdate=!0,ut.rotation.y+=N*.005;const lt=9,at=pt+Math.sin(V*.1)*.06,dt=Pt(ft+Math.cos(V*.15)*.04,.3,Math.PI-.3);m.position.set(lt*Math.sin(dt)*Math.sin(at),lt*Math.cos(dt),lt*Math.sin(dt)*Math.cos(at)),m.lookAt(0,0,0),g.render(_,m)}function k(N){N.pointerType!=="mouse"&&N.pointerType!=="touch"||N.pointerType==="mouse"&&N.button!==0||(it=!0,ht=N.clientX,bt=N.clientY,N.currentTarget.setPointerCapture?.(N.pointerId))}function B(N){if(!it)return;const V=N.clientX-ht,j=N.clientY-bt;ht=N.clientX,bt=N.clientY,pt-=V*.006,ft=Pt(ft-j*.006,.35,Math.PI-.35)}function Y(){it=!1}return ve(async()=>{Gt(),Zt(),M();const N=l.value;N?.addEventListener("pointerdown",k),N?.addEventListener("pointermove",B),N?.addEventListener("pointerup",Y),N?.addEventListener("pointercancel",Y),window.addEventListener("resize",Zt),G=new MutationObserver(Nt),G.observe(document.documentElement,{attributes:!0,attributeFilter:["data-theme"]}),await h(),p(),A=setInterval(h,1e4),D=setInterval(p,15e3),C=setInterval(()=>{r.joy=s.joy,r.curious=s.curious,r.empathy=s.empathy,r.stress=s.stress},100)}),An(()=>{Lt=!0,cancelAnimationFrame(x),clearInterval(C),clearInterval(A),clearInterval(D);const N=l.value;N?.removeEventListener("pointerdown",k),N?.removeEventListener("pointermove",B),N?.removeEventListener("pointerup",Y),N?.removeEventListener("pointercancel",Y),window.removeEventListener("resize",Zt),G?.disconnect(),_&&_.traverse(V=>{V.geometry?.dispose?.(),Array.isArray(V.material)?V.material.forEach(j=>j.dispose?.()):V.material?.dispose?.()}),g?.dispose(),E?.remove(),g=_=m=E=null}),(N,V)=>(L(),P("div",Ey,[u("div",{class:"core-stage",ref_key:"stageEl",ref:l},[u("div",Ty,[V[3]||(V[3]=u("div",{class:"core-stage-label"},"LIVE EMOTION CORE",-1)),u("div",wy,[u("div",Ay,[V[0]||(V[0]=u("span",{class:"lbl"},"COHERENCE",-1)),u("span",Cy,I(r.coherence),1)]),u("div",Ry,[V[1]||(V[1]=u("span",{class:"lbl"},"RESONANCE",-1)),u("span",Ly,I(r.resonance),1)]),u("div",Py,[V[2]||(V[2]=u("span",{class:"lbl"},"ENTROPY",-1)),u("span",Dy,I(r.entropy),1)])])]),u("div",Uy,[V[4]||(V[4]=u("div",{class:"cn-title"},"洛玖",-1)),u("div",Iy,I(a.value),1)])],512),u("div",Ny,[u("div",ky,[V[5]||(V[5]=u("div",{class:"panel-title"},[ue(" 情绪频谱 "),u("span",{class:"pt-right"},"实时")],-1)),(L(),P(Et,null,It(t,j=>u("div",{key:j.key,class:"emo-row"},[u("div",Fy,[u("span",Oy,[u("span",{class:"color-dot",style:se({background:j.color,boxShadow:"0 0 6px "+j.color})},null,4),ue(" "+I(j.name),1)]),u("span",By,I(c(r[j.key])),1)]),u("div",zy,[u("div",{class:"emo-fill",style:se({width:c(r[j.key]),background:j.color,color:j.color})},null,4)])])),64))]),u("div",Hy,[V[6]||(V[6]=u("div",{class:"panel-title"},[ue(" 核心事件流 "),u("span",{class:"pt-right"},"LIVE")],-1)),u("div",Vy,[(L(!0),P(Et,null,It(o.value,(j,S)=>(L(),P("div",{key:S,class:"event-line"},[u("span",Gy,I(j.time),1),u("span",Wy,[u("span",$y,I(j.kind),1),ue(" "+I(j.content),1)])]))),128)),o.value.length?Ut("",!0):(L(),P("div",Xy,"暂无近期事件 · 静默中"))])])])]))}},jy=Me(qy,[["__scopeId","data-v-9b581e46"]]),Yy={class:"state-row"},Ky={key:0,class:"state-time"},Zy={key:1,class:"state-diary"},Jy={class:"diary-num"},Qy={class:"card-row"},tb={class:"card half"},eb={key:0,class:"signal-list"},nb={class:"signal-name"},ib={class:"signal-gauge"},sb={class:"signal-level"},rb={key:1,class:"empty"},ob={class:"card half"},ab={key:0,class:"count-chip"},lb={key:0,class:"loop-list"},cb={class:"loop-reason"},ub={class:"loop-meta"},db={class:"loop-kind"},hb={key:0},fb={key:1,class:"empty"},pb={class:"card"},mb={key:0,class:"stream-list"},_b={class:"stream-body"},gb={class:"stream-content"},vb={key:0,class:"recall-state"},xb={class:"stream-time"},yb={key:1,class:"empty"},bb=3e3,Mb={__name:"MindView",setup(n){const t=yt(null);let e=null;const i=ee(()=>t.value?.body||[]),s=ee(()=>t.value?.loops||[]),r=ee(()=>t.value?.recent_stream||[]),a={sensation:"感官",inner:"内心",acted:"行动",digested:"沉淀"},o={idle:"走神",digest:"睡前整理"};function l(_){return a[_]||_}function c(_){return o[_]||_}function d(_){return Math.max(0,Math.min(1,_))*100+"%"}function h(_){return Number(_??0).toFixed(2)}function p(_){return _<.34?"var(--info)":_<.67?"var(--warning)":"var(--primary)"}function f(_){const m=_-Math.floor(Date.now()/1e3);return m<=0?"随时":m<60?m+" 秒后":m<3600?Math.floor(m/60)+" 分钟后":m<86400?Math.floor(m/3600)+" 小时后":Math.floor(m/86400)+" 天后"}function v(_){return _?new Date(_*1e3).toLocaleTimeString("zh-CN",{hour12:!1,hour:"2-digit",minute:"2-digit"}):""}async function g(){try{t.value=await Ot("/api/mind/now")}catch{}}return ve(()=>{g(),e=setInterval(g,bb),window.addEventListener("refresh-all",g)}),An(()=>{clearInterval(e),window.removeEventListener("refresh-all",g)}),(_,m)=>(L(),P("div",null,[u("div",Yy,[u("div",{class:ie(["state-chip",t.value?.night?"is-night":"is-awake"])},[m[0]||(m[0]=u("span",{class:"state-dot"},null,-1)),ue(I(t.value?.night?"夜间 · 睡眠中":"清醒"),1)],2),t.value?.time?(L(),P("div",Ky,I(t.value.time),1)):Ut("",!0),t.value?(L(),P("div",Zy,[u("span",Jy,I(t.value.diary_today),1),m[1]||(m[1]=u("span",{class:"diary-label"},"今日日记",-1))])):Ut("",!0)]),u("div",Qy,[u("div",tb,[m[2]||(m[2]=u("h3",null,"身体信号",-1)),i.value.length?(L(),P("div",eb,[(L(!0),P(Et,null,It(i.value,E=>(L(),P("div",{key:E.name,class:"signal-row"},[u("span",nb,I(E.name),1),u("div",ib,[u("div",{class:"signal-fill",style:se({width:d(E.level),background:p(E.level)})},null,4)]),u("span",sb,I(h(E.level)),1)]))),128))])):(L(),P("div",rb,"暂无信号"))]),u("div",ob,[u("h3",null,[m[3]||(m[3]=ue("活跃心事 ",-1)),s.value.length?(L(),P("span",ab,I(s.value.length),1)):Ut("",!0)]),s.value.length?(L(),P("div",lb,[(L(!0),P(Et,null,It(s.value,E=>(L(),P("div",{key:E.id,class:"loop-item"},[u("div",cb,I(E.reason),1),u("div",ub,[u("span",db,I(c(E.kind)),1),u("span",null,"到期 "+I(f(E.due_at)),1),E.about_user?(L(),P("span",hb,"关于 "+I(E.about_user),1)):Ut("",!0)])]))),128))])):(L(),P("div",fb,"没有惦记的事"))])]),u("div",pb,[m[4]||(m[4]=u("h3",null,"最近意识流",-1)),r.value.length?(L(),P("div",mb,[(L(!0),P(Et,null,It(r.value,(E,b)=>(L(),P("div",{key:b,class:ie(["stream-item",["kind-"+E.kind,{"is-recall":E.recall}]])},[u("span",{class:ie(["stream-badge",E.recall?"badge-recall":"badge-"+E.kind])},I(E.recall?"联想":l(E.kind)),3),u("div",_b,[u("div",gb,[ue(I(E.content),1),E.recall?(L(),P("span",vb,I(E.recall.completed?"已自动处理":"待处理"),1)):Ut("",!0)]),u("div",xb,I(v(E.time)),1)])],2))),128))])):(L(),P("div",yb,"此刻尚无意识流动"))])]))}},Sb=Me(Mb,[["__scopeId","data-v-3ff49ea2"]]),Eb={class:"card"},Tb={class:"date-bar"},wb={key:0,class:"date-chips"},Ab=["onClick"],Cb={key:1,class:"empty-inline"},Rb={class:"card"},Lb={class:"filter-bar"},Pb={key:0,class:"count-chip"},Db={class:"kind-filters"},Ub=["onClick"],Ib={key:0,class:"empty"},Nb={key:1,class:"timeline"},kb={class:"tl-rail"},Fb={key:0,class:"tl-line"},Ob={class:"tl-body"},Bb={class:"tl-head"},zb={class:"tl-time"},Hb={key:0,class:"tl-about"},Vb={key:1,class:"recall-source"},Gb={key:2,class:"recall-status"},Wb={class:"tl-content"},$b={key:2,class:"empty"},Xb={__name:"StreamView",setup(n){const t=yt([]),e=yt(""),i=yt([]),s=yt("all"),r=yt(!1),a=[{value:"sensation",label:"感官"},{value:"inner",label:"内心"},{value:"acted",label:"行动"},{value:"digested",label:"沉淀"},{value:"recall",label:"联想"}],o=Object.fromEntries(a.map(m=>[m.value,m.label]));function l(m){return o[m]||m}function c(m){return!!m.recall}function d(m){return c(m)?"recall":m.kind}const h=ee(()=>s.value==="all"?i.value:s.value==="recall"?i.value.filter(c):i.value.filter(m=>m.kind===s.value&&!c(m)));function p(m){return m?new Date(m*1e3).toLocaleString("zh-CN",{hour12:!1,month:"2-digit",day:"2-digit",hour:"2-digit",minute:"2-digit"}):""}async function f(){try{const m=await Ot("/api/mind/stream");t.value=(m.dates||[]).slice().sort().reverse(),!e.value&&t.value.length&&await g(t.value[0])}catch{}}async function v(m){r.value=!0;try{const E=await Ot(`/api/mind/stream/${encodeURIComponent(m)}`);i.value=(E.events||[]).slice().sort((b,x)=>x.time-b.time)}catch{i.value=[]}r.value=!1}async function g(m){e.value=m,s.value="all",await v(m)}let _;return ve(async()=>{await f(),_=window.setInterval(()=>{e.value&&v(e.value)},5e3)}),An(()=>window.clearInterval(_)),(m,E)=>(L(),P("div",null,[u("div",Eb,[u("div",Tb,[E[1]||(E[1]=u("h3",null,"日期",-1)),t.value.length?(L(),P("div",wb,[(L(!0),P(Et,null,It(t.value,b=>(L(),P("button",{key:b,class:ie(["date-chip",{active:b===e.value}]),onClick:x=>g(b)},I(b),11,Ab))),128))])):(L(),P("div",Cb,"暂无记录"))])]),u("div",Rb,[u("div",Lb,[u("h3",null,[E[2]||(E[2]=ue("事件 ",-1)),h.value.length?(L(),P("span",Pb,I(h.value.length),1)):Ut("",!0)]),u("div",Db,[u("button",{class:ie(["kind-btn",{active:s.value==="all"}]),onClick:E[0]||(E[0]=b=>s.value="all")},"全部",2),(L(),P(Et,null,It(a,b=>u("button",{key:b.value,class:ie(["kind-btn",{active:s.value===b.value}]),onClick:x=>s.value=b.value},I(b.label),11,Ub)),64))])]),r.value?(L(),P("div",Ib,"读取中…")):h.value.length?(L(),P("div",Nb,[(L(!0),P(Et,null,It(h.value,(b,x)=>(L(),P("div",{key:x,class:ie(["tl-item",["kind-"+b.kind,{"is-recall":c(b),"is-recall-completed":b.recall?.completed}]])},[u("div",kb,[E[3]||(E[3]=u("span",{class:"tl-dot"},null,-1)),x<h.value.length-1?(L(),P("span",Fb)):Ut("",!0)]),u("div",Ob,[u("div",Bb,[u("span",{class:ie(["tl-badge","badge-"+d(b)])},I(l(d(b))),3),u("span",zb,I(p(b.time)),1),b.about?(L(),P("span",Hb,"关于 "+I(b.about),1)):Ut("",!0),b.recall?(L(),P("span",Vb,I(b.recall.source),1)):Ut("",!0),b.recall?(L(),P("span",Gb,I(b.recall.completed?"已自动处理":"待处理"),1)):Ut("",!0)]),u("div",Wb,I(b.content),1)])],2))),128))])):(L(),P("div",$b,"这一天她没有留下事件"))])]))}},qb=Me(Xb,[["__scopeId","data-v-3f47feb5"]]),jb={class:"tab-bar"},Yb={class:"tab-count"},Kb={class:"tab-count"},Zb={key:0,class:"diary-list"},Jb={class:"diary-head"},Qb={class:"diary-date"},t1={key:0,class:"diary-about"},e1={class:"diary-content"},n1={key:0,class:"diary-feeling"},i1={key:1,class:"card"},s1={key:0,class:"person-grid"},r1={class:"person-head"},o1={class:"person-name"},a1={class:"person-uid"},l1={key:0,class:"person-address"},c1={class:"person-fields"},u1={key:0,class:"field"},d1={key:1,class:"field"},h1={key:2,class:"field"},f1={key:1,class:"person-section"},p1={class:"say-list"},m1={key:2,class:"person-section"},_1={class:"memory-list"},g1={key:1,class:"card"},v1={__name:"MindMemoryView",setup(n){const t=yt("diary"),e=yt([]),i=yt([]);async function s(){try{e.value=(await Ot("/api/mind/diary")).entries||[]}catch{}try{i.value=(await Ot("/api/mind/persons")).persons||[]}catch{}}return ve(()=>{s(),window.addEventListener("refresh-all",s)}),(r,a)=>(L(),P("div",null,[u("div",jb,[u("button",{class:ie(["tab-btn",{active:t.value==="diary"}]),onClick:a[0]||(a[0]=o=>t.value="diary")},[a[2]||(a[2]=ue(" 日记 ",-1)),u("span",Yb,I(e.value.length),1)],2),u("button",{class:ie(["tab-btn",{active:t.value==="persons"}]),onClick:a[1]||(a[1]=o=>t.value="persons")},[a[3]||(a[3]=ue(" 人物档案 ",-1)),u("span",Kb,I(i.value.length),1)],2)]),t.value==="diary"?(L(),P(Et,{key:0},[e.value.length?(L(),P("div",Zb,[(L(!0),P(Et,null,It(e.value,o=>(L(),P("div",{key:o.id,class:"card diary-card"},[u("div",Jb,[u("span",Qb,I(o.date),1),o.about?(L(),P("span",t1,"关于 "+I(o.about),1)):Ut("",!0)]),u("div",e1,I(o.content),1),o.feeling?(L(),P("div",n1,I(o.feeling),1)):Ut("",!0)]))),128))])):(L(),P("div",i1,[...a[4]||(a[4]=[u("div",{class:"empty"},"还没有日记",-1)])]))],64)):(L(),P(Et,{key:1},[i.value.length?(L(),P("div",s1,[(L(!0),P(Et,null,It(i.value,o=>(L(),P("div",{key:o.user_id,class:"card person-card"},[u("div",r1,[u("span",o1,I(o.file.display_name||`用户 ${o.user_id}`),1),u("span",a1,I(o.user_id),1)]),o.file.address?(L(),P("div",l1,"她叫他："+I(o.file.address),1)):Ut("",!0),u("dl",c1,[o.file.impression?(L(),P("div",u1,[a[5]||(a[5]=u("dt",null,"印象",-1)),u("dd",null,I(o.file.impression),1)])):Ut("",!0),o.file.my_feeling?(L(),P("div",d1,[a[6]||(a[6]=u("dt",null,"她的感觉",-1)),u("dd",null,I(o.file.my_feeling),1)])):Ut("",!0),o.file.mode?(L(),P("div",h1,[a[7]||(a[7]=u("dt",null,"相处",-1)),u("dd",null,I(o.file.mode),1)])):Ut("",!0)]),o.file.want_to_say?.length?(L(),P("div",f1,[a[8]||(a[8]=u("div",{class:"section-label"},"想对他说",-1)),u("ul",p1,[(L(!0),P(Et,null,It(o.file.want_to_say,(l,c)=>(L(),P("li",{key:c},I(l),1))),128))])])):Ut("",!0),o.file.memories?.length?(L(),P("div",m1,[a[9]||(a[9]=u("div",{class:"section-label"},"共同经历",-1)),u("ul",_1,[(L(!0),P(Et,null,It(o.file.memories,(l,c)=>(L(),P("li",{key:c},I(l),1))),128))])])):Ut("",!0)]))),128))])):(L(),P("div",g1,[...a[10]||(a[10]=[u("div",{class:"empty"},"她还没有记住任何人",-1)])]))],64))]))}},x1=Me(v1,[["__scopeId","data-v-9d4cead7"]]),y1={class:"card"},b1={class:"head-row"},M1={key:0,class:"count-chip"},S1={key:0,class:"summary"},E1={class:"alert-chip"},T1={key:0,class:"table-wrap"},w1={class:"table"},A1={class:"cell-time"},C1={class:"cell-user"},R1={class:"gate-chip"},L1={class:"cell-detail"},P1={key:1,class:"empty"},D1={__name:"SecurityView",setup(n){const t=yt([]),e={perception:"感知",translation:"转译",inner_dialog:"内心独白",consolidation:"睡前整理",persona:"人格"},i={zero_tolerance_blacklist:"零容忍拉黑",rejected:"拒收",quarantined:"隔离",warned:"警告"};function s(c){return e[c]||c||"—"}function r(c){return i[c]||c||"—"}const a=ee(()=>t.value.filter(c=>c.action==="zero_tolerance_blacklist").length);function o(c){return c?new Date(c*1e3).toLocaleString("zh-CN",{hour12:!1,year:"numeric",month:"2-digit",day:"2-digit",hour:"2-digit",minute:"2-digit",second:"2-digit"}):"—"}async function l(){try{t.value=(await Ot("/api/mind/security")).events||[]}catch{}}return ve(()=>{l(),window.addEventListener("refresh-all",l)}),(c,d)=>(L(),P("div",null,[u("div",y1,[u("div",b1,[u("h3",null,[d[0]||(d[0]=ue("审计事件 ",-1)),t.value.length?(L(),P("span",M1,I(t.value.length),1)):Ut("",!0)]),a.value?(L(),P("div",S1,[u("span",E1,"零容忍拉黑 "+I(a.value),1)])):Ut("",!0)]),t.value.length?(L(),P("div",T1,[u("table",w1,[d[1]||(d[1]=u("thead",null,[u("tr",null,[u("th",null,"时间"),u("th",null,"用户"),u("th",null,"滤壳"),u("th",null,"处置"),u("th",null,"详情")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(t.value,(h,p)=>(L(),P("tr",{key:p,class:ie({"row-blacklist":h.action==="zero_tolerance_blacklist"})},[u("td",A1,I(o(h.time)),1),u("td",C1,I(h.user_id??"—"),1),u("td",null,[u("span",R1,I(s(h.gate)),1)]),u("td",null,[u("span",{class:ie(["action-chip",{zero:h.action==="zero_tolerance_blacklist"}])},I(r(h.action)),3)]),u("td",L1,I(h.detail),1)],2))),128))])])])):(L(),P("div",P1,"滤壳尚未记录任何事件"))])]))}},U1=Me(D1,[["__scopeId","data-v-10a6d06d"]]),I1={class:"card"},N1={class:"group-bar"},k1={key:0,class:"group-chips"},F1=["onClick"],O1={key:1,class:"empty-inline"},B1={key:0,class:"card"},z1={class:"card"},H1={key:0,class:"count-chip"},V1={key:0,class:"threads"},G1={class:"thread-head"},W1={class:"thread-title"},$1={class:"thread-time"},X1={class:"intensity-track"},q1={key:0,class:"thread-members"},j1={class:"transcript"},Y1={class:"tr-name"},K1={class:"tr-text"},Z1={key:1,class:"waiting"},J1={key:1,class:"empty"},Q1={class:"grid-2"},tM={class:"card"},eM={key:0,class:"attn-list"},nM={class:"attn-name"},iM={class:"attn-track"},sM={class:"attn-value"},rM={key:1,class:"empty"},oM={class:"card"},aM={key:0,class:"bond-list"},lM={class:"bond-pair"},cM={key:1,class:"empty"},uM={key:2,class:"ignored"},dM={key:2,class:"card"},hM={__name:"SocialView",setup(n){const t=yt([]),e=yt(null),i=yt(null),s=yt(!1);let r=null;const a=ee(()=>(i.value?.topics||[]).slice().sort((x,C)=>C.intensity-x.intensity)),o=ee(()=>Object.entries(i.value?.participants||{}).map(([x,C])=>({uid:Number(x),attention:C.attention||0,last_seen:C.last_seen||0})).sort((x,C)=>C.attention-x.attention).slice(0,12)),l=ee(()=>{const x=[];for(const[C,A]of Object.entries(i.value?.bonds||{}))for(const[D,G]of Object.entries(A))G>=.25&&x.push({a:Number(C),b:Number(D),v:G});return x.sort((C,A)=>A.v-C.v).slice(0,8)}),c=ee(()=>{const x=i.value;return!x||!x.ignored_streak||!x.last_bot_speech?"":`她上次开口后已连续 ${x.ignored_streak} 条消息没人接`}),d=ee(()=>{const x={};for(const C of i.value?.topics||[])for(const A of C.transcript||[])A.speaker&&A.name&&!x[A.speaker]&&(x[A.speaker]=A.name);return x});function h(x){return d.value[x]||`用户${x}`}function p(x){return(x.participants||[]).filter(C=>C!==0).map(h)}function f(x){return x>=.6?"很热":x>=.3?"正聊":"凉了"}function v(x){return x>=.6?"hot":x>=.3?"warm":"cold"}function g(x){return x>=.5?"很熟":"认识"}function _(x){return x>=.5?"close":"known"}function m(x){if(!x)return"";const C=Math.floor(Date.now()/1e3)-x;return C<60?"刚刚":C<3600?`${Math.floor(C/60)} 分钟前`:C<86400?`${Math.floor(C/3600)} 小时前`:`${Math.floor(C/86400)} 天前`}async function E(){try{const x=await Ot("/api/mind/social");t.value=x.groups||[],!e.value&&t.value.length?await b(t.value[0]):e.value&&!t.value.includes(e.value)&&(e.value=t.value[0]||null,e.value?await b(e.value):i.value=null)}catch{}}async function b(x){e.value=x,s.value=!0;try{const C=await Ot(`/api/mind/social/${x}`);i.value=C.state||null}catch{i.value=null}s.value=!1}return ve(async()=>{await E(),r=setInterval(E,15e3)}),An(()=>{r&&clearInterval(r)}),(x,C)=>(L(),P("div",null,[u("div",I1,[u("div",N1,[C[0]||(C[0]=u("h3",null,"群聊",-1)),t.value.length?(L(),P("div",k1,[(L(!0),P(Et,null,It(t.value,A=>(L(),P("button",{key:A,class:ie(["group-chip",{active:A===e.value}]),onClick:D=>b(A)}," 群 "+I(A),11,F1))),128))])):(L(),P("div",O1,"还没有任何群的 social 状态——等群里有消息后这里会出现"))])]),s.value?(L(),P("div",B1,[...C[1]||(C[1]=[u("div",{class:"empty"},"读取中…",-1)])])):i.value?(L(),P(Et,{key:1},[u("div",z1,[u("h3",null,[C[2]||(C[2]=ue("话题线程 ",-1)),a.value.length?(L(),P("span",H1,I(a.value.length),1)):Ut("",!0)]),a.value.length?(L(),P("div",V1,[(L(!0),P(Et,null,It(a.value,A=>(L(),P("div",{key:A.id,class:"thread"},[u("div",G1,[u("span",W1,"「"+I(A.title)+"」",1),u("span",{class:ie(["heat",v(A.intensity)])},I(f(A.intensity)),3),u("span",$1,I(m(A.last_active)),1)]),u("div",X1,[u("div",{class:"intensity-fill",style:se({width:Math.round(A.intensity*100)+"%"})},null,4)]),p(A).length?(L(),P("div",q1,I(p(A).join("、")),1)):Ut("",!0),u("div",j1,[(L(!0),P(Et,null,It(A.transcript,(D,G)=>(L(),P("div",{key:G,class:ie(["tr-line",{"tr-bot":D.is_bot}])},[u("span",Y1,I(D.is_bot?"她":D.name),1),u("span",K1,I(D.text),1)],2))),128))]),A.unanswered?(L(),P("div",Z1,[C[3]||(C[3]=u("span",{class:"waiting-icon"},"⏳",-1)),ue(" "+I(h(A.unanswered.from))+" 问「"+I(A.unanswered.text)+"」还没人接 ",1)])):Ut("",!0)]))),128))])):(L(),P("div",J1,"此刻没有活跃话题——群里很安静"))]),u("div",Q1,[u("div",tM,[C[4]||(C[4]=u("h3",null,"参与者注意力",-1)),o.value.length?(L(),P("div",eM,[(L(!0),P(Et,null,It(o.value,A=>(L(),P("div",{key:A.uid,class:"attn-row"},[u("span",nM,I(h(A.uid)),1),u("div",iM,[u("div",{class:"attn-fill",style:se({width:Math.round(A.attention*100)+"%"})},null,4)]),u("span",sM,I(A.attention.toFixed(2)),1)]))),128))])):(L(),P("div",rM,"暂无参与者"))]),u("div",oM,[C[6]||(C[6]=u("h3",null,"熟络关系",-1)),l.value.length?(L(),P("div",aM,[(L(!0),P(Et,null,It(l.value,(A,D)=>(L(),P("div",{key:D,class:"bond-row"},[u("span",lM,I(h(A.a))+" ↔ "+I(h(A.b)),1),u("span",{class:ie(["bond-level",_(A.v)])},I(g(A.v)),3)]))),128))])):(L(),P("div",cM,"还没有观察到来一往的对话")),c.value?(L(),P("div",uM,[C[5]||(C[5]=u("span",{class:"ignored-icon"},"🌙",-1)),ue(" "+I(c.value),1)])):Ut("",!0)])])],64)):e.value?(L(),P("div",dM,[...C[7]||(C[7]=[u("div",{class:"empty"},"该群暂无社会状态",-1)])])):Ut("",!0)]))}},fM=Me(hM,[["__scopeId","data-v-e3c8dfc0"]]),pM={class:"stat-grid"},mM=["innerHTML"],_M={class:"stat-info"},gM={class:"stat-value"},vM={class:"stat-label"},xM={class:"stat-sub"},yM={key:0,class:"stat-trend"},bM={class:"chart-grid"},MM={class:"card chart-card"},SM={class:"chart-container"},EM={viewBox:"0 0 160 160",width:"160",height:"160"},TM=["stroke-dasharray"],wM={x:"80",y:"74","text-anchor":"middle",fill:"var(--text)","font-size":"26","font-weight":"700"},AM={class:"card chart-card"},CM={class:"chart-container"},RM={viewBox:"0 0 160 160",width:"160",height:"160"},LM=["stroke-dasharray"],PM={x:"80",y:"74","text-anchor":"middle",fill:"var(--text)","font-size":"26","font-weight":"700"},DM={class:"chart-legend"},UM={class:"legend-item"},IM={class:"legend-item"},NM={class:"card chart-card"},kM={class:"chart-container"},FM={viewBox:"0 0 160 160",width:"160",height:"160"},OM=["stroke-dasharray"],BM={x:"80",y:"74","text-anchor":"middle",fill:"var(--text)","font-size":"26","font-weight":"700"},zM={class:"card data-card"},HM={key:0,class:"active-lists"},VM={key:0,class:"active-section"},GM={class:"active-section-title"},WM={class:"active-chips"},$M={key:1,class:"active-section"},XM={class:"active-section-title"},qM={class:"active-chips"},jM={key:1,class:"empty-sm"},YM={__name:"DashboardView",setup(n){const t=yt({}),e=yt([]),i=yt([]);let s=null;const r={brain:'<svg viewBox="0 0 24 24" fill="none" width="20" height="20"><path d="M12 3a4 4 0 00-4 4c0 1.5.7 2.8 1.7 3.7C8.3 11.5 7 13 7 15v1a4 4 0 008 0v-1c0-2-1.3-3.5-2.7-4.3C14.3 9.8 15 8.5 15 7a4 4 0 00-3-3.9z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',chat:'<svg viewBox="0 0 24 24" fill="none" width="20" height="20"><path d="M4 12a8 8 0 1116 0H4z" stroke="currentColor" stroke-width="1.5"/><path d="M8 8h8M8 12h6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',smile:'<svg viewBox="0 0 24 24" fill="none" width="20" height="20"><circle cx="12" cy="12" r="8" stroke="currentColor" stroke-width="1.5"/><circle cx="9" cy="10" r="1" fill="currentColor"/><circle cx="15" cy="10" r="1" fill="currentColor"/><path d="M8 14c1 1.5 2.5 2 4 2s3-.5 4-2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',db:'<svg viewBox="0 0 24 24" fill="none" width="20" height="20"><ellipse cx="12" cy="6" rx="8" ry="3" stroke="currentColor" stroke-width="1.5"/><path d="M4 6v4c0 1.7 3.6 3 8 3s8-1.3 8-3V6" stroke="currentColor" stroke-width="1.5"/><path d="M4 10v4c0 1.7 3.6 3 8 3s8-1.3 8-3v-4" stroke="currentColor" stroke-width="1.5"/></svg>',sticker:'<svg viewBox="0 0 24 24" fill="none" width="20" height="20"><rect x="4" y="4" width="16" height="16" rx="2" stroke="currentColor" stroke-width="1.5"/><circle cx="9" cy="10" r="1.5" fill="currentColor"/><path d="M6 18l4-5 4 5H6z" fill="currentColor" opacity="0.5"/><path d="M13 16l3-5 3 5H13z" fill="currentColor" opacity="0.5"/></svg>'},a=ee(()=>[{label:"记忆条目",value:t.value.memory_entries,sub:"长期记忆总计",icon:r.brain,color:"var(--primary)",trend:null},{label:"记忆用户",value:t.value.memory_users,sub:"已记录用户",icon:r.db,color:"var(--info)",trend:null},{label:"表情包",value:t.value.sticker_count,sub:"已注册",icon:r.sticker,color:"var(--warning)",trend:null},{label:"群聊",value:t.value.active_groups,sub:"进行中",icon:r.chat,color:"var(--info)",trend:null},{label:"私聊",value:t.value.active_users,sub:"进行中",icon:r.chat,color:"var(--info)",trend:null},{label:"情绪用户",value:t.value.emotion_users,sub:"追踪中",icon:r.smile,color:"var(--warning)",trend:null}]),o=ee(()=>(t.value.active_groups||0)+(t.value.active_users||0)),l=ee(()=>{const p=t.value.memory_entries||0,f=Math.min(p/1e3,1),v=2*Math.PI*64;return`${v*f} ${v*(1-f)}`}),c=ee(()=>{const p=o.value,f=Math.min(p/20,1),v=2*Math.PI*64;return`${v*f} ${v*(1-f)}`}),d=ee(()=>{const p=t.value.emotion_users||0,f=Math.min(p/50,1),v=2*Math.PI*64;return`${v*f} ${v*(1-f)}`});async function h(){try{t.value=await Ot("/api/dashboard");const p=await Ot("/api/conversations");e.value=p.groups||[],i.value=p.private_users||[]}catch{}}return ve(()=>{h(),s=setInterval(h,15e3),window.addEventListener("refresh-all",h)}),An(()=>{clearInterval(s),window.removeEventListener("refresh-all",h)}),(p,f)=>(L(),P("div",null,[u("div",pM,[(L(!0),P(Et,null,It(a.value,v=>(L(),P("div",{class:"card",key:v.label},[u("div",{class:"stat-icon",style:se({color:v.color}),innerHTML:v.icon},null,12,mM),u("div",_M,[u("div",gM,I(v.value??"-"),1),u("div",vM,I(v.label),1),u("div",xM,I(v.sub),1)]),v.trend?(L(),P("div",yM,[u("span",{class:ie(v.trend>0?"up":"down")},I(v.trend>0?"+":"")+I(v.trend)+"%",3)])):Ut("",!0)]))),128))]),u("div",bM,[u("div",MM,[f[3]||(f[3]=u("h3",{class:"card-title"},"记忆分布",-1)),u("div",SM,[(L(),P("svg",EM,[f[0]||(f[0]=u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--border)","stroke-width":"16"},null,-1)),u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--primary)","stroke-width":"16","stroke-dasharray":l.value,"stroke-dashoffset":"0",transform:"rotate(-90 80 80)","stroke-linecap":"round",style:{transition:"stroke-dasharray 0.8s ease"}},null,8,TM),u("text",wM,I(t.value.memory_entries??0),1),f[1]||(f[1]=u("text",{x:"80",y:"94","text-anchor":"middle",fill:"var(--text-2)","font-size":"11"},"条记忆",-1))])),f[2]||(f[2]=vs('<div class="chart-legend" data-v-73a547e7><div class="legend-item" data-v-73a547e7><span class="dot" style="background:var(--primary);" data-v-73a547e7></span> 全局记忆</div><div class="legend-item" data-v-73a547e7><span class="dot" style="background:var(--info);" data-v-73a547e7></span> 群记忆</div><div class="legend-item" data-v-73a547e7><span class="dot" style="background:var(--accent);" data-v-73a547e7></span> 表情包</div></div>',1))])]),u("div",AM,[f[8]||(f[8]=u("h3",{class:"card-title"},"活跃会话",-1)),u("div",CM,[(L(),P("svg",RM,[f[4]||(f[4]=u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--border)","stroke-width":"16"},null,-1)),u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--info)","stroke-width":"16","stroke-dasharray":c.value,"stroke-dashoffset":"0",transform:"rotate(-90 80 80)","stroke-linecap":"round",style:{transition:"stroke-dasharray 0.8s ease"}},null,8,LM),u("text",PM,I(o.value),1),f[5]||(f[5]=u("text",{x:"80",y:"94","text-anchor":"middle",fill:"var(--text-2)","font-size":"11"},"活跃",-1))])),u("div",DM,[u("div",UM,[f[6]||(f[6]=u("span",{class:"dot",style:{background:"var(--info)"}},null,-1)),ue(" 群聊: "+I(t.value.active_groups??0),1)]),u("div",IM,[f[7]||(f[7]=u("span",{class:"dot",style:{background:"var(--info)"}},null,-1)),ue(" 私聊: "+I(t.value.active_users??0),1)])])])]),u("div",NM,[f[12]||(f[12]=u("h3",{class:"card-title"},"情绪分布",-1)),u("div",kM,[(L(),P("svg",FM,[f[9]||(f[9]=u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--border)","stroke-width":"16"},null,-1)),u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--warning)","stroke-width":"16","stroke-dasharray":d.value,"stroke-dashoffset":"0",transform:"rotate(-90 80 80)","stroke-linecap":"round",style:{transition:"stroke-dasharray 0.8s ease"}},null,8,OM),u("text",BM,I(t.value.emotion_users??0),1),f[10]||(f[10]=u("text",{x:"80",y:"94","text-anchor":"middle",fill:"var(--text-2)","font-size":"11"},"用户",-1))])),f[11]||(f[11]=vs('<div class="chart-legend" data-v-73a547e7><div class="legend-item" data-v-73a547e7><span class="dot" style="background:var(--warning);" data-v-73a547e7></span> 情绪追踪用户</div><div class="legend-item" data-v-73a547e7><span class="dot" style="background:var(--warning);" data-v-73a547e7></span> 记忆用户</div></div>',1))])])]),u("div",zM,[f[13]||(f[13]=u("h3",{class:"card-title"},"最近活跃",-1)),e.value.length||i.value.length?(L(),P("div",HM,[e.value.length?(L(),P("div",VM,[u("div",GM,"群聊 ("+I(e.value.length)+")",1),u("div",WM,[(L(!0),P(Et,null,It(e.value,v=>(L(),P("span",{key:v,class:"chip"},"群 "+I(v),1))),128))])])):Ut("",!0),i.value.length?(L(),P("div",$M,[u("div",XM,"私聊 ("+I(i.value.length)+")",1),u("div",qM,[(L(!0),P(Et,null,It(i.value,v=>(L(),P("span",{key:v,class:"chip"},"用户 "+I(v),1))),128))])])):Ut("",!0)])):(L(),P("div",jM,"暂无活跃会话"))])]))}},KM=Me(YM,[["__scopeId","data-v-73a547e7"]]),ZM={key:0,class:"config-error-banner"},JM={class:"error-body"},QM={class:"config-layout"},tS={class:"config-nav card"},eS=["onClick"],nS={class:"config-content"},iS={key:0,class:"card"},sS={class:"card-header"},rS={class:"field-list"},oS={class:"field-label"},aS={class:"field-value"},lS={key:1,class:"array-chips"},cS={key:0,class:"text-muted"},uS={key:2,class:"mono"},dS={key:1,class:"card"},hS={class:"card modal"},fS={style:{"margin-bottom":"16px"}},pS={class:"edit-fields"},mS=["type","onUpdate:modelValue"],_S=["onUpdate:modelValue"],gS=["onUpdate:modelValue"],vS={class:"modal-actions"},xS={__name:"ConfigView",setup(n){const t=yt(null),e=yt(""),i=yt("general"),s=yt(!1),r=ir({}),a=[{id:"general",label:"基础配置",color:"var(--info)",fields:[{key:"api_key",label:"API 密钥",type:"string"},{key:"base_url",label:"API 地址",type:"string"},{key:"model",label:"模型名称",type:"string"},{key:"bot_name",label:"Bot 名称",type:"string"},{key:"self_qq",label:"Bot QQ 号",type:"number"},{key:"admin_qq",label:"管理员 QQ",type:"number"},{key:"darling_qq",label:"Darling QQ",type:"number"},{key:"prompts",label:"人设文件",type:"string"},{key:"whitelist",label:"白名单",type:"array"},{key:"blacklist",label:"黑名单",type:"array"},{key:"auto_start_users",label:"自动启动用户",type:"array"},{key:"auto_start_groups",label:"自动启动群",type:"array"}]},{id:"ai",label:"AI 参数",color:"var(--primary)",fields:[{key:"ai.frequency_penalty",label:"频率惩罚",type:"number"},{key:"ai.presence_penalty",label:"存在惩罚",type:"number"},{key:"ai.temperature",label:"温度",type:"number"},{key:"ai.top_p",label:"Top P",type:"number"},{key:"ai.max_tokens",label:"最大 Tokens",type:"number"},{key:"ai.request_timeout",label:"请求超时(秒)",type:"number"},{key:"ai.analysis_max_tokens",label:"分析最大 Tokens",type:"number"},{key:"ai.analysis_temperature",label:"分析温度",type:"number"}]},{id:"conversation",label:"对话",color:"var(--success)",fields:[{key:"conversation.max_history",label:"最大历史轮数",type:"number"},{key:"conversation.batch_timeout_ms",label:"批次超时(ms)",type:"number"},{key:"conversation.max_typing_delay_ms",label:"最大打字延迟(ms)",type:"number"},{key:"conversation.reply_follow_up_secs",label:"跟进回复间隔(秒)",type:"number"},{key:"conversation.action_descriptions",label:"允许动作描述",type:"bool"}]},{id:"memory",label:"记忆",color:"var(--accent)",fields:[{key:"memory.auto_summarize_threshold",label:"自动摘要阈值",type:"number"},{key:"memory.working_memory_expire_hours",label:"工作记忆过期(小时)",type:"number"}]},{id:"emotion",label:"情绪",color:"var(--warning)",fields:[{key:"emotion.decay_delay_secs",label:"衰减延迟(秒)",type:"number"},{key:"emotion.affinity_threshold",label:"好感阈值",type:"number"}]},{id:"proactive",label:"主动对话",color:"var(--danger)",fields:[{key:"proactive.quiet_start",label:"免打扰开始(时)",type:"number"},{key:"proactive.quiet_end",label:"免打扰结束(时)",type:"number"},{key:"proactive.check_interval",label:"检查间隔(秒)",type:"number"}]},{id:"vision",label:"识图",color:"var(--info)",fields:[{key:"vision.api_key",label:"API 密钥",type:"string"},{key:"vision.base_url",label:"API 地址",type:"string"},{key:"vision.model",label:"模型",type:"string"},{key:"vision.max_tokens",label:"最大 Tokens",type:"number"}]},{id:"embedding",label:"向量嵌入",color:"var(--success)",fields:[{key:"embedding.api_key",label:"API 密钥",type:"string"},{key:"embedding.base_url",label:"API 地址",type:"string"},{key:"embedding.model",label:"模型",type:"string"}]},{id:"style",label:"回复风格",color:"var(--accent)",fields:[{key:"style.omit_subject",label:"省略主语",type:"bool"},{key:"style.punctuation_style",label:"标点风格",type:"string"},{key:"style.max_reply_chars",label:"最大回复字数",type:"number"}]},{id:"anti_injection",label:"防注入",color:"var(--danger)",fields:[{key:"anti_injection.input.sensitive_action",label:"敏感内容处理",type:"string"},{key:"anti_injection.output.action",label:"输出处理",type:"string"},{key:"anti_injection.behavior.rate_limit",label:"速率限制",type:"bool"},{key:"anti_injection.behavior.max_messages_per_minute",label:"每分钟上限",type:"number"},{key:"anti_injection.behavior.max_messages_per_hour",label:"每小时上限",type:"number"},{key:"anti_injection.behavior.reputation_threshold",label:"信誉阈值",type:"number"},{key:"anti_injection.behavior.auto_ban",label:"自动封禁",type:"bool"},{key:"anti_injection.behavior.auto_ban_threshold",label:"封禁触发次数",type:"number"},{key:"anti_injection.detection_whitelist",label:"检测白名单(不封禁)",type:"array"}]},{id:"quota",label:"配额",color:"var(--success)",fields:[{key:"quota.enabled",label:"启用",type:"bool"},{key:"quota.segment_minutes",label:"配额段长度(分)",type:"number"}]},{id:"humanity",label:"人性化",color:"var(--accent)",fields:[{key:"humanity.social_battery_enabled",label:"社交电量",type:"bool"},{key:"humanity.battery_capacity",label:"电池容量",type:"number"},{key:"humanity.battery_drain_rate",label:"消耗速率",type:"number"},{key:"humanity.battery_recharge_rate",label:"恢复速率",type:"number"},{key:"humanity.speak_gate",label:"开口门限(越低越爱插话)",type:"number"},{key:"humanity.cognitive_biases_enabled",label:"认知偏差",type:"bool"},{key:"humanity.cognitive_biases.confirmation_bias",label:"确认偏误",type:"number"},{key:"humanity.attention_enabled",label:"注意力模型",type:"bool"},{key:"humanity.response_timing_enabled",label:"变速回复",type:"bool"},{key:"humanity.unpredictability_enabled",label:"不可预测性",type:"bool"},{key:"humanity.whim_probability",label:"心血来潮概率",type:"number"},{key:"humanity.opinion_drift_rate",label:"观点漂移率",type:"number"},{key:"humanity.forgetting_rate",label:"自然遗忘率",type:"number"},{key:"humanity.circadian_enabled",label:"昼夜节律",type:"bool"},{key:"humanity.circadian_amplitude",label:"节律振幅",type:"number"},{key:"humanity.wish_enabled",label:"愿望系统",type:"bool"},{key:"humanity.foraging_enabled",label:"信息觅食",type:"bool"},{key:"humanity.flashback_probability",label:"闪回概率",type:"number"}]},{id:"sticker",label:"表情包",color:"var(--accent)",fields:[{key:"sticker.steal_emoji",label:"自动收集表情",type:"bool"},{key:"sticker.max_reg_num",label:"最大注册数",type:"number"},{key:"sticker.do_replace",label:"自动替换",type:"bool"}]},{id:"log",label:"日志",color:"var(--text-2)",fields:[{key:"log.enabled",label:"日志文件输出",type:"bool"},{key:"log.level",label:"日志级别",type:"string"}]},{id:"admin",label:"管理",color:"var(--text-2)",fields:[{key:"admin.token",label:"管理 Token",type:"string"},{key:"admin.port",label:"管理端口",type:"number"}]}],o=ee(()=>a.find(f=>f.id===i.value)||a[0]);function l(f){if(!t.value)return null;const v=f.key.split(".");let g=t.value;for(const _ of v){if(g==null||typeof g!="object")return null;g=g[_]}return g}function c(){s.value=!0;for(const f of Object.keys(r))delete r[f];for(const f of o.value.fields){const v=l(f);v!=null&&(r[f.key]=f.type==="array"?Array.isArray(v)?v.join(", "):String(v):v)}}async function d(){try{t.value=await Ot("/api/config")}catch{}try{const f=await Ot("/api/config/status");e.value=f.ok?"":f.error||"未知错误"}catch{e.value=""}}async function h(){const f={};for(const v of o.value.fields){if(r[v.key]===""||r[v.key]===void 0)continue;const g=v.key.split(".");let _=f;for(let m=0;m<g.length;m++)m===g.length-1?v.type==="number"?_[g[m]]=Number(r[v.key]):v.type==="bool"?_[g[m]]=r[v.key]===!0||r[v.key]==="true":v.type==="array"?_[g[m]]=String(r[v.key]).split(",").map(E=>E.trim()).filter(Boolean):_[g[m]]=r[v.key]:(_[g[m]]=_[g[m]]||{},_=_[g[m]])}try{await Ot("/api/config",{method:"PUT",body:JSON.stringify(f)}),s.value=!1,d()}catch(v){alert("保存失败: "+v.message)}}async function p(){try{const f=await Ot("/api/config/reload",{method:"POST"});alert(f.message||"配置已重新载入"),d()}catch(f){alert("重载失败: "+f.message)}}return ve(()=>{d(),window.addEventListener("refresh-all",d)}),(f,v)=>(L(),P("div",null,[e.value?(L(),P("div",ZM,[v[3]||(v[3]=u("span",{class:"error-icon"},"!",-1)),u("div",JM,[v[2]||(v[2]=u("strong",null,"配置解析异常",-1)),u("span",null,I(e.value),1)]),u("button",{class:"btn btn-ghost btn-sm",onClick:d},"刷新")])):Ut("",!0),u("div",QM,[u("div",tS,[(L(),P(Et,null,It(a,g=>u("div",{class:"nav-section",key:g.id},[u("div",{class:ie(["nav-item",{active:i.value===g.id}]),onClick:_=>i.value=g.id},[u("span",{class:"nav-dot",style:se({background:g.color})},null,4),ue(" "+I(g.label),1)],10,eS)])),64))]),u("div",nS,[t.value?(L(),P("div",iS,[u("div",sS,[u("h3",null,[u("span",{class:"sec-dot",style:se({background:o.value.color})},null,4),ue(" "+I(o.value.label),1)]),u("button",{class:"btn btn-ghost btn-sm",onClick:c},"编辑")]),u("div",rS,[(L(!0),P(Et,null,It(o.value.fields,g=>(L(),P("div",{key:g.key,class:"field-item"},[u("div",oS,I(g.label),1),u("div",aS,[g.type==="bool"?(L(),P(Et,{key:0},[u("span",{class:ie(["toggle-dot",{on:l(g)}])},null,2),u("span",null,I(l(g)?"是":"否"),1)],64)):g.type==="array"?(L(),P("span",lS,[(L(!0),P(Et,null,It(l(g)||[],(_,m)=>(L(),P("span",{key:m,class:"chip-sm"},I(_),1))),128)),(l(g)||[]).length?Ut("",!0):(L(),P("span",cS,"空"))])):(L(),P("span",uS,I(l(g)??"-"),1))])]))),128))])])):(L(),P("div",dS,[...v[4]||(v[4]=[u("div",{class:"empty"},"加载配置中...",-1)])])),u("div",{class:"card notice-card"},[v[5]||(v[5]=u("svg",{viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},[u("path",{d:"M10 2l7 3v5c0 4-3 7-7 8-4-1-7-4-7-8V5l7-3z",stroke:"currentColor","stroke-width":"1.5"}),u("path",{d:"M9 9h2v5H9zM9 6h2v2H9z",fill:"currentColor"})],-1)),v[6]||(v[6]=u("span",null,"修改配置后点击「保存」再「重新载入配置」即可生效，无需重启。",-1)),u("button",{class:"btn btn-primary btn-sm",onClick:p,style:{"margin-left":"auto","flex-shrink":"0"}},"重新载入配置")])])]),s.value?(L(),P("div",{key:1,class:"modal-overlay",onClick:v[1]||(v[1]=il(g=>s.value=!1,["self"]))},[u("div",hS,[u("h3",fS,"编辑 "+I(o.value.label),1),u("div",pS,[(L(!0),P(Et,null,It(o.value.fields,g=>(L(),P("div",{key:g.key,class:"edit-field"},[u("label",null,I(g.label),1),g.type==="string"||g.type==="number"?Ee((L(),P("input",{key:0,type:g.type==="number"?"number":"text","onUpdate:modelValue":_=>r[g.key]=_,class:"glass-input"},null,8,mS)),[[jp,r[g.key]]]):g.type==="bool"?Ee((L(),P("select",{key:1,"onUpdate:modelValue":_=>r[g.key]=_,class:"glass-select"},[...v[7]||(v[7]=[u("option",{value:!0},"是",-1),u("option",{value:!1},"否",-1)])],8,_S)),[[si,r[g.key]]]):g.type==="array"?Ee((L(),P("input",{key:2,"onUpdate:modelValue":_=>r[g.key]=_,class:"glass-input",placeholder:"逗号分隔多个值"},null,8,gS)),[[Je,r[g.key]]]):Ut("",!0)]))),128))]),u("div",vS,[u("button",{class:"btn btn-ghost",onClick:v[0]||(v[0]=g=>s.value=!1)},"取消"),u("button",{class:"btn btn-primary",onClick:h},"保存")])])])):Ut("",!0)]))}},yS=Me(xS,[["__scopeId","data-v-e6128b71"]]),bS={class:"card"},MS={class:"card-header"},SS={class:"header-actions"},ES=["disabled"],TS={key:0,class:"empty"},wS={key:1},AS={class:"section-label"},CS={class:"chip-list"},RS=["onClick"],LS={key:0,class:"empty",style:{padding:"16px"}},PS={class:"section-label",style:{"margin-top":"16px"}},DS={class:"chip-list"},US=["onClick"],IS={key:0,class:"empty",style:{padding:"16px"}},NS={__name:"ConversationsView",setup(n){const t=yt(null),e=yt(""),i=yt(""),s=yt("group"),r=ee(()=>e.value?(t.value?.groups||[]).filter(c=>String(c).includes(e.value)):t.value?.groups||[]);async function a(){try{t.value=await Ot("/api/conversations")}catch{}}async function o(c,d,h){await Ot(`/api/conversations/${d}/${c}/${h?"enable":"disable"}`,{method:"POST"}),a()}async function l(){i.value.trim()&&(await o(i.value.trim(),s.value,!0),i.value="")}return ve(()=>{a(),window.addEventListener("refresh-all",a)}),(c,d)=>(L(),P("div",null,[u("div",bS,[u("div",MS,[d[4]||(d[4]=u("h3",null,"对话管理",-1)),u("div",SS,[Ee(u("input",{"onUpdate:modelValue":d[0]||(d[0]=h=>i.value=h),placeholder:"输入 QQ/群号添加",class:"glass-input",style:{width:"160px"}},null,512),[[Je,i.value]]),Ee(u("select",{"onUpdate:modelValue":d[1]||(d[1]=h=>s.value=h),class:"glass-select",style:{width:"80px"}},[...d[3]||(d[3]=[u("option",{value:"group"},"群聊",-1),u("option",{value:"private"},"私聊",-1)])],512),[[si,s.value]]),u("button",{class:"btn btn-primary btn-sm",disabled:!i.value.trim(),onClick:l},"＋ 添加",8,ES),Ee(u("input",{"onUpdate:modelValue":d[2]||(d[2]=h=>e.value=h),placeholder:"搜索...",class:"glass-input",style:{width:"120px"}},null,512),[[Je,e.value]]),u("button",{class:"btn btn-ghost btn-sm",onClick:a},"↻ 刷新")])]),t.value?(L(),P("div",wS,[u("div",AS,"群聊 ("+I(t.value.groups?.length||0)+")",1),u("div",CS,[(L(!0),P(Et,null,It(r.value,h=>(L(),P("div",{key:h,class:"chip"},[u("span",null,I(h),1),u("button",{class:"chip-close",onClick:p=>o(h,"group",!1),title:"关闭"},"✕",8,RS)]))),128)),t.value.groups?.length?Ut("",!0):(L(),P("div",LS,"无活跃群聊"))]),u("div",PS,"私聊 ("+I(t.value.private_users?.length||0)+")",1),u("div",DS,[(L(!0),P(Et,null,It(t.value.private_users,h=>(L(),P("div",{key:h,class:"chip chip-priv"},[u("span",null,I(h),1),u("button",{class:"chip-close",onClick:p=>o(h,"private",!1),title:"关闭"},"✕",8,US)]))),128)),t.value.private_users?.length?Ut("",!0):(L(),P("div",IS,"无活跃私聊"))])])):(L(),P("div",TS,"加载中..."))])]))}},kS=Me(NS,[["__scopeId","data-v-e466c855"]]),FS={class:"stat-grid"},OS={key:0,class:"card"},BS={class:"stat-sub"},zS={__name:"QuotaView",setup(n){const t=yt({});async function e(){try{const i=await Ot("/api/quota");t.value={enabled:i.enabled,segment_minutes:i.segment_minutes,segments:i.segments||[]}}catch{}}return ve(()=>{e(),window.addEventListener("refresh-all",e)}),(i,s)=>(L(),P("div",null,[u("div",FS,[t.value.enabled!==void 0?(L(),P("div",OS,[u("div",{class:"stat-value",style:se({color:t.value.enabled?"var(--success)":"var(--text-3)"})},I(t.value.enabled?"已启用":"已禁用"),5),s[0]||(s[0]=u("div",{class:"stat-label"},"配额系统",-1)),u("div",BS,I(t.value.segment_minutes||5)+" 分钟/段",1)])):Ut("",!0)])]))}},HS=Me(zS,[["__scopeId","data-v-6734b806"]]),VS={class:"stat-grid"},GS={class:"stat-label"},WS={class:"stat-sub"},$S={class:"card"},XS={class:"card-header"},qS={class:"badge"},jS={class:"header-actions"},YS={key:0,class:"empty"},KS={key:1,class:"sticker-grid"},ZS={class:"sticker-img"},JS=["src","alt"],QS={class:"sticker-info"},tE={key:0,class:"sticker-tags"},eE={class:"chip-sm"},nE={key:1,class:"text-muted"},iE={class:"sticker-actions"},sE=["onClick"],rE={__name:"StickerView",setup(n){const t=yt([]),e=yt(""),i=yt([{label:"表情包",value:"-",sub:"已注册",color:"var(--primary)"}]),s=ee(()=>{if(!e.value)return t.value;const o=e.value.toLowerCase();return t.value.filter(l=>((l.description||l.vlm_description||"")+" "+l.hash).toLowerCase().includes(o))});async function r(){try{const o=await Ot("/api/sticker");t.value=o.stickers||[],i.value[0].value=t.value.length}catch{}}async function a(o){await Ot("/api/sticker/"+o,{method:"POST"}),r()}return ve(()=>{r(),window.addEventListener("refresh-all",r)}),(o,l)=>(L(),P("div",null,[u("div",VS,[(L(!0),P(Et,null,It(i.value,c=>(L(),P("div",{class:"card",key:c.label},[u("div",{class:"stat-value",style:se({color:c.color})},I(c.value),5),u("div",GS,I(c.label),1),u("div",WS,I(c.sub),1)]))),128))]),u("div",$S,[u("div",XS,[u("h3",null,[l[2]||(l[2]=ue("表情包 ",-1)),u("span",qS,I(t.value.length),1)]),u("div",jS,[Ee(u("input",{"onUpdate:modelValue":l[0]||(l[0]=c=>e.value=c),placeholder:"搜索标签...",class:"glass-input"},null,512),[[Je,e.value]]),u("button",{class:"btn btn-ghost btn-sm",onClick:r},"↻ 刷新")])]),t.value.length?(L(),P("div",KS,[(L(!0),P(Et,null,It(s.value,(c,d)=>(L(),P("div",{key:d,class:"sticker-card"},[u("div",ZS,[u("img",{src:"/api/sticker/image/"+c.hash,alt:c.hash,loading:"lazy",onError:l[1]||(l[1]=h=>h.target.style.display="none")},null,40,JS)]),u("div",QS,[c.description||c.vlm_description?(L(),P("div",tE,[u("span",eE,I((c.description||c.vlm_description||"").slice(0,30)),1)])):(L(),P("span",nE,"无描述")),u("div",iE,[u("span",{class:ie(["badge-sm",c.is_banned?"off":"on"])},I(c.is_banned?"禁用":"启用"),3),u("button",{class:"btn btn-ghost btn-xs",onClick:h=>a(c.hash)},I(c.is_banned?"启用":"禁用"),9,sE)])])]))),128))])):(L(),P("div",YS,"暂无表情包"))])]))}},oE=Me(rE,[["__scopeId","data-v-f26e5395"]]),aE={class:"card"},lE={class:"card-header"},cE={class:"header-actions"},uE={value:""},dE=["value"],hE={key:0,class:"empty"},fE={key:1,class:"table-wrap"},pE={class:"mono"},mE={class:"truncate"},_E={class:"mono"},gE={class:"actions"},vE=["onClick"],xE={class:"card modal"},yE={class:"modal-actions"},bE={__name:"UserMemory",setup(n){const t=yt({}),e=yt([]),i=yt(""),s=yt(""),r=yt(""),a=yt(""),o=yt(!1),l=yt(""),c=yt(""),d=yt("Normal");function h(b){return b?new Date(b*1e3).toLocaleString("zh-CN"):"-"}function p(b){return(typeof b=="string"?b:Object.keys(b||{})[0]||"Normal").toLowerCase()}function f(b){return typeof b=="string"?b:Object.keys(b||{})[0]||"Normal"}const v=ee(()=>{let b=e.value;if(s.value){const x=s.value.toLowerCase();b=b.filter(C=>(C.content||"").toLowerCase().includes(x))}if(r.value){const x=new Date(r.value).getTime()/1e3;b=b.filter(C=>(C.created||0)>=x)}if(a.value){const x=new Date(a.value).getTime()/1e3+86400;b=b.filter(C=>(C.created||0)<x)}return b});async function g(){const b=await Ot("/api/memory");t.value=b.users||{},_()}function _(){const b=i.value;if(b){const x=t.value[b];e.value=(x?.entries||[]).map((C,A)=>({...C,uid:b,idx:A}))}else{const x=[];for(const[C,A]of Object.entries(t.value))(A.entries||[]).forEach((D,G)=>x.push({...D,uid:C,idx:G}));e.value=x}}async function m(b,x){await Ot(`/api/memory/${b}/${x}`,{method:"DELETE"}),g()}async function E(){!l.value||!c.value||(await Ot(`/api/memory/${l.value}`,{method:"POST",body:JSON.stringify({content:c.value,importance:d.value})}),o.value=!1,l.value="",c.value="",g())}return ve(()=>{g(),window.addEventListener("refresh-all",g)}),(b,x)=>(L(),P("div",null,[u("div",aE,[u("div",lE,[x[12]||(x[12]=u("h3",null,"用户记忆",-1)),u("div",cE,[Ee(u("input",{"onUpdate:modelValue":x[0]||(x[0]=C=>s.value=C),placeholder:"搜索...",class:"glass-input"},null,512),[[Je,s.value]]),Ee(u("select",{"onUpdate:modelValue":x[1]||(x[1]=C=>i.value=C),onChange:_,class:"glass-select"},[u("option",uE,"全部用户 ("+I(Object.keys(t.value).length)+")",1),(L(!0),P(Et,null,It(t.value,(C,A)=>(L(),P("option",{key:A,value:A},"用户 "+I(A)+" ("+I((C.entries||[]).length)+")",9,dE))),128))],544),[[si,i.value]]),Ee(u("input",{type:"date","onUpdate:modelValue":x[2]||(x[2]=C=>r.value=C),class:"glass-input",style:{width:"130px"}},null,512),[[Je,r.value]]),x[10]||(x[10]=u("span",{class:"sep"},"~",-1)),Ee(u("input",{type:"date","onUpdate:modelValue":x[3]||(x[3]=C=>a.value=C),class:"glass-input",style:{width:"130px"}},null,512),[[Je,a.value]]),u("button",{class:"btn btn-primary btn-sm",onClick:x[4]||(x[4]=C=>o.value=!0)},"＋ 添加"),x[11]||(x[11]=u("a",{class:"btn btn-ghost btn-sm",href:"/api/memory/export",target:"_blank"},"导出",-1))])]),v.value.length?(L(),P("div",fE,[u("table",null,[x[13]||(x[13]=u("thead",null,[u("tr",null,[u("th",null,"用户"),u("th",null,"内容"),u("th",null,"重要性"),u("th",null,"创建时间"),u("th",null,"操作")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(v.value,(C,A)=>(L(),P("tr",{key:A},[u("td",pE,I(C.uid),1),u("td",mE,I(C.content),1),u("td",null,[u("span",{class:ie("tag tag-"+p(C.importance))},I(f(C.importance)),3)]),u("td",_E,I(h(C.created)),1),u("td",gE,[u("button",{class:"btn btn-ghost btn-xs",onClick:D=>m(C.uid,C.idx)},"删除",8,vE)])]))),128))])])])):(L(),P("div",hE,"暂无记忆"))]),o.value?(L(),P("div",{key:0,class:"modal-overlay",onClick:x[9]||(x[9]=il(C=>o.value=!1,["self"]))},[u("div",xE,[x[15]||(x[15]=u("h3",{style:{"margin-bottom":"16px"}},"添加用户记忆",-1)),x[16]||(x[16]=u("label",null,"用户 ID",-1)),Ee(u("input",{"onUpdate:modelValue":x[5]||(x[5]=C=>l.value=C),class:"glass-input",style:{"margin-bottom":"12px"}},null,512),[[Je,l.value]]),x[17]||(x[17]=u("label",null,"内容",-1)),Ee(u("textarea",{"onUpdate:modelValue":x[6]||(x[6]=C=>c.value=C),class:"glass-input",style:{"margin-bottom":"12px"}},null,512),[[Je,c.value]]),x[18]||(x[18]=u("label",null,"重要性",-1)),Ee(u("select",{"onUpdate:modelValue":x[7]||(x[7]=C=>d.value=C),class:"glass-select",style:{"margin-bottom":"16px"}},[...x[14]||(x[14]=[u("option",{value:"Normal"},"普通",-1),u("option",{value:"Important"},"重要",-1),u("option",{value:"Permanent"},"永久",-1)])],512),[[si,d.value]]),u("div",yE,[u("button",{class:"btn btn-ghost",onClick:x[8]||(x[8]=C=>o.value=!1)},"取消"),u("button",{class:"btn btn-primary",onClick:E},"保存")])])])):Ut("",!0)]))}},ME=Me(bE,[["__scopeId","data-v-43f65072"]]),SE={class:"card"},EE={class:"card-header"},TE={class:"header-actions"},wE=["value"],AE={key:0,class:"empty"},CE={key:1,class:"msg-flow"},RE={class:"msg-body"},LE={class:"msg-header"},PE={class:"msg-time"},DE={key:0,class:"msg-tag"},UE={class:"msg-text"},IE={__name:"WorkingMemory",setup(n){const t=yt([]),e=yt([]),i=yt(""),s=yt(!1);let r=null;function a(l){return l?new Date(l*1e3).toLocaleTimeString("zh-CN"):"-"}async function o(){try{const c=(await Ot("/api/working-memory")).groups||{};if(e.value=Object.keys(c),i.value){const d=c[i.value];t.value=(d?.entries||[]).reverse().slice(0,100).map(h=>({...h,is_bot:!1}))}else{const d=[];for(const[h,p]of Object.entries(c))(p.entries||[]).forEach(f=>d.push({...f,group_id:h,is_bot:!1}));t.value=d.reverse().slice(0,200)}}catch{}}return ve(()=>{o(),r=setInterval(()=>{s.value&&o()},5e3),window.addEventListener("refresh-all",o)}),An(()=>{clearInterval(r),window.removeEventListener("refresh-all",o)}),(l,c)=>(L(),P("div",null,[u("div",SE,[u("div",EE,[c[3]||(c[3]=u("h3",null,[ue("工作记忆 "),u("span",{class:"badge"},"群聊消息流")],-1)),u("div",TE,[Ee(u("select",{"onUpdate:modelValue":c[0]||(c[0]=d=>i.value=d),onChange:o,class:"glass-select"},[c[2]||(c[2]=u("option",{value:""},"全部群",-1)),(L(!0),P(Et,null,It(e.value,d=>(L(),P("option",{key:d,value:d},"群 "+I(d),9,wE))),128))],544),[[si,i.value]]),u("button",{class:"btn btn-ghost btn-sm",onClick:c[1]||(c[1]=d=>s.value=!s.value)},I(s.value?"⏸ 暂停":"▶ 自动"),1)])]),t.value.length?(L(),P("div",CE,[(L(!0),P(Et,null,It(t.value,(d,h)=>(L(),P("div",{key:h,class:ie(["msg-item",{isBot:d.is_bot}])},[u("div",{class:"msg-avatar",style:se({background:d.is_bot?"var(--primary)":"var(--text-3)"})},I(d.is_bot?"B":"U"),5),u("div",RE,[u("div",LE,[u("span",{class:ie(["msg-user",{bot:d.is_bot}])},I(d.is_bot?"Bot":"user_id:"+d.user_id),3),u("span",PE,I(a(d.timestamp)),1),d.bot_replied?(L(),P("span",DE,"已回复")):Ut("",!0)]),u("div",UE,I(d.content),1)])],2))),128))])):(L(),P("div",AE,"暂无工作记忆"))])]))}},NE=Me(IE,[["__scopeId","data-v-53bb8a7c"]]),kE={class:"stat-grid"},FE={class:"stat-label"},OE={class:"stat-sub"},BE={class:"card"},zE={key:0,class:"empty"},HE={key:1,class:"table-wrap"},VE={class:"mono"},GE={class:"bar-wrap"},WE={class:"bar-val"},$E={key:1,class:"text-muted"},XE={class:"mono"},qE={__name:"EmotionView",setup(n){const t=yt({}),e=ee(()=>Object.entries(t.value).filter(([o])=>o!=="0").map(([o,l])=>({user_id:o,current:l?.current||"neutral",secondary:l?.secondary||null,intensity:l?.intensity||0,interaction_rate:l?.interaction_rate||0,last_update:l?.last_update||0})).sort((o,l)=>l.intensity-o.intensity)),i=ee(()=>[{label:"追踪用户",value:e.value.length,sub:"有情绪记录",color:"var(--info)"},{label:"活跃情绪",value:[...new Set(e.value.map(o=>o.current))].length,sub:"不同情绪类型",color:"var(--success)"}]);function s(o){return{neutral:"var(--text-3)",happy:"var(--success)",sad:"var(--info)",thinking:"var(--primary)",surprised:"var(--warning)",angry:"var(--danger)",shy:"var(--accent)",worried:"var(--warning)",tired:"var(--text-3)",excited:"var(--warning)",like:"var(--accent)"}[o?.toLowerCase()]||"var(--text-3)"}function r(o){return{neutral:"平静",happy:"开心",sad:"难过",thinking:"沉思",surprised:"惊讶",angry:"不悦",shy:"害羞",worried:"担忧",tired:"疲惫",excited:"兴奋",like:"心动"}[o?.toLowerCase()]||o||"未知"}async function a(){try{t.value=await Ot("/api/emotion")}catch{}}return ve(()=>{a(),window.addEventListener("refresh-all",a)}),(o,l)=>(L(),P("div",null,[u("div",kE,[(L(!0),P(Et,null,It(i.value,c=>(L(),P("div",{class:"card",key:c.label},[u("div",{class:"stat-value",style:se({color:c.color})},I(c.value??"-"),5),u("div",FE,I(c.label),1),u("div",OE,I(c.sub),1)]))),128))]),u("div",BE,[l[1]||(l[1]=u("div",{class:"card-header"},[u("h3",null,"情绪状态")],-1)),e.value.length?(L(),P("div",HE,[u("table",null,[l[0]||(l[0]=u("thead",null,[u("tr",null,[u("th",null,"用户ID"),u("th",null,"情绪"),u("th",null,"强度"),u("th",null,"次要情绪"),u("th",null,"互动频率")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(e.value,c=>(L(),P("tr",{key:c.user_id},[u("td",VE,I(c.user_id),1),u("td",null,[u("span",{class:"emotion-badge",style:se({background:s(c.current)})},I(r(c.current)),5)]),u("td",null,[u("div",GE,[u("div",{class:"bar",style:se({width:(c.intensity||0)*100+"%",background:s(c.current)})},null,4)]),u("span",WE,I(((c.intensity||0)*100).toFixed(0))+"%",1)]),u("td",null,[c.secondary?(L(),P("span",{key:0,class:"emotion-badge",style:se({background:s(c.secondary),opacity:.7})},I(r(c.secondary)),5)):(L(),P("span",$E,"—"))]),u("td",XE,I((c.interaction_rate||0).toFixed(1))+"/h",1)]))),128))])])])):(L(),P("div",zE,"暂无情绪数据"))])]))}},jE=Me(qE,[["__scopeId","data-v-abe732f7"]]),YE={class:"card"},KE={class:"card-header"},ZE={class:"header-actions"},JE=["disabled"],QE={key:0,class:"empty"},tT={key:1,class:"chip-list"},eT={class:"mono"},nT=["onClick"],iT={__name:"BlocklistView",setup(n){const t=yt([]),e=yt("");async function i(){try{const a=await Ot("/api/blocklist");t.value=a.blocked||[]}catch{}}async function s(a){await Ot("/api/blocklist/"+a,{method:"DELETE"}),i()}async function r(){e.value.trim()&&(await Ot("/api/blocklist",{method:"POST",body:JSON.stringify({user_id:Number(e.value.trim())})}),e.value="",i())}return ve(()=>{i(),window.addEventListener("refresh-all",i)}),(a,o)=>(L(),P("div",null,[u("div",YE,[u("div",KE,[o[1]||(o[1]=u("h3",null,"黑名单管理",-1)),u("div",ZE,[Ee(u("input",{"onUpdate:modelValue":o[0]||(o[0]=l=>e.value=l),placeholder:"输入 QQ 号添加",class:"glass-input",style:{width:"160px"}},null,512),[[Je,e.value]]),u("button",{class:"btn btn-primary btn-sm",onClick:r,disabled:!e.value.trim()},"＋ 添加",8,JE),u("button",{class:"btn btn-ghost btn-sm",onClick:i},"↻ 刷新")])]),t.value.length?(L(),P("div",tT,[(L(!0),P(Et,null,It(t.value,(l,c)=>(L(),P("div",{key:c,class:"chip"},[u("span",eT,I(l),1),u("button",{class:"chip-close",onClick:d=>s(l)},"✕",8,nT)]))),128))])):(L(),P("div",QE,"暂无黑名单用户"))])]))}},sT=Me(iT,[["__scopeId","data-v-2fd8560d"]]),rT={class:"card"},oT={key:0,class:"empty"},aT={key:1,class:"table-wrap"},lT={class:"mono"},cT={class:"bar-wrap",style:{width:"80px"}},uT={class:"mono"},dT={class:"mono"},hT={class:"actions"},fT=["onClick"],pT=["onClick"],mT=["onClick"],_T={__name:"AntiInjectionView",setup(n){const t=yt([]);async function e(){try{const o=(await Ot("/api/anti-injection/users")).users||[];for(const l of o)try{const c=await Ot("/api/anti-injection/"+l.user_id);Object.assign(l,c)}catch{}o.sort((l,c)=>l.user_id-c.user_id),t.value=o}catch{}}async function i(a){await Ot("/api/anti-injection/"+a+"/unban",{method:"POST"}),e()}async function s(a){await Ot("/api/anti-injection/"+a+"/enable-vision",{method:"POST"}),e()}async function r(a){await Ot("/api/anti-injection/"+a+"/reset-reputation",{method:"POST"}),e()}return ve(()=>{e(),window.addEventListener("refresh-all",e)}),(a,o)=>(L(),P("div",null,[u("div",rT,[u("div",{class:"card-header"},[o[0]||(o[0]=u("h3",null,"防注入系统",-1)),u("button",{class:"btn btn-ghost btn-sm",onClick:e},"↻ 刷新")]),t.value.length?(L(),P("div",aT,[u("table",null,[o[1]||(o[1]=u("thead",null,[u("tr",null,[u("th",null,"用户 ID"),u("th",null,"信誉评分"),u("th",null,"违规次数"),u("th",null,"静默封禁"),u("th",null,"识图禁用"),u("th",null,"回复惩罚"),u("th",null,"操作")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(t.value,l=>(L(),P("tr",{key:l.user_id},[u("td",lT,I(l.user_id),1),u("td",null,[u("div",cT,[u("div",{class:"bar",style:se({width:Math.min((l.reputation||0)*100,100)+"%",background:l.reputation>.6?"var(--success)":l.reputation>.3?"var(--warning)":"var(--danger)"})},null,4)])]),u("td",uT,I(l.violation_count||0),1),u("td",null,[u("span",{class:ie(["badge",l.silent_banned?"badge-danger":"badge-safe"])},I(l.silent_banned?"是":"否"),3)]),u("td",null,[u("span",{class:ie(["badge",l.vision_disabled?"badge-warn":"badge-safe"])},I(l.vision_disabled?"是":"否"),3)]),u("td",dT,I(l.penalty_multiplier?l.penalty_multiplier.toFixed(2)+"x":"-"),1),u("td",hT,[l.silent_banned?(L(),P("button",{key:0,class:"btn btn-ghost btn-xs",onClick:c=>i(l.user_id)},"解封",8,fT)):Ut("",!0),l.vision_disabled?(L(),P("button",{key:1,class:"btn btn-ghost btn-xs",onClick:c=>s(l.user_id)},"恢复识图",8,pT)):Ut("",!0),u("button",{class:"btn btn-ghost btn-xs",onClick:c=>r(l.user_id)},"重置信誉",8,mT)])]))),128))])])])):(L(),P("div",oT,"暂无用户风险数据"))])]))}},gT=Me(_T,[["__scopeId","data-v-de301834"]]),vT={class:"card"},xT={key:0,class:"empty"},yT={key:1},bT={class:"stat-row"},MT={class:"stat-item"},ST={class:"stat-num"},ET={class:"stat-item"},TT={class:"stat-num"},wT={__name:"ArchiveView",setup(n){const t=yt(null);async function e(){try{t.value=await Ot("/api/archive")}catch{}}return ve(()=>{e(),window.addEventListener("refresh-all",e)}),(i,s)=>(L(),P("div",null,[u("div",vT,[u("div",{class:"card-header"},[s[0]||(s[0]=u("h3",null,"归档统计",-1)),u("button",{class:"btn btn-ghost btn-sm",onClick:e},"↻ 刷新")]),t.value?(L(),P("div",yT,[u("div",bT,[u("div",MT,[u("div",ST,I(Array.isArray(t.value.working_memory)?t.value.working_memory.length:t.value.working_memory||0),1),s[1]||(s[1]=u("div",{class:"stat-lbl"},"工作记忆归档",-1))]),u("div",ET,[u("div",TT,I(Array.isArray(t.value.long_term)?t.value.long_term.length:t.value.long_term||0),1),s[2]||(s[2]=u("div",{class:"stat-lbl"},"长期记忆归档",-1))])])])):(L(),P("div",xT,"加载中..."))])]))}},AT=Me(wT,[["__scopeId","data-v-94e997b2"]]),CT={class:"stat-grid"},RT={class:"stat-value"},LT={class:"stat-label"},PT={class:"card"},DT={class:"card-header"},UT={class:"header-actions"},IT=["value"],NT={key:0,class:"empty"},kT={key:1,class:"empty"},FT={key:2,class:"card-list"},OT={class:"mono"},BT={class:"list-time"},zT={class:"list-actions"},HT=["onClick"],VT=["onClick"],GT={__name:"BackupsView",setup(n){const t=yt([]),e=yt({}),i=yt(""),s=yt([]),r=yt(!1);function a(f){return{working_memory:"工作记忆",emotion:"情绪",blocklist:"黑名单",archive:"归档"}[f]||f}function o(f){return f?f<1024?f+"B":f<1024*1024?(f/1024).toFixed(1)+"KB":(f/1024/1024).toFixed(1)+"MB":"-"}async function l(){try{const f=await Ot("/api/backups");t.value=f.types||[],e.value=f.counts||{},!i.value&&t.value.length&&(i.value=t.value[0]),await c()}catch{r.value=!1}}async function c(){if(i.value){r.value=!0;try{const f=await Ot("/api/backups/"+i.value);s.value=f.backups||[]}catch{}r.value=!1}}Wr(i,c);async function d(){await Ot("/api/backups",{method:"POST",body:JSON.stringify({action:"create",type:i.value})}),l()}async function h(f){confirm("确认恢复 "+f+" ？")&&(await Ot("/api/backups",{method:"POST",body:JSON.stringify({action:"restore",type:i.value,filename:f})}),l())}async function p(f){confirm("确认删除 "+f+" ？")&&(await Ot("/api/backups",{method:"POST",body:JSON.stringify({action:"delete",type:i.value,filename:f})}),l())}return ve(()=>{l(),window.addEventListener("refresh-all",l)}),(f,v)=>(L(),P("div",null,[u("div",CT,[(L(!0),P(Et,null,It(e.value,(g,_)=>(L(),P("div",{class:"card",key:_},[u("div",RT,I(g),1),u("div",LT,I(a(_)),1)]))),128))]),u("div",PT,[u("div",DT,[v[1]||(v[1]=u("h3",null,"备份详情",-1)),u("div",UT,[Ee(u("select",{"onUpdate:modelValue":v[0]||(v[0]=g=>i.value=g),class:"glass-select"},[(L(!0),P(Et,null,It(t.value,g=>(L(),P("option",{key:g,value:g},I(a(g)),9,IT))),128))],512),[[si,i.value]]),u("button",{class:"btn btn-primary btn-sm",onClick:d},"＋ 创建"),u("button",{class:"btn btn-ghost btn-sm",onClick:l},"↻ 刷新")])]),r.value?(L(),P("div",NT,"加载中...")):s.value.length?(L(),P("div",FT,[(L(!0),P(Et,null,It(s.value,(g,_)=>(L(),P("div",{key:_,class:"list-item"},[u("span",OT,I(g.filename),1),u("span",BT,I(o(g.size)),1),u("div",zT,[u("button",{class:"btn btn-ghost btn-xs",onClick:m=>h(g.filename)},"恢复",8,HT),u("button",{class:"btn btn-ghost btn-xs",style:{color:"var(--danger)"},onClick:m=>p(g.filename)},"删除",8,VT)])]))),128))])):(L(),P("div",kT,"暂无该类型备份"))])]))}},WT=Me(GT,[["__scopeId","data-v-81042d3d"]]),$T={class:"card"},XT={class:"card-header"},qT={class:"badge"},jT={class:"header-actions"},YT={key:0,class:"empty"},KT={key:1,class:"table-wrap"},ZT={class:"mono"},JT={class:"mono"},QT={class:"mono"},tw=["title"],ew=["title"],nw={__name:"MemoryOpsLog",setup(n){const t=yt([]),e=yt(""),i=yt(""),s=yt(!1);let r=null;function a(p){return p?new Date(p*1e3).toLocaleString("zh-CN"):"-"}function o(p){return p.startsWith("review")?"review":p.includes("extract")||p.includes("summarize")?"ai":p==="forget"||p==="forget_all"?"forget":p==="correct"?"correct":"add"}function l(p){return{add:"添加",add_group:"添加(群)",forget:"遗忘",forget_all:"全部遗忘",correct:"修正",ai_extract:"AI提取",keyword_extract:"关键词提取",auto_summarize:"自动摘要",review_add:"审查+",review_remove:"审查-",review_update:"审查~"}[p]||p}const c=ee(()=>{let p=t.value;return e.value&&(p=p.filter(f=>f.operation===e.value)),i.value&&(p=p.filter(f=>String(f.user_id).includes(i.value))),p});async function d(){try{const p=await Ot("/api/memory-ops-log");t.value=(p.entries||[]).reverse()}catch{}}async function h(){confirm("确认清空所有内存操作日志？")&&(await Ot("/api/memory-ops-log/clear",{method:"POST"}),d())}return ve(()=>{d(),r=setInterval(()=>{s.value&&d()},3e3),window.addEventListener("refresh-all",d)}),An(()=>{clearInterval(r),window.removeEventListener("refresh-all",d)}),(p,f)=>(L(),P("div",null,[u("div",$T,[u("div",XT,[u("h3",null,[f[3]||(f[3]=ue("内存操作监视 ",-1)),u("span",qT,I(t.value.length)+" 条",1)]),u("div",jT,[Ee(u("select",{"onUpdate:modelValue":f[0]||(f[0]=v=>e.value=v),class:"glass-select"},[...f[4]||(f[4]=[vs('<option value="" data-v-17ee96fc>全部操作</option><option value="add" data-v-17ee96fc>添加</option><option value="add_group" data-v-17ee96fc>添加(群)</option><option value="forget" data-v-17ee96fc>遗忘</option><option value="forget_all" data-v-17ee96fc>全部遗忘</option><option value="correct" data-v-17ee96fc>修正</option><option value="ai_extract" data-v-17ee96fc>AI 提取</option><option value="keyword_extract" data-v-17ee96fc>关键词提取</option><option value="auto_summarize" data-v-17ee96fc>自动摘要</option><option value="review_add" data-v-17ee96fc>审查添加</option><option value="review_remove" data-v-17ee96fc>审查删除</option><option value="review_update" data-v-17ee96fc>审查更新</option>',12)])],512),[[si,e.value]]),Ee(u("input",{"onUpdate:modelValue":f[1]||(f[1]=v=>i.value=v),placeholder:"用户ID...",class:"glass-input",style:{width:"100px"}},null,512),[[Je,i.value]]),u("button",{class:"btn btn-ghost btn-sm",onClick:f[2]||(f[2]=v=>s.value=!s.value)},I(s.value?"⏸ 暂停":"▶ 自动刷新"),1),u("button",{class:"btn btn-ghost btn-sm",onClick:d},"刷新"),u("button",{class:"btn btn-danger btn-sm",onClick:h},"清空")])]),c.value.length?(L(),P("div",KT,[u("table",null,[f[5]||(f[5]=u("thead",null,[u("tr",null,[u("th",null,"时间"),u("th",null,"操作"),u("th",null,"用户"),u("th",null,"群"),u("th",null,"内容"),u("th",null,"重要性"),u("th",null,"详情")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(c.value,(v,g)=>(L(),P("tr",{key:g},[u("td",ZT,I(a(v.timestamp)),1),u("td",null,[u("span",{class:ie("tag tag-"+o(v.operation))},I(l(v.operation)),3)]),u("td",JT,I(v.user_id||"-"),1),u("td",QT,I(v.group_id||"-"),1),u("td",{class:"truncate",title:v.content_preview},I(v.content_preview),9,tw),u("td",null,[u("span",{class:ie("tag tag-imp-"+(v.importance||"normal"))},I(v.importance||"-"),3)]),u("td",{class:"detail-cell",title:v.detail},I(v.detail),9,ew)]))),128))])])])):(L(),P("div",YT,"暂无内存操作日志"))])]))}},iw=Me(nw,[["__scopeId","data-v-17ee96fc"]]),sw={class:"stat-grid"},rw={class:"stat-value"},ow={class:"stat-label"},aw={class:"bar-wrap"},lw={class:"card"},cw={class:"stat-value"},uw={class:"plan-grid"},dw={class:"card-header"},hw={key:0,class:"badge"},fw={key:0,class:"empty"},pw={key:1,class:"goal-list"},mw=["title"],_w={key:0,viewBox:"0 0 20 20",fill:"none",width:"20",height:"20"},gw={key:1,viewBox:"0 0 20 20",fill:"none",width:"20",height:"20"},vw={class:"goal-body"},xw={class:"goal-content"},yw={class:"goal-id"},bw={class:"goal-meta"},Mw={key:1,class:"done-note"},Sw={key:2,class:"progress-note"},Ew={key:3,class:"done-time"},Tw={key:0,class:"card"},ww={class:"card-header"},Aw={class:"badge"},Cw={class:"table-wrap"},Rw={class:"mono"},Lw={class:"tag-kind"},Pw={__name:"ScheduleView",setup(n){const t=["day","week","month"];function e(h){return{label:h,period:"",items:[],total:0,done:0}}const i=yt({day:e("今日"),week:e("本周"),month:e("本月")}),s=yt([]);function r(h){return i.value[h]||e("")}function a(h){const p=r(h);return p.total>0?Math.round(p.done/p.total*100):0}function o(h){return{Monday:"周一",Tuesday:"周二",Wednesday:"周三",Thursday:"周四",Friday:"周五",Saturday:"周六",Sunday:"周日"}[h]||h}function l(h){return h?new Date(h*1e3).toLocaleString("zh-CN"):"-"}async function c(){try{const h=await Ot("/api/schedule"),p=h.timeframes||{};for(const f of t)i.value[f]=p[f]||e("");s.value=h.history||[]}catch{}}let d;return ve(()=>{c(),d=window.setInterval(c,5e3),window.addEventListener("refresh-all",c)}),An(()=>{window.clearInterval(d),window.removeEventListener("refresh-all",c)}),(h,p)=>(L(),P("div",null,[u("div",sw,[(L(),P(Et,null,It(t,f=>u("div",{class:"card",key:f},[u("div",rw,I(r(f).done)+"/"+I(r(f).total),1),u("div",ow,I(r(f).label)+"完成",1),u("div",aw,[u("div",{class:"bar",style:se({width:a(f)+"%"})},null,4)])])),64)),u("div",lw,[u("div",cw,I(s.value.length),1),p[0]||(p[0]=u("div",{class:"stat-label"},"状态变更记录",-1)),p[1]||(p[1]=u("div",{class:"stat-sub"},"运行时计划状态",-1))])]),u("div",uw,[(L(),P(Et,null,It(t,f=>u("div",{class:"card",key:f},[u("div",dw,[u("h3",null,[ue(I(i.value[f].label)+"的事 ",1),i.value[f].period?(L(),P("span",hw,I(i.value[f].period),1)):Ut("",!0)])]),i.value[f].items.length?(L(),P("div",pw,[(L(!0),P(Et,null,It(i.value[f].items,v=>(L(),P("div",{key:v.id,class:ie(["goal-item",{done:v.completed}])},[u("div",{class:"goal-check",title:v.completed?"已自动完成":"进行中"},[v.completed?(L(),P("svg",_w,[...p[2]||(p[2]=[u("circle",{cx:"10",cy:"10",r:"8",fill:"var(--success)"},null,-1),u("path",{d:"M6 10l3 3 5-5",stroke:"#fff","stroke-width":"2","stroke-linecap":"round"},null,-1)])])):(L(),P("svg",gw,[...p[3]||(p[3]=[u("circle",{cx:"10",cy:"10",r:"7",stroke:"var(--text-3)","stroke-width":"1.5"},null,-1)])]))],8,mw),u("div",vw,[u("div",xw,[u("span",yw,I(v.id),1),ue(I(v.content),1)]),u("div",bw,[v.target_day?(L(),P("span",{key:0,class:ie(["day-badge",(v.target_day||"").toLowerCase()])},I(o(v.target_day)),3)):Ut("",!0),v.completion_note?(L(),P("span",Mw,I(v.completion_note),1)):Ut("",!0),v.progress&&v.progress.length?(L(),P("span",Sw,I(v.progress[v.progress.length-1]),1)):Ut("",!0),v.completed&&v.completed_at?(L(),P("span",Ew,I(l(v.completed_at)),1)):Ut("",!0)])])],2))),128))])):(L(),P("div",fw,"还没有计划"))])),64))]),s.value.length?(L(),P("div",Tw,[u("div",ww,[u("h3",null,[p[4]||(p[4]=ue("状态变更历史 ",-1)),u("span",Aw,I(s.value.length)+" 条",1)])]),u("div",Cw,[u("table",null,[p[5]||(p[5]=u("thead",null,[u("tr",null,[u("th",null,"时间"),u("th",null,"类型"),u("th",null,"内容")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(s.value.slice().reverse(),(f,v)=>(L(),P("tr",{key:v},[u("td",Rw,I(l(f.time)),1),u("td",null,[u("span",Lw,I(f.kind),1)]),u("td",null,I(f.content),1)]))),128))])])])])):Ut("",!0),p[6]||(p[6]=vs('<div class="card" data-v-4d7162d3><div class="card-header" data-v-4d7162d3><h3 data-v-4d7162d3>计划系统说明</h3></div><div class="info-list" data-v-4d7162d3><div class="info-item" data-v-4d7162d3>日/周/月共用一套模型：每条计划有稳定编号（d/w/m 前缀），她通过编号指认要动哪一条。</div><div class="info-item" data-v-4d7162d3>每日计划：跨天时自动生成当日的 2-4 件事。</div><div class="info-item" data-v-4d7162d3>每周计划：跨周时生成周目标并分配到具体某天。</div><div class="info-item" data-v-4d7162d3>每月计划：跨月时生成本月目标。</div><div class="info-item" data-v-4d7162d3>完成判定由运行时自动记录：她通过 finish_plan 完成、用 note_progress 记进展、用 add_plan 临时加事。</div><div class="info-item" data-v-4d7162d3>本页只读展示每日、每周、每月计划，定时从后端重新读取，不维护自己的状态副本。</div><div class="info-item" data-v-4d7162d3>数据存储于 data/plugin_ai_chat/</div></div></div>',1))]))}},Dw=Me(Pw,[["__scopeId","data-v-4d7162d3"]]),Uw={class:"stat-grid"},Iw={class:"stat-label"},Nw={class:"stat-sub"},kw={class:"chart-grid"},Fw={class:"card chart-card"},Ow={class:"chart-container"},Bw={viewBox:"0 0 160 160",width:"170",height:"170"},zw=["stroke-dasharray"],Hw={x:"80",y:"74","text-anchor":"middle",fill:"var(--text)","font-size":"20","font-weight":"700"},Vw={class:"chart-legend"},Gw={class:"legend-item"},Ww={class:"legend-item"},$w={class:"legend-item"},Xw={class:"card chart-card"},qw={class:"chart-container"},jw={viewBox:"0 0 160 160",width:"170",height:"170"},Yw=["stroke-dasharray"],Kw={x:"80",y:"74","text-anchor":"middle",fill:"var(--text)","font-size":"20","font-weight":"700"},Zw={class:"card"},Jw={class:"card-header"},Qw={key:0,class:"empty"},tA={key:1},eA={class:"pr-name"},nA={class:"pr-bars"},iA={class:"pr-stat"},sA={class:"mono"},rA={class:"pr-stat"},oA={class:"mono"},aA={class:"pr-stat"},lA={class:"mono"},cA={class:"pr-stat"},uA={class:"mono"},dA={class:"pr-bar-wrap"},hA={class:"card"},fA={key:0,class:"empty"},pA={key:1,class:"table-wrap"},mA={class:"mono"},_A={class:"tag"},gA={class:"mono",style:{"font-size":"11px"}},vA={class:"mono"},xA={class:"mono"},yA={class:"mono",style:{"font-weight":"600"}},bA={class:"mono",style:{color:"var(--success)"}},MA={class:"mono",style:{color:"var(--danger)"}},SA={__name:"AnalyticsView",setup(n){const t=yt({}),e=yt("total_tokens");function i(v){return v==null?"-":v>=1e6?(v/1e6).toFixed(1)+"M":v>=1e3?(v/1e3).toFixed(1)+"K":String(v)}function s(v){return v?new Date(v*1e3).toLocaleString("zh-CN"):"-"}function r(v){if(v.cache_hit>0){var g=v.cache_hit+v.cache_miss;return(v.cache_hit/Math.max(g,1)*100).toFixed(1)+"%"}return"-"}const a=ee(()=>t.value.total_tokens||0),o=ee(()=>Math.max(...c.value.map(g=>g.total_tokens),1)),l=ee(()=>[{label:"总调用次数",value:t.value.total_calls??"-",sub:"API 请求",color:"var(--primary)"},{label:"总 Tokens",value:i(t.value.total_tokens),sub:"Prompt + Completion",color:"var(--info)"},{label:"平均每次",value:t.value.total_calls>0?i(Math.round(t.value.total_tokens/t.value.total_calls)):"-",sub:"Tokens/调用",color:"var(--success)"},{label:"缓存命中率",value:t.value.cache_hit_ratio||"0%",sub:"节省 Tokens",color:"var(--warning)"}]),c=ee(()=>{const v=t.value.by_prompt||[],g=e.value;return[...v].sort((_,m)=>(m[g]||0)-(_[g]||0))}),d=ee(()=>t.value.recent||[]),h=ee(()=>{const v=a.value,g=Math.min(v/1e7,1),_=2*Math.PI*64;return`${_*g} ${_*(1-g)}`}),p=ee(()=>{const v=t.value.total_calls||0,g=Math.min(v/5e3,1),_=2*Math.PI*64;return`${_*g} ${_*(1-g)}`});async function f(){try{t.value=await Ot("/api/analytics")}catch{}}return ve(()=>{f(),window.addEventListener("refresh-all",f)}),(v,g)=>(L(),P("div",null,[u("div",Uw,[(L(!0),P(Et,null,It(l.value,_=>(L(),P("div",{class:"card",key:_.label},[u("div",{class:"stat-value",style:se({color:_.color})},I(_.value),5),u("div",Iw,I(_.label),1),u("div",Nw,I(_.sub),1)]))),128))]),u("div",kw,[u("div",Fw,[g[6]||(g[6]=u("h3",{class:"card-title"},"Token 分布",-1)),u("div",Ow,[(L(),P("svg",Bw,[g[1]||(g[1]=u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--surface)","stroke-width":"20"},null,-1)),u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--primary)","stroke-width":"20","stroke-dasharray":h.value,"stroke-dashoffset":"0",transform:"rotate(-90 80 80)","stroke-linecap":"round",style:{transition:"stroke-dasharray 1s ease"}},null,8,zw),u("text",Hw,I(i(a.value)),1),g[2]||(g[2]=u("text",{x:"80",y:"94","text-anchor":"middle",fill:"var(--text-2)","font-size":"10"},"总 Tokens",-1))])),u("div",Vw,[u("div",Gw,[g[3]||(g[3]=u("span",{class:"dot",style:{background:"var(--primary)"}},null,-1)),ue(" Prompt: "+I(i(t.value.total_prompt_tokens||0)),1)]),u("div",Ww,[g[4]||(g[4]=u("span",{class:"dot",style:{background:"var(--info)"}},null,-1)),ue(" Completion: "+I(i(t.value.total_completion_tokens||0)),1)]),u("div",$w,[g[5]||(g[5]=u("span",{class:"dot",style:{background:"var(--success)"}},null,-1)),ue(" 缓存命中: "+I(t.value.cache_hit_ratio||"0%"),1)])])])]),u("div",Xw,[g[10]||(g[10]=u("h3",{class:"card-title"},"调用分布",-1)),u("div",qw,[(L(),P("svg",jw,[g[7]||(g[7]=u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--surface)","stroke-width":"20"},null,-1)),u("circle",{cx:"80",cy:"80",r:"64",fill:"none",stroke:"var(--warning)","stroke-width":"20","stroke-dasharray":p.value,"stroke-dashoffset":"0",transform:"rotate(-90 80 80)","stroke-linecap":"round",style:{transition:"stroke-dasharray 1s ease"}},null,8,Yw),u("text",Kw,I(t.value.total_calls||0),1),g[8]||(g[8]=u("text",{x:"80",y:"94","text-anchor":"middle",fill:"var(--text-2)","font-size":"10"},"总调用",-1))])),g[9]||(g[9]=vs('<div class="chart-legend" data-v-aedd035b><div class="legend-item" data-v-aedd035b><span class="dot" style="background:var(--warning);" data-v-aedd035b></span> API 调用次数</div><div class="legend-item" data-v-aedd035b><span class="dot" style="background:var(--accent);" data-v-aedd035b></span> 缓存命中率</div></div>',1))])])]),u("div",Zw,[u("div",Jw,[g[12]||(g[12]=u("h3",null,"按 Prompt 类型统计",-1)),Ee(u("select",{"onUpdate:modelValue":g[0]||(g[0]=_=>e.value=_),class:"glass-select"},[...g[11]||(g[11]=[u("option",{value:"total_tokens"},"按总 Tokens",-1),u("option",{value:"calls"},"按调用次数",-1),u("option",{value:"avg_total"},"按平均 Tokens",-1)])],512),[[si,e.value]])]),c.value.length?(L(),P("div",tA,[(L(!0),P(Et,null,It(c.value,(_,m)=>(L(),P("div",{key:m,class:"prompt-row"},[u("div",eA,I(_.name),1),u("div",nA,[u("div",iA,[g[13]||(g[13]=u("label",null,"调用",-1)),u("span",sA,I(_.calls),1)]),u("div",rA,[g[14]||(g[14]=u("label",null,"Token",-1)),u("span",oA,I(i(_.total_tokens)),1)]),u("div",aA,[g[15]||(g[15]=u("label",null,"平均",-1)),u("span",lA,I(i(_.avg_total)),1)]),u("div",cA,[g[16]||(g[16]=u("label",null,"缓存",-1)),u("span",uA,I(r(_)),1)])]),u("div",dA,[u("div",{class:"pr-bar bar-pt",style:se({width:Math.min(_.prompt_tokens/o.value*100,100)+"%"}),title:"Prompt Tokens"},null,4),u("div",{class:"pr-bar bar-ct",style:se({width:Math.min(_.completion_tokens/o.value*100,100)+"%"}),title:"Completion Tokens"},null,4)])]))),128))])):(L(),P("div",Qw,"暂无数据"))]),u("div",hA,[g[18]||(g[18]=u("div",{class:"card-header"},[u("h3",null,"最近调用")],-1)),d.value.length?(L(),P("div",pA,[u("table",null,[g[17]||(g[17]=u("thead",null,[u("tr",null,[u("th",null,"时间"),u("th",null,"Prompt"),u("th",null,"模型"),u("th",null,"Prompt Tokens"),u("th",null,"Completion"),u("th",null,"总 Tokens"),u("th",null,"缓存命中"),u("th",null,"缓存未中")])],-1)),u("tbody",null,[(L(!0),P(Et,null,It(d.value,(_,m)=>(L(),P("tr",{key:m},[u("td",mA,I(s(_.time)),1),u("td",null,[u("span",_A,I(_.prompt),1)]),u("td",gA,I(_.model?.split("/")?.pop()||_.model),1),u("td",vA,I(_.prompt_tokens),1),u("td",xA,I(_.completion_tokens),1),u("td",yA,I(_.total_tokens),1),u("td",bA,I(_.cache_hit||"-"),1),u("td",MA,I(_.cache_miss||"-"),1)]))),128))])])])):(L(),P("div",fA,"暂无记录"))])]))}},EA=Me(SA,[["__scopeId","data-v-aedd035b"]]),TA={class:"stat-grid"},wA=["innerHTML"],AA={class:"stat-value"},CA={class:"stat-label"},RA={class:"stat-sub"},LA={class:"toggle-grid"},PA={class:"card-row"},DA={key:0,class:"card half"},UA={class:"battery-visual"},IA={class:"kv-grid"},NA={class:"kv"},kA={class:"kv"},FA={class:"kv"},OA={class:"kv"},BA={class:"kv"},zA={key:1,class:"card half"},HA={class:"rhythm-bars"},VA={class:"rhythm-label"},GA={class:"rhythm-bar-wrap"},WA={class:"rhythm-pct"},$A={class:"kv-grid",style:{"margin-top":"12px"}},XA={class:"kv"},qA={class:"kv"},jA={class:"card-row"},YA={key:0,class:"card half"},KA={class:"kv-grid"},ZA={class:"kv"},JA={class:"kv"},QA={class:"kv"},t2={class:"kv"},e2={key:1,class:"card half"},n2={class:"kv-grid"},i2={class:"kv"},s2={class:"kv"},r2={class:"kv"},o2={class:"kv"},a2={key:0,class:"card"},l2={__name:"HumanityView",setup(n){const t=yt(null),e={social_battery:"社交电量",circadian:"昼夜节律",attention:"注意力",cognitive_biases:"认知偏差",response_timing:"变速回复",unpredictability:"不可预测性"},i=ee(()=>{if(!t.value?.circadian)return[];const o=t.value.circadian;return[{label:"精力",value:o.energy_level,color:"var(--info)"},{label:"思维",value:o.cognitive_clarity,color:"var(--primary)"},{label:"耐心",value:o.patience_level,color:"var(--success)"},{label:"社交",value:o.sociability,color:"var(--warning)"},{label:"幽默",value:o.humor_sensitivity,color:"var(--accent)"}]}),s=ee(()=>{if(!t.value?.social_battery)return"var(--success)";const o=t.value.social_battery.percentage;return o<.15?"var(--danger)":o<.3?"var(--accent)":o<.5?"var(--warning)":"var(--success)"}),r=ee(()=>{const o=t.value,l={battery:'<svg viewBox="0 0 20 20" fill="none" width="22" height="22"><rect x="2" y="6" width="14" height="10" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M17 9v4M6 9v4M10 9v4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',brain:'<svg viewBox="0 0 20 20" fill="none" width="22" height="22"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><path d="M7 8h6M7 11h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',heart:'<svg viewBox="0 0 20 20" fill="none" width="22" height="22"><path d="M10 17c-3-2-6-4.5-6-8a4 4 0 016-2.5A4 4 0 0116 9c0 3.5-3 6-6 8z" stroke="currentColor" stroke-width="1.5"/></svg>'};return[{label:"电量",value:o?.social_battery?(o.social_battery.percentage*100).toFixed(0)+"%":"—",sub:"社交电量",color:"var(--success)",icon:l.battery},{label:"注意力",value:o?.attention?(o.attention.attention_level*100).toFixed(0)+"%":"—",sub:"注意力水平",color:"var(--info)",icon:l.brain},{label:"关系",value:o?.relationship_count??"—",sub:"已记录",color:"var(--accent)",icon:l.heart}]});async function a(){try{t.value=await Ot("/api/humanity")}catch{}}return ve(()=>{a(),window.addEventListener("refresh-all",a)}),(o,l)=>(L(),P("div",null,[u("div",TA,[(L(!0),P(Et,null,It(r.value,c=>(L(),P("div",{class:"card",key:c.label},[u("div",{class:"stat-icon",style:se({color:c.color}),innerHTML:c.icon},null,12,wA),u("div",AA,I(c.value??"-"),1),u("div",CA,I(c.label),1),u("div",RA,I(c.sub),1)]))),128))]),l[20]||(l[20]=u("div",{class:"section-title"},"功能开关",-1)),u("div",LA,[(L(!0),P(Et,null,It(t.value?.config_enabled||{},(c,d)=>(L(),P("div",{key:d,class:ie(["toggle-chip",{on:c}])},I(e[d]||d),3))),128))]),u("div",PA,[t.value?.social_battery?(L(),P("div",DA,[l[5]||(l[5]=u("h3",null,"社交电量",-1)),u("div",UA,[u("div",{class:"battery-fill",style:se({width:t.value.social_battery.percentage*100+"%",background:s.value})},null,4)]),u("div",IA,[u("div",NA,[l[0]||(l[0]=u("span",null,"电量",-1)),u("strong",null,I(t.value.social_battery.level.toFixed(1))+" / "+I(t.value.social_battery.capacity),1)]),u("div",kA,[l[1]||(l[1]=u("span",null,"百分比",-1)),u("strong",null,I((t.value.social_battery.percentage*100).toFixed(1))+"%",1)]),u("div",FA,[l[2]||(l[2]=u("span",null,"倦怠",-1)),u("strong",{style:se({color:t.value.social_battery.is_burned_out?"var(--danger)":"var(--success)"})},I(t.value.social_battery.is_burned_out?"是":"否"),5)]),u("div",OA,[l[3]||(l[3]=u("span",null,"模式",-1)),u("strong",null,I(t.value.social_battery.is_passive_mode?"被动":"主动"),1)]),u("div",BA,[l[4]||(l[4]=u("span",null,"活跃分钟",-1)),u("strong",null,I(t.value.social_battery.active_minutes),1)])])])):Ut("",!0),t.value?.circadian?(L(),P("div",zA,[l[8]||(l[8]=u("h3",null,"昼夜节律",-1)),u("div",HA,[(L(!0),P(Et,null,It(i.value,c=>(L(),P("div",{key:c.label,class:"rhythm-row"},[u("span",VA,I(c.label),1),u("div",GA,[u("div",{class:"rhythm-bar",style:se({width:c.value*100+"%",background:c.color})},null,4)]),u("span",WA,I((c.value*100).toFixed(0))+"%",1)]))),128))]),u("div",$A,[u("div",XA,[l[6]||(l[6]=u("span",null,"当前时间",-1)),u("strong",null,I(t.value.circadian.current_hour.toFixed(1))+"时",1)]),u("div",qA,[l[7]||(l[7]=u("span",null,"免打扰",-1)),u("strong",{style:se({color:t.value.circadian.is_quiet_hours?"var(--warning)":"var(--success)"})},I(t.value.circadian.is_quiet_hours?"是":"否"),5)])])])):Ut("",!0)]),u("div",jA,[t.value?.attention?(L(),P("div",YA,[l[13]||(l[13]=u("h3",null,"注意力",-1)),u("div",KA,[u("div",ZA,[l[9]||(l[9]=u("span",null,"注意力水平",-1)),u("strong",null,I((t.value.attention.attention_level*100).toFixed(0))+"%",1)]),u("div",JA,[l[10]||(l[10]=u("span",null,"心流状态",-1)),u("strong",null,I((t.value.attention.flow_state*100).toFixed(0))+"%",1)]),u("div",QA,[l[11]||(l[11]=u("span",null,"专注话题",-1)),u("strong",null,I(t.value.attention.focused_topic||"—"),1)]),u("div",t2,[l[12]||(l[12]=u("span",null,"心流恢复中",-1)),u("strong",{style:se({color:t.value.attention.flow_recovering?"var(--warning)":"var(--success)"})},I(t.value.attention.flow_recovering?"是":"否"),5)])])])):Ut("",!0),t.value?.cognitive_biases?(L(),P("div",e2,[l[18]||(l[18]=u("h3",null,"认知偏差",-1)),u("div",n2,[u("div",i2,[l[14]||(l[14]=u("span",null,"确认偏误",-1)),u("strong",null,I(t.value.cognitive_biases.confirmation_bias.toFixed(2)),1)]),u("div",s2,[l[15]||(l[15]=u("span",null,"情绪一致性",-1)),u("strong",null,I(t.value.cognitive_biases.mood_congruence.toFixed(2)),1)]),u("div",r2,[l[16]||(l[16]=u("span",null,"锚定效应",-1)),u("strong",null,I(t.value.cognitive_biases.anchoring_strength.toFixed(2)),1)]),u("div",o2,[l[17]||(l[17]=u("span",null,"可得性启发",-1)),u("strong",null,I(t.value.cognitive_biases.availability_heuristic.toFixed(2)),1)])])])):Ut("",!0)]),!t.value||!t.value.social_battery&&!t.value.circadian&&!t.value.attention?(L(),P("div",a2,[...l[19]||(l[19]=[u("div",{class:"empty"},"人性化功能未启用，请在配置中开启相关选项",-1)])])):Ut("",!0)]))}},c2=Me(l2,[["__scopeId","data-v-057a939c"]]),u2={class:"card"},d2={class:"card-header"},h2={class:"header-actions"},f2={key:0,class:"empty"},p2={key:1,class:"rel-grid"},m2=["onClick"],_2={class:"rel-header"},g2={class:"rel-uid"},v2={class:"rel-bars"},x2={class:"rel-bar-label"},y2={class:"rel-bar-wrap"},b2={class:"rel-bar-pct"},M2={class:"rel-footer"},S2={class:"card modal modal-lg"},E2={class:"modal-header"},T2={class:"detail-grid"},w2={class:"detail-section"},A2={class:"kv-grid"},C2={class:"kv"},R2={class:"kv"},L2={class:"kv"},P2={class:"kv"},D2={class:"kv"},U2={class:"kv"},I2={class:"kv"},N2={class:"kv"},k2={class:"detail-section"},F2={class:"kv-grid"},O2={class:"kv"},B2={class:"kv"},z2={class:"kv"},H2={style:{color:"var(--success)"}},V2={class:"kv"},G2={style:{color:"var(--danger)"}},W2={class:"kv"},$2={class:"kv"},X2={key:0,class:"detail-section",style:{"margin-top":"12px"}},q2={class:"impression-text"},j2={key:1,class:"detail-section",style:{"margin-top":"12px"}},Y2={class:"event-list"},K2={class:"event-type"},Z2={key:0,class:"event-detail"},J2={class:"event-time"},Q2={__name:"RelationshipsView",setup(n){const t=yt({}),e=yt(""),i=yt(null),s=yt(null),r=ee(()=>{if(!e.value)return t.value;const p=e.value,f={};for(const[v,g]of Object.entries(t.value))v.includes(p)&&(f[v]=g);return f});function a(p){return[{label:"信任",value:p.trust||0,color:"var(--info)"},{label:"亲密",value:p.intimacy||0,color:"var(--accent)"},{label:"好感",value:p.affection||0,color:"var(--warning)"},{label:"互惠",value:p.reciprocity||.5,color:"var(--success)"}]}function o(p){return{stranger:"var(--text-3)",acquaintance:"var(--info)",regular:"var(--success)",close:"var(--accent)",confidant:"var(--warning)",antagonistic:"var(--danger)",admiring:"var(--primary)"}[p]||"var(--text-3)"}function l(p){return{stranger:"陌生人",acquaintance:"认识",regular:"常客",close:"亲近",confidant:"知己",antagonistic:"对立",admiring:"仰慕"}[p]||p}function c(p){if(!p)return"";const f=Math.floor(Date.now()/1e3)-p;return f<60?"刚刚":f<3600?Math.floor(f/60)+"分钟前":f<86400?Math.floor(f/3600)+"小时前":Math.floor(f/86400)+"天前"}async function d(p){try{s.value=p,i.value=await Ot(`/api/relationships/${p}`)}catch{i.value=null}}async function h(){try{const p=await Ot("/api/relationships");t.value=p.relationships||{}}catch{}}return ve(()=>{h(),window.addEventListener("refresh-all",h)}),(p,f)=>(L(),P("div",null,[u("div",u2,[u("div",d2,[f[3]||(f[3]=u("h3",null,"关系管理",-1)),u("div",h2,[Ee(u("input",{"onUpdate:modelValue":f[0]||(f[0]=v=>e.value=v),placeholder:"搜索用户ID...",class:"glass-input",style:{width:"150px"}},null,512),[[Je,e.value]]),u("button",{class:"btn btn-ghost btn-sm",onClick:h},"↻ 刷新")])]),Object.keys(t.value).length?(L(),P("div",p2,[(L(!0),P(Et,null,It(r.value,(v,g)=>(L(),P("div",{key:g,class:"rel-card",onClick:_=>d(g)},[u("div",_2,[u("span",g2,"用户 "+I(g),1),u("span",{class:"rel-type",style:se({background:o(v.relationship_type)})},I(l(v.relationship_type)),5)]),u("div",v2,[(L(!0),P(Et,null,It(a(v),_=>(L(),P("div",{class:"rel-bar-row",key:_.label},[u("span",x2,I(_.label),1),u("div",y2,[u("div",{class:"rel-bar",style:se({width:_.value*100+"%",background:_.color})},null,4)]),u("span",b2,I((_.value*100).toFixed(0))+"%",1)]))),128))]),u("div",M2,[u("span",null,"互动 "+I(v.interaction_count||0)+"次",1),u("span",null,I(c(v.last_interaction)),1)])],8,m2))),128))])):(L(),P("div",f2,"暂无关系数据"))]),i.value?(L(),P("div",{key:0,class:"modal-overlay",onClick:f[2]||(f[2]=il(v=>i.value=null,["self"]))},[u("div",S2,[u("div",E2,[u("h3",null,"用户 "+I(s.value)+" 的关系详情",1),u("button",{class:"btn btn-ghost btn-xs",onClick:f[1]||(f[1]=v=>i.value=null)},"✕")]),u("div",T2,[u("div",w2,[f[12]||(f[12]=u("h4",null,"关系维度",-1)),u("div",A2,[u("div",C2,[f[4]||(f[4]=u("span",null,"信任",-1)),u("strong",null,I((i.value.trust*100).toFixed(0))+"%",1)]),u("div",R2,[f[5]||(f[5]=u("span",null,"亲密",-1)),u("strong",null,I((i.value.intimacy*100).toFixed(0))+"%",1)]),u("div",L2,[f[6]||(f[6]=u("span",null,"默契",-1)),u("strong",null,I((i.value.rapport*100).toFixed(0))+"%",1)]),u("div",P2,[f[7]||(f[7]=u("span",null,"好感",-1)),u("strong",null,I((i.value.affection*100).toFixed(0))+"%",1)]),u("div",D2,[f[8]||(f[8]=u("span",null,"互惠",-1)),u("strong",null,I((i.value.reciprocity*100).toFixed(0))+"%",1)]),u("div",U2,[f[9]||(f[9]=u("span",null,"紧张",-1)),u("strong",{style:se({color:i.value.tension>.5?"var(--danger)":"inherit"})},I((i.value.tension*100).toFixed(0))+"%",5)]),u("div",I2,[f[10]||(f[10]=u("span",null,"烦躁",-1)),u("strong",{style:se({color:i.value.annoyance>.5?"var(--warning)":"inherit"})},I((i.value.annoyance*100).toFixed(0))+"%",5)]),u("div",N2,[f[11]||(f[11]=u("span",null,"好奇",-1)),u("strong",null,I((i.value.curiosity*100).toFixed(0))+"%",1)])])]),u("div",k2,[f[19]||(f[19]=u("h4",null,"互动统计",-1)),u("div",F2,[u("div",O2,[f[13]||(f[13]=u("span",null,"关系类型",-1)),u("strong",null,I(l(i.value.relationship_type)),1)]),u("div",B2,[f[14]||(f[14]=u("span",null,"总互动",-1)),u("strong",null,I(i.value.interaction_count),1)]),u("div",z2,[f[15]||(f[15]=u("span",null,"积极互动",-1)),u("strong",H2,I(i.value.positive_interactions),1)]),u("div",V2,[f[16]||(f[16]=u("span",null,"消极互动",-1)),u("strong",G2,I(i.value.negative_interactions),1)]),u("div",W2,[f[17]||(f[17]=u("span",null,"连续忽视",-1)),u("strong",{style:se({color:i.value.ignore_streak>2?"var(--warning)":"inherit"})},I(i.value.ignore_streak)+"次",5)]),u("div",$2,[f[18]||(f[18]=u("span",null,"共享记忆",-1)),u("strong",null,I(i.value.shared_memories_count),1)])])])]),i.value.impression?(L(),P("div",X2,[f[20]||(f[20]=u("h4",null,"印象",-1)),u("div",q2,I(i.value.impression),1)])):Ut("",!0),(i.value.recent_events||[]).length?(L(),P("div",j2,[f[21]||(f[21]=u("h4",null,"最近事件",-1)),u("div",Y2,[(L(!0),P(Et,null,It(i.value.recent_events,(v,g)=>(L(),P("div",{key:g,class:"event-item"},[u("span",K2,I(v.event),1),v.detail?(L(),P("span",Z2,I(v.detail),1)):Ut("",!0),u("span",J2,I(c(v.timestamp)),1)]))),128))])])):Ut("",!0)])])):Ut("",!0)]))}},tC=Me(Q2,[["__scopeId","data-v-02592b70"]]),eC={key:0,class:"login-page"},nC={class:"login-card"},iC={class:"login-input-group"},sC=["disabled"],rC={key:0,class:"login-err"},oC={key:1,class:"login-footer num"},aC={key:1,class:"shell"},lC={class:"rail-scroll"},cC={key:0,class:"rail-sep"},uC={class:"rail-group-label"},dC=["onClick","title"],hC=["innerHTML"],fC={class:"rail-text"},pC={class:"topbar"},mC={class:"tb-left"},_C={class:"tb-title"},gC={class:"tb-sub"},vC={class:"tb-right"},xC=["title"],yC={key:0,viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},bC={key:1,viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},MC={class:"stage"},SC={__name:"App",setup(n){const t={core:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7.5" stroke="currentColor" stroke-width="1.5"/><circle cx="10" cy="10" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M10 2.5v2M10 15.5v2M2.5 10h2M15.5 10h2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',dashboard:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="11" y="3" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="3" y="11" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/><rect x="11" y="11" width="6" height="6" rx="1.5" stroke="currentColor" stroke-width="1.5"/></svg>',analytics:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 17V7l4 4 3-6 3 3 4-5v14H3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></svg>',schedule:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="4" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M3 8h14M7 1v3M13 1v3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',config:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M10 1v2M10 17v2M1 10h2M17 10h2M4.22 4.22l1.42 1.42M14.36 14.36l1.42 1.42M4.22 15.78l1.42-1.42M14.36 5.64l1.42-1.42" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',conversations:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 10a7 7 0 1114 0 7 7 0 01-7 7H3l2-3a7 7 0 01-2-4z" stroke="currentColor" stroke-width="1.5"/><path d="M7 8h6M7 11h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',quota:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><path d="M10 6v4l3 2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',sticker:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><circle cx="7.5" cy="8.5" r="1.5" fill="currentColor"/><path d="M5 15l3-4 3 4H5z" fill="currentColor" opacity="0.5"/><path d="M11 13l3-5 3 5H11z" fill="currentColor" opacity="0.5"/></svg>',mind:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M2 10h3l2-5 3 10 2-5h6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',memory:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="4" y="3" width="12" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h6M7 10h6" stroke="currentColor" stroke-width="1.5"/></svg>',"working-memory":'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="3" width="14" height="14" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h6M7 10h6" stroke="currentColor" stroke-width="1.5"/></svg>',stream:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M2 6c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M2 10c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.7"/><path d="M2 14c3-2 5 2 8 0s5 2 8 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4"/></svg>',emotion:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><circle cx="7.5" cy="8.5" r="1" fill="currentColor"/><circle cx="12.5" cy="8.5" r="1" fill="currentColor"/><path d="M7 12.5c.8 1 2 1.5 3 1.5s2.2-.5 3-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',diary:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M4 3h11a1 1 0 011 1v12a1 1 0 01-1 1H4V3z" stroke="currentColor" stroke-width="1.5"/><path d="M4 3v14M13 3v14" stroke="currentColor" stroke-width="1.5"/><path d="M7 7h4M7 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',humanity:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 3a7 7 0 017 7c0 2.5-1 4.5-2.5 5.5S10 17 10 17s-3.5-.5-4.5-1.5S3 12.5 3 10a7 7 0 017-7z" stroke="currentColor" stroke-width="1.5"/><circle cx="8" cy="9" r="1" fill="currentColor"/><circle cx="12" cy="9" r="1" fill="currentColor"/><path d="M7 12.5c.8 1 2 1.5 3 1.5s2.2-.5 3-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',blocklist:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="10" cy="10" r="7" stroke="currentColor" stroke-width="1.5"/><path d="M6 6l8 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',"anti-injection":'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 2l7 3v5c0 4-3 7-7 8-4-1-7-4-7-8V5l7-3z" stroke="currentColor" stroke-width="1.5"/><path d="M7 10l2 2 4-4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',archive:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><rect x="3" y="6" width="14" height="11" rx="2" stroke="currentColor" stroke-width="1.5"/><path d="M2 4a1 1 0 011-1h14a1 1 0 011 1v2H2V4z" stroke="currentColor" stroke-width="1.5"/><path d="M8 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>',backups:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M10 3a5 5 0 00-4.5 2.8A4 4 0 003 10a4 4 0 004 4h1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M10 3a5 5 0 014.5 2.8A4 4 0 0117 10a4 4 0 01-4 4h-1.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><path d="M10 10v5M7 12.5l3-2.5 3 2.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>',"memory-ops-log":'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 3h14v14H3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/><path d="M7 7h6M7 10h4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><circle cx="14" cy="14" r="2" fill="currentColor" opacity="0.6"/></svg>',relationships:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="7" cy="7" r="3" stroke="currentColor" stroke-width="1.5"/><circle cx="13" cy="7" r="3" stroke="currentColor" stroke-width="1.5"/><path d="M3 17c0-2.2 1.8-4 4-4s4 1.8 4 4M9 17c0-2.2 1.8-4 4-4s4 1.8 4 4" stroke="currentColor" stroke-width="1.5"/></svg>',security:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><path d="M3 3h14l-5.5 6.5V16l-3 1.5v-8L3 3z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round"/></svg>',social:'<svg viewBox="0 0 20 20" fill="none" width="18" height="18"><circle cx="6" cy="6" r="2.5" stroke="currentColor" stroke-width="1.5"/><circle cx="14.5" cy="8" r="2" stroke="currentColor" stroke-width="1.5"/><circle cx="9" cy="14" r="2" stroke="currentColor" stroke-width="1.5"/><path d="M7.5 7.5L13 9M7.8 12.5L10.5 9.5" stroke="currentColor" stroke-width="1.2" stroke-linecap="round" opacity="0.6"/></svg>'},e=yt(!1),i=yt(""),s=yt(""),r=yt("core"),a=yt(!1),o=yt(""),l=yt(!0),c=yt("online");let d=null;const h=[{id:"core",name:"控制总览",group:"概览",icon:t.core,desc:"AI 情绪核心 · 四维实时频谱",comp:jy},{id:"mind",name:"此刻",group:"心灵",icon:t.mind,desc:"她的此刻：状态、身体与意识流",comp:Sb},{id:"stream",name:"意识流",group:"心灵",icon:t.stream,desc:"她的人生时间线",comp:qb},{id:"social",name:"群里的势",group:"心灵",icon:t.social,desc:"她的社会感知：话题线、注意力、关系与等待",comp:fM},{id:"mind-memory",name:"日记与人物",group:"心灵",icon:t.diary,desc:"她的日记与人物档案",comp:x1},{id:"security",name:"滤壳与安全",group:"心灵",icon:t.security,desc:"滤壳审计记录",comp:U1},{id:"dashboard",name:"仪表盘",group:"概览",icon:t.dashboard,desc:"系统总览与关键指标",comp:KM},{id:"analytics",name:"Token 分析",group:"概览",icon:t.analytics,desc:"API 用量与 Prompt 统计",comp:EA},{id:"schedule",name:"日程计划",group:"管理",icon:t.schedule,desc:"周/月计划管理",comp:Dw},{id:"config",name:"配置",group:"管理",icon:t.config,desc:"Bot 与 AI 参数",comp:yS},{id:"conversations",name:"对话管理",group:"管理",icon:t.conversations,desc:"活跃会话控制",comp:kS},{id:"quota",name:"配额",group:"管理",icon:t.quota,desc:"回复配额",comp:HS},{id:"sticker",name:"表情包",group:"管理",icon:t.sticker,desc:"表情管理",comp:oE},{id:"memory",name:"用户记忆",group:"数据",icon:t.memory,desc:"长期记忆",comp:ME},{id:"working-memory",name:"工作记忆",group:"数据",icon:t["working-memory"],desc:"短期工作记忆",comp:NE},{id:"memory-ops-log",name:"内存监视",group:"数据",icon:t["memory-ops-log"],desc:"内存操作日志",comp:iw},{id:"emotion",name:"情绪",group:"数据",icon:t.emotion,desc:"情绪状态",comp:jE},{id:"humanity",name:"人性化",group:"数据",icon:t.humanity,desc:"人性化状态监控",comp:c2},{id:"relationships",name:"关系",group:"数据",icon:t.relationships,desc:"关系动力学",comp:tC},{id:"blocklist",name:"黑名单",group:"系统",icon:t.blocklist,desc:"用户管理",comp:sT},{id:"anti-injection",name:"防注入",group:"系统",icon:t["anti-injection"],desc:"安全防护",comp:gT},{id:"archive",name:"归档",group:"系统",icon:t.archive,desc:"数据归档",comp:AT},{id:"backups",name:"备份",group:"系统",icon:t.backups,desc:"数据备份",comp:WT}],p=ee(()=>{const D=[];for(const G of h){let T=D.find(R=>R.label===G.group);T||D.push(T={label:G.group,items:[]}),T.items.push(G)}return D}),f=ee(()=>h.find(D=>D.id===r.value)),v=ee(()=>h.find(D=>D.id===r.value)?.comp),g=ee(()=>{const D=f.value;return D?`${D.group} · ${D.desc}`:"NEXUS // SENTIENT CONSOLE"});function _(D){l.value=D,document.documentElement.setAttribute("data-theme",D?"dark":"light"),localStorage.setItem("ai-chat-theme",D?"dark":"light")}function m(){_(!l.value)}function E(){const D=localStorage.getItem("ai-chat-theme");_(D!=="light")}async function b(){s.value="";const D=i.value.trim();if(D)try{await nc(D)?e.value=!0:s.value="Token 验证失败"}catch{s.value="网络错误"}}function x(){Bd(),e.value=!1}function C(){window.dispatchEvent(new CustomEvent("refresh-all"))}async function A(){try{await Ot("/api/version"),c.value="online"}catch{c.value="offline"}}return ve(async()=>{E();const D=rm();if(D)try{await nc(D)&&(e.value=!0)}catch{}try{const G=await Ot("/api/version");o.value=G.version}catch{}A(),d=setInterval(A,3e4)}),An(()=>{d&&clearInterval(d)}),(D,G)=>e.value?(L(),P("div",aC,[u("aside",{class:ie(["rail",{open:a.value}])},[G[5]||(G[5]=u("div",{class:"rail-logo",title:"Luo9 AI Chat"},"洛",-1)),u("div",lC,[(L(!0),P(Et,null,It(p.value,(T,R)=>(L(),P(Et,{key:T.label},[R>0?(L(),P("div",cC)):Ut("",!0),u("div",uC,I(T.label),1),(L(!0),P(Et,null,It(T.items,J=>(L(),P("a",{key:J.id,class:ie(["rail-btn",{active:r.value===J.id}]),onClick:rt=>{r.value=J.id,a.value=!1},title:T.label+" · "+J.name},[u("span",{class:"rail-icon",innerHTML:J.icon},null,8,hC),u("span",fC,I(J.name),1)],10,dC))),128))],64))),128))])],2),a.value?(L(),P("div",{key:0,class:"overlay",onClick:G[1]||(G[1]=T=>a.value=!1)})):Ut("",!0),u("header",pC,[u("div",mC,[u("button",{class:"menu-btn",onClick:G[2]||(G[2]=T=>a.value=!a.value),"aria-label":"菜单"},[...G[6]||(G[6]=[u("svg",{viewBox:"0 0 20 20",fill:"none",width:"18",height:"18"},[u("path",{d:"M3 5h14M3 10h14M3 15h14",stroke:"currentColor","stroke-width":"1.5","stroke-linecap":"round"})],-1)])]),u("span",_C,I(f.value?.name||"此刻"),1),u("span",gC,I(g.value),1)]),u("div",vC,[u("div",{class:ie(["tb-status",{offline:c.value==="offline"}])},[G[7]||(G[7]=u("span",{class:"dot"},null,-1)),u("span",null,I(c.value==="offline"?"核心离线":"核心在线 · 情感引擎活跃"),1)],2),u("button",{class:"tb-icon",onClick:m,title:l.value?"切换亮金模式":"切换黑金模式"},[l.value?(L(),P("svg",yC,[...G[8]||(G[8]=[u("circle",{cx:"10",cy:"10",r:"4",stroke:"currentColor","stroke-width":"1.5"},null,-1),u("path",{d:"M10 2v2M10 16v2M2 10h2M16 10h2M4.22 4.22l1.42 1.42M14.36 14.36l1.42 1.42M4.22 15.78l1.42-1.42M14.36 5.64l1.42-1.42",stroke:"currentColor","stroke-width":"1.5","stroke-linecap":"round"},null,-1)])])):(L(),P("svg",bC,[...G[9]||(G[9]=[u("path",{d:"M10 2a8 8 0 100 16 6 6 0 010-12 6 6 0 000-4z",fill:"currentColor"},null,-1)])]))],8,xC),u("button",{class:"tb-icon",onClick:C,title:"刷新"},[...G[10]||(G[10]=[u("svg",{viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},[u("path",{d:"M14.5 5.5A6.5 6.5 0 104 10.5M14.5 2v3.5H11M5.5 14.5A6.5 6.5 0 0016 9.5M5.5 18V14.5H9",stroke:"currentColor","stroke-width":"1.5","stroke-linecap":"round","stroke-linejoin":"round"})],-1)])]),u("button",{class:"tb-icon",onClick:x,title:"退出"},[...G[11]||(G[11]=[u("svg",{viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},[u("path",{d:"M7 17H4a1 1 0 01-1-1V4a1 1 0 011-1h3M13 14l4-4-4-4M17 10H7",stroke:"currentColor","stroke-width":"1.5","stroke-linecap":"round","stroke-linejoin":"round"})],-1)])])])]),u("main",MC,[(L(),P("div",{class:"page-content",key:r.value},[(L(),Dd(Nf(v.value)))]))])])):(L(),P("div",eC,[u("div",nC,[G[4]||(G[4]=vs('<div class="login-core"><span class="core-ring"></span><span class="core-ring r2"></span><span class="core-dot"></span></div><h1 class="login-title">Luo9 AI Chat</h1><p class="login-sub num">SENTIENT KERNEL · 管理控制台</p>',3)),u("div",iC,[Ee(u("input",{"onUpdate:modelValue":G[0]||(G[0]=T=>i.value=T),type:"password",placeholder:"输入管理员 Token",onKeydown:Qp(b,["enter"]),autofocus:""},null,544),[[Je,i.value]]),u("button",{onClick:b,disabled:!i.value.trim(),class:"login-btn"},[...G[3]||(G[3]=[u("span",null,"进入圣所",-1),u("svg",{viewBox:"0 0 20 20",fill:"none",width:"16",height:"16"},[u("path",{d:"M4 10h12M12 6l4 4-4 4",stroke:"currentColor","stroke-width":"1.6","stroke-linecap":"round","stroke-linejoin":"round"})],-1)])],8,sC)]),s.value?(L(),P("div",rC,I(s.value),1)):Ut("",!0),o.value?(L(),P("div",oC,[u("span",null,"v"+I(o.value),1)])):Ut("",!0)])]))}},EC=nm(SC);EC.mount("#app");const vh=document.createElement("style");vh.textContent=`
  /* ── 黑金圣所 · 高级黑金拟人控制台 ────────────────────────── */
  :root {
    --gold: #F5A623;
    --gold-dim: rgba(245, 166, 35, 0.5);
    --gold-glow: rgba(245, 166, 35, 0.3);
    /* NEXUS 参考稿的命名别名（新旧两套 token 并存） */
    --gold-light: #FFC85C;
    --gold-deep: #B87A12;
    --black: #0A0A0D;
    --text-0: #F2EFE8;
    --text-1: #A8A49B;
    --border-strong: rgba(245, 166, 35, 0.25);
    --font-mono: 'JetBrains Mono', 'Cascadia Code', 'SF Mono', 'Fira Code', Consolas, monospace;
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif;

    /* 亮色 = 金调纸面（不用纯白，保持金色血统） */
    --bg: #F6F0E2;
    --bg-alt: #FBF6EA;
    --surface: rgba(253, 249, 239, 0.82);
    --surface-hover: rgba(255, 252, 244, 0.96);
    --surface-solid: #FDFAF2;
    --glass: rgba(253, 249, 239, 0.7);
    --glass-border: rgba(154, 111, 27, 0.22);
    --glass-shadow: 0 6px 22px rgba(90, 66, 15, 0.10);
    --glass-shadow-lg: 0 12px 34px rgba(90, 66, 15, 0.16);
    --text: #2B230F;
    --text-2: #7C6E4E;
    --text-3: #A99A76;
    --text-0: #2B230F;
    --text-1: #7C6E4E;
    --border-strong: rgba(154, 111, 27, 0.3);
    --primary: #B47A12;
    --primary-hover: #96630C;
    --primary-subtle: rgba(180, 122, 18, 0.10);
    --primary-glow: rgba(180, 122, 18, 0.20);
    --accent: #C15F3C;
    --accent-subtle: rgba(193, 95, 60, 0.10);
    --success: #6E8440;
    --success-subtle: rgba(110, 132, 64, 0.12);
    --warning: #B8862B;
    --warning-subtle: rgba(184, 134, 43, 0.12);
    --danger: #A94438;
    --danger-subtle: rgba(169, 68, 56, 0.10);
    --info: #6F7E8C;
    --info-subtle: rgba(111, 126, 140, 0.12);
    --border: rgba(154, 111, 27, 0.26);
    --border-light: rgba(154, 111, 27, 0.13);
    --chip-solid: rgba(154, 111, 27, 0.06);
    --radius: 14px;
    --radius-sm: 12px;
    --radius-xs: 10px;
    --radius-full: 9999px;
    --transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    --transition-fast: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  }
  [data-theme="dark"] {
    --bg: #0A0A0D;
    --bg-alt: #0E0E13;
    --surface: rgba(18, 18, 22, 0.75);
    --surface-hover: rgba(24, 24, 29, 0.9);
    --surface-solid: #121216;
    --glass: rgba(18, 18, 22, 0.75);
    --glass-border: rgba(245, 166, 35, 0.25);
    --glass-shadow: 0 8px 28px rgba(0, 0, 0, 0.5);
    --glass-shadow-lg: 0 10px 34px rgba(245, 166, 35, 0.08);
    --text: #EAEef2;
    --text-2: #8A8F98;
    --text-3: #5C616A;
    --text-0: #F2EFE8;
    --text-1: #A8A49B;
    --border-strong: rgba(245, 166, 35, 0.25);
    --primary: #F5A623;
    --primary-hover: #FFB84D;
    --primary-subtle: rgba(245, 166, 35, 0.08);
    --primary-glow: rgba(245, 166, 35, 0.18);
    --accent: #E09520;
    --accent-subtle: rgba(224, 149, 32, 0.10);
    --success: #A8B863;
    --success-subtle: rgba(168, 184, 99, 0.12);
    --warning: #D2A754;
    --warning-subtle: rgba(210, 167, 84, 0.12);
    --danger: #D0684F;
    --danger-subtle: rgba(208, 104, 79, 0.12);
    --info: #93A6C0;
    --info-subtle: rgba(147, 166, 192, 0.12);
    --border: rgba(245, 166, 35, 0.20);
    --border-light: rgba(245, 166, 35, 0.09);
    --chip-solid: rgba(245, 166, 35, 0.06);
  }
  *, *::before, *::after { box-sizing: border-box; }
  body {
    font-family: var(--font-sans);
    background: var(--bg);
    color: var(--text);
    margin: 0;
    line-height: 1.5;
    letter-spacing: 0.02em;
    font-variant-numeric: tabular-nums;
    -webkit-font-smoothing: antialiased;
  }
  /* 深空星云 + 极淡能量网格（固定背景层） */
  body::before, body::after {
    content: '';
    position: fixed;
    inset: 0;
    pointer-events: none;
    z-index: 0;
  }
  body::before {
    background:
      radial-gradient(circle at 30% 40%, rgba(245, 166, 35, 0.035) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(245, 166, 35, 0.022) 0%, transparent 40%);
  }
  body::after {
    background-image:
      linear-gradient(rgba(245, 166, 35, 0.02) 1px, transparent 1px),
      linear-gradient(90deg, rgba(245, 166, 35, 0.02) 1px, transparent 1px);
    background-size: 48px 48px;
    opacity: 0.7;
  }
  [data-theme="light"] body::after { opacity: 0.35; }
  input, select, textarea, button { font-family: inherit; }
  ::-webkit-scrollbar { width: 5px; height: 5px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: var(--gold-dim); border-radius: 4px; }
  ::-webkit-scrollbar-thumb:hover { background: var(--gold); }
  ::selection { background: var(--primary); color: #0A0A0D; }
  [data-theme="light"] ::selection { color: #FDFAF2; }
  @keyframes fadeIn { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes slideUp { from { opacity: 0; transform: translateY(16px); } to { opacity: 1; transform: translateY(0); } }
  @keyframes pulseDot { 0%, 100% { opacity: 1; transform: scale(1); } 50% { opacity: 0.45; transform: scale(0.75); } }

  /* ── 玻璃拟态卡片：半透明 + 模糊 + 顶部金色渐变线 + 细金边 ── */
  .card {
    position: relative;
    padding: 18px;
    border-radius: var(--radius);
    background: var(--surface);
    backdrop-filter: blur(14px);
    -webkit-backdrop-filter: blur(14px);
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
    overflow: hidden;
    transition: var(--transition);
    margin-bottom: 16px;
    z-index: 1;
  }
  .card::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--gold), transparent);
    opacity: 0.7;
  }
  .card:hover { border-color: rgba(245, 166, 35, 0.45); box-shadow: var(--glass-shadow-lg); }
  .card-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 14px; }
  .card-header h3 {
    font-size: 13px; font-weight: 500; display: flex; align-items: center; gap: 8px;
    font-family: var(--font-mono); letter-spacing: 0.14em; text-transform: uppercase;
    color: var(--text);
  }

  /* ── 通用表单元件（金调） ── */
  .btn {
    display: inline-flex; align-items: center; justify-content: center; gap: 6px;
    padding: 7px 14px; border: 1px solid transparent; border-radius: var(--radius-sm);
    font-size: 13px; font-weight: 500; cursor: pointer;
    transition: var(--transition-fast);
  }
  .btn-primary { background: var(--primary); color: #0A0A0D; border-color: var(--primary); }
  [data-theme="light"] .btn-primary { color: #FDFAF2; }
  .btn-primary:hover { background: var(--primary-hover); border-color: var(--primary-hover); }
  .btn-secondary { background: var(--primary-subtle); color: var(--primary); border-color: var(--glass-border); }
  .btn-secondary:hover { background: var(--primary-glow); border-color: var(--gold-dim); }
  .btn-danger { background: var(--danger-subtle); color: var(--danger); border-color: transparent; }
  .btn-danger:hover { background: var(--danger); color: white; }
  .btn-ghost { background: transparent; color: var(--text-2); }
  .btn-ghost:hover { background: var(--primary-subtle); color: var(--primary); }
  .btn-sm { padding: 4px 10px; font-size: 12px; }

  .input {
    padding: 7px 12px; border-radius: var(--radius-sm);
    border: 1px solid var(--border); background: var(--chip-solid);
    color: var(--text); font-size: 13px; outline: none; transition: var(--transition-fast);
  }
  .input:focus { border-color: var(--gold-dim); box-shadow: 0 0 0 3px var(--primary-glow); }
  .input-sm { padding: 4px 8px; font-size: 12px; }
  select.input option { background: var(--surface-solid); color: var(--text); }

  .tag {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 2px 8px; border-radius: var(--radius-full);
    font-size: 11px; font-weight: 500;
    font-family: var(--font-mono); letter-spacing: 0.05em;
  }
  .tag-primary { background: var(--primary-subtle); color: var(--primary); }
  .tag-info { background: var(--info-subtle); color: var(--info); }
  .tag-warning { background: var(--warning-subtle); color: var(--warning); }
  .tag-danger { background: var(--danger-subtle); color: var(--danger); }
  .tag-success { background: var(--success-subtle); color: var(--success); }

  .table { width: 100%; border-collapse: collapse; font-size: 13px; }
  .table th {
    text-align: left; padding: 8px 12px; font-weight: 500; color: var(--text-3);
    border-bottom: 1px solid var(--border); font-size: 11px;
    font-family: var(--font-mono); letter-spacing: 0.12em; text-transform: uppercase;
  }
  .table td { padding: 8px 12px; border-bottom: 1px solid var(--border-light); }
  .table tr:hover td { background: var(--primary-subtle); }

  .empty { text-align: center; padding: 32px; color: var(--text-3); font-size: 13px; }
  .loading { text-align: center; padding: 32px; color: var(--text-3); font-size: 13px; }

  /* 全屏 3D 核心画布（由 CoreView 挂载/卸载） */
  #core-webgl {
    position: fixed;
    inset: 0;
    z-index: 0;
    display: block;
    pointer-events: none;
  }
  [data-theme="light"] #core-webgl { opacity: 0.5; }

  /* ── 精密仪表：大数字一律等宽细字重 ── */
  .stat-value, .balance-number {
    font-family: var(--font-mono) !important;
    font-weight: 300 !important;
    letter-spacing: 0.05em;
    font-variant-numeric: tabular-nums;
  }
  .num {
    font-family: var(--font-mono);
    font-weight: 300;
    letter-spacing: 0.05em;
    font-variant-numeric: tabular-nums;
  }
  .gold-text { color: var(--gold); }
`;document.head.appendChild(vh);</script>
  <style rel="stylesheet" crossorigin>.ov-layout[data-v-9b581e46]{display:grid;grid-template-columns:1fr 340px;gap:20px;height:100%;min-height:480px}.core-stage[data-v-9b581e46]{position:relative;border-radius:14px;overflow:hidden;background:radial-gradient(ellipse at center,rgba(245,166,35,.04),transparent 65%);border:1px solid var(--border);cursor:grab;user-select:none;touch-action:pan-y}.core-stage[data-v-9b581e46]:active{cursor:grabbing}.core-stage-overlay[data-v-9b581e46]{position:absolute;inset:0;pointer-events:none;display:flex;flex-direction:column;justify-content:space-between;padding:22px 24px}.core-stage-label[data-v-9b581e46]{font-size:10.5px;letter-spacing:2.5px;text-transform:uppercase;color:#f5a623b3;font-family:var(--font-mono);display:flex;align-items:center;gap:10px}.core-stage-label[data-v-9b581e46]:before{content:"";width:24px;height:1px;background:var(--gold);opacity:.6}.core-readout[data-v-9b581e46]{display:flex;gap:28px}.core-readout-item[data-v-9b581e46]{display:flex;flex-direction:column;gap:4px}.core-readout-item .lbl[data-v-9b581e46]{font-size:10px;letter-spacing:1.5px;text-transform:uppercase;color:var(--text-3);font-family:var(--font-mono)}.core-readout-item .val[data-v-9b581e46]{font-size:20px;font-weight:300;font-family:var(--font-mono);font-variant-numeric:tabular-nums;color:var(--gold);letter-spacing:.5px}.core-name[data-v-9b581e46]{text-align:center;position:absolute;bottom:24%;left:0;right:0;pointer-events:none}.core-name .cn-title[data-v-9b581e46]{font-size:40px;font-weight:300;letter-spacing:14px;text-indent:14px;color:var(--text-0, #F2EFE8);text-shadow:0 0 30px rgba(245,166,35,.55)}[data-theme=light] .core-name .cn-title[data-v-9b581e46]{color:var(--text);text-shadow:0 0 24px rgba(245,166,35,.35)}.core-name .cn-sub[data-v-9b581e46]{font-size:10.5px;letter-spacing:4px;color:var(--gold);text-transform:uppercase;margin-top:8px;font-family:var(--font-mono);opacity:.85}.ov-right[data-v-9b581e46]{display:flex;flex-direction:column;gap:20px}.glass[data-v-9b581e46]{background:#16161d8c;backdrop-filter:blur(20px) saturate(140%);-webkit-backdrop-filter:blur(20px) saturate(140%);border:1px solid var(--border);border-radius:14px;position:relative;overflow:hidden}[data-theme=light] .glass[data-v-9b581e46]{background:#fdf9efb3;border-color:var(--glass-border)}.glass[data-v-9b581e46]:before{content:"";position:absolute;top:0;left:0;right:0;height:1px;background:linear-gradient(90deg,transparent,rgba(245,166,35,.4),transparent)}.ov-panel[data-v-9b581e46]{padding:20px 22px}.events-panel[data-v-9b581e46]{flex:1;min-height:200px;display:flex;flex-direction:column}.panel-title[data-v-9b581e46]{font-size:10.5px;letter-spacing:2px;text-transform:uppercase;color:var(--text-2);font-family:var(--font-mono);display:flex;align-items:center;gap:10px;margin-bottom:18px}.panel-title[data-v-9b581e46]:before{content:"";width:3px;height:12px;background:var(--gold);border-radius:3px;box-shadow:0 0 8px var(--gold)}.panel-title .pt-right[data-v-9b581e46]{margin-left:auto;font-size:10px;letter-spacing:.5px;color:var(--text-3);text-transform:none}.emo-row[data-v-9b581e46]{margin-bottom:15px}.emo-row[data-v-9b581e46]:last-child{margin-bottom:0}.emo-head[data-v-9b581e46]{display:flex;justify-content:space-between;align-items:baseline;margin-bottom:7px}.emo-name[data-v-9b581e46]{font-size:12px;color:var(--text-1);display:flex;align-items:center;gap:8px}.emo-name .color-dot[data-v-9b581e46]{width:6px;height:6px;border-radius:2px}.emo-val[data-v-9b581e46]{font-size:13px;font-weight:600;font-family:var(--font-mono);font-variant-numeric:tabular-nums;color:var(--gold)}.emo-track[data-v-9b581e46]{height:3px;background:#ffffff0d;border-radius:3px;overflow:hidden;position:relative}[data-theme=light] .emo-track[data-v-9b581e46]{background:#9a6f1b1f}.emo-fill[data-v-9b581e46]{height:100%;border-radius:3px;position:relative;transition:width .6s cubic-bezier(.4,0,.2,1)}.emo-fill[data-v-9b581e46]:after{content:"";position:absolute;right:0;top:50%;transform:translateY(-50%);width:6px;height:6px;border-radius:50%;background:currentColor;box-shadow:0 0 8px currentColor}.event-stream[data-v-9b581e46]{display:flex;flex-direction:column;overflow-y:auto;flex:1;max-height:320px}.event-line[data-v-9b581e46]{display:flex;gap:12px;padding:9px 0;border-bottom:1px solid rgba(245,166,35,.05);font-size:12px;line-height:1.45;color:var(--text-1)}.event-line[data-v-9b581e46]:last-child{border-bottom:none}.event-time[data-v-9b581e46]{font-size:10.5px;color:var(--text-3);font-family:var(--font-mono);flex-shrink:0;padding-top:2px;min-width:42px}.event-text[data-v-9b581e46]{min-width:0;word-break:break-all}.event-text .hl[data-v-9b581e46]{color:var(--gold);margin-right:4px}.event-empty[data-v-9b581e46]{font-size:12px;color:var(--text-3);text-align:center;padding:28px 0;font-family:var(--font-mono);letter-spacing:1px}@media(max-width:1180px){.ov-layout[data-v-9b581e46]{grid-template-columns:1fr;height:auto}.core-stage[data-v-9b581e46]{min-height:420px}.events-panel[data-v-9b581e46]{min-height:unset}.event-stream[data-v-9b581e46]{max-height:260px}}.state-row[data-v-3ff49ea2]{display:flex;align-items:center;gap:14px;flex-wrap:wrap;margin-bottom:20px}.state-chip[data-v-3ff49ea2]{display:inline-flex;align-items:center;gap:8px;padding:6px 14px;border-radius:var(--radius-full);font-size:13px;font-weight:600;border:1px solid var(--border)}.state-chip.is-awake[data-v-3ff49ea2]{background:var(--success-subtle);color:var(--success);border-color:var(--success)}.state-chip.is-night[data-v-3ff49ea2]{background:var(--info-subtle);color:var(--info);border-color:var(--info)}.state-dot[data-v-3ff49ea2]{width:8px;height:8px;border-radius:50%;background:currentColor}.state-time[data-v-3ff49ea2]{font-size:13px;color:var(--text-2)}.state-diary[data-v-3ff49ea2]{display:inline-flex;align-items:baseline;gap:6px;margin-left:auto}.diary-num[data-v-3ff49ea2]{font-size:20px;font-weight:700;color:var(--primary)}.diary-label[data-v-3ff49ea2]{font-size:12px;color:var(--text-2)}.card-row[data-v-3ff49ea2]{display:flex;gap:16px;margin-bottom:16px}.half[data-v-3ff49ea2]{flex:1;min-width:0}.card h3[data-v-3ff49ea2]{font-size:14px;font-weight:600;margin-bottom:12px}.count-chip[data-v-3ff49ea2]{display:inline-flex;align-items:center;justify-content:center;min-width:18px;height:18px;padding:0 6px;margin-left:6px;border-radius:var(--radius-full);background:var(--primary-subtle);color:var(--primary);font-size:11px;font-weight:600}.signal-list[data-v-3ff49ea2]{display:flex;flex-direction:column;gap:10px}.signal-row[data-v-3ff49ea2]{display:flex;align-items:center;gap:10px}.signal-name[data-v-3ff49ea2]{width:72px;font-size:12px;color:var(--text-2);flex-shrink:0}.signal-gauge[data-v-3ff49ea2]{flex:1;height:6px;background:var(--border-light);border-radius:3px;overflow:hidden}.signal-fill[data-v-3ff49ea2]{height:100%;border-radius:3px;transition:width .6s ease,background .6s ease;min-width:2px}.signal-level[data-v-3ff49ea2]{width:40px;text-align:right;font-size:12px;color:var(--text-2);font-variant-numeric:tabular-nums}.loop-list[data-v-3ff49ea2]{display:flex;flex-direction:column;gap:8px}.loop-item[data-v-3ff49ea2]{padding:10px 14px;background:var(--bg-alt);border:1px solid var(--border);border-left:3px solid var(--primary);border-radius:var(--radius-xs)}.loop-reason[data-v-3ff49ea2]{font-size:13px;line-height:1.5}.loop-meta[data-v-3ff49ea2]{display:flex;align-items:center;gap:10px;flex-wrap:wrap;margin-top:6px;font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums}.loop-kind[data-v-3ff49ea2]{padding:1px 8px;border-radius:var(--radius-full);background:var(--warning-subtle);color:var(--warning);font-weight:500}.stream-list[data-v-3ff49ea2]{display:flex;flex-direction:column;gap:8px;max-height:480px;overflow-y:auto}.stream-item[data-v-3ff49ea2]{display:flex;gap:10px;align-items:flex-start;padding:10px 14px;background:var(--bg-alt);border:1px solid var(--border);border-radius:var(--radius-xs)}.stream-item.kind-inner[data-v-3ff49ea2]{border-color:var(--primary);box-shadow:inset 0 0 0 1px var(--primary);background:var(--primary-subtle)}.stream-badge[data-v-3ff49ea2]{flex-shrink:0;margin-top:1px;padding:2px 8px;border-radius:var(--radius-full);font-size:11px;font-weight:600;line-height:1.4}.badge-sensation[data-v-3ff49ea2]{background:var(--info-subtle);color:var(--info)}.badge-inner[data-v-3ff49ea2]{background:var(--primary);color:var(--surface-solid)}.badge-acted[data-v-3ff49ea2]{background:var(--accent-subtle);color:var(--accent)}.badge-digested[data-v-3ff49ea2]{background:var(--border-light);color:var(--text-2)}.stream-body[data-v-3ff49ea2]{flex:1;min-width:0}.stream-content[data-v-3ff49ea2]{font-size:13px;line-height:1.55;white-space:pre-wrap;word-break:break-word}.stream-item.is-recall[data-v-3ff49ea2]{padding:8px;border-left:3px solid var(--warning);background:var(--warning-subtle);border-radius:var(--radius-xs)}.badge-recall[data-v-3ff49ea2]{background:var(--warning-subtle);color:var(--warning)}.recall-state[data-v-3ff49ea2]{margin-left:8px;font-size:10px;color:var(--success)}.stream-time[data-v-3ff49ea2]{margin-top:4px;font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums}.empty[data-v-3ff49ea2]{text-align:center;padding:28px;color:var(--text-3);font-size:13px}@media(max-width:768px){.card-row[data-v-3ff49ea2]{flex-direction:column}}.card h3[data-v-3f47feb5]{font-size:14px;font-weight:600;margin-bottom:10px}.date-bar[data-v-3f47feb5]{display:flex;align-items:center;gap:16px}.date-bar h3[data-v-3f47feb5]{margin-bottom:0;flex-shrink:0}.date-chips[data-v-3f47feb5]{display:flex;flex-wrap:wrap;gap:8px}.date-chip[data-v-3f47feb5]{padding:5px 14px;border-radius:var(--radius-full);border:1px solid var(--border);background:var(--bg-alt);color:var(--text-2);font-size:12px;font-weight:500;cursor:pointer;transition:var(--transition-fast);font-variant-numeric:tabular-nums}.date-chip[data-v-3f47feb5]:hover{border-color:var(--primary);color:var(--primary)}.date-chip.active[data-v-3f47feb5]{background:var(--primary);border-color:var(--primary);color:var(--surface-solid)}.empty-inline[data-v-3f47feb5]{font-size:13px;color:var(--text-3)}.filter-bar[data-v-3f47feb5]{display:flex;align-items:center;justify-content:space-between;gap:12px;flex-wrap:wrap}.count-chip[data-v-3f47feb5]{display:inline-flex;align-items:center;justify-content:center;min-width:18px;height:18px;padding:0 6px;margin-left:6px;border-radius:var(--radius-full);background:var(--primary-subtle);color:var(--primary);font-size:11px;font-weight:600}.kind-filters[data-v-3f47feb5]{display:flex;flex-wrap:wrap;gap:6px}.kind-btn[data-v-3f47feb5]{padding:4px 12px;border-radius:var(--radius-full);border:1px solid var(--border);background:transparent;color:var(--text-2);font-size:12px;font-weight:500;cursor:pointer;transition:var(--transition-fast)}.kind-btn[data-v-3f47feb5]:hover{border-color:var(--primary);color:var(--primary)}.kind-btn.active[data-v-3f47feb5]{background:var(--primary-subtle);border-color:var(--primary);color:var(--primary)}.timeline[data-v-3f47feb5]{display:flex;flex-direction:column;margin-top:6px}.tl-item[data-v-3f47feb5]{display:flex;gap:12px}.tl-rail[data-v-3f47feb5]{display:flex;flex-direction:column;align-items:center;width:14px;flex-shrink:0;padding-top:16px}.tl-dot[data-v-3f47feb5]{width:9px;height:9px;border-radius:50%;background:var(--border);flex-shrink:0}.tl-item.kind-inner .tl-dot[data-v-3f47feb5]{background:var(--primary)}.tl-item.kind-acted .tl-dot[data-v-3f47feb5]{background:var(--accent)}.tl-item.kind-sensation .tl-dot[data-v-3f47feb5]{background:var(--info)}.tl-item.is-recall .tl-dot[data-v-3f47feb5]{background:var(--warning)}.tl-line[data-v-3f47feb5]{flex:1;width:2px;background:var(--border-light);min-height:14px}.tl-body[data-v-3f47feb5]{flex:1;min-width:0;padding-bottom:16px}.tl-head[data-v-3f47feb5]{display:flex;align-items:center;gap:8px;flex-wrap:wrap}.tl-badge[data-v-3f47feb5]{padding:2px 8px;border-radius:var(--radius-full);font-size:11px;font-weight:600;line-height:1.4}.badge-sensation[data-v-3f47feb5]{background:var(--info-subtle);color:var(--info)}.badge-inner[data-v-3f47feb5]{background:var(--primary);color:var(--surface-solid)}.badge-acted[data-v-3f47feb5]{background:var(--accent-subtle);color:var(--accent)}.badge-digested[data-v-3f47feb5]{background:var(--border-light);color:var(--text-2)}.tl-time[data-v-3f47feb5]{font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums}.tl-about[data-v-3f47feb5]{font-size:11px;color:var(--text-3)}.recall-source[data-v-3f47feb5],.recall-status[data-v-3f47feb5]{font-size:10px;padding:2px 6px;border-radius:4px;background:var(--warning-subtle);color:var(--warning)}.recall-status[data-v-3f47feb5]{background:var(--success-subtle);color:var(--success)}.tl-content[data-v-3f47feb5]{margin-top:5px;font-size:13px;line-height:1.55;white-space:pre-wrap;word-break:break-word}.tl-item.kind-inner .tl-content[data-v-3f47feb5]{color:var(--text)}.tl-item.is-recall .tl-content[data-v-3f47feb5]{padding:8px 10px;border-left:3px solid var(--warning);background:var(--warning-subtle);border-radius:4px}.tl-item.is-recall-completed .tl-content[data-v-3f47feb5]{opacity:.72}.empty[data-v-3f47feb5]{text-align:center;padding:32px;color:var(--text-3);font-size:13px}.tab-bar[data-v-9d4cead7]{display:flex;gap:8px;margin-bottom:18px}.tab-btn[data-v-9d4cead7]{display:inline-flex;align-items:center;gap:6px;padding:7px 16px;border-radius:var(--radius-full);border:1px solid var(--border);background:var(--bg-alt);color:var(--text-2);font-size:13px;font-weight:500;cursor:pointer;transition:var(--transition-fast)}.tab-btn[data-v-9d4cead7]:hover{border-color:var(--primary);color:var(--primary)}.tab-btn.active[data-v-9d4cead7]{background:var(--primary);border-color:var(--primary);color:var(--surface-solid)}.tab-count[data-v-9d4cead7]{min-width:18px;height:18px;padding:0 6px;display:inline-flex;align-items:center;justify-content:center;border-radius:var(--radius-full);font-size:11px;background:var(--border-light);color:var(--text-2);font-variant-numeric:tabular-nums}.tab-btn.active .tab-count[data-v-9d4cead7]{background:#ffffff40;color:var(--surface-solid)}.diary-list[data-v-9d4cead7]{display:flex;flex-direction:column;gap:12px}.diary-card[data-v-9d4cead7]{margin-bottom:0}.diary-head[data-v-9d4cead7]{display:flex;align-items:center;gap:10px;margin-bottom:8px}.diary-date[data-v-9d4cead7]{font-size:12px;font-weight:600;color:var(--primary);padding:2px 10px;border-radius:var(--radius-full);background:var(--primary-subtle);font-variant-numeric:tabular-nums}.diary-about[data-v-9d4cead7]{font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums}.diary-content[data-v-9d4cead7]{font-size:13px;line-height:1.7;white-space:pre-wrap;word-break:break-word}.diary-feeling[data-v-9d4cead7]{margin-top:10px;padding:8px 12px;border-radius:var(--radius-xs);background:var(--primary-subtle);color:var(--primary);font-size:12px;line-height:1.5}.person-grid[data-v-9d4cead7]{display:grid;grid-template-columns:repeat(auto-fill,minmax(300px,1fr));gap:14px}.person-card[data-v-9d4cead7]{margin-bottom:0}.person-head[data-v-9d4cead7]{display:flex;align-items:baseline;justify-content:space-between;gap:8px}.person-name[data-v-9d4cead7]{font-size:15px;font-weight:700;color:var(--text)}.person-uid[data-v-9d4cead7]{font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums}.person-address[data-v-9d4cead7]{margin-top:4px;font-size:12px;color:var(--primary);font-weight:500}.person-fields[data-v-9d4cead7]{margin:12px 0 0;display:flex;flex-direction:column;gap:8px}.field[data-v-9d4cead7]{display:flex;gap:10px;align-items:baseline}.field dt[data-v-9d4cead7]{flex-shrink:0;width:58px;font-size:11px;color:var(--text-3)}.field dd[data-v-9d4cead7]{margin:0;font-size:13px;line-height:1.6;color:var(--text)}.person-section[data-v-9d4cead7]{margin-top:12px}.section-label[data-v-9d4cead7]{font-size:11px;font-weight:600;color:var(--text-3);text-transform:uppercase;letter-spacing:.6px;margin-bottom:6px}.say-list[data-v-9d4cead7],.memory-list[data-v-9d4cead7]{margin:0;padding:0 0 0 16px;display:flex;flex-direction:column;gap:4px}.say-list li[data-v-9d4cead7],.memory-list li[data-v-9d4cead7]{font-size:12px;line-height:1.6;color:var(--text-2)}.say-list li[data-v-9d4cead7]{color:var(--accent)}.empty[data-v-9d4cead7]{text-align:center;padding:40px;color:var(--text-3);font-size:13px}.card h3[data-v-10a6d06d]{font-size:14px;font-weight:600}.head-row[data-v-10a6d06d]{display:flex;align-items:center;justify-content:space-between;gap:12px;margin-bottom:14px}.count-chip[data-v-10a6d06d]{display:inline-flex;align-items:center;justify-content:center;min-width:18px;height:18px;padding:0 6px;margin-left:6px;border-radius:var(--radius-full);background:var(--primary-subtle);color:var(--primary);font-size:11px;font-weight:600}.alert-chip[data-v-10a6d06d]{padding:4px 12px;border-radius:var(--radius-full);background:var(--accent-subtle);color:var(--accent);border:1px solid var(--accent);font-size:12px;font-weight:600;font-variant-numeric:tabular-nums}.table-wrap[data-v-10a6d06d]{overflow-x:auto}.table td[data-v-10a6d06d],.table th[data-v-10a6d06d]{vertical-align:top}.cell-time[data-v-10a6d06d]{white-space:nowrap;color:var(--text-2);font-size:12px;font-variant-numeric:tabular-nums}.cell-user[data-v-10a6d06d]{white-space:nowrap;font-variant-numeric:tabular-nums}.cell-detail[data-v-10a6d06d]{min-width:200px;word-break:break-word;line-height:1.5}.gate-chip[data-v-10a6d06d]{display:inline-block;padding:2px 8px;border-radius:var(--radius-full);background:var(--border-light);color:var(--text-2);font-size:11px;font-weight:500;white-space:nowrap}.action-chip[data-v-10a6d06d]{display:inline-block;padding:2px 8px;border-radius:var(--radius-full);background:var(--warning-subtle);color:var(--warning);font-size:11px;font-weight:600;white-space:nowrap}.action-chip.zero[data-v-10a6d06d]{background:var(--accent-subtle);color:var(--accent)}.row-blacklist td[data-v-10a6d06d],.row-blacklist:hover td[data-v-10a6d06d]{background:var(--accent-subtle)}.empty[data-v-10a6d06d]{text-align:center;padding:40px;color:var(--text-3);font-size:13px}.card h3[data-v-e3c8dfc0]{font-size:14px;font-weight:600;margin-bottom:10px}.group-bar[data-v-e3c8dfc0]{display:flex;align-items:center;gap:16px}.group-bar h3[data-v-e3c8dfc0]{margin-bottom:0;flex-shrink:0}.group-chips[data-v-e3c8dfc0]{display:flex;flex-wrap:wrap;gap:8px}.group-chip[data-v-e3c8dfc0]{padding:5px 14px;border-radius:var(--radius-full);border:1px solid var(--border);background:var(--bg-alt);color:var(--text-2);font-size:12px;font-weight:500;cursor:pointer;transition:var(--transition-fast);font-variant-numeric:tabular-nums}.group-chip[data-v-e3c8dfc0]:hover{border-color:var(--primary);color:var(--primary)}.group-chip.active[data-v-e3c8dfc0]{background:var(--primary);border-color:var(--primary);color:var(--surface-solid)}.empty-inline[data-v-e3c8dfc0]{font-size:13px;color:var(--text-3)}.empty[data-v-e3c8dfc0]{font-size:13px;color:var(--text-3);padding:8px 0}.count-chip[data-v-e3c8dfc0]{display:inline-flex;align-items:center;justify-content:center;min-width:18px;height:18px;padding:0 6px;margin-left:6px;border-radius:var(--radius-full);background:var(--primary-subtle);color:var(--primary);font-size:11px;font-weight:600}.threads[data-v-e3c8dfc0]{display:flex;flex-direction:column;gap:12px}.thread[data-v-e3c8dfc0]{padding:12px 14px;border-radius:var(--radius-sm);border:1px solid var(--border);background:var(--bg-alt)}.thread-head[data-v-e3c8dfc0]{display:flex;align-items:center;gap:10px;flex-wrap:wrap}.thread-title[data-v-e3c8dfc0]{font-size:13px;font-weight:600}.heat[data-v-e3c8dfc0]{font-size:11px;font-weight:600;padding:1px 8px;border-radius:var(--radius-full)}.heat.hot[data-v-e3c8dfc0]{background:var(--danger-subtle, #fde8e4);color:var(--danger, #d65a4a)}.heat.warm[data-v-e3c8dfc0]{background:var(--accent-subtle);color:var(--accent)}.heat.cold[data-v-e3c8dfc0]{background:var(--border-light);color:var(--text-3)}.thread-time[data-v-e3c8dfc0]{font-size:11px;color:var(--text-3);margin-left:auto}.intensity-track[data-v-e3c8dfc0]{height:4px;border-radius:var(--radius-full);background:var(--border-light);margin:8px 0;overflow:hidden}.intensity-fill[data-v-e3c8dfc0]{height:100%;border-radius:var(--radius-full);background:var(--primary);transition:width .4s ease}.thread-members[data-v-e3c8dfc0]{font-size:12px;color:var(--text-2);margin-bottom:8px}.transcript[data-v-e3c8dfc0]{display:flex;flex-direction:column;gap:3px}.tr-line[data-v-e3c8dfc0]{font-size:12px;color:var(--text-2);display:flex;gap:6px}.tr-name[data-v-e3c8dfc0]{color:var(--text-3);flex-shrink:0}.tr-name[data-v-e3c8dfc0]:after{content:"："}.tr-bot .tr-name[data-v-e3c8dfc0]{color:var(--primary);font-weight:600}.waiting[data-v-e3c8dfc0]{margin-top:8px;padding:7px 10px;font-size:12px;border-radius:var(--radius-xs);background:var(--accent-subtle);color:var(--accent);display:flex;align-items:center;gap:6px}.grid-2[data-v-e3c8dfc0]{display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-top:16px}@media(max-width:900px){.grid-2[data-v-e3c8dfc0]{grid-template-columns:1fr}}.attn-list[data-v-e3c8dfc0]{display:flex;flex-direction:column;gap:8px}.attn-row[data-v-e3c8dfc0]{display:flex;align-items:center;gap:10px}.attn-name[data-v-e3c8dfc0]{font-size:12px;color:var(--text-2);width:72px;flex-shrink:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.attn-track[data-v-e3c8dfc0]{flex:1;height:6px;border-radius:var(--radius-full);background:var(--border-light);overflow:hidden}.attn-fill[data-v-e3c8dfc0]{height:100%;border-radius:var(--radius-full);background:linear-gradient(90deg,var(--primary),var(--accent));transition:width .4s ease}.attn-value[data-v-e3c8dfc0]{font-size:11px;color:var(--text-3);font-variant-numeric:tabular-nums;width:34px;text-align:right}.bond-list[data-v-e3c8dfc0]{display:flex;flex-direction:column;gap:6px}.bond-row[data-v-e3c8dfc0]{display:flex;align-items:center;justify-content:space-between;gap:8px}.bond-pair[data-v-e3c8dfc0]{font-size:12px;color:var(--text-2)}.bond-level[data-v-e3c8dfc0]{font-size:11px;font-weight:600;padding:1px 8px;border-radius:var(--radius-full)}.bond-level.close[data-v-e3c8dfc0]{background:var(--primary);color:var(--surface-solid)}.bond-level.known[data-v-e3c8dfc0]{background:var(--primary-subtle);color:var(--primary)}.ignored[data-v-e3c8dfc0]{margin-top:12px;padding:7px 10px;font-size:12px;border-radius:var(--radius-xs);background:var(--border-light);color:var(--text-2);display:flex;align-items:center;gap:6px}.card[data-v-73a547e7]{padding:18px;border-radius:var(--radius);background:var(--surface-solid);border:1px solid var(--border);box-shadow:var(--glass-shadow);transition:var(--transition)}.card[data-v-73a547e7]:hover{box-shadow:var(--glass-shadow-lg)}.stat-grid[data-v-73a547e7]{display:grid;grid-template-columns:repeat(auto-fill,minmax(170px,1fr));gap:12px;margin-bottom:20px}.stat-icon[data-v-73a547e7]{margin-bottom:10px}.stat-value[data-v-73a547e7]{font-size:26px;font-weight:700;letter-spacing:-.5px;line-height:1}.stat-label[data-v-73a547e7]{font-size:13px;color:var(--text-2);margin-top:4px;font-weight:500}.stat-sub[data-v-73a547e7]{font-size:11px;color:var(--text-3);margin-top:2px}.chart-grid[data-v-73a547e7]{display:grid;grid-template-columns:repeat(auto-fill,minmax(260px,1fr));gap:12px;margin-bottom:20px}.chart-card[data-v-73a547e7]{text-align:center}.card-title[data-v-73a547e7]{font-size:13px;font-weight:600;margin-bottom:14px;text-align:left;color:var(--text-2)}.chart-container[data-v-73a547e7]{display:flex;align-items:center;justify-content:center;gap:20px;flex-wrap:wrap}.chart-legend[data-v-73a547e7]{display:flex;flex-direction:column;gap:6px;text-align:left}.legend-item[data-v-73a547e7]{display:flex;align-items:center;gap:6px;font-size:12px;color:var(--text-2)}.dot[data-v-73a547e7]{width:8px;height:8px;border-radius:50%;flex-shrink:0}.data-card[data-v-73a547e7]{margin-bottom:20px}.active-lists[data-v-73a547e7]{display:flex;flex-direction:column;gap:14px}.active-section-title[data-v-73a547e7]{font-size:12px;font-weight:600;color:var(--text-2);margin-bottom:6px}.active-chips[data-v-73a547e7]{display:flex;flex-wrap:wrap;gap:6px}.chip[data-v-73a547e7]{font-size:12px;padding:4px 10px;background:var(--primary-subtle);border-radius:var(--radius-full);color:var(--primary);font-family:system-ui,sans-serif;font-weight:500}.empty-sm[data-v-73a547e7]{text-align:center;padding:20px;color:var(--text-3);font-size:13px}@media(max-width:768px){.stat-grid[data-v-73a547e7]{grid-template-columns:repeat(2,1fr)}.chart-grid[data-v-73a547e7]{grid-template-columns:1fr}.chart-container[data-v-73a547e7]{flex-direction:column}}@media(max-width:480px){.stat-grid[data-v-73a547e7]{grid-template-columns:1fr}}.config-layout[data-v-e6128b71]{display:flex;gap:20px;align-items:flex-start}.nav-section[data-v-e6128b71]{margin-bottom:2px}.nav-item[data-v-e6128b71]{display:flex;align-items:center;gap:8px;padding:7px 10px;border-radius:var(--radius-xs);font-size:12px;font-weight:500;cursor:pointer;transition:var(--transition);color:var(--text-2)}.nav-item[data-v-e6128b71]:hover{background:var(--surface-hover);color:var(--text)}.nav-item.active[data-v-e6128b71]{background:var(--primary);color:#fff}.nav-dot[data-v-e6128b71]{width:6px;height:6px;border-radius:50%;flex-shrink:0}.config-content[data-v-e6128b71]{flex:1;min-width:0}.card-header[data-v-e6128b71]{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}.card-header h3[data-v-e6128b71]{font-size:15px;font-weight:600;display:flex;align-items:center;gap:8px}.sec-dot[data-v-e6128b71]{width:10px;height:10px;border-radius:50%;flex-shrink:0}.field-list[data-v-e6128b71]{display:flex;flex-direction:column}.field-item[data-v-e6128b71]{display:flex;align-items:center;padding:8px 0;border-bottom:1px solid var(--border);font-size:13px}.field-item[data-v-e6128b71]:last-child{border-bottom:none}.field-label[data-v-e6128b71]{width:150px;color:var(--text-2);font-weight:500;flex-shrink:0}.field-value[data-v-e6128b71]{flex:1;display:flex;align-items:center;gap:6px}.mono[data-v-e6128b71]{font-family:monospace;font-size:12px;color:var(--text)}.text-muted[data-v-e6128b71]{color:var(--text-3)}.toggle-dot[data-v-e6128b71]{width:8px;height:8px;border-radius:50%;background:var(--text-3);flex-shrink:0}.toggle-dot.on[data-v-e6128b71]{background:var(--success)}.array-chips[data-v-e6128b71]{display:flex;flex-wrap:wrap;gap:4px}.chip-sm[data-v-e6128b71]{font-size:11px;padding:2px 8px;background:var(--surface-hover);border-radius:4px;color:var(--text-2)}.notice-card[data-v-e6128b71]{display:flex;align-items:center;gap:10px;font-size:12px;color:var(--text-2);line-height:1.5}.btn[data-v-e6128b71]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer;transition:var(--transition)}.btn-primary[data-v-e6128b71]{background:var(--primary);color:#fff}.btn-ghost[data-v-e6128b71]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-e6128b71]{padding:4px 10px;font-size:12px}.empty[data-v-e6128b71]{text-align:center;padding:32px;color:var(--text-3)}.modal-overlay[data-v-e6128b71]{position:fixed;inset:0;background:#00000080;z-index:200;display:flex;align-items:center;justify-content:center}.modal[data-v-e6128b71]{width:480px;max-height:80vh;overflow-y:auto;padding:24px}.edit-fields[data-v-e6128b71]{display:flex;flex-direction:column;gap:12px}.edit-field[data-v-e6128b71]{display:flex;flex-direction:column;gap:4px}.edit-field label[data-v-e6128b71]{font-size:12px;font-weight:600;color:var(--text-2)}.glass-input[data-v-e6128b71],.glass-select[data-v-e6128b71]{padding:8px 12px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:13px;outline:none;width:100%}.glass-input[data-v-e6128b71]:focus,.glass-select[data-v-e6128b71]:focus{border-color:var(--primary);box-shadow:0 0 0 3px var(--primary-glow)}.modal-actions[data-v-e6128b71]{display:flex;gap:8px;justify-content:flex-end;margin-top:16px}.config-error-banner[data-v-e6128b71]{display:flex;align-items:center;gap:12px;padding:12px 16px;margin-bottom:16px;background:var(--danger-subtle);border:1px solid var(--danger);border-radius:var(--radius-sm);font-size:13px;color:var(--danger)}.error-icon[data-v-e6128b71]{flex-shrink:0;width:22px;height:22px;border-radius:50%;background:var(--danger);color:#fff;font-weight:700;font-size:14px;display:flex;align-items:center;justify-content:center}.error-body[data-v-e6128b71]{flex:1;display:flex;flex-direction:column;gap:2px}.error-body strong[data-v-e6128b71]{font-size:13px}.error-body span[data-v-e6128b71]{font-size:12px;opacity:.8;word-break:break-all}@media(max-width:768px){.config-layout[data-v-e6128b71]{flex-direction:column}.config-nav[data-v-e6128b71]{width:100%;position:static;display:flex;flex-wrap:wrap;gap:4px;padding:10px}.nav-section[data-v-e6128b71]{display:inline}.nav-section .nav-item[data-v-e6128b71]{display:inline-flex;margin:2px}.field-label[data-v-e6128b71]{width:100px;font-size:12px}.modal[data-v-e6128b71]{width:95vw;max-width:480px}}.card-header[data-v-e466c855]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:8px;margin-bottom:16px}.card-header h3[data-v-e466c855]{font-size:15px;font-weight:600}.header-actions[data-v-e466c855]{display:flex;gap:6px;align-items:center;flex-wrap:wrap}.glass-input[data-v-e466c855],.glass-select[data-v-e466c855]{padding:6px 10px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:12px;outline:none}.btn[data-v-e466c855]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-primary[data-v-e466c855]{background:var(--primary);color:#fff}.btn-ghost[data-v-e466c855]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-e466c855]{padding:4px 10px;font-size:12px}.empty[data-v-e466c855]{text-align:center;padding:20px;color:var(--text-3);font-size:13px}.section-label[data-v-e466c855]{font-size:11px;font-weight:600;color:var(--text-3);text-transform:uppercase;letter-spacing:.5px;margin-bottom:8px}.chip-list[data-v-e466c855]{display:flex;flex-wrap:wrap;gap:8px}.chip[data-v-e466c855]{display:flex;align-items:center;gap:6px;padding:6px 12px;background:var(--surface-hover);border-radius:20px;font-size:13px;font-weight:500}.chip-priv[data-v-e466c855]{background:#6366f11a;color:var(--primary)}.chip-close[data-v-e466c855]{background:none;border:none;cursor:pointer;color:var(--text-3);font-size:12px;padding:0;line-height:1}.chip-close[data-v-e466c855]:hover{color:var(--danger)}.stat-grid[data-v-6734b806]{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:16px;margin-bottom:16px}.card[data-v-6734b806]{padding:20px}.stat-value[data-v-6734b806]{font-size:24px;font-weight:700}.stat-label[data-v-6734b806]{font-size:13px;color:var(--text-2);margin-top:4px}.stat-sub[data-v-6734b806]{font-size:11px;color:var(--text-3)}.stat-grid[data-v-f26e5395]{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:16px;margin-bottom:16px}.card-header[data-v-f26e5395]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;margin-bottom:16px}.card-header h3[data-v-f26e5395]{font-size:15px;font-weight:600}.header-actions[data-v-f26e5395]{display:flex;gap:8px}.glass-input[data-v-f26e5395]{padding:8px 12px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:13px;outline:none}.badge[data-v-f26e5395]{font-size:10px;font-weight:500;padding:2px 8px;border-radius:20px;background:var(--primary-glow);color:var(--primary)}.stat-value[data-v-f26e5395]{font-size:28px;font-weight:700}.stat-label[data-v-f26e5395]{font-size:13px;color:var(--text-2)}.stat-sub[data-v-f26e5395]{font-size:11px;color:var(--text-3)}.btn[data-v-f26e5395]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-ghost[data-v-f26e5395]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-f26e5395]{padding:4px 10px;font-size:12px}.btn-xs[data-v-f26e5395]{padding:3px 8px;font-size:11px}.empty[data-v-f26e5395]{text-align:center;padding:32px;color:var(--text-3)}.sticker-grid[data-v-f26e5395]{display:grid;grid-template-columns:repeat(auto-fill,minmax(130px,1fr));gap:12px}.sticker-card[data-v-f26e5395]{padding:8px;border-radius:var(--radius-sm);background:var(--surface-hover);transition:var(--transition)}.sticker-card[data-v-f26e5395]:hover{transform:translateY(-2px)}.sticker-img[data-v-f26e5395]{width:100%;aspect-ratio:1;overflow:hidden;border-radius:var(--radius-xs);background:var(--surface)}.sticker-img img[data-v-f26e5395]{width:100%;height:100%;object-fit:contain}.sticker-info[data-v-f26e5395]{display:flex;flex-direction:column;gap:4px;margin-top:6px}.sticker-tags[data-v-f26e5395]{display:flex;flex-wrap:wrap;gap:3px}.chip-sm[data-v-f26e5395]{font-size:10px;padding:1px 6px;background:var(--surface);border-radius:3px;color:var(--text-2)}.text-muted[data-v-f26e5395]{font-size:10px;color:var(--text-3)}.sticker-actions[data-v-f26e5395]{display:flex;align-items:center;justify-content:space-between}.badge-sm[data-v-f26e5395]{font-size:10px;font-weight:500;padding:1px 6px;border-radius:3px}.badge-sm.on[data-v-f26e5395]{background:#34d39926;color:var(--success)}.badge-sm.off[data-v-f26e5395]{background:#ef44441a;color:var(--danger)}.card-header[data-v-43f65072]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:8px;margin-bottom:16px}.card-header h3[data-v-43f65072]{font-size:15px;font-weight:600}.header-actions[data-v-43f65072]{display:flex;gap:6px;flex-wrap:wrap;align-items:center}.sep[data-v-43f65072]{color:var(--text-3);font-size:12px}.glass-input[data-v-43f65072],.glass-select[data-v-43f65072]{padding:6px 10px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:12px;outline:none}.btn[data-v-43f65072]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-primary[data-v-43f65072]{background:var(--primary);color:#fff}.btn-ghost[data-v-43f65072]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-43f65072]{padding:4px 10px;font-size:12px}.btn-xs[data-v-43f65072]{padding:3px 8px;font-size:11px}.empty[data-v-43f65072]{text-align:center;padding:32px;color:var(--text-3)}.table-wrap[data-v-43f65072]{overflow-x:auto}table[data-v-43f65072]{width:100%;border-collapse:collapse;font-size:13px}th[data-v-43f65072]{text-align:left;padding:8px 12px;font-weight:600;font-size:11px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border)}td[data-v-43f65072]{padding:8px 12px;border-bottom:1px solid var(--border)}tr:hover td[data-v-43f65072]{background:var(--surface-hover)}.mono[data-v-43f65072]{font-family:monospace;font-size:12px;color:var(--text-2)}.truncate[data-v-43f65072]{max-width:300px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.tag[data-v-43f65072]{font-size:11px;padding:2px 8px;border-radius:4px;font-weight:500}.tag-permanent[data-v-43f65072]{background:var(--warning-subtle);color:var(--warning)}.tag-important[data-v-43f65072]{background:var(--info-subtle);color:var(--info)}.tag-normal[data-v-43f65072]{background:var(--surface);color:var(--text-2)}.modal-overlay[data-v-43f65072]{position:fixed;inset:0;background:#00000080;z-index:200;display:flex;align-items:center;justify-content:center}.modal[data-v-43f65072]{width:420px;padding:24px}.modal label[data-v-43f65072]{display:block;font-size:12px;font-weight:600;margin-bottom:4px;color:var(--text-2)}.modal-actions[data-v-43f65072]{display:flex;gap:8px;justify-content:flex-end}.card-header[data-v-53bb8a7c]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;margin-bottom:16px}.card-header h3[data-v-53bb8a7c]{font-size:15px;font-weight:600}.header-actions[data-v-53bb8a7c]{display:flex;gap:8px}.badge[data-v-53bb8a7c]{font-size:10px;font-weight:500;padding:2px 8px;border-radius:20px;background:var(--primary-glow);color:var(--primary)}.glass-input[data-v-53bb8a7c],.glass-select[data-v-53bb8a7c]{padding:8px 12px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:13px;outline:none}.btn[data-v-53bb8a7c]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer;transition:var(--transition)}.btn-ghost[data-v-53bb8a7c]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-53bb8a7c]{padding:4px 10px;font-size:12px}.empty[data-v-53bb8a7c]{text-align:center;padding:40px;color:var(--text-3)}.msg-flow[data-v-53bb8a7c]{display:flex;flex-direction:column;gap:8px;max-height:70vh;overflow-y:auto}.msg-item[data-v-53bb8a7c]{display:flex;gap:10px;padding:8px;border-radius:var(--radius-sm);transition:var(--transition)}.msg-item[data-v-53bb8a7c]:hover{background:var(--surface-hover)}.msg-avatar[data-v-53bb8a7c]{width:28px;height:28px;border-radius:50%;display:flex;align-items:center;justify-content:center;color:#fff;font-size:12px;font-weight:700;flex-shrink:0}.msg-body[data-v-53bb8a7c]{flex:1;min-width:0}.msg-header[data-v-53bb8a7c]{display:flex;align-items:center;gap:8px;margin-bottom:2px}.msg-user[data-v-53bb8a7c]{font-size:12px;font-weight:600}.msg-user.bot[data-v-53bb8a7c]{color:var(--primary)}.msg-time[data-v-53bb8a7c]{font-size:11px;color:var(--text-3)}.msg-tag[data-v-53bb8a7c]{font-size:10px;padding:1px 6px;border-radius:3px;background:#34d39926;color:var(--success)}.msg-text[data-v-53bb8a7c]{font-size:13px;line-height:1.4;word-break:break-word}.stat-grid[data-v-abe732f7]{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:16px;margin-bottom:16px}.card-header h3[data-v-abe732f7]{font-size:15px;font-weight:600;margin-bottom:12px}.stat-value[data-v-abe732f7]{font-size:28px;font-weight:700;letter-spacing:-.5px}.stat-label[data-v-abe732f7]{font-size:13px;color:var(--text-2);margin-top:4px}.stat-sub[data-v-abe732f7]{font-size:11px;color:var(--text-3)}.table-wrap[data-v-abe732f7]{overflow-x:auto}table[data-v-abe732f7]{width:100%;border-collapse:collapse;font-size:13px}th[data-v-abe732f7]{text-align:left;padding:8px 12px;font-weight:600;font-size:11px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border)}td[data-v-abe732f7]{padding:8px 12px;border-bottom:1px solid var(--border)}tr:hover td[data-v-abe732f7]{background:var(--surface-hover)}.mono[data-v-abe732f7]{font-family:monospace;font-size:12px;color:var(--text-2)}.emotion-badge[data-v-abe732f7]{padding:2px 8px;border-radius:4px;font-size:11px;font-weight:500;color:#fff}.bar-wrap[data-v-abe732f7]{width:80px;height:6px;background:var(--surface);border-radius:3px;overflow:hidden;display:inline-block;vertical-align:middle}.bar[data-v-abe732f7]{height:100%;border-radius:3px;transition:width .5s ease}.bar-val[data-v-abe732f7]{font-size:11px;color:var(--text-2);margin-left:6px}.text-muted[data-v-abe732f7]{color:var(--text-3)}.empty[data-v-abe732f7]{text-align:center;padding:40px;color:var(--text-3)}.card-header[data-v-2fd8560d]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;margin-bottom:16px}.card-header h3[data-v-2fd8560d]{font-size:15px;font-weight:600}.header-actions[data-v-2fd8560d]{display:flex;gap:6px;align-items:center}.glass-input[data-v-2fd8560d]{padding:8px 12px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:13px;outline:none}.btn[data-v-2fd8560d]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-primary[data-v-2fd8560d]{background:var(--primary);color:#fff}.btn-ghost[data-v-2fd8560d]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-2fd8560d]{padding:4px 10px;font-size:12px}.empty[data-v-2fd8560d]{text-align:center;padding:32px;color:var(--text-3)}.chip-list[data-v-2fd8560d]{display:flex;flex-wrap:wrap;gap:8px}.chip[data-v-2fd8560d]{display:flex;align-items:center;gap:6px;padding:6px 12px;background:var(--surface-hover);border-radius:20px;font-size:13px}.chip-close[data-v-2fd8560d]{background:none;border:none;cursor:pointer;color:var(--text-3);font-size:12px;padding:0}.chip-close[data-v-2fd8560d]:hover{color:var(--danger)}.mono[data-v-2fd8560d]{font-family:monospace;font-size:13px}.card-header[data-v-de301834]{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}.card-header h3[data-v-de301834]{font-size:15px;font-weight:600}.btn[data-v-de301834]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-ghost[data-v-de301834]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-de301834]{padding:4px 10px;font-size:12px}.btn-xs[data-v-de301834]{padding:3px 8px;font-size:11px}.empty[data-v-de301834]{text-align:center;padding:32px;color:var(--text-3)}.table-wrap[data-v-de301834]{overflow-x:auto}table[data-v-de301834]{width:100%;border-collapse:collapse;font-size:13px}th[data-v-de301834]{text-align:left;padding:8px 12px;font-weight:600;font-size:11px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border)}td[data-v-de301834]{padding:8px 12px;border-bottom:1px solid var(--border)}tr:hover td[data-v-de301834]{background:var(--surface-hover)}.mono[data-v-de301834]{font-family:monospace;font-size:12px;color:var(--text-2)}.bar-wrap[data-v-de301834]{height:6px;background:var(--surface);border-radius:3px;overflow:hidden}.bar[data-v-de301834]{height:100%;border-radius:3px;transition:width .5s ease}.actions[data-v-de301834]{display:flex;gap:4px}.badge[data-v-de301834]{font-size:10px;font-weight:500;padding:2px 8px;border-radius:4px}.badge-danger[data-v-de301834]{background:#ef444426;color:var(--danger)}.badge-safe[data-v-de301834]{background:#34d39926;color:var(--success)}.badge-warn[data-v-de301834]{background:#fbbf2426;color:var(--warning)}.card-header[data-v-94e997b2]{display:flex;align-items:center;justify-content:space-between;margin-bottom:12px}.card-header h3[data-v-94e997b2]{font-size:15px;font-weight:600}.btn[data-v-94e997b2]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-ghost[data-v-94e997b2]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-94e997b2]{padding:4px 10px;font-size:12px}.empty[data-v-94e997b2]{text-align:center;padding:24px;color:var(--text-3)}.stat-row[data-v-94e997b2]{display:flex;gap:16px}.stat-item[data-v-94e997b2]{flex:1;text-align:center;padding:24px;border-radius:var(--radius-sm);background:var(--surface-hover)}.stat-num[data-v-94e997b2]{font-size:32px;font-weight:700;color:var(--primary)}.stat-lbl[data-v-94e997b2]{display:block;font-size:13px;color:var(--text-2);margin-top:4px}.stat-grid[data-v-81042d3d]{display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:12px;margin-bottom:16px}.card-header[data-v-81042d3d]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;margin-bottom:12px}.card-header h3[data-v-81042d3d]{font-size:15px;font-weight:600}.header-actions[data-v-81042d3d]{display:flex;gap:8px;align-items:center}.glass-select[data-v-81042d3d]{padding:6px 10px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:12px;outline:none}.stat-value[data-v-81042d3d]{font-size:24px;font-weight:700;color:var(--primary)}.stat-label[data-v-81042d3d]{font-size:12px;color:var(--text-2);margin-top:2px}.btn[data-v-81042d3d]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-primary[data-v-81042d3d]{background:var(--primary);color:#fff}.btn-ghost[data-v-81042d3d]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-sm[data-v-81042d3d]{padding:4px 10px;font-size:12px}.btn-xs[data-v-81042d3d]{padding:3px 8px;font-size:11px}.empty[data-v-81042d3d]{text-align:center;padding:24px;color:var(--text-3)}.card-list[data-v-81042d3d]{display:flex;flex-direction:column;gap:2px}.list-item[data-v-81042d3d]{display:flex;align-items:center;gap:8px;padding:8px;border-radius:var(--radius-xs)}.list-item[data-v-81042d3d]:hover{background:var(--surface-hover)}.mono[data-v-81042d3d]{font-family:monospace;font-size:12px;flex:1}.list-time[data-v-81042d3d]{font-size:11px;color:var(--text-3)}.list-actions[data-v-81042d3d]{display:flex;gap:4px}.card-header[data-v-17ee96fc]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px;margin-bottom:16px}.card-header h3[data-v-17ee96fc]{font-size:15px;font-weight:600}.header-actions[data-v-17ee96fc]{display:flex;gap:8px;align-items:center;flex-wrap:wrap}.badge[data-v-17ee96fc]{font-size:10px;font-weight:500;padding:2px 8px;border-radius:20px;background:var(--primary-glow);color:var(--primary)}.glass-input[data-v-17ee96fc],.glass-select[data-v-17ee96fc]{padding:6px 10px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:12px;outline:none}.btn[data-v-17ee96fc]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer;transition:var(--transition)}.btn-ghost[data-v-17ee96fc]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-danger[data-v-17ee96fc]{background:var(--danger);color:#fff}.btn-sm[data-v-17ee96fc]{padding:4px 10px;font-size:12px}.empty[data-v-17ee96fc]{text-align:center;padding:40px;color:var(--text-3)}.table-wrap[data-v-17ee96fc]{overflow-x:auto}table[data-v-17ee96fc]{width:100%;border-collapse:collapse;font-size:13px}th[data-v-17ee96fc]{text-align:left;padding:8px 10px;font-weight:600;font-size:11px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border);white-space:nowrap}td[data-v-17ee96fc]{padding:7px 10px;border-bottom:1px solid var(--border)}tr:hover td[data-v-17ee96fc]{background:var(--surface-hover)}.mono[data-v-17ee96fc]{font-family:monospace;font-size:12px;color:var(--text-2)}.truncate[data-v-17ee96fc]{max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.detail-cell[data-v-17ee96fc]{max-width:180px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;font-size:12px;color:var(--text-2)}.tag[data-v-17ee96fc]{font-size:10px;padding:2px 7px;border-radius:4px;font-weight:600;white-space:nowrap}.tag-add[data-v-17ee96fc]{background:var(--success-subtle);color:var(--success)}.tag-forget[data-v-17ee96fc]{background:var(--danger-subtle);color:var(--danger)}.tag-correct[data-v-17ee96fc]{background:var(--warning-subtle);color:var(--warning)}.tag-ai[data-v-17ee96fc]{background:var(--info-subtle);color:var(--info)}.tag-review[data-v-17ee96fc]{background:var(--accent-subtle);color:var(--accent)}.tag-imp-permanent[data-v-17ee96fc]{background:var(--warning-subtle);color:var(--warning)}.tag-imp-important[data-v-17ee96fc]{background:var(--info-subtle);color:var(--info)}.tag-imp-normal[data-v-17ee96fc]{background:var(--surface);color:var(--text-2)}.stat-grid[data-v-4d7162d3]{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:16px;margin-bottom:16px}.plan-grid[data-v-4d7162d3]{display:grid;grid-template-columns:repeat(auto-fill,minmax(340px,1fr));gap:16px;margin-bottom:16px}.card-header[data-v-4d7162d3]{display:flex;align-items:center;gap:8px;margin-bottom:16px}.card-header h3[data-v-4d7162d3]{font-size:15px;font-weight:600}.badge[data-v-4d7162d3]{font-size:10px;font-weight:500;padding:2px 8px;border-radius:20px;background:var(--primary-glow);color:var(--primary)}.stat-value[data-v-4d7162d3]{font-size:22px;font-weight:700;letter-spacing:-.3px}.stat-label[data-v-4d7162d3]{font-size:12px;color:var(--text-2);margin-top:2px}.stat-sub[data-v-4d7162d3]{font-size:11px;color:var(--text-3)}.bar-wrap[data-v-4d7162d3]{height:4px;background:var(--surface);border-radius:2px;overflow:hidden;margin-top:6px}.bar[data-v-4d7162d3]{height:100%;background:var(--primary);border-radius:2px;transition:width .6s ease}.empty[data-v-4d7162d3]{text-align:center;padding:24px;color:var(--text-3);font-size:13px}.goal-list[data-v-4d7162d3]{display:flex;flex-direction:column;gap:2px}.goal-item[data-v-4d7162d3]{display:flex;align-items:flex-start;gap:10px;padding:8px 0}.goal-check[data-v-4d7162d3]{flex-shrink:0;margin-top:1px}.goal-body[data-v-4d7162d3]{flex:1}.goal-content[data-v-4d7162d3]{font-size:13px;line-height:1.4}.goal-id[data-v-4d7162d3]{font-family:monospace;font-size:10px;color:var(--text-3);margin-right:6px;padding:1px 4px;border-radius:3px;background:var(--surface)}.goal-item.done .goal-content[data-v-4d7162d3]{color:var(--text-3);text-decoration:line-through}.goal-meta[data-v-4d7162d3]{margin-top:2px;display:flex;gap:8px;align-items:center;flex-wrap:wrap}.day-badge[data-v-4d7162d3]{font-size:10px;font-weight:500;padding:1px 6px;border-radius:4px;background:var(--primary-glow);color:var(--primary)}.done-time[data-v-4d7162d3],.done-note[data-v-4d7162d3]{font-size:10px;color:var(--success)}.progress-note[data-v-4d7162d3]{font-size:10px;color:var(--text-3)}.table-wrap[data-v-4d7162d3]{overflow-x:auto}table[data-v-4d7162d3]{width:100%;border-collapse:collapse;font-size:13px}th[data-v-4d7162d3]{text-align:left;padding:8px 12px;font-weight:600;font-size:11px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border)}td[data-v-4d7162d3]{padding:6px 12px;border-bottom:1px solid var(--border)}tr:hover td[data-v-4d7162d3]{background:var(--surface-hover)}.mono[data-v-4d7162d3]{font-family:monospace;font-size:11px;color:var(--text-2);white-space:nowrap}.tag-kind[data-v-4d7162d3]{font-size:10px;padding:1px 6px;border-radius:3px;background:var(--primary-glow);color:var(--primary)}.info-list[data-v-4d7162d3]{display:flex;flex-direction:column;gap:8px}.info-item[data-v-4d7162d3]{font-size:13px;color:var(--text-2)}.stat-grid[data-v-aedd035b]{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:16px;margin-bottom:16px}.card-header[data-v-aedd035b]{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}.card-header h3[data-v-aedd035b]{font-size:15px;font-weight:600}.glass-select[data-v-aedd035b]{padding:8px 12px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:13px;outline:none}.stat-value[data-v-aedd035b]{font-size:26px;font-weight:700;letter-spacing:-.5px}.stat-label[data-v-aedd035b]{font-size:13px;color:var(--text-2);margin-top:4px}.stat-sub[data-v-aedd035b]{font-size:11px;color:var(--text-3)}.chart-grid[data-v-aedd035b]{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:16px;margin-bottom:16px}.chart-card[data-v-aedd035b]{text-align:center}.card-title[data-v-aedd035b]{font-size:14px;font-weight:600;margin-bottom:16px;text-align:left}.chart-container[data-v-aedd035b]{display:flex;align-items:center;justify-content:center;gap:20px;flex-wrap:wrap}.chart-legend[data-v-aedd035b]{display:flex;flex-direction:column;gap:8px;text-align:left}.legend-item[data-v-aedd035b]{display:flex;align-items:center;gap:8px;font-size:12px;color:var(--text-2)}.dot[data-v-aedd035b]{width:8px;height:8px;border-radius:50%;flex-shrink:0}.empty[data-v-aedd035b]{text-align:center;padding:32px;color:var(--text-3)}.prompt-row[data-v-aedd035b]{padding:10px 0;border-bottom:1px solid var(--border)}.prompt-row[data-v-aedd035b]:last-child{border-bottom:none}.pr-name[data-v-aedd035b]{font-size:13px;font-weight:600;margin-bottom:4px;text-transform:capitalize}.pr-bars[data-v-aedd035b]{display:flex;gap:16px;font-size:12px;margin-bottom:6px}.pr-stat[data-v-aedd035b]{display:flex;gap:4px;align-items:center}.pr-stat label[data-v-aedd035b]{color:var(--text-3)}.mono[data-v-aedd035b]{font-family:monospace;font-size:12px;color:var(--text-2)}.pr-bar-wrap[data-v-aedd035b]{height:6px;background:var(--surface);border-radius:3px;overflow:hidden;display:flex;gap:2px}.pr-bar[data-v-aedd035b]{height:100%;border-radius:3px;transition:width .5s ease}.bar-pt[data-v-aedd035b]{background:var(--primary)}.bar-ct[data-v-aedd035b]{background:var(--info)}.table-wrap[data-v-aedd035b]{overflow-x:auto}table[data-v-aedd035b]{width:100%;border-collapse:collapse;font-size:12px}th[data-v-aedd035b]{text-align:left;padding:8px 10px;font-weight:600;font-size:10px;color:var(--text-3);text-transform:uppercase;border-bottom:1px solid var(--border)}td[data-v-aedd035b]{padding:6px 10px;border-bottom:1px solid var(--border)}tr:hover td[data-v-aedd035b]{background:var(--surface-hover)}.tag[data-v-aedd035b]{font-size:10px;padding:2px 6px;border-radius:4px;background:var(--primary-glow);color:var(--primary)}.stat-grid[data-v-057a939c]{display:grid;grid-template-columns:repeat(auto-fill,minmax(170px,1fr));gap:16px;margin-bottom:24px}.card h3[data-v-057a939c]{font-size:14px;font-weight:600;margin-bottom:12px}.stat-icon[data-v-057a939c]{margin-bottom:8px}.stat-value[data-v-057a939c]{font-size:26px;font-weight:700;letter-spacing:-.5px}.stat-label[data-v-057a939c]{font-size:13px;color:var(--text-2);margin-top:4px;font-weight:500}.stat-sub[data-v-057a939c]{font-size:11px;color:var(--text-3);margin-top:2px}.card-row[data-v-057a939c]{display:flex;gap:16px}.half[data-v-057a939c]{flex:1;min-width:0}.kv-grid[data-v-057a939c]{display:grid;grid-template-columns:1fr 1fr;gap:8px}.kv[data-v-057a939c]{display:flex;justify-content:space-between;align-items:center;font-size:13px;padding:4px 0;border-bottom:1px solid var(--border)}.kv span[data-v-057a939c]{color:var(--text-2)}.kv strong[data-v-057a939c]{font-weight:600}.battery-visual[data-v-057a939c]{height:24px;background:var(--surface);border-radius:6px;overflow:hidden;margin-bottom:12px}.battery-fill[data-v-057a939c]{height:100%;border-radius:6px;transition:width .8s ease,background .8s ease;min-width:2px}.rhythm-bars[data-v-057a939c]{display:flex;flex-direction:column;gap:6px}.rhythm-row[data-v-057a939c]{display:flex;align-items:center;gap:8px}.rhythm-label[data-v-057a939c]{width:32px;font-size:11px;color:var(--text-2);flex-shrink:0}.rhythm-bar-wrap[data-v-057a939c]{flex:1;height:8px;background:var(--surface);border-radius:4px;overflow:hidden}.rhythm-bar[data-v-057a939c]{height:100%;border-radius:4px;transition:width .8s ease}.rhythm-pct[data-v-057a939c]{width:36px;font-size:11px;color:var(--text-2);text-align:right;flex-shrink:0}.section-title[data-v-057a939c]{font-size:13px;font-weight:600;color:var(--text-2);margin-bottom:8px}.toggle-grid[data-v-057a939c]{display:flex;flex-wrap:wrap;gap:8px;margin-bottom:20px}.toggle-chip[data-v-057a939c]{padding:4px 12px;border-radius:20px;font-size:11px;font-weight:500;background:var(--surface);border:1px solid var(--border);color:var(--text-3)}.toggle-chip.on[data-v-057a939c]{background:var(--primary-glow);border-color:var(--primary);color:var(--primary)}.empty[data-v-057a939c]{text-align:center;padding:40px;color:var(--text-3)}@media(max-width:768px){.card-row[data-v-057a939c]{flex-direction:column}}.card-header[data-v-02592b70]{display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:8px;margin-bottom:16px}.card-header h3[data-v-02592b70]{font-size:15px;font-weight:600}.header-actions[data-v-02592b70]{display:flex;gap:6px;align-items:center}.glass-input[data-v-02592b70]{padding:6px 10px;border-radius:var(--radius-xs);border:1px solid var(--border);background:var(--surface);color:var(--text);font-size:12px;outline:none}.btn[data-v-02592b70]{padding:8px 14px;border:none;border-radius:var(--radius-xs);font-size:13px;font-weight:500;cursor:pointer}.btn-ghost[data-v-02592b70]{background:var(--surface);color:var(--text);border:1px solid var(--border)}.btn-xs[data-v-02592b70]{padding:3px 8px;font-size:11px}.btn-sm[data-v-02592b70]{padding:4px 10px;font-size:12px}.empty[data-v-02592b70]{text-align:center;padding:40px;color:var(--text-3)}.rel-grid[data-v-02592b70]{display:grid;grid-template-columns:repeat(auto-fill,minmax(280px,1fr));gap:12px}.rel-card[data-v-02592b70]{padding:14px;border-radius:var(--radius-sm);background:var(--surface-hover);cursor:pointer;transition:var(--transition)}.rel-card[data-v-02592b70]:hover{box-shadow:var(--glass-shadow-lg);transform:translateY(-1px)}.rel-header[data-v-02592b70]{display:flex;align-items:center;justify-content:space-between;margin-bottom:10px}.rel-uid[data-v-02592b70]{font-size:13px;font-weight:600;font-family:monospace}.rel-type[data-v-02592b70]{font-size:10px;font-weight:600;padding:2px 8px;border-radius:4px;color:#fff}.rel-bars[data-v-02592b70]{display:flex;flex-direction:column;gap:4px;margin-bottom:8px}.rel-bar-row[data-v-02592b70]{display:flex;align-items:center;gap:6px}.rel-bar-label[data-v-02592b70]{width:28px;font-size:10px;color:var(--text-3);flex-shrink:0}.rel-bar-wrap[data-v-02592b70]{flex:1;height:4px;background:var(--surface);border-radius:2px;overflow:hidden}.rel-bar[data-v-02592b70]{height:100%;border-radius:2px;transition:width .5s ease}.rel-bar-pct[data-v-02592b70]{width:30px;font-size:10px;color:var(--text-3);text-align:right;flex-shrink:0}.rel-footer[data-v-02592b70]{display:flex;justify-content:space-between;font-size:11px;color:var(--text-3)}.modal-overlay[data-v-02592b70]{position:fixed;inset:0;background:#00000080;z-index:200;display:flex;align-items:center;justify-content:center}.modal[data-v-02592b70]{width:480px;max-height:80vh;overflow-y:auto;padding:24px}.modal-lg[data-v-02592b70]{width:600px}.modal-header[data-v-02592b70]{display:flex;align-items:center;justify-content:space-between;margin-bottom:16px}.modal-header h3[data-v-02592b70]{font-size:16px;font-weight:600}.detail-grid[data-v-02592b70]{display:grid;grid-template-columns:1fr 1fr;gap:16px}.detail-section h4[data-v-02592b70]{font-size:12px;font-weight:600;color:var(--text-2);margin-bottom:8px;text-transform:uppercase;letter-spacing:.5px}.kv-grid[data-v-02592b70]{display:grid;grid-template-columns:1fr 1fr;gap:6px}.kv[data-v-02592b70]{display:flex;justify-content:space-between;align-items:center;font-size:13px;padding:4px 0;border-bottom:1px solid var(--border-light)}.kv span[data-v-02592b70]{color:var(--text-2)}.kv strong[data-v-02592b70]{font-weight:600}.impression-text[data-v-02592b70]{font-size:13px;line-height:1.5;padding:10px;background:var(--surface-hover);border-radius:var(--radius-xs)}.event-list[data-v-02592b70]{display:flex;flex-direction:column;gap:6px}.event-item[data-v-02592b70]{display:flex;align-items:center;gap:8px;font-size:12px}.event-type[data-v-02592b70]{font-weight:600;color:var(--primary)}.event-detail[data-v-02592b70]{flex:1;color:var(--text-2)}.event-time[data-v-02592b70]{color:var(--text-3);font-size:11px}@media(max-width:768px){.detail-grid[data-v-02592b70]{grid-template-columns:1fr}}.shell{position:relative;z-index:10;height:100vh;display:grid;grid-template-columns:68px 1fr;grid-template-rows:56px 1fr;grid-template-areas:"rail topbar" "rail stage";overflow:hidden}.rail{grid-area:rail;background:#10101599;backdrop-filter:blur(24px) saturate(140%);-webkit-backdrop-filter:blur(24px) saturate(140%);border-right:1px solid var(--border);display:flex;flex-direction:column;align-items:stretch;padding:12px 0;position:relative;z-index:30;overflow:visible;transition:width .2s ease}[data-theme=light] .rail{background:#f6f0e2b8}.rail-logo{width:36px;height:36px;margin:0 auto 14px;border-radius:11px;background:linear-gradient(135deg,var(--gold),var(--gold-deep, #B87A12));display:flex;align-items:center;justify-content:center;font-size:16px;font-weight:800;color:var(--black, #0A0A0D);flex-shrink:0;box-shadow:0 0 24px #f5a62366,inset 0 1px #ffffff4d;position:relative}.rail-logo:after{content:"";position:absolute;inset:-3px;border-radius:14px;border:1px solid rgba(245,166,35,.3);animation:logoPulse 3s ease-in-out infinite}@keyframes logoPulse{0%,to{opacity:.3;transform:scale(1)}50%{opacity:.8;transform:scale(1.05)}}.rail-scroll{flex:1;overflow-y:auto;overflow-x:hidden;padding:4px 12px 16px;display:flex;flex-direction:column;align-items:stretch}.rail-sep{height:1px;margin:9px 4px;background:linear-gradient(90deg,transparent,var(--border-strong),transparent);flex-shrink:0}.rail-group-label{font-size:9px;font-weight:600;letter-spacing:1.5px;text-transform:uppercase;color:var(--text-3);padding:4px 6px 3px;white-space:nowrap;opacity:0;height:0;overflow:hidden;transition:opacity .18s ease}.rail-btn{width:42px;height:42px;margin:0 auto 3px;border-radius:11px;display:flex;align-items:center;justify-content:flex-start;gap:12px;padding:0 12px;cursor:pointer;color:var(--text-2);transition:all .2s cubic-bezier(.4,0,.2,1);position:relative;flex-shrink:0;text-decoration:none;white-space:nowrap}.rail-icon{display:flex;align-items:center;justify-content:center;width:18px;height:18px;flex-shrink:0}.rail-text{font-size:13px;font-weight:500;opacity:0;transition:opacity .18s ease}.rail-btn:hover:not(.active){background:#f5a62314;color:var(--gold)}[data-theme=light] .rail-btn:hover:not(.active){background:var(--primary-subtle);color:var(--primary)}.rail-btn.active{background:#f5a6231f;color:var(--gold)}[data-theme=light] .rail-btn.active{background:var(--primary-subtle);color:var(--primary)}.rail-btn.active:before{content:"";position:absolute;left:-12px;top:50%;transform:translateY(-50%);width:3px;height:20px;background:var(--gold);border-radius:0 3px 3px 0;box-shadow:0 0 12px var(--gold)}.rail:hover{width:224px;box-shadow:12px 0 34px #0006;background:#101015eb}[data-theme=light] .rail:hover{background:#f6f0e2f2;box-shadow:12px 0 34px #5a420f24}.rail:hover .rail-text{opacity:1}.rail:hover .rail-group-label{opacity:1;height:auto;padding:4px 6px 3px}.rail:hover .rail-btn{width:auto;align-self:stretch;margin-left:0;margin-right:0}.rail:not(:hover) .rail-btn{align-self:center}.topbar{grid-area:topbar;background:#10101566;backdrop-filter:blur(24px) saturate(140%);-webkit-backdrop-filter:blur(24px) saturate(140%);border-bottom:1px solid var(--border);display:flex;align-items:center;justify-content:space-between;padding:0 26px;position:relative;z-index:20}[data-theme=light] .topbar{background:#f6f0e299}.tb-left{display:flex;align-items:baseline;gap:16px;min-width:0}.menu-btn{display:none;align-items:center;justify-content:center;width:32px;height:32px;padding:0;background:none;border:1px solid var(--border);border-radius:9px;cursor:pointer;color:var(--text-1);align-self:center}.menu-btn:hover{color:var(--gold);border-color:var(--border-strong)}.tb-title{font-size:15px;font-weight:600;letter-spacing:.3px;white-space:nowrap}.tb-sub{font-size:11.5px;color:var(--text-2);font-family:var(--font-mono);letter-spacing:.5px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;min-width:0}.tb-right{display:flex;align-items:center;gap:10px;flex-shrink:0}.tb-status{display:flex;align-items:center;gap:7px;font-size:11.5px;color:var(--text-1);padding:6px 12px;border-radius:20px;background:#f5a6230f;border:1px solid var(--border);font-family:var(--font-mono);letter-spacing:.3px}.tb-status.offline{color:var(--danger);background:var(--danger-subtle);border-color:var(--danger-subtle)}.tb-status .dot{width:6px;height:6px;border-radius:50%;background:var(--gold);box-shadow:0 0 8px var(--gold);animation:pulse 2s infinite}.tb-status.offline .dot{background:var(--danger);box-shadow:0 0 8px var(--danger)}@keyframes pulse{0%,to{opacity:1}50%{opacity:.3}}.tb-icon{width:34px;height:34px;border-radius:9px;border:1px solid var(--border);background:transparent;color:var(--text-1);cursor:pointer;display:flex;align-items:center;justify-content:center;transition:all .15s;padding:0}.tb-icon:hover{color:var(--gold);border-color:var(--border-strong);background:#f5a6230d}.stage{grid-area:stage;position:relative;overflow-y:auto;overflow-x:hidden;padding:24px 28px;z-index:10}.stage::-webkit-scrollbar{width:5px}.stage::-webkit-scrollbar-thumb{background:#f5a62340;border-radius:5px}.stage::-webkit-scrollbar-track{background:transparent}.page-content{min-height:100%;animation:fadeIn .3s ease}.stage:has(.ov-layout) .page-content{height:100%;min-height:0}.login-page{position:relative;z-index:1;min-height:100vh;display:flex;align-items:center;justify-content:center;background:var(--bg);padding:20px}.login-card{width:360px;padding:40px 36px;text-align:center;background:var(--surface);backdrop-filter:blur(14px);border-radius:var(--radius);border:1px solid var(--glass-border);box-shadow:var(--glass-shadow-lg);position:relative;overflow:hidden}.login-card:before{content:"";position:absolute;top:0;left:0;right:0;height:1px;background:linear-gradient(90deg,transparent,var(--gold),transparent)}.login-core{position:relative;width:72px;height:72px;margin:0 auto 18px}.login-core .core-ring{position:absolute;inset:0;border:1px solid var(--gold-dim);border-radius:50%;animation:pulse 2.4s ease-in-out infinite}.login-core .core-ring.r2{inset:-8px;opacity:.4;animation-delay:.6s}.login-core .core-dot{position:absolute;inset:22px;border-radius:50%;background:radial-gradient(circle,var(--gold) 0%,rgba(245,166,35,.2) 70%,transparent 100%);box-shadow:0 0 24px var(--gold-glow);animation:pulse 1.8s ease-in-out infinite}.login-title{font-size:22px;font-weight:400;letter-spacing:.12em}.login-sub{font-size:11px;color:var(--gold-dim);margin:6px 0 22px;letter-spacing:.18em;text-transform:uppercase}[data-theme=light] .login-sub{color:var(--primary)}.login-input-group{display:flex;flex-direction:column;gap:10px}.login-input-group input{padding:11px 14px;border-radius:var(--radius-sm);border:1px solid var(--border);background:var(--chip-solid);color:var(--text);font-size:14px;outline:none;transition:var(--transition-fast)}.login-input-group input:focus{border-color:var(--gold-dim);box-shadow:0 0 0 3px var(--primary-glow)}.login-btn{display:flex;align-items:center;justify-content:center;gap:8px;padding:11px;border:1px solid var(--primary);border-radius:var(--radius-sm);background:var(--primary);color:#0a0a0d;font-size:14px;font-weight:600;letter-spacing:.08em;cursor:pointer;transition:var(--transition-fast)}[data-theme=light] .login-btn{color:#fdfaf2}.login-btn:hover{background:var(--primary-hover);border-color:var(--primary-hover)}.login-btn:disabled{opacity:.5;cursor:not-allowed}.login-err{color:var(--danger);font-size:13px;margin-top:10px}.login-footer{margin-top:20px;font-size:12px;color:var(--text-3);display:flex;justify-content:center;gap:6px}.overlay{position:fixed;inset:0;background:#00000073;z-index:29}@media(max-width:768px){.shell{grid-template-columns:1fr;grid-template-rows:56px 1fr;grid-template-areas:"topbar" "stage"}.rail{position:fixed;top:0;left:0;bottom:0;width:224px;transform:translate(-260px);transition:transform .25s ease;z-index:40;background:#101015f5;box-shadow:none}[data-theme=light] .rail{background:#f6f0e2f7}.rail.open{transform:translate(0);box-shadow:12px 0 34px #0006}.rail .rail-text{opacity:1}.rail .rail-group-label{opacity:1;height:auto}.rail .rail-btn{width:auto;align-self:stretch}.menu-btn{display:flex}.topbar{padding:0 14px;gap:10px}.tb-left{gap:10px}.tb-sub,.tb-status{display:none}.stage{padding:14px 14px 28px}}</style>
</head>
<body>
  <div id="app"></div>
</body>
</html>
"##;
