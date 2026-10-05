#![cfg_attr(feature = "ritual_rustdoc_nightly", feature(doc_cfg))]
//! Functions that provide access to C++ operators

/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& stream, const QWebEngineHistory& history)```</span>.
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_web_engine_history(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    history: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QWebEngineHistory>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QWebEngineHistory>>::cast_into(history)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QWebEngineScript& arg2)```</span>.
#[inline(always)]
pub unsafe fn shl_q_debug_q_web_engine_script(
    arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QWebEngineScript>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__2(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QWebEngineScript>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QList<QWebEngineScript>& list)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_list_of_q_web_engine_script(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    list: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QListOfQWebEngineScript>>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__13(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QListOfQWebEngineScript>>::cast_into(
                list,
            )
            .as_raw_ptr(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QWebEngineContextMenuData::MediaFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_media_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_web_engine_context_menu_data::MediaFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__15(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QWebEngineContextMenuData::EditFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_edit_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_web_engine_context_menu_data::EditFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__16(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QFlags<QWebEnginePage::FindFlag>& flags)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_debug_q_flags_find_flag(
    debug: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDebug>>,
    flags: ::qt_core::QFlags<crate::q_web_engine_page::FindFlag>,
) -> ::cpp_core::CppBox<::qt_core::QDebug> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__17(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDebug>>::cast_into(debug)
                .as_raw_ptr(),
            flags.to_int(),
        )
    };
    ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QWebEngineContextMenuData::MediaFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_media_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_web_engine_context_menu_data::MediaFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__18(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QWebEngineContextMenuData::EditFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_edit_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_web_engine_context_menu_data::EditFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__19(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// <p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
///
/// Calls C++ function: <span style='color: green;'>```QDataStream& operator<<(QDataStream& s, QFlags<QWebEnginePage::FindFlag> e)```</span>.
///
/// Warning: no exact match found in C++ documentation. Below is the <a href="http://doc.qt.io/qt-5/qbitarray.html#operator-lt-lt">C++ documentation</a> for <span style='color: green;'>```QDataStream &operator<<(QDataStream &out, const QBitArray &ba)```</span>:<div style='border: 1px solid #5CFF95; background: #D6FFE4; padding: 16px;'><p>Writes bit array <i>ba</i> to stream <i>out</i>.</p>
/// <p><b>See also </b><a href="http://doc.qt.io/qt-5/datastreamformat.html">Format of the QDataStream operators</a>.</p></div>
#[inline(always)]
pub unsafe fn shl_q_data_stream_q_flags_find_flag(
    s: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    e: ::qt_core::QFlags<crate::q_web_engine_page::FindFlag>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__20(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(s)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            e.to_int(),
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}

/// Calls C++ function: <span style='color: green;'>```QDataStream& operator>>(QDataStream& stream, QWebEngineHistory& history)```</span>.
#[inline(always)]
pub unsafe fn shr(
    stream: impl ::cpp_core::CastInto<::cpp_core::Ref<::qt_core::QDataStream>>,
    history: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::QWebEngineHistory>>,
) -> ::cpp_core::Ref<::qt_core::QDataStream> {
    let ffi_result = {
        crate::__ffi::ctr_qt_web_engine_widgets_ffi_operator__1(
            ::cpp_core::CastInto::<::cpp_core::Ref<::qt_core::QDataStream>>::cast_into(stream)
                .as_raw_ptr() as *mut ::qt_core::QDataStream,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::QWebEngineHistory>>::cast_into(history)
                .as_raw_ptr() as *mut crate::QWebEngineHistory,
        )
    };
    ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
}
