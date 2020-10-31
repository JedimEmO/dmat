
(function(l, r) { if (l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (window.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(window.document);
(function () {
    'use strict';

    function add_event(elem, name, f) {
            elem.addEventListener(name, f, {
                capture: true,
                once: false,
                passive: true
            });
        }

        function remove_event(elem, name, f) {
            elem.removeEventListener(name, f, true);
        }

    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
        : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

                } else {
                    state.a = a;
                }
            }
        };
        real.original = state;

        return real;
    }

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }
    function __wbg_adapter_16(arg0, arg1, arg2) {
        try {
            wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd065bc04ad9dc635(arg0, arg1, addBorrowedObject(arg2));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function __wbg_adapter_19(arg0, arg1, arg2) {
        try {
            wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3fe6e8ed8d913779(arg0, arg1, addBorrowedObject(arg2));
        } finally {
            heap[stack_pointer++] = undefined;
        }
    }

    function __wbg_adapter_22(arg0, arg1, arg2) {
        wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0e1e071992ce4d68(arg0, arg1, addHeapObject(arg2));
    }

    function getCachedStringFromWasm0(ptr, len) {
        if (ptr === 0) {
            return getObject(len);
        } else {
            return getStringFromWasm0(ptr, len);
        }
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function handleError(f) {
        return function () {
            try {
                return f.apply(this, arguments);

            } catch (e) {
                wasm.__wbindgen_exn_store(addHeapObject(e));
            }
        };
    }

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {

            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {

            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            input = (document.currentScript && document.currentScript.src || new URL('index.js', document.baseURI).href).replace(/\.js$/, '_bg.wasm');
        }
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            var ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_cb_drop = function(arg0) {
            const obj = takeObject(arg0).original;
            if (obj.cnt-- == 1) {
                obj.a = 0;
                return true;
            }
            var ret = false;
            return ret;
        };
        imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
            var ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_addevent_ccc6ab519df88e22 = function(arg0, arg1, arg2, arg3) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            add_event(getObject(arg0), v0, getObject(arg3));
        };
        imports.wbg.__wbg_removeevent_0f7fcdc74d4840b2 = function(arg0, arg1, arg2, arg3) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            remove_event(getObject(arg0), v0, getObject(arg3));
        };
        imports.wbg.__wbg_instanceof_Window_adf3196bdc02b386 = function(arg0) {
            var ret = getObject(arg0) instanceof Window;
            return ret;
        };
        imports.wbg.__wbg_document_6cc8d0b87c0a99b9 = function(arg0) {
            var ret = getObject(arg0).document;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_alert_af5c272d926011d5 = handleError(function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            getObject(arg0).alert(v0);
        });
        imports.wbg.__wbg_body_8c888fe47d81765f = function(arg0) {
            var ret = getObject(arg0).body;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        };
        imports.wbg.__wbg_createElement_5bdf88a5af9f17c5 = handleError(function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            var ret = getObject(arg0).createElement(v0);
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_createTextNode_b3c9e3cb02f83ab5 = function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            var ret = getObject(arg0).createTextNode(v0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_appendChild_77215fd672b162c5 = handleError(function(arg0, arg1) {
            var ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_insertBefore_ea385f1d7ec76e50 = handleError(function(arg0, arg1, arg2) {
            var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_removeChild_f4829fcd2e376e1b = handleError(function(arg0, arg1) {
            var ret = getObject(arg0).removeChild(getObject(arg1));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_replaceChild_a97c4b16a9298934 = handleError(function(arg0, arg1, arg2) {
            var ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_classList_1777292e1272c00a = function(arg0) {
            var ret = getObject(arg0).classList;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_instanceof_HtmlElement_9cd64b297a10eb1e = function(arg0) {
            var ret = getObject(arg0) instanceof HTMLElement;
            return ret;
        };
        imports.wbg.__wbg_add_8b1639d7f0da2a2a = handleError(function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            getObject(arg0).add(v0);
        });
        imports.wbg.__wbg_remove_fd6b39cc66e1c2ff = handleError(function(arg0, arg1, arg2) {
            var v0 = getCachedStringFromWasm0(arg1, arg2);
            getObject(arg0).remove(v0);
        });
        imports.wbg.__wbg_call_8e95613cc6524977 = handleError(function(arg0, arg1) {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_newnoargs_f3b8a801d5d4b079 = function(arg0, arg1) {
            var v0 = getCachedStringFromWasm0(arg0, arg1);
            var ret = new Function(v0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_resolve_2529512c3bb73938 = function(arg0) {
            var ret = Promise.resolve(getObject(arg0));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_then_4a7a614abbbe6d81 = function(arg0, arg1) {
            var ret = getObject(arg0).then(getObject(arg1));
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_self_07b2f89e82ceb76d = handleError(function() {
            var ret = self.self;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_window_ba85d88572adc0dc = handleError(function() {
            var ret = window.window;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_globalThis_b9277fc37e201fe5 = handleError(function() {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        });
        imports.wbg.__wbg_global_e16303fe83e1d57f = handleError(function() {
            var ret = global.global;
            return addHeapObject(ret);
        });
        imports.wbg.__wbindgen_is_undefined = function(arg0) {
            var ret = getObject(arg0) === undefined;
            return ret;
        };
        imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
            var ret = debugString(getObject(arg1));
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbindgen_closure_wrapper72 = function(arg0, arg1, arg2) {
            var ret = makeMutClosure(arg0, arg1, 17, __wbg_adapter_16);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper263 = function(arg0, arg1, arg2) {
            var ret = makeMutClosure(arg0, arg1, 115, __wbg_adapter_19);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper324 = function(arg0, arg1, arg2) {
            var ret = makeMutClosure(arg0, arg1, 133, __wbg_adapter_22);
            return addHeapObject(ret);
        };

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        const { instance, module } = await load(await input, imports);

        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    }

    init("js/assets/dominator-material-component-showcase-9568ed7e.wasm").catch(console.error);

}());
//# sourceMappingURL=index.js.map
