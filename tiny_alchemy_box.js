/* tslint:disable */
import * as wasm from './tiny_alchemy_box_bg';

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

let WASM_VECTOR_LEN = 0;

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
}
/**
* @param {string} arg0
* @returns {void}
*/
export function process(arg0) {
    const ptr0 = passStringToWasm(arg0);
    const len0 = WASM_VECTOR_LEN;
    try {
        return wasm.process(ptr0, len0);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

export function __widl_instanceof_CanvasRenderingContext2D(idx) {
    return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0;
}

const __widl_f_begin_path_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.beginPath || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.beginPath does not exist`);
};

export function __widl_f_begin_path_CanvasRenderingContext2D(arg0) {
    __widl_f_begin_path_CanvasRenderingContext2D_target.call(getObject(arg0));
}

const __widl_f_fill_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fill || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fill does not exist`);
};

export function __widl_f_fill_CanvasRenderingContext2D(arg0) {
    __widl_f_fill_CanvasRenderingContext2D_target.call(getObject(arg0));
}

const __widl_f_stroke_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.stroke || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.stroke does not exist`);
};

export function __widl_f_stroke_CanvasRenderingContext2D(arg0) {
    __widl_f_stroke_CanvasRenderingContext2D_target.call(getObject(arg0));
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
return {}
}

const __widl_f_line_width_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'lineWidth').get || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineWidth does not exist`);
};

export function __widl_f_line_width_CanvasRenderingContext2D(arg0) {
    return __widl_f_line_width_CanvasRenderingContext2D_target.call(getObject(arg0));
}

const __widl_f_set_line_width_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'lineWidth').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineWidth does not exist`);
};

export function __widl_f_set_line_width_CanvasRenderingContext2D(arg0, arg1) {
    __widl_f_set_line_width_CanvasRenderingContext2D_target.call(getObject(arg0), arg1);
}

const __widl_f_arc_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.arc || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.arc does not exist`);
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

export function __widl_f_arc_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4, arg5, exnptr) {
    try {
        __widl_f_arc_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4, arg5);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

const __widl_f_line_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.lineTo || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineTo does not exist`);
};

export function __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    __widl_f_line_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);
}

const __widl_f_move_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.moveTo || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.moveTo does not exist`);
};

export function __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    __widl_f_move_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);
}

const __widl_f_clear_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.clearRect || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.clearRect does not exist`);
};

export function __widl_f_clear_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    __widl_f_clear_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

const __widl_f_fill_text_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillText || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillText does not exist`);
};

export function __widl_f_fill_text_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        __widl_f_fill_text_CanvasRenderingContext2D_target.call(getObject(arg0), varg1, arg3, arg4);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

const __widl_f_font_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'font').get || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.font does not exist`);
};

export function __widl_f_font_CanvasRenderingContext2D(ret, arg0) {

    const retptr = passStringToWasm(__widl_f_font_CanvasRenderingContext2D_target.call(getObject(arg0)));
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

const __widl_f_set_font_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'font').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.font does not exist`);
};

export function __widl_f_set_font_CanvasRenderingContext2D(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_font_CanvasRenderingContext2D_target.call(getObject(arg0), varg1);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

const __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {
    throw new Error(`wasm-bindgen: Document.getElementById does not exist`);
};

export function __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

const __widl_f_get_elements_by_tag_name_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementsByTagName || function() {
    throw new Error(`wasm-bindgen: Document.getElementsByTagName does not exist`);
};

export function __widl_f_get_elements_by_tag_name_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    return addHeapObject(__widl_f_get_elements_by_tag_name_Document_target.call(getObject(arg0), varg1));
}

const __widl_f_client_width_Element_target = GetOwnOrInheritedPropertyDescriptor(typeof Element === 'undefined' ? null : Element.prototype, 'clientWidth').get || function() {
    throw new Error(`wasm-bindgen: Element.clientWidth does not exist`);
};

export function __widl_f_client_width_Element(arg0) {
    return __widl_f_client_width_Element_target.call(getObject(arg0));
}

const __widl_f_set_inner_html_Element_target = GetOwnOrInheritedPropertyDescriptor(typeof Element === 'undefined' ? null : Element.prototype, 'innerHTML').set || function() {
    throw new Error(`wasm-bindgen: Element.innerHTML does not exist`);
};

export function __widl_f_set_inner_html_Element(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_inner_html_Element_target.call(getObject(arg0), varg1);
}

export function __widl_instanceof_HTMLCanvasElement(idx) {
    return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0;
}

const __widl_f_get_context_HTMLCanvasElement_target = typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype.getContext || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.getContext does not exist`);
};

export function __widl_f_get_context_HTMLCanvasElement(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = __widl_f_get_context_HTMLCanvasElement_target.call(getObject(arg0), varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

const __widl_f_width_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'width').get || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.width does not exist`);
};

export function __widl_f_width_HTMLCanvasElement(arg0) {
    return __widl_f_width_HTMLCanvasElement_target.call(getObject(arg0));
}

const __widl_f_set_width_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'width').set || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.width does not exist`);
};

export function __widl_f_set_width_HTMLCanvasElement(arg0, arg1) {
    __widl_f_set_width_HTMLCanvasElement_target.call(getObject(arg0), arg1);
}

const __widl_f_height_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'height').get || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.height does not exist`);
};

export function __widl_f_height_HTMLCanvasElement(arg0) {
    return __widl_f_height_HTMLCanvasElement_target.call(getObject(arg0));
}

const __widl_f_set_height_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'height').set || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.height does not exist`);
};

export function __widl_f_set_height_HTMLCanvasElement(arg0, arg1) {
    __widl_f_set_height_HTMLCanvasElement_target.call(getObject(arg0), arg1);
}

const __widl_f_item_HTMLCollection_target = typeof HTMLCollection === 'undefined' ? null : HTMLCollection.prototype.item || function() {
    throw new Error(`wasm-bindgen: HTMLCollection.item does not exist`);
};

export function __widl_f_item_HTMLCollection(arg0, arg1) {

    const val = __widl_f_item_HTMLCollection_target.call(getObject(arg0), arg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

const __widl_f_set_hidden_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'hidden').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.hidden does not exist`);
};

export function __widl_f_set_hidden_HTMLElement(arg0, arg1) {
    __widl_f_set_hidden_HTMLElement_target.call(getObject(arg0), arg1 !== 0);
}

export function __widl_instanceof_Window(idx) {
    return getObject(idx) instanceof Window ? 1 : 0;
}

export function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

export function __wbg_newnoargs_6a80f84471205fc8(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

export function __wbg_call_582b20dfcad7fee4(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
}

export function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

