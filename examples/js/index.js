!function(){"use strict";let n;const e=new Array(128).fill(void 0);function t(n){return e[n]}e.push(void 0,null,!0,!1);let _=e.length;function r(n){_===e.length&&e.push(e.length+1);const t=_;return _=e[t],e[t]=n,t}const o="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&o.decode();let c=null;function u(){return null!==c&&0!==c.byteLength||(c=new Uint8Array(n.memory.buffer)),c}function i(n,e){return n>>>=0,o.decode(u().subarray(n,n+e))}function b(n){const r=t(n);return function(n){n<132||(e[n]=_,_=n)}(n),r}function f(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=f(n[0]));for(let _=1;_<e;_++)t+=", "+f(n[_]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let _;if(!(t.length>1))return toString.call(n);if(_=t[1],"Object"==_)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:_}let a=0;const w="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},g="function"==typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const t=w.encode(n);return e.set(t),{read:n.length,written:t.length}};function s(n,e,t){if(void 0===t){const t=w.encode(n),_=e(t.length,1)>>>0;return u().subarray(_,_+t.length).set(t),a=t.length,_}let _=n.length,r=e(_,1)>>>0;const o=u();let c=0;for(;c<_;c++){const e=n.charCodeAt(c);if(e>127)break;o[r+c]=e}if(c!==_){0!==c&&(n=n.slice(c)),r=t(r,_,_=c+3*n.length,1)>>>0;const e=u().subarray(r+c,r+_);c+=g(n,e).written}return a=c,r}let l=null;function d(){return null!==l&&0!==l.byteLength||(l=new Int32Array(n.memory.buffer)),l}function m(e,t,_,r){const o={a:e,b:t,cnt:1,dtor:_},c=(...e)=>{o.cnt++;const t=o.a;o.a=0;try{return r(t,o.b,...e)}finally{0==--o.cnt?n.__wbindgen_export_2.get(o.dtor)(t,o.b):o.a=t}};return c.original=o,c}let y=128;function h(t,_,r){try{n._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc2ea6bbb994c6f6e(t,_,function(n){if(1==y)throw new Error("out of js stack");return e[--y]=n,y}(r))}finally{e[y++]=void 0}}function p(e,t){n._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hac898d347ae1ef72(e,t)}function v(e,t,_){n._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4e5dc33fc9119c3e(e,t,r(_))}function E(e,t){n._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h122873551ea4f741(e,t)}function A(e,t){try{return e.apply(this,t)}catch(e){n.__wbindgen_exn_store(r(e))}}function T(n,e){return 0===n?t(e):i(n,e)}function S(n){return null==n}function L(){const e={wbg:{}};return e.wbg.__wbindgen_object_clone_ref=function(n){return r(t(n))},e.wbg.__wbg_setinnerHTML_b089587252408b67=function(n,e,_){var r=T(e,_);t(n).innerHTML=r},e.wbg.__wbg_classList_5f2fc1d67656292e=function(n){return r(t(n).classList)},e.wbg.__wbg_setdata_8eae47221c7bc167=function(n,e,_){var r=T(e,_);t(n).data=r},e.wbg.__wbg_new_a76f6bcb38f791ea=function(){return A((function(n,e){var t=T(n,e);return r(new URL(t))}),arguments)},e.wbg.__wbg_hash_2b57e787945b2db0=function(e,_){const r=s(t(_).hash,n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r},e.wbg.__wbg_new_b51585de1b234aff=function(){return r(new Object)},e.wbg.__wbg_blur_53431c003c82bf53=function(){return A((function(n){t(n).blur()}),arguments)},e.wbg.__wbg_focus_dbcbbbb2a04c0e1f=function(){return A((function(n){t(n).focus()}),arguments)},e.wbg.__wbindgen_string_new=function(n,e){return r(i(n,e))},e.wbg.__wbg_style_3801009b2339aa94=function(n){return r(t(n).style)},e.wbg.__wbg_removeProperty_298c1bf4d39723f9=function(){return A((function(e,_,r,o){var c=T(r,o);const u=s(t(_).removeProperty(c),n.__wbindgen_malloc,n.__wbindgen_realloc),i=a;d()[e/4+1]=i,d()[e/4+0]=u}),arguments)},e.wbg.__wbg_setProperty_b553f5bd44a17563=function(){return A((function(n,e,_,r,o,c,u){var i=T(e,_),b=T(r,o),f=T(c,u);t(n).setProperty(i,b,f)}),arguments)},e.wbg.__wbg_getPropertyValue_45480a940d0053a0=function(){return A((function(e,_,r,o){var c=T(r,o);const u=s(t(_).getPropertyValue(c),n.__wbindgen_malloc,n.__wbindgen_realloc),i=a;d()[e/4+1]=i,d()[e/4+0]=u}),arguments)},e.wbg.__wbg_instanceof_HtmlSelectElement_75d8a9ac3b088f08=function(n){let e;try{e=t(n)instanceof HTMLSelectElement}catch{e=!1}return e},e.wbg.__wbg_value_c45528fab757534f=function(e,_){const r=s(t(_).value,n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r},e.wbg.__wbg_pushState_1145414a47c0b629=function(){return A((function(n,e,_,r,o,c){var u=T(_,r),i=T(o,c);t(n).pushState(t(e),u,i)}),arguments)},e.wbg.__wbg_error_788ae33f81d3b84b=function(n){console.error(t(n))},e.wbg.__wbg_body_674aec4c1c0910cd=function(n){const e=t(n).body;return S(e)?0:r(e)},e.wbg.__wbg_instanceof_HtmlElement_6f4725d4677c7968=function(n){let e;try{e=t(n)instanceof HTMLElement}catch{e=!1}return e},e.wbg.__wbindgen_object_drop_ref=function(n){b(n)},e.wbg.__wbg_instanceof_SvgAnimationElement_040dc56a7f077803=function(n){let e;try{e=t(n)instanceof SVGAnimationElement}catch{e=!1}return e},e.wbg.__wbg_instanceof_SvgElement_83d0bd65c48eb95d=function(n){let e;try{e=t(n)instanceof SVGElement}catch{e=!1}return e},e.wbg.__wbg_appendChild_51339d4cde00ee22=function(){return A((function(n,e){return r(t(n).appendChild(t(e)))}),arguments)},e.wbg.__wbg_document_f7ace2b956f30a4f=function(n){const e=t(n).document;return S(e)?0:r(e)},e.wbg.__wbg_location_56243dba507f472d=function(n){return r(t(n).location)},e.wbg.__wbg_href_d62a28e4fc1ab948=function(){return A((function(e,_){const r=s(t(_).href,n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r}),arguments)},e.wbg.__wbg_history_3c2280e6b2a9316e=function(){return A((function(n){return r(t(n).history)}),arguments)},e.wbg.__wbg_createElement_4891554b28d3388b=function(){return A((function(n,e,_){var o=T(e,_);return r(t(n).createElement(o))}),arguments)},e.wbg.__wbg_createElementNS_119acf9e82482041=function(){return A((function(n,e,_,o,c){var u=T(e,_),i=T(o,c);return r(t(n).createElementNS(u,i))}),arguments)},e.wbg.__wbg_createTextNode_2fd22cd7e543f938=function(n,e,_){var o=T(e,_);return r(t(n).createTextNode(o))},e.wbg.__wbg_createComment_6b5ea2660a7c961a=function(n,e,_){var o=T(e,_);return r(t(n).createComment(o))},e.wbg.__wbg_setAttribute_e7e80b478b7b8b2f=function(){return A((function(n,e,_,r,o){var c=T(e,_),u=T(r,o);t(n).setAttribute(c,u)}),arguments)},e.wbg.__wbg_removeAttribute_d8404da431968808=function(){return A((function(n,e,_){var r=T(e,_);t(n).removeAttribute(r)}),arguments)},e.wbg.__wbg_add_3eafedc4b2a28db0=function(){return A((function(n,e,_){var r=T(e,_);t(n).add(r)}),arguments)},e.wbg.__wbg_remove_8ae45e50cb58bb66=function(){return A((function(n,e,_){var r=T(e,_);t(n).remove(r)}),arguments)},e.wbg.__wbg_removeChild_973429f368206138=function(){return A((function(n,e){return r(t(n).removeChild(t(e)))}),arguments)},e.wbg.__wbg_insertBefore_ffa01d4b747c95fc=function(){return A((function(n,e,_){return r(t(n).insertBefore(t(e),t(_)))}),arguments)},e.wbg.__wbg_replaceChild_3ec13b15218637aa=function(){return A((function(n,e,_){return r(t(n).replaceChild(t(e),t(_)))}),arguments)},e.wbg.__wbg_instanceof_HtmlInputElement_31b50e0cf542c524=function(n){let e;try{e=t(n)instanceof HTMLInputElement}catch{e=!1}return e},e.wbg.__wbg_value_9423da9d988ee8cf=function(e,_){const r=s(t(_).value,n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r},e.wbg.__wbg_instanceof_HtmlTextAreaElement_348d0e222e16eec4=function(n){let e;try{e=t(n)instanceof HTMLTextAreaElement}catch{e=!1}return e},e.wbg.__wbg_value_3c5f08ffc2b7d6f9=function(e,_){const r=s(t(_).value,n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r},e.wbg.__wbindgen_is_object=function(n){const e=t(n);return"object"==typeof e&&null!==e},e.wbg.__wbg_crypto_c48a774b022d20ac=function(n){return r(t(n).crypto)},e.wbg.__wbg_process_298734cf255a885d=function(n){return r(t(n).process)},e.wbg.__wbg_versions_e2e78e134e3e5d01=function(n){return r(t(n).versions)},e.wbg.__wbg_node_1cd7a5d853dbea79=function(n){return r(t(n).node)},e.wbg.__wbindgen_is_string=function(n){return"string"==typeof t(n)},e.wbg.__wbg_require_8f08ceecec0f4fee=function(){return A((function(){return r(module.require)}),arguments)},e.wbg.__wbindgen_is_function=function(n){return"function"==typeof t(n)},e.wbg.__wbg_call_01734de55d61e11d=function(){return A((function(n,e,_){return r(t(n).call(t(e),t(_)))}),arguments)},e.wbg.__wbg_msCrypto_bcb970640f50a1e8=function(n){return r(t(n).msCrypto)},e.wbg.__wbg_newwithlength_e5d69174d6984cd7=function(n){return r(new Uint8Array(n>>>0))},e.wbg.__wbindgen_cb_drop=function(n){const e=b(n).original;if(1==e.cnt--)return e.a=0,!0;return!1},e.wbg.__wbg_self_1ff1d729e9aae938=function(){return A((function(){return r(self.self)}),arguments)},e.wbg.__wbg_window_5f4faef6c12b79ec=function(){return A((function(){return r(window.window)}),arguments)},e.wbg.__wbg_globalThis_1d39714405582d3c=function(){return A((function(){return r(globalThis.globalThis)}),arguments)},e.wbg.__wbg_global_651f05c6a0944d1c=function(){return A((function(){return r(global.global)}),arguments)},e.wbg.__wbindgen_is_undefined=function(n){return void 0===t(n)},e.wbg.__wbg_newnoargs_581967eacc0e2604=function(n,e){var t=T(n,e);return r(new Function(t))},e.wbg.__wbg_call_cb65541d95d71282=function(){return A((function(n,e){return r(t(n).call(t(e)))}),arguments)},e.wbg.__wbg_set_092e06b0f9d71865=function(){return A((function(n,e,_){return Reflect.set(t(n),t(e),t(_))}),arguments)},e.wbg.__wbindgen_memory=function(){return r(n.memory)},e.wbg.__wbg_buffer_085ec1f694018c4f=function(n){return r(t(n).buffer)},e.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa=function(n,e,_){return r(new Uint8Array(t(n),e>>>0,_>>>0))},e.wbg.__wbg_randomFillSync_dc1e9a60c158336d=function(){return A((function(n,e){t(n).randomFillSync(b(e))}),arguments)},e.wbg.__wbg_subarray_13db269f57aa838d=function(n,e,_){return r(t(n).subarray(e>>>0,_>>>0))},e.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab=function(){return A((function(n,e){t(n).getRandomValues(t(e))}),arguments)},e.wbg.__wbg_new_8125e318e6245eed=function(n){return r(new Uint8Array(t(n)))},e.wbg.__wbg_set_5cf90238115182c3=function(n,e,_){t(n).set(t(e),_>>>0)},e.wbg.__wbindgen_debug_string=function(e,_){const r=s(f(t(_)),n.__wbindgen_malloc,n.__wbindgen_realloc),o=a;d()[e/4+1]=o,d()[e/4+0]=r},e.wbg.__wbindgen_throw=function(n,e){throw new Error(i(n,e))},e.wbg.__wbindgen_rethrow=function(n){throw b(n)},e.wbg.__wbg_then_f7e06ee3c11698eb=function(n,e){return r(t(n).then(t(e)))},e.wbg.__wbg_resolve_53698b95aaf7fcf8=function(n){return r(Promise.resolve(t(n)))},e.wbg.__wbg_error_d9bce418caafb712=function(n,e,_,r){console.error(t(n),t(e),t(_),t(r))},e.wbg.__wbg_warn_dfc0e0cf544a13bd=function(n,e,_,r){console.warn(t(n),t(e),t(_),t(r))},e.wbg.__wbg_info_bb52f40b06f679de=function(n,e,_,r){console.info(t(n),t(e),t(_),t(r))},e.wbg.__wbg_log_ea7093e35e3efd07=function(n,e,_,r){console.log(t(n),t(e),t(_),t(r))},e.wbg.__wbg_debug_9b8701f894da9929=function(n,e,_,r){console.debug(t(n),t(e),t(_),t(r))},e.wbg.__wbg_performance_2c295061c8b01e0b=function(n){const e=t(n).performance;return S(e)?0:r(e)},e.wbg.__wbg_now_0cfdc90c97d0c24b=function(n){return t(n).now()},e.wbg.__wbg_instanceof_Window_9029196b662bc42a=function(n){let e;try{e=t(n)instanceof Window}catch{e=!1}return e},e.wbg.__wbg_target_f171e89c61e2bccf=function(n){const e=t(n).target;return S(e)?0:r(e)},e.wbg.__wbg_addEventListener_a5963e26cd7b176b=function(){return A((function(n,e,_,r,o){var c=T(e,_);t(n).addEventListener(c,t(r),t(o))}),arguments)},e.wbg.__wbg_removeEventListener_782040b4432709cb=function(){return A((function(n,e,_,r,o){var c=T(e,_);t(n).removeEventListener(c,t(r),0!==o)}),arguments)},e.wbg.__wbg_beginElement_16a0ac1dc0af70ff=function(){return A((function(n){t(n).beginElement()}),arguments)},e.wbg.__wbg_setTimeout_eb1a0d116c26d9f6=function(){return A((function(n,e,_){return t(n).setTimeout(t(e),_)}),arguments)},e.wbg.__wbindgen_closure_wrapper501=function(n,e,t){return r(m(n,e,28,h))},e.wbg.__wbindgen_closure_wrapper1069=function(e,t,_){const o=function(e,t,_,r){const o={a:e,b:t,cnt:1,dtor:_},c=(...e)=>{o.cnt++;try{return r(o.a,o.b,...e)}finally{0==--o.cnt&&(n.__wbindgen_export_2.get(o.dtor)(o.a,o.b),o.a=0)}};return c.original=o,c}(e,t,28,p);return r(o)},e.wbg.__wbindgen_closure_wrapper2047=function(n,e,t){return r(m(n,e,59,v))},e.wbg.__wbindgen_closure_wrapper2101=function(n,e,t){return r(m(n,e,59,E))},e}async function x(e){if(void 0!==n)return n;const t=L();("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e));const{instance:_,module:r}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await e,t);return function(e,t){return n=e.exports,x.__wbindgen_wasm_module=t,l=null,c=null,n.__wbindgen_start(),n}(_,r)}x("js/assets/dmat_showcase-3a20c5ab.wasm").catch(console.error)}();
//# sourceMappingURL=index.js.map
